use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Index, Range};

use itertools::Itertools;
use petgraph::algo::dominators::Dominators;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::Graph;

use ppc750cl::formatter::FormattedIns;
use ppc750cl::{Ins, Opcode};

use crate::slices::{BasicSlices, CodeIdx};

#[derive(Default)]
pub struct BasicBlock<'a> {
    pub range: Range<CodeIdx>,
    pub code: &'a [Ins],
    pub data_refs: HashMap<CodeIdx, u32>,
}

impl<'a> PartialEq for BasicBlock<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl<'a> Eq for BasicBlock<'a> {}

impl<'a> Hash for BasicBlock<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.range.hash(state)
    }
}

impl<'a> BasicBlock<'a> {
    pub fn from_code_slice(range: Range<CodeIdx>, complete_code: &'a [Ins]) -> BasicBlock {
        let start_idx = complete_code.first().unwrap().addr / 4;
        assert!(start_idx <= range.start);
        let offset = (range.start - start_idx) as usize;
        let code = &complete_code[offset..(offset + (range.len() as usize))];
        BasicBlock {
            range,
            code,
            data_refs: Self::detect_data_refs(code),
        }
    }

    /// Very simple algorithm to detect data references.
    fn detect_data_refs(code: &[Ins]) -> HashMap<CodeIdx, u32> {
        let mut defs = HashMap::<u8, u16>::new();
        let mut data_refs = HashMap::<CodeIdx, u32>::new();
        for ins in code {
            match ins.op {
                Opcode::Addis => {
                    if ins.field_rA() == 0 {
                        // lis
                        defs.insert(ins.field_rD() as u8, ins.field_uimm() as u16);
                    } else {
                        defs.remove(&(ins.field_rD() as u8));
                    }
                }
                Opcode::Addi => {
                    if let Some(hi) = defs.get(&(ins.field_rA() as u8)) {
                        data_refs.insert(
                            ins.addr / 4,
                            ((*hi as u32) << 16) + (ins.field_uimm() as u32),
                        );
                    }
                    defs.remove(&(ins.field_rD() as u8));
                }
                _ => (),
            }
        }
        data_refs
    }
}

impl<'a> Display for BasicBlock<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>#8x}", self.range.start * 4)
    }
}

impl<'a> Debug for BasicBlock<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "// {:0>#8x}..{:0>#8x}",
            self.range.start * 4,
            self.range.end * 4
        )?;
        for ins in self.code {
            writeln!(f, "{}", FormattedIns(ins.clone()))?;
            if let Some(addr) = self.data_refs.get(&(ins.addr / 4)) {
                writeln!(f, "  ref: {:0>#8x}", addr)?;
            }
        }
        Ok(())
    }
}

/// A control-flow graph of a function.
pub struct FlowGraph<'a> {
    pub graph: Graph<BasicBlock<'a>, ()>,
    pub root_idx: NodeIndex,
}

impl<'a> FlowGraph<'a> {
    /// Creates a control-flow graph from basic slices.
    pub fn from_basic_slices(slices: &BasicSlices, code: &'a [Ins]) -> Self {
        assert!(!code.is_empty(), "Attempt to create empty flow graph");
        // Walk set cuts and create basic blocks.
        let mut graph = Graph::new();
        let mut node_by_addr = BTreeMap::<u32, NodeIndex<DefaultIx>>::new();
        let mut block_start: CodeIdx = code[0].addr / 4;
        for cut in &slices.cuts {
            if *cut > block_start {
                node_by_addr.insert(
                    block_start,
                    graph.add_node(BasicBlock::from_code_slice(block_start..*cut, code)),
                );
            }
            block_start = *cut;
        }
        // Last block.
        let func_end: CodeIdx = (code.last().unwrap().addr / 4) + 1;
        if func_end > block_start {
            node_by_addr.insert(
                block_start,
                graph.add_node(BasicBlock::from_code_slice(block_start..func_end, code)),
            );
        }
        // Walk set of branches and connect graph.
        for branch in &slices.branches {
            let src_node_idx = match node_by_addr.range(..branch.0 + 1).last() {
                None => continue,
                Some(idx) => *idx.1,
            };
            debug_assert!(graph[src_node_idx].range.contains(&branch.0));
            let dst_node_idx = match node_by_addr.range(..branch.1 + 1).last() {
                None => continue,
                Some(idx) => *idx.1,
            };
            debug_assert!(graph[dst_node_idx].range.contains(&branch.1));
            graph.add_edge(src_node_idx, dst_node_idx, ());
        }
        // Walk blocks and re-connect nodes that were split off.
        for (src_node_idx, dst_node_idx) in node_by_addr.values().tuple_windows::<(_, _)>() {
            // Get pairs of two blocks as a sliding window.
            let src_block: &BasicBlock = &graph[*src_node_idx];
            let dst_block: &BasicBlock = &graph[*dst_node_idx];
            assert_eq!(src_block.range.end, dst_block.range.start);
            // Get last instruction of left block.
            // Unless it's an unconditional branch, we can connect the blocks.
            let last_ins = &src_block.code.last().unwrap();
            if last_ins.code == 0x4E800020
                || (last_ins.op == Opcode::B && last_ins.field_BO() == 0b10100)
            {
                continue;
            }
            // Execution can continue past the last instruction of a block,
            // so re-connect two blocks that were split off.
            if !graph.contains_edge(*src_node_idx, *dst_node_idx) {
                graph.add_edge(*src_node_idx, *dst_node_idx, ());
            }
        }
        Self {
            graph,
            root_idx: *node_by_addr.index(node_by_addr.keys().next().unwrap()),
        }
    }

    pub fn dominators(&self) -> Dominators<NodeIndex> {
        petgraph::algo::dominators::simple_fast(&self.graph, self.root_idx)
    }
}
