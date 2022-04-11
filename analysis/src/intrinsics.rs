use ppc750cl::{Ins, Opcode};

/// Points to various compiler intrinsics.
#[derive(Default)]
pub struct IntrinsicsLocs {
    pub save_fpr_sled: Option<u32>,
    pub rest_fpr_sled: Option<u32>,
    pub save_gpr_sled: Option<u32>,
    pub rest_gpr_sled: Option<u32>,
}

impl IntrinsicsLocs {
    pub fn from_code(code: &[Ins]) -> Self {
        let mut this = Self::default();
        this.match_code(code);
        this
    }

    fn match_code(&mut self, mut code: &[Ins]) {
        while !code.is_empty() {
            if Self::match_sled(code, Opcode::Stfd, -0x90, 8) {
                self.save_fpr_sled = Some(code[0].addr)
            } else if Self::match_sled(code, Opcode::Lfd, -0x90, 8) {
                self.rest_fpr_sled = Some(code[0].addr)
            } else if Self::match_sled(code, Opcode::Stw, -0x48, 4) {
                self.save_gpr_sled = Some(code[0].addr)
            } else if Self::match_sled(code, Opcode::Lwz, -0x48, 4) {
                self.rest_gpr_sled = Some(code[0].addr)
            }
            code = &code[1..];
        }
    }

    fn match_sled(code: &[Ins], op: Opcode, mut offset: isize, stride: isize) -> bool {
        let mut code_iter = code.iter();
        for reg in 14..=31 {
            let ins = match code_iter.next() {
                None => return false,
                Some(v) => v,
            };
            if !Self::is_sled_ins(ins, op, reg, offset) {
                return false;
            }
            offset += stride;
        }
        // blr
        code_iter.next().map(|i| i.is_blr()).unwrap_or(false)
    }

    fn is_sled_ins(ins: &Ins, op: Opcode, reg: usize, offset: isize) -> bool {
        ins.op == op
            && ins.field_rD() == reg
            && ins.field_offset() == offset
            && ins.field_rA() == 11
    }
}
