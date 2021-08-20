use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

use clap::clap_app;
use petgraph::dot::{Config as DotConfig, Dot};
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::Graph;

use itertools::Itertools;
use ppc750cl::{disasm_iter, Ins, Opcode};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Control flow graph analysis for PowerPC 750CL")
        (@arg ADDR: --addr +required +takes_value "Address")
        (@arg INPUT: +required "Binary input file")
    )
    .get_matches();

    let addr = matches.value_of("ADDR").unwrap();
    let addr: u32 = ::parse_int::parse(addr).expect("Invalid address flag");

    let file_path = matches.value_of("INPUT").unwrap();
    let bytes = std::fs::read(file_path).expect("Failed to read file");

    // Create control flow graph.
    let ins_list: Vec<Ins> = disasm_iter(&bytes, addr).collect();
    let basic_slices = BasicSlices::from_code(&ins_list);
    let graph = basic_slices.to_control_flow_graph(&ins_list);

    // Output graphviz.
    let graphviz = Dot::with_config(
        &graph,
        &[DotConfig::EdgeNoLabel, DotConfig::GraphContentOnly],
    );
    println!(
        concat!(
            "digraph func {{\n",
            "node [shape=record fontname=Arial];\n",
            "{:?}\n",
            "}}"
        ),
        graphviz
    );
}

/// The instruction address divided by four.
type CodeIdx = u32;

struct BasicSlices {
    /// The indexes separating instructions into basic blocks.
    /// Used to create a list of consecutive basic blocks.
    cuts: BTreeSet<CodeIdx>,
    /// The possible branches from one instruction to another.
    /// Used to link together basic blocks into a directed graph.
    branches: HashSet<(CodeIdx, CodeIdx)>,
}

impl BasicSlices {
    /// Computes basic slices from instructions.
    fn from_code(code: &[Ins]) -> Self {
        let mut cuts = BTreeSet::<CodeIdx>::new();
        let mut branches = HashSet::<(CodeIdx, CodeIdx)>::new();
        for ins in code {
            let cur_index = ins.addr / 4;
            let is_control_flow_ins = match ins.op {
                // Direct branches are control flow instructions if they don't save the link register.
                // If they do, we encountered a function call.
                Opcode::B | Opcode::Bc => ins.lk() == 0,
                // Switch table
                Opcode::Bcctr => panic!("jump tables not supported yet"),
                _ => false,
            };
            if !is_control_flow_ins {
                continue;
            }
            // We encountered some kind of control flow instruction.
            if ins.code != Opcode::BLR {
                // There's a possibility that branch can be taken.
                // Branch destinations are always the first instruction of a block.
                // Thus, we also found the end of another block.
                let new_index = ins.branch_dest().unwrap() / 4;
                cuts.insert(new_index);
                branches.insert((cur_index, new_index));
            }
            if is_conditional_branch(ins) {
                // There's a possibility that branch is not taken.
                // End block anyways.
                cuts.insert(cur_index + 1);
                branches.insert((cur_index, cur_index + 1));
            }
        }
        Self { cuts, branches }
    }

    /// Creates a control-flow graph.
    fn to_control_flow_graph<'a>(&self, code: &'a [Ins]) -> Graph<BasicBlock<'a>, ()> {
        if code.is_empty() {
            return Graph::new();
        }
        // Walk set cuts and create basic blocks.
        let mut graph = Graph::new();
        let mut node_by_addr = BTreeMap::<u32, NodeIndex<DefaultIx>>::new();
        let mut block_start: CodeIdx = code[0].addr / 4;
        for cut in &self.cuts {
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
        for branch in &self.branches {
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
            let last_ins = src_block.code.last().unwrap();
            if last_ins.code == Opcode::BLR
                || (last_ins.op == Opcode::B && last_ins.bo() == 0b10100)
            {
                continue;
            }
            // Execution can continue past the last instruction of a block,
            // so re-connect two blocks that were split off.
            if !graph.contains_edge(*src_node_idx, *dst_node_idx) {
                graph.add_edge(*src_node_idx, *dst_node_idx, ());
            }
        }
        graph
    }
}

fn is_conditional_branch(ins: &Ins) -> bool {
    match ins.op {
        Opcode::Bc | Opcode::Bcctr | Opcode::Bclr => (),
        _ => return false,
    };
    // Check whether bits "branch always".
    ins.bo() & 0b10100 != 0b10100
}

struct BasicBlock<'a> {
    range: Range<CodeIdx>,
    code: &'a [Ins],
}

impl<'a> BasicBlock<'a> {
    fn from_code_slice(range: Range<CodeIdx>, complete_code: &'a [Ins]) -> BasicBlock {
        let start_idx = complete_code.first().unwrap().addr / 4;
        assert!(start_idx <= range.start);
        let offset = (range.start - start_idx) as usize;
        let code = &complete_code[offset..(offset + (range.len() as usize))];
        BasicBlock { range, code }
    }
}

impl<'a> Display for BasicBlock<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:0>#8x}..{:0>#8x}",
            self.range.start * 4,
            self.range.end * 4
        )
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
            writeln!(f, "{}", ins.to_string())?;
        }
        Ok(())
    }
}
