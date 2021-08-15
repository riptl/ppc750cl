#![allow(clippy::unusual_byte_groupings)]

use std::io::Write;
use std::ops::Range;

use num_traits::AsPrimitive;

use ppc750cl_macros::write_asm;

#[macro_use]
mod macros;
pub mod formatter;
mod isa;

pub use crate::formatter::AsmFormatter;
use crate::formatter::SimpleFormatter;
pub use crate::isa::Opcode;

#[derive(Default, Clone)]
pub struct Ins {
    pub code: u32,
    pub addr: u32,
    pub op: Opcode,
}

#[inline(always)]
fn bit(x: u32, idx: usize) -> u8 {
    ((x >> (32 - idx - 1)) & 1) as u8
}

#[inline(always)]
fn bits<F>(x: u32, range: Range<usize>) -> F
where
    F: 'static + std::marker::Copy,
    u32: AsPrimitive<F>,
{
    let masked: u32 = (x >> (32 - range.end)) & ((1 << range.len()) - 1);
    masked.as_()
}

macro_rules! ins_bit {
    ($func:ident, $idx:expr) => {
        fn $func(&self) -> u8 {
            bit(self.code, $idx)
        }
    };
}

macro_rules! ins_field {
    ($func:ident, $return_type:tt, $range:expr) => {
        fn $func(&self) -> $return_type {
            debug_assert!(
                ($range).len() / 8 <= (std::mem::size_of::<$return_type>()),
                "{:?} does not fit in {}",
                $range,
                stringify!($return_type)
            );
            bits(self.code, $range)
        }
    };
}

impl Ins {
    pub fn new(code: u32, addr: u32) -> Self {
        let op = Opcode::from_code(code);
        Self { code, addr, op }
    }

    //ins_bit!(w, 21);
    ins_bit!(rc, 31);
    ins_bit!(aa, 30);
    ins_bit!(lk, 31);
    ins_bit!(l, 10);
    ins_bit!(oe, 21);
    ins_bit!(w, 16);

    // Registers
    ins_field!(s, u8, 6..11);
    ins_field!(d, u8, 6..11);
    ins_field!(a, u8, 11..16);
    ins_field!(b, u8, 16..21);
    ins_field!(c, u8, 21..26);
    // Condition registers
    ins_field!(crb_d, u8, 6..11);
    ins_field!(crb_a, u8, 11..16);
    ins_field!(crb_b, u8, 16..21);

    ins_field!(crm, u8, 12..20);
    ins_field!(sr, u8, 12..16);
    fn spr(&self) -> u16 {
        bits::<u16>(self.code, 11..16) | (bits::<u16>(self.code, 16..21) << 5)
    }
    ins_field!(fm, u16, 7..15);
    ins_field!(crf_d, u8, 6..9);
    ins_field!(crf_s, u8, 11..14);
    ins_field!(simm, i16, 16..32);
    ins_field!(uimm, u16, 16..32);
    ins_field!(bo, u8, 6..11);
    ins_field!(bi, u8, 11..16);
    ins_field!(sh, u8, 16..21);
    ins_field!(mb, u8, 21..26);
    ins_field!(me, u8, 26..31);
    //ins_field!(bd, u16, 16..30);
    ins_field!(li, u32, 6..30);
    ins_field!(to, u8, 6..11);
    // Paired-single fields.
    ins_field!(ps_l, u8, 17..20);
    ins_field!(ps_d, u16, 20..32);

