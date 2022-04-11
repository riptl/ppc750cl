use std::collections::{BTreeSet, HashSet};

use ppc750cl::{Ins, Opcode};

/// The instruction address divided by four.
pub type CodeIdx = u32;

/// Basic block cuts, forward edges, and function addresses.
#[derive(Default)]
pub struct BasicSlices {
    /// The indexes separating instructions into basic blocks.
    /// Used to create a list of consecutive basic blocks.
    /// No code follows the last cut.
    pub cuts: BTreeSet<CodeIdx>,
    /// The possible branches from one instruction to another.
    /// Used to link together basic blocks into a directed graph.
    pub branches: HashSet<(CodeIdx, CodeIdx)>,
    /// The indexes marking where functions begin.
    pub funcs: BTreeSet<CodeIdx>,
}

impl BasicSlices {
    /// Computes basic slices from instructions.
    pub fn from_code(mut code: &[Ins]) -> Self {
        let mut this = Self::default();
        while !code.is_empty() {
            // Skip over zero bytes.
            if code[0].code == 0 {
                code = &code[1..];
                continue;
            }
            // Analyze next function.
            this.funcs.insert(code[0].addr / 4);
            code = this.next_function(code);
        }
        this
    }

    fn next_function<'a>(&mut self, code: &'a [Ins]) -> &'a [Ins] {
        let start_addr = code[0].addr;

        // Try to find function prologue.

        // Walk all local control flow instructions.
        // This means all branch instructions that do not save the link register.
        //
        // If the link register is saved, assume there is a back-edge to the next instruction,
        // i.e. irrelevant to local control flow.
        let mut pos = 0usize;
        for (_pos, ins) in code.iter().enumerate() {
            pos = _pos;
            let cur_index = ins.addr / 4;
            if ins.code == 0 {
                self.cuts.insert(cur_index);
                break; // assume zeros are padding between funcs
            }
            if !ins.is_branch() {
                continue;
            }
            if ins.field_LK() {
                self.funcs.insert(ins.branch_dest().unwrap());
                continue;
            }
            // TODO scan for "mtlr" or "mtctr" instructions.
            if ins.is_blr() {
                // There's a possibility that branch can be taken.
                // Branch destinations are always the first instruction of a block.
                // Thus, we also found the end of another block.
                let new_index = ins.branch_dest().unwrap() / 4;
                self.cuts.insert(new_index);
                self.branches.insert((cur_index, new_index));
                continue;
            }
            if ins.op == Opcode::Bcctr || ins.op == Opcode::Bclr {
                // Indirect branch without LR save.
                if !ins.field_LK() {
                    // panic!("local indirect branches unsupported: {:#08x}", ins.addr);
                    // sadbo
                    self.cuts.insert(cur_index);
                }
            } else {
                // Direct branch without LR save.
                if ins.branch_dest().unwrap() < start_addr {
                    // Assume branch without LR save before function start is a tail call.
                    continue;
                }
                self.cuts.insert(cur_index + 1);
                if ins.is_conditional_branch() {
                    self.branches.insert((cur_index, cur_index + 1));
                }
            }
        }

        &code[pos..]
    }
}

/*
struct StackFrame {
    size: u32,
}

impl StackFrame {
    fn from_prologue(code: &[Ins]) -> Option<StackFrame> {
        let mut code_iter = code.iter();
        // stwu r1, -0x???(r1)
        let update_sp = code_iter.next()?;
        if !Self::is_alloc_stack(update_sp) {
            return None;
        }
        // mflr r0
        let backup_lr = code_iter.next()?;
        if !Self::is_load_lr(backup_lr) {
            return None;
        }

        None
    }

    fn is_alloc_stack(ins: &Ins) -> bool {
        ins.op == Opcode::Stwu
            && ins.field_rD() == 1
            && ins.field_rA() == 1
            && ins.field_offset() < 0
    }

    fn is_free_stack(ins: &Ins) -> bool {
        ins.op == Opcode::Addi && ins.field_rD() == 1 && ins.field_rA() == 1 && ins.field_simm() > 0
    }

    fn is_load_lr(ins: &Ins) -> bool {
        ins.op == Opcode::Mfspr && ins.field_rD() == 0 && ins.field_spr() == 8
    }

    fn is_store_lr(ins: &Ins) -> bool {
        ins.op == Opcode::Mtspr && ins.field_rS() == 0 && ins.field_spr() == 8
    }

    fn is_backup_lr(ins: &Ins) -> bool {
        ins.op == Opcode::Stw
            && ins.field_rS() == 0
            && ins.field_rA() == 1
            && ins.field_offset() > 0
    }

    fn is_restore_lr(ins: &Ins) -> bool {
        ins.op == Opcode::Lwz
            && ins.field_rD() == 0
            && ins.field_rA() == 1
            && ins.field_offset() > 0
    }
}
*/
