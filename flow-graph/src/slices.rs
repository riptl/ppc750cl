use std::collections::{BTreeSet, HashSet};

use ppc750cl::{Ins, Opcode};

/// The instruction address divided by four.
pub type CodeIdx = u32;

pub struct BasicSlices {
    /// The indexes separating instructions into basic blocks.
    /// Used to create a list of consecutive basic blocks.
    pub cuts: BTreeSet<CodeIdx>,
    /// The possible branches from one instruction to another.
    /// Used to link together basic blocks into a directed graph.
    pub branches: HashSet<(CodeIdx, CodeIdx)>,
}

impl BasicSlices {
    /// Computes basic slices from instructions.
    pub fn from_code(code: &[Ins]) -> Self {
        let mut cuts = BTreeSet::<CodeIdx>::new();
        let mut branches = HashSet::<(CodeIdx, CodeIdx)>::new();
        for ins in code {
            let cur_index = ins.addr / 4;
            let is_control_flow_ins = match ins.op {
                // Direct branches are control flow instructions if they don't save the link register.
                // If they do, we encountered a function call.
                Opcode::B | Opcode::Bc => !ins.field_LK(),
                // Switch table
                Opcode::Bcctr => panic!("jump tables not supported yet"),
                _ => false,
            };
            if !is_control_flow_ins {
                continue;
            }
            // We encountered some kind of control flow instruction.
            if ins.field_BO() == 20 && ins.field_BI() == 0 {
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
}

fn is_conditional_branch(ins: &Ins) -> bool {
    match ins.op {
        Opcode::Bc | Opcode::Bcctr | Opcode::Bclr => (),
        _ => return false,
    };
    // Check whether bits "branch always".
    ins.field_BO() & 0b10100 != 0b10100
}