    fn write_asm_form_reg123<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_reg123_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_reg123_oe_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_oe(self.oe())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_reg12_simm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_simm(self.simm())?;
        Ok(())
    }

    fn write_asm_form_reg12_uimm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_uimm(self.uimm())?;
        Ok(())
    }

    fn write_asm_form_reg12_offset<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_offset_open(self.simm())?;
        out.write_gpr(self.a())?;
        out.write_offset_close()?;
        Ok(())
    }

    fn write_asm_form_fr1_reg2_offset<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_offset_open(self.simm())?;
        out.write_gpr(self.a())?;
        out.write_offset_close()?;
        Ok(())
    }

    fn write_asm_form_fr1_reg23<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_mtfsf<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_fm(self.fm())?;
        out.write_operand_separator()?;
        out.write_fpr(self.b())?;
        Ok(())
    }

    fn write_asm_mtfsfi<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{}", bits::<u8>(self.code, 16..20))?;
        Ok(())
    }

    fn write_asm_form_reg1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        Ok(())
    }

    fn write_asm_form_reg12_oe_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_oe(self.oe())?;
        out.write_lk(self.lk())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        Ok(())
    }

    fn write_asm_form_reg13<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_reg21_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.s())?;
        Ok(())
    }

    fn write_asm_form_fr1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        Ok(())
    }

    fn write_asm_form_fr13<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_fpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_fr123<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_fpr(self.a())?;
        out.write_operand_separator()?;
        out.write_fpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_fr1243<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_fpr(self.a())?;
        out.write_operand_separator()?;
        out.write_fpr(self.c())?;
        out.write_operand_separator()?;
        out.write_fpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_fr124<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_fpr(self.a())?;
        out.write_operand_separator()?;
        out.write_fpr(self.c())?;
        Ok(())
    }

    fn write_asm_form_condreg1_fr23<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        out.write_operand_separator()?;
        out.write_fpr(self.a())?;
        out.write_operand_separator()?;
        out.write_fpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_condreg1_fr13_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        out.write_operand_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_fpr(self.b())?;
        Ok(())
    }

    fn write_asm_b<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_lk(self.lk())?;
        out.write_aa(self.aa())?;
        out.write_opcode_separator()?;
        out.write_branch_target(self.li(), self.addr)?;
        Ok(())
    }

    fn write_asm_bc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_lk(self.lk())?;
        out.write_aa(self.aa())?;
        out.write_opcode_separator()?;
        write!(out.writer(), "{}", self.bo())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{}", self.bi())?;
        out.write_operand_separator()?;
        out.write_branch_target(self.li(), self.addr)?;
        Ok(())
    }

    fn write_asm_branch_cond_to_reg<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name = match self.op {
            Opcode::Bcctr => match self.lk() != 0 {
                false => "bcctr",
                true => "bcctrl",
            },
            Opcode::Bclr => match self.lk() != 0 {
                false => match (self.bo(), self.bi()) {
                    (0b01100, 0b00000) => return write!(out.writer(), "bltlr"),
                    (0b00100, 0b01010) => return write!(out.writer(), "bnelr cr2"),
                    (0b10000, 0b00000) => return write!(out.writer(), "bdnzlr"),
                    (0b10100, 0b00000) => return write!(out.writer(), "blr"),
                    _ => "bclr",
                },
                true => "bclrl",
            },
            _ => disasm_unreachable!(self.code),
        };
        out.write_mnemonic(name)?;
        out.write_opcode_separator()?;
        write!(out.writer(), "{}", self.bo())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{}", self.bi())?;
        Ok(())
    }

    fn write_asm_cmp<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{}", self.l())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_cmp_simm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{}", self.l())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_simm(self.simm())?;
        Ok(())
    }

    fn write_asm_cmp_uimm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{}", self.l())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_uimm(self.uimm())?;
        Ok(())
    }

    fn write_asm_form_condreg1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name = match self.op {
            Opcode::Mcrxr => "mcrxr",
            Opcode::Mtfsb0 => match self.rc() != 0 {
                false => "mtfsb0",
                true => "mtfsb0.",
            },
            Opcode::Mtfsb1 => match self.rc() != 0 {
                false => "mtfsb1",
                true => "mtfsb1.",
            },
            _ => disasm_unreachable!(self.code),
        };
        out.write_mnemonic(name)?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        Ok(())
    }

    fn write_asm_form_condreg12<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name = match self.op {
            Opcode::Mcrf => "mcrf",
            Opcode::Mcrfs => "mcrfs",
            _ => disasm_unreachable!(self.code),
        };
        out.write_mnemonic(name)?;
        out.write_opcode_separator()?;
        out.write_cr(self.crf_d())?;
        out.write_operand_separator()?;
        out.write_cr(self.crf_s())?;
        Ok(())
    }

    fn write_asm_form_condreg123<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_cr(self.crb_d())?;
        out.write_operand_separator()?;
        out.write_cr(self.crb_a())?;
        out.write_operand_separator()?;
        out.write_cr(self.crb_b())?;
        Ok(())
    }

    fn write_asm_form_reg23<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_form_reg213<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name = match self.op {
            Opcode::Eqv => "eqv",
            Opcode::Nand => "nand",
            Opcode::Nor => "nor",
            Opcode::Or => {
                if self.s() == self.b() {
                    return write!(out.writer(), "mr r{}, r{}", self.a(), self.s());
                } else {
                    "or"
                }
            }
            Opcode::Orc => "orc",
            Opcode::Slw => "slw",
            Opcode::Sraw => "sraw",
            Opcode::Srw => "srw",
            _ => disasm_unreachable!(self.code),
        };
        out.write_mnemonic(name)?;
        out.write_rc(self.rc())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.s())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        Ok(())
    }

    fn write_asm_rlw_imm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name_prefix = if self.rc() != 0 { "." } else { "" };
        write!(
            out.writer(),
            "{}{} r{}, r{}, {}, {}, {}",
            self.op.mnemonic(),
            name_prefix,
            self.a(),
            self.s(),
            self.sh(),
            self.mb(),
            self.me()
        )
    }

    fn write_asm_rlw_reg<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        assert_eq!(self.op, Opcode::Rlwnm);
        let name_prefix = if self.rc() != 0 { "." } else { "" };
        write!(
            out.writer(),
            "rlwnm{} r{}, r{}, r{}, {}, {}",
            name_prefix,
            self.a(),
            self.s(),
            self.b(),
            self.mb(),
            self.me()
        )
    }

    fn write_asm_form_reg12_nb<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write!(
            out.writer(),
            "{} r{}, r{}, {}",
            self.op.mnemonic(),
            self.d(),
            self.a(),
            self.b()
        )
    }

    fn write_asm_form_reg1_spr<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name = match self.op {
            Opcode::Mfspr => match self.spr() {
                1 => return write!(out.writer(), "mfxer r{}", self.s()),
                8 => return write!(out.writer(), "mflr r{}", self.s()),
                9 => return write!(out.writer(), "mfctr r{}", self.s()),
                _ => "mfspr",
            },
            Opcode::Mftb => "mftb",
            _ => disasm_unreachable!(self.code),
        };
        write!(out.writer(), "{} r{}, {}", name, self.d(), self.spr())
    }

    fn write_asm_form_spr_reg1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name = match self.op {
            Opcode::Mtspr => match self.spr() {
                1 => return write!(out.writer(), "mtxer r{}", self.s()),
                8 => return write!(out.writer(), "mtlr r{}", self.s()),
                9 => return write!(out.writer(), "mtctr r{}", self.s()),
                _ => "mtspr",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out.writer(), "{} {}, r{}", name, self.spr(), self.s())
    }

    fn write_asm_form_reg1_sr<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        out.write_sr(self.sr())?;
        Ok(())
    }

    fn write_asm_form_sr_reg1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_sr(self.sr())?;
        out.write_operand_separator()?;
        out.write_gpr(self.s())?;
        Ok(())
    }

    fn write_asm_mtcrf<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write!(
            out.writer(),
            "{} {}, r{}",
            self.op.mnemonic(),
            self.crm(),
            self.s()
        )
    }

    fn write_asm_srawi<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        write!(
            out.writer(),
            "{}{} r{}, r{}, {}",
            self.op.mnemonic(),
            name_suffix,
            self.s(),
            self.a(),
            self.sh()
        )
    }

    fn write_asm_tw<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write!(
            out.writer(),
            "{} {}, r{}, r{}",
            self.op.mnemonic(),
            self.to(),
            self.a(),
            self.b()
        )
    }

    fn write_asm_twi<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write!(
            out.writer(),
            "{} {}, r{}, {}",
            self.op.mnemonic(),
            self.to(),
            self.a(),
            self.simm()
        )
    }

    fn write_asm_psq<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc, oe) -> mnemonic;
            (d) -> fpr;
            (ps_d) -> offset_unsigned;
            (a) -> gpr;
            (w) -> mode;
            (ps_l) -> qr;
        });
        Ok(())
    }

    fn write_asm_psq_x<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_fpr(self.d())?;
        out.write_operand_separator()?;
        out.write_gpr(self.a())?;
        out.write_operand_separator()?;
        out.write_gpr(self.b())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{}", self.w())?;
        out.write_operand_separator()?;
        out.write_qr(self.ps_l())?;
        Ok(())
    }

    pub fn write_string<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        match self.op {
            Opcode::Illegal => write!(out.writer(), "<illegal>"),

            // Standalone instructions
            Opcode::Eieio
            | Opcode::Isync
            | Opcode::Rfi
            | Opcode::Sc
            | Opcode::Sync
            | Opcode::Tlbsync => out.write_mnemonic(self.op.mnemonic()),

            // General purpose register only
            Opcode::Mfcr | Opcode::Mfmsr | Opcode::Mtmsr | Opcode::Tlbie => {
                self.write_asm_form_reg1(out)
            }
            Opcode::Addme | Opcode::Addze | Opcode::Neg | Opcode::Subfme | Opcode::Subfze => {
                self.write_asm_form_reg12_oe_rc(out)
            }
            Opcode::Mfsrin | Opcode::Mtsrin => self.write_asm_form_reg13(out),
            Opcode::Cntlzw | Opcode::Extsb | Opcode::Extsh => self.write_asm_form_reg21_rc(out),
            Opcode::Dcbf
            | Opcode::Dcbi
            | Opcode::Dcbst
            | Opcode::Dcbt
            | Opcode::Dcbtst
            | Opcode::Dcbz
            | Opcode::DcbzL
            | Opcode::Icbi => self.write_asm_form_reg23(out),
            Opcode::Eciwx
            | Opcode::Ecowx
            | Opcode::Lhaux
            | Opcode::Lhax
            | Opcode::Lbzux
            | Opcode::Lbzx
            | Opcode::Lhbrx
            | Opcode::Lhzux
            | Opcode::Lhzx
            | Opcode::Lswx
            | Opcode::Lwarx
            | Opcode::Lwbrx
            | Opcode::Lwzux
            | Opcode::Lwzx
            | Opcode::Stbx
            | Opcode::Stbux
            | Opcode::Sthbrx
            | Opcode::Sthx
            | Opcode::Sthux
            | Opcode::Stswx
            | Opcode::Stwbrx
            | Opcode::Stwcx_
            | Opcode::Stwx
            | Opcode::Stwux => self.write_asm_form_reg123(out),
            Opcode::And | Opcode::Andc | Opcode::Mulhw | Opcode::Mulhwu | Opcode::Xor => {
                self.write_asm_form_reg123_rc(out)
            }
            Opcode::Add
            | Opcode::Addc
            | Opcode::Adde
            | Opcode::Divw
            | Opcode::Divwu
            | Opcode::Mullw
            | Opcode::Subf
            | Opcode::Subfc
            | Opcode::Subfe => self.write_asm_form_reg123_oe_rc(out),
            Opcode::Eqv
            | Opcode::Nand
            | Opcode::Nor
            | Opcode::Or
            | Opcode::Orc
            | Opcode::Slw
            | Opcode::Sraw
            | Opcode::Srw => self.write_asm_form_reg213(out),

            // General purpose shifts
            Opcode::Rlwimi | Opcode::Rlwinm => self.write_asm_rlw_imm(out),
            Opcode::Rlwnm => self.write_asm_rlw_reg(out),

            // General purpose register misc
            Opcode::Addi
            | Opcode::Addic
            | Opcode::Addic_
            | Opcode::Addis
            | Opcode::Mulli
            | Opcode::Subfic => self.write_asm_form_reg12_simm(out),
            Opcode::Andi_
            | Opcode::Andis_
            | Opcode::Ori
            | Opcode::Oris
            | Opcode::Xori
            | Opcode::Xoris => self.write_asm_form_reg12_uimm(out),
            Opcode::Lbz
            | Opcode::Lbzu
            | Opcode::Lha
            | Opcode::Lhau
            | Opcode::Lhz
            | Opcode::Lhzu
            | Opcode::Lmw
            | Opcode::Lwz
            | Opcode::Lwzu
            | Opcode::Stb
            | Opcode::Stbu
            | Opcode::Sth
            | Opcode::Sthu
            | Opcode::Stmw
            | Opcode::Stw
            | Opcode::Stwu => self.write_asm_form_reg12_offset(out),
            Opcode::Lswi | Opcode::Stswi => self.write_asm_form_reg12_nb(out),
            Opcode::Mfspr | Opcode::Mftb => self.write_asm_form_reg1_spr(out),
            Opcode::Mtspr => self.write_asm_form_spr_reg1(out),
            Opcode::Mfsr => self.write_asm_form_reg1_sr(out),
            Opcode::Mtsr => self.write_asm_form_sr_reg1(out),
            Opcode::Mtcrf => self.write_asm_mtcrf(out),
            Opcode::Srawi => self.write_asm_srawi(out),
            Opcode::Tw => self.write_asm_tw(out),
            Opcode::Twi => self.write_asm_twi(out),

            // Branch instructions
            Opcode::B => self.write_asm_b(out),
            Opcode::Bc => self.write_asm_bc(out),
            Opcode::Bcctr | Opcode::Bclr => self.write_asm_branch_cond_to_reg(out),

            // Compare instructions
            Opcode::Cmp | Opcode::Cmpl => self.write_asm_cmp(out),
            Opcode::Cmpi => self.write_asm_cmp_simm(out),
            Opcode::Cmpli => self.write_asm_cmp_uimm(out),

            // Floating point register only instructions
            Opcode::Mffs => self.write_asm_form_fr1(out),
            Opcode::Fabs
            | Opcode::Fmr
            | Opcode::Fnabs
            | Opcode::Fneg
            | Opcode::Fres
            | Opcode::Frsp
            | Opcode::Frsqrte
            | Opcode::PsAbs
            | Opcode::PsMr
            | Opcode::PsNabs
            | Opcode::PsNeg
            | Opcode::PsRes
            | Opcode::PsRsqrte => self.write_asm_form_fr13(out),
            Opcode::Fadd
            | Opcode::Fadds
            | Opcode::Fdiv
            | Opcode::Fdivs
            | Opcode::Fsub
            | Opcode::Fsubs
            | Opcode::PsAdd
            | Opcode::PsDiv
            | Opcode::PsMerge00
            | Opcode::PsMerge01
            | Opcode::PsMerge10
            | Opcode::PsMerge11
            | Opcode::PsSub => self.write_asm_form_fr123(out),
            Opcode::Fmul | Opcode::Fmuls | Opcode::PsMul | Opcode::PsMuls0 | Opcode::PsMuls1 => {
                self.write_asm_form_fr124(out)
            }
            Opcode::Fmadd
            | Opcode::Fmadds
            | Opcode::Fmsub
            | Opcode::Fmsubs
            | Opcode::Fnmadd
            | Opcode::Fnmadds
            | Opcode::Fnmsub
            | Opcode::Fnmsubs
            | Opcode::Fsel
            | Opcode::PsMadd
            | Opcode::PsMadds0
            | Opcode::PsMadds1
            | Opcode::PsMsub
            | Opcode::PsNmadd
            | Opcode::PsNmsub
            | Opcode::PsSel
            | Opcode::PsSum0
            | Opcode::PsSum1 => self.write_asm_form_fr1243(out),

            // Floating point register misc instructions
            Opcode::Fctiw | Opcode::Fctiwz => self.write_asm_form_condreg1_fr13_rc(out),
            Opcode::Fcmpo
            | Opcode::Fcmpu
            | Opcode::PsCmpo0
            | Opcode::PsCmpo1
            | Opcode::PsCmpu0
            | Opcode::PsCmpu1 => self.write_asm_form_condreg1_fr23(out),
            Opcode::Lfd
            | Opcode::Lfdu
            | Opcode::Lfs
            | Opcode::Lfsu
            | Opcode::Stfd
            | Opcode::Stfdu
            | Opcode::Stfs
            | Opcode::Stfsu => self.write_asm_form_fr1_reg2_offset(out),
            Opcode::Lfdux
            | Opcode::Lfdx
            | Opcode::Lfsux
            | Opcode::Lfsx
            | Opcode::Stfdux
            | Opcode::Stfdx
            | Opcode::Stfiwx
            | Opcode::Stfsux
            | Opcode::Stfsx => self.write_asm_form_fr1_reg23(out),
            Opcode::Mtfsf => self.write_asm_mtfsf(out),

            // Condition register only
            Opcode::Mcrxr | Opcode::Mtfsb0 | Opcode::Mtfsb1 => self.write_asm_form_condreg1(out),
            Opcode::Mcrf | Opcode::Mcrfs => self.write_asm_form_condreg12(out),
            Opcode::Crand
            | Opcode::Crandc
            | Opcode::Creqv
            | Opcode::Crnand
            | Opcode::Crnor
            | Opcode::Cror
            | Opcode::Crorc
            | Opcode::Crxor => self.write_asm_form_condreg123(out),

            // Condition register misc
            Opcode::Mtfsfi => self.write_asm_mtfsfi(out),

            // Paired-single instructions
            Opcode::PsqL | Opcode::PsqLu | Opcode::PsqSt | Opcode::PsqStu => {
                self.write_asm_psq(out)
            }
            Opcode::PsqLx | Opcode::PsqLux | Opcode::PsqStx | Opcode::PsqStux => {
                self.write_asm_psq_x(out)
            }
        }
    }
}

