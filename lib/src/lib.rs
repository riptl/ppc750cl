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

macro_rules! ins_ufield {
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

macro_rules! ins_ifield {
    ($func:ident, $range:expr) => {
        fn $func(&self) -> i32 {
            debug_assert!(
                ($range).len() / 8 <= (std::mem::size_of::<i32>()),
                "{:?} does not fit in {}",
                $range,
                stringify!(i32)
            );
            let mut x = bits::<u32>(self.code, $range);
            if x >> (($range).len() - 1) == 1 {
                x = (x ^ ((1 << ($range).len()) - 1)) + 1;
                return -(x as i32);
            }
            x as i32
        }
    };
    ($func:ident, $range:expr, $shift:literal) => {
        fn $func(&self) -> i32 {
            debug_assert!(
                ($range).len() / 8 <= (std::mem::size_of::<i32>()),
                "{:?} does not fit in {}",
                $range,
                stringify!(i32)
            );
            let mut x = bits::<u32>(self.code, $range);
            if x >> (($range).len() - 1) == 1 {
                x = (x ^ ((1 << ($range).len()) - 1)) + 1;
                return -((x << $shift) as i32);
            }
            (x << $shift) as i32
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
    ins_ufield!(s, u8, 6..11);
    ins_ufield!(d, u8, 6..11);
    ins_ufield!(a, u8, 11..16);
    ins_ufield!(b, u8, 16..21);
    ins_ufield!(c, u8, 21..26);
    // Condition registers
    ins_ufield!(crb_d, u8, 6..11);
    ins_ufield!(crb_a, u8, 11..16);
    ins_ufield!(crb_b, u8, 16..21);

    ins_ufield!(crm, u8, 12..20);
    ins_ufield!(sr, u8, 12..16);
    fn spr(&self) -> u16 {
        bits::<u16>(self.code, 11..16) | (bits::<u16>(self.code, 16..21) << 5)
    }
    ins_ufield!(fm, u16, 7..15);
    ins_ufield!(crf_d, u8, 6..9);
    ins_ufield!(crf_s, u8, 11..14);
    ins_ifield!(simm, 16..32);
    ins_ufield!(uimm, u16, 16..32);
    ins_ufield!(bo, u8, 6..11);
    ins_ufield!(bi, u8, 11..16);
    ins_ufield!(sh, u8, 16..21);
    ins_ufield!(mb, u8, 21..26);
    ins_ufield!(me, u8, 26..31);
    fn me_31sub(&self) -> u8 {
        31 - self.me()
    }
    ins_ifield!(bd, 16..30, 2);
    ins_ifield!(li, 6..30, 2);
    ins_ufield!(to, u8, 6..11);
    // Paired-single fields.
    ins_ufield!(ps_l, u8, 17..20);
    ins_ifield!(ps_d, 20..32);

    fn write_asm_form_reg123<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: gpr;
            a: gpr0;
            b: gpr;
        })
    }

    fn write_asm_form_reg123_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            d: gpr;
            a: gpr;
            b: gpr;
        })
    }

    fn write_asm_form_reg123_oe_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, oe, rc): mnemonic;
            d: gpr;
            a: gpr;
            b: gpr;
        })
    }

    fn write_asm_form_reg12_simm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Addi && self.a() == 0 {
            write_asm!(out, self => {
                "li": mnemonic;
                d: gpr;
                simm: simm;
            })
        } else if self.op == Opcode::Addis && self.a() == 0 {
            write_asm!(out, self => {
                "lis": mnemonic;
                d: gpr;
                simm: simm;
            })
        } else {
            write_asm!(out, self => {
                (op.mnemonic): mnemonic;
                d: gpr;
                a: gpr;
                simm: simm;
            })
        }
    }

    fn write_asm_form_reg12_uimm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Addis && self.a() == 0 {
            return write_asm!(out, self => {
                "lis": mnemonic;
                d: gpr;
                uimm: uimm;
            });
        }
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: gpr;
            a: gpr;
            uimm: uimm;
        })
    }

    fn write_asm_form_reg21_uimm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Ori && self.a() == 0 && self.s() == 0 && self.uimm() == 0 {
            return write_asm!(out, self => { "nop": mnemonic });
        }
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            a: gpr;
            s: gpr;
            uimm: uimm;
        })
    }

    fn write_asm_form_reg12_offset<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: gpr;
            simm: offset;
            a: gpr0;
        })
    }

    fn write_asm_form_fr1_reg2_offset<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: fpr;
            simm: offset;
            a: gpr;
        })
    }

    fn write_asm_form_fr1_reg23<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: fpr;
            a: gpr;
            b: gpr;
        })
    }

    fn write_asm_mtfsf<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            fm: fm;
            b: fpr;
        })
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
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: gpr;
        })
    }

    fn write_asm_form_reg12_oe_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, oe, rc): mnemonic;
            d: gpr;
            a: gpr;
        })
    }

    fn write_asm_form_reg13<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: gpr;
            b: gpr;
        })
    }

    fn write_asm_form_reg21_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            a: gpr;
            s: gpr;
        })
    }

    fn write_asm_form_fr1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            d: fpr;
        })
    }

    fn write_asm_form_fr13_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            d: fpr;
            b: fpr;
        })
    }

    fn write_asm_form_fr123<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            d: fpr;
            a: fpr;
            b: fpr;
        })
    }

    fn write_asm_form_fr1243<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            d: fpr;
            a: fpr;
            c: fpr;
            b: fpr;
        })
    }

    fn write_asm_form_fr124<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            d: fpr;
            a: fpr;
            c: fpr;
        })
    }

    fn write_asm_form_condreg1_fr23<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            crf_d: cr;
            a: fpr;
            b: fpr;
        })
    }

    fn write_asm_form_condreg1_fr13_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            crf_d: cr;
            d: fpr;
            b: fpr;
        })
    }

    fn write_asm_b<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, lk, aa): mnemonic;
            li: branch_target;
        })
    }

    // Ported from
    // https://github.com/dolphin-emu/dolphin/blob/master/Source/Core/Common/GekkoDisassembler.cpp
    #[inline]
    fn write_asm_branch<F, W>(&self, out: &mut F, bname: &str) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.bo() & 4 != 0 {
            if self.bo() & 16 != 0 {
                return if self.bi() == 0 {
                    out.write_mnemonic(&("b".to_owned() + bname))?;
                    out.write_lk(self.lk())?;
                    out.write_aa(self.aa())?;
                    Ok(())
                } else {
                    write_asm!(out, self => {
                        (op.mnemonic, lk, aa): mnemonic;
                        bo: mode;
                        bi: mode;
                        bd: branch_target;
                    })
                };
            } else {
                let condition = (bit(self.code, 7) << 2) | (bits::<u8>(self.code, 14..16));
                let mnemonic_base = match condition {
                    0 => "bge",
                    1 => "ble",
                    2 => "bne",
                    3 => "bns",
                    4 => "blt",
                    5 => "bgt",
                    6 => "beq",
                    _ => {
                        return write_asm!(out, self => {
                            (op.mnemonic, lk, aa): mnemonic;
                            bo: mode;
                            bi: mode;
                            bd: branch_target;
                        })
                    }
                };
                // TODO avoid string concatenation
                out.write_mnemonic(&(mnemonic_base.to_owned() + bname))?;
                out.write_aa(self.aa())?;
                out.write_lk(self.lk())?;
                if bname.is_empty() {
                    out.write_opcode_separator()?;
                    if self.crf_s() != 0 {
                        out.write_cr(self.bi() >> 2)?;
                        out.write_operand_separator()?;
                    }
                    out.write_branch_target(self.bd(), self.addr)?;
                } else {
                    if self.crf_s() != 0 {
                        out.write_opcode_separator()?;
                        out.write_cr(self.bi() >> 2)?;
                    }
                }
            }
        } else {
            let mnemonic_base = match self.bo() >> 1 {
                0 => "bdnzf",
                1 => "bdnf",
                4 => "bdnzt",
                5 => "bdzt",
                8 | 12 => "bdnz",
                9 | 13 => "bdz",
                _ => {
                    return write_asm!(out, self => {
                        (op.mnemonic, lk, aa): mnemonic;
                        bo: mode;
                        bi: mode;
                        bd: branch_target;
                    })
                }
            };
            // TODO avoid string concatenation
            out.write_mnemonic(&(mnemonic_base.to_owned() + bname))?;
            out.write_aa(self.aa())?;
            out.write_lk(self.lk())?;
            if bname.is_empty() {
                out.write_opcode_separator()?;
                if (self.bo() & 16) == 0 {
                    out.write_mode(self.bi())?;
                    out.write_operand_separator()?;
                }
                out.write_branch_target(self.bd(), self.addr)?;
            } else {
                if (self.bo() & 16) == 0 {
                    out.write_opcode_separator()?;
                    out.write_mode(self.bi())?;
                }
            }
        }
        Ok(())
    }

    fn write_asm_cmp<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        match (self.op, self.crf_d(), self.l()) {
            (Opcode::Cmp, 0, 0) => {
                return write_asm!(out, self => {
                    "cmpw": mnemonic;
                    a: gpr;
                    b: gpr;
                })
            }
            (Opcode::Cmp, _, 0) => {
                return write_asm!(out, self => {
                    "cmpw": mnemonic;
                    crf_d: cr;
                    a: gpr;
                    b: gpr;
                })
            }
            (Opcode::Cmpl, 0, 0) => {
                return write_asm!(out, self => {
                    "cmplw": mnemonic;
                    a: gpr;
                    b: gpr;
                })
            }
            (Opcode::Cmpl, _, 0) => {
                return write_asm!(out, self => {
                    "cmplw": mnemonic;
                    crf_d: cr;
                    a: gpr;
                    b: gpr;
                })
            }
            _ => (),
        }
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            crf_d: cr;
            l: mode;
            a: gpr;
            b: gpr;
        })
    }

    fn write_asm_cmp_simm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        match (self.op, self.crf_d(), self.l()) {
            (Opcode::Cmpi, 0, 0) => {
                return write_asm!(out, self => {
                    "cmpwi": mnemonic;
                    a: gpr;
                    simm: simm;
                })
            }
            (Opcode::Cmpi, _, 0) => {
                return write_asm!(out, self => {
                    "cmpwi": mnemonic;
                    crf_d: cr;
                    a: gpr;
                    simm: simm;
                })
            }
            _ => (),
        }
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            crf_d: cr;
            l: mode;
            a: gpr;
            simm: simm;
        })
    }

    fn write_asm_cmp_uimm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        match (self.op, self.crf_d(), self.l()) {
            (Opcode::Cmpli, 0, 0) => {
                return write_asm!(out, self => {
                    "cmplwi": mnemonic;
                    a: gpr;
                    uimm: uimm;
                })
            }
            (Opcode::Cmpli, _, 0) => {
                return write_asm!(out, self => {
                    "cmplwi": mnemonic;
                    crf_d: cr;
                    a: gpr;
                    uimm: uimm;
                })
            }
            _ => (),
        }
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            crf_d: cr;
            l: mode;
            a: gpr;
            uimm: uimm;
        })
    }

    fn write_asm_form_condreg1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            crf_d: cr;
        })
    }

    fn write_asm_form_condreg1_rc<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            crf_d: cr;
        })
    }

    fn write_asm_form_condreg12<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            crf_d: cr;
            crf_s: cr;
        })
    }

    fn write_asm_form_condreg123<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            crb_d: cr;
            crb_a: cr;
            crb_b: cr;
        })
    }

    fn write_asm_form_reg23<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            a: gpr0;
            b: gpr0;
        })
    }

    fn write_asm_form_reg213<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Or && self.s() == self.b() {
            return write_asm!(out, self => {
                "mr": mnemonic;
                a: gpr;
                s: gpr;
            });
        }
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            a: gpr;
            s: gpr;
            b: gpr;
        })
    }

    fn write_asm_rlw_imm<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Rlwinm && self.sh() == 0 && self.me() == 31 {
            return write_asm!(out, self => {
                ("clrlwi", rc): mnemonic;
                a: gpr;
                s: gpr;
                mb: uimm;
            });
        }
        if self.op == Opcode::Rlwinm && self.mb() == 0 && self.me() == 31 {
            return write_asm!(out, self => {
                ("rotlwi", rc): mnemonic;
                a: gpr;
                s: gpr;
                sh: uimm;
            });
        }
        if self.op == Opcode::Rlwinm && self.mb() == 0 && 31 - self.sh() == self.me() {
            return write_asm!(out, self => {
                ("slwi", rc): mnemonic;
                a: gpr;
                s: gpr;
                me_31sub: uimm;
            });
        }
        if self.op == Opcode::Rlwinm && self.me() == 31 && 32 - self.mb() == self.sh() {
            return write_asm!(out, self => {
                ("srwi", rc): mnemonic;
                a: gpr;
                s: gpr;
                mb: uimm;
            });
        }
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            a: gpr;
            s: gpr;
            sh: mode;
            mb: uimm;
            me: uimm;
        })
    }

    fn write_asm_rlw_reg<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            a: gpr;
            s: gpr;
            b: gpr;
            mb: uimm;
            me: uimm;
        })
    }

    fn write_asm_form_reg12_nb<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: gpr;
            a: gpr;
            b: mode;
        })
    }

    fn write_asm_form_reg1_spr<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Mfspr {
            match self.spr() {
                1 => return write_asm!(out, self => { "mfxer": mnemonic; s: gpr }),
                8 => return write_asm!(out, self => { "mflr": mnemonic; s: gpr }),
                9 => return write_asm!(out, self => { "mfctr": mnemonic; s: gpr }),
                18 => return write_asm!(out, self => { "mfdsisr": mnemonic; s: gpr }),
                397 => return write_asm!(out, self => { "mfdbatu": mnemonic; s: gpr }),
                571 => return write_asm!(out, self => { "mftdu": mnemonic; s: gpr }),
                _ => (),
            };
        }
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        out.write_gpr(self.d())?;
        out.write_operand_separator()?;
        write!(out.writer(), "{:#x}", self.spr())?;
        Ok(())
    }

    fn write_asm_form_spr_reg1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Mtspr {
            match self.spr() {
                1 => return write_asm!(out, self => { "mtxer": mnemonic; s: gpr }),
                8 => return write_asm!(out, self => { "mtlr": mnemonic; s: gpr }),
                9 => return write_asm!(out, self => { "mtctr": mnemonic; s: gpr }),
                18 => return write_asm!(out, self => { "mtdsisr": mnemonic; s: gpr }),
                397 => return write_asm!(out, self => { "mtdbatu": mnemonic; s: gpr }),
                571 => return write_asm!(out, self => { "mttdu": mnemonic; s: gpr }),
                _ => (),
            };
        }
        out.write_mnemonic(self.op.mnemonic())?;
        out.write_opcode_separator()?;
        write!(out.writer(), "{:#x}", self.spr())?;
        out.write_operand_separator()?;
        out.write_gpr(self.s())?;
        Ok(())
    }

    fn write_asm_form_reg1_sr<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: gpr;
        })?;
        // TODO ugly
        out.write_operand_separator()?;
        write!(out.writer(), "{:#x}", self.s())?;
        Ok(())
    }

    fn write_asm_form_sr_reg1<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            sr: uimm;
            s: gpr;
        })
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
        write_asm!(out, self => {
            (op.mnemonic, rc): mnemonic;
            a: gpr;
            s: gpr;
            sh: uimm;
        })
    }

    fn write_asm_tw<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            to: mode;
            a: gpr;
            b: gpr;
        })
    }

    fn write_asm_twi<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        if self.op == Opcode::Twi && self.to() == 31 {
            return write_asm!(out, self => {
                "twui": mnemonic;
                a: gpr;
                simm: simm;
            });
        }
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            to: mode;
            a: gpr;
            simm: simm;
        })
    }

    fn write_asm_psq<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: fpr;
            ps_d: offset;
            a: gpr;
            w: mode;
            ps_l: qr;
        })
    }

    fn write_asm_psq_x<F, W>(&self, out: &mut F) -> std::io::Result<()>
    where
        F: AsmFormatter<W>,
        W: Write,
    {
        write_asm!(out, self => {
            (op.mnemonic): mnemonic;
            d: fpr;
            a: gpr;
            b: gpr;
            w: mode;
            ps_l: qr;
        })
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
            Opcode::Mulhw | Opcode::Mulhwu => self.write_asm_form_reg123_rc(out),
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
            | Opcode::Srw
            | Opcode::Xor
            | Opcode::And
            | Opcode::Andc => self.write_asm_form_reg213(out),

            // General purpose shifts
            Opcode::Rlwimi | Opcode::Rlwinm => self.write_asm_rlw_imm(out),
            Opcode::Rlwnm => self.write_asm_rlw_reg(out),

            // General purpose register misc
            Opcode::Addi | Opcode::Addic | Opcode::Addic_ | Opcode::Mulli | Opcode::Subfic => {
                self.write_asm_form_reg12_simm(out)
            }
            Opcode::Addis => self.write_asm_form_reg12_uimm(out),
            Opcode::Andi_
            | Opcode::Andis_
            | Opcode::Ori
            | Opcode::Oris
            | Opcode::Xori
            | Opcode::Xoris => self.write_asm_form_reg21_uimm(out),
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
            Opcode::Bc => self.write_asm_branch(out, ""),
            Opcode::Bcctr => self.write_asm_branch(out, "ctr"),
            Opcode::Bclr => self.write_asm_branch(out, "lr"),

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
            | Opcode::Fctiwz
            | Opcode::PsAbs
            | Opcode::PsMr
            | Opcode::PsNabs
            | Opcode::PsNeg
            | Opcode::PsRes
            | Opcode::PsRsqrte => self.write_asm_form_fr13_rc(out),
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
            Opcode::Fctiw => self.write_asm_form_condreg1_fr13_rc(out),
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
            Opcode::Mcrxr => self.write_asm_form_condreg1(out),
            Opcode::Mtfsb0 | Opcode::Mtfsb1 => self.write_asm_form_condreg1_rc(out),
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
        assert_asm!(0x7c00052a, "stswx r0, 0, r0");
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
        assert_asm!(0xD05F0000, "stfs f2, 0(r31)");
        assert_asm!(0xD03F0004, "stfs f1, 4(r31)");
        assert_asm!(0xD3FF0008, "stfs f31, 8(r31)");
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