impl ToString for Ins {
    fn to_string(&self) -> String {
        let buf = Vec::<u8>::new();
        let mut formatter = SimpleFormatter { writer: buf };
        self.write_string(&mut formatter).unwrap();
        unsafe { String::from_utf8_unchecked(formatter.writer) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        assert_eq!(
            bits::<u32>(0b00000101100000000000000000000000u32, 5..9),
            0b1011u32
        );
        assert_eq!(bit(0b00000101100000000000000000000000u32, 5), 1);
    }
    #[test]
    fn test_opcodes() {
        macro_rules! assert_op {
            ($code:expr, $op:expr) => {{
                assert_eq!(Ins::new($code, 0x8000_0000u32).op, $op)
            }};
        }

        // twi
        assert_op!(0b000011_00000_00000_0000000000000000, Opcode::Twi);
        // ps_cmpu0
        assert_op!(0b000100_00000_00000_00000_0000000000_0, Opcode::PsCmpu0);
        assert_op!(0b000100_00000_00000_00000_0000000000_1, Opcode::Illegal);
        assert_op!(0b000100_00001_00000_00000_0000000000_0, Opcode::Illegal);
        // psq_lx
        assert_op!(0b000100_00001_00000_00000_0000000110_0, Opcode::PsqLx);
        assert_op!(0b000100_00001_00000_00000_0000000110_1, Opcode::Illegal);
        assert_op!(0b000100_00001_00000_00000_0000000111_0, Opcode::PsqStx);
        assert_op!(0b000100_00001_00000_00000_0000000111_1, Opcode::Illegal);
        assert_op!(0x7c000278, Opcode::Xor);
        // TODO more tests
    }

    #[test]
    fn test_to_string() {
        macro_rules! assert_asm {
            ($code:expr, $disasm:expr) => {{
                assert_eq!(Ins::new($code, 0x8000_0000u32).to_string(), $disasm)
            }};
        }
        assert_asm!(0x4c000000, "mcrf cr0, cr0");
        assert_asm!(0x7c000278, "xor r0, r0, r0");
        assert_asm!(0x10000014, "ps_sum0 f0, f0, f0, f0");
        assert_asm!(0x10000032, "ps_mul f0, f0, f0");
        assert_asm!(0x7c00052a, "stswx r0, r0, r0");
        assert_asm!(0x9421ffc0, "stwu r1, -0x40(r1)");
        assert_asm!(0x7C0802A6, "mflr r0");
        assert_asm!(0x90010044, "stw r0, 0x44(r1)");
        assert_asm!(0xDBE10030, "stfd f31, 0x30(r1)");
        assert_asm!(0xF3E10038, "psq_st f31, 0x38(r1), 0, qr0");
        assert_asm!(0xDBC10020, "stfd f30, 0x20(r1)");
        assert_asm!(0xF3C10028, "psq_st f30, 0x28(r1), 0, qr0");
        assert_asm!(0xDBA10010, "stfd f29, 0x10(r1)");
        assert_asm!(0xF3A10018, "psq_st f29, 0x18(r1), 0, qr0");
        assert_asm!(0x93E1000C, "stw r31, 0xc(r1)");
        assert_asm!(0xFFE01890, "fmr f31, f3");
        assert_asm!(0x7C7F1B78, "mr r31, r3");
        assert_asm!(0xFFA00890, "fmr f29, f1");
        assert_asm!(0xFFC01090, "fmr f30, f2");
        assert_asm!(0xFC20F890, "fmr f1, f31");
        assert_asm!(0xEC3D0072, "fmuls f1, f29, f1");
        assert_asm!(0xEC1D0772, "fmuls f0, f29, f29");
        assert_asm!(0xEC5E0828, "fsubs f2, f30, f1");
        assert_asm!(0xEC21007A, "fmadds f1, f1, f1, f0");
        assert_asm!(0xD05F0000, "stfs f2, 0x0(r31)");
        assert_asm!(0xD03F0004, "stfs f1, 0x4(r31)");
        assert_asm!(0xD3FF0008, "stfs f31, 0x8(r31)");
        assert_asm!(0xE3E10038, "psq_l f31, 0x38(r1), 0, qr0");
        assert_asm!(0xCBE10030, "lfd f31, 0x30(r1)");
        assert_asm!(0xE3C10028, "psq_l f30, 0x28(r1), 0, qr0");
        assert_asm!(0xCBC10020, "lfd f30, 0x20(r1)");
        assert_asm!(0xE3A10018, "psq_l f29, 0x18(r1), 0, qr0");
        assert_asm!(0xCBA10010, "lfd f29, 0x10(r1)");
        assert_asm!(0x80010044, "lwz r0, 0x44(r1)");
        assert_asm!(0x83E1000C, "lwz r31, 0xc(r1)");
        assert_asm!(0x7C0803A6, "mtlr r0");
        assert_asm!(0x38210040, "addi r1, r1, 0x40");
        assert_asm!(0x4E800020, "blr");
    }
}
