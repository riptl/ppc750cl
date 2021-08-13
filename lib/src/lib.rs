use std::fmt::Write;
use std::ops::Range;

use num_traits::AsPrimitive;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Opcode {
    Illegal = -1,
    // 750CL extensions
    Twi,
    PsCmpu0,
    PsqLx,
    PsqStx,
    PsSum0,
    PsSum1,
    PsMuls0,
    PsMuls1,
    PsMadds0,
    PsMadds1,
    PsDiv,
    PsSub,
    PsAdd,
    PsSel,
    PsRes,
    PsMul,
    PsRsqrte,
    PsMsub,
    PsMadd,
    PsNmsub,
    PsNmadd,
    PsCmpo0,
    PsqLux,
    PsqStux,
    PsNeg,
    PsCmpu1,
    PsMr,
    PsCmpo1,
    PsNabs,
    PsAbs,
    PsMerge00,
    PsMerge01,
    PsMerge10,
    PsMerge11,
    DcbzL,
    Mulli,
    Subfic,
    Cmpli,
    Cmpi,
    Addic,
    Addic_,
    Addi,
    Addis,
    Bc,
    Sc,
    B,
    Mcrf,
    Bclr,
    Crnor,
    Rfi,
    Crandc,
    Isync,
    Crxor,
    Crnand,
    Crand,
    Creqv,
    Crorc,
    Cror,
    Bcctr,
    Rlwimi,
    Rlwinm,
    Rlwnm,
    Ori,
    Oris,
    Xori,
    Xoris,
    Andi_,
    Andis_,
    Cmp,
    Tw,
    Subfc,
    Addc,
    Mulhwu,
    Mfcr,
    Lwarx,
    Lwzx,
    Slw,
    Cntlzw,
    And,
    Cmpl,
    Subf,
    Dcbst,
    Lwzux,
    Andc,
    Mulhw,
    Mfmsr,
    Dcbf,
    Lbzx,
    Neg,
    Lbzux,
    Nor,
    Subfe,
    Adde,
    Mtcrf,
    Mtmsr,
    Stwcx_,
    Stwx,
    Stwux,
    Subfze,
    Addze,
    Mtsr,
    Stbx,
    Subfme,
    Addme,
    Mullw,
    Mtsrin,
    Dcbtst,
    Stbux,
    Add,
    Dcbt,
    Lhzx,
    Eqv,
    Tlbie,
    Eciwx,
    Lhzux,
    Xor,
    Mfspr,
    Lhax,
    Mftb,
    Lhaux,
    Sthx,
    Orc,
    Ecowx,
    Sthux,
    Or,
    Divwu,
    Mtspr,
    Dcbi,
    Nand,
    Divw,
    Mcrxr,
    Lswx,
    Lwbrx,
    Lfsx,
    Srw,
    Tlbsync,
    Lfsux,
    Mfsr,
    Lswi,
    Sync,
    Lfdx,
    Lfdux,
    Mfsrin,
    Stswx,
    Stwbrx,
    Stfsx,
    Stfsux,
    Stswi,
    Stfdx,
    Stfdux,
    Lhbrx,
    Sraw,
    Srawi,
    Eieio,
    Sthbrx,
    Extsh,
    Extsb,
    Icbi,
    Stfiwx,
    Dcbz,
    Lwz,
    Lwzu,
    Lbz,
    Lbzu,
    Stw,
    Stwu,
    Stb,
    Stbu,
    Lhz,
    Lhzu,
    Lha,
    Lhau,
    Sth,
    Sthu,
    Lmw,
    Stmw,
    Lfs,
    Lfsu,
    Lfd,
    Lfdu,
    Stfs,
    Stfsu,
    Stfd,
    Stfdu,
    PsqL,
    PsqLu,
    Fdivs,
    Fsubs,
    Fadds,
    Fres,
    Fmuls,
    Fmsubs,
    Fmadds,
    Fnmsubs,
    Fnmadds,
    PsqSt,
    PsqStu,
    Fcmpu,
    Frsp,
    Fctiw,
    Fctiwz,
    Fdiv,
    Fsub,
    Fadd,
    Fsel,
    Fmul,
    Frsqrte,
    Fmsub,
    Fmadd,
    Fnmsub,
    Fnmadd,
    Fcmpo,
    Mtfsb1,
    Fneg,
    Mcrfs,
    Mtfsb0,
    Fmr,
    Mtfsfi,
    Fnabs,
    Fabs,
    Mffs,
    Mtfsf,
    // Basic instructions
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::Illegal
    }
}

#[derive(Default, Clone)]
pub struct Ins {
    pub code: u32,
    pub op: Opcode,
}

#[inline(always)]
fn bit(x: u32, idx: usize) -> bool {
    ((x >> (32 - idx - 1)) & 1) == 1
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

#[inline(always)]
fn zero_bits(x: u32, range: Range<usize>) -> bool {
    bits::<u32>(x, range) == 0
}

macro_rules! disasm_unreachable {
    ($msg:expr $(,)?) => {{
        panic!(
            "internal error: entered unreachable code disassembling instruction 0x{:08x}",
            $msg
        )
    }};
}

macro_rules! ins_bit {
    ($func:ident, $idx:expr) => {
        fn $func(&self) -> bool {
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
    fn new(code: u32) -> Self {
        Ins {
            code,
            ..Default::default()
        }
    }

    fn illegal() -> Self {
        Default::default()
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
    ins_field!(spr, u16, 11..21);
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

    pub fn disasm(x: u32) -> Self {
        let family = bits(x, 0..6);
        match family {
            0b000011 => {
                let mut ins = Ins::new(x);
                ins.op = Opcode::Twi;
                ins
            }
            0b000100 => Self::disasm_cl_ext(x),
            0b000111..=0b001111 => Self::disasm_basic1(x),
            0b010000 => Self::disasm_bc(x),
            0b010001 => Self::disasm_sc(x),
            0b010010 => Self::disasm_b(x),
            0b010011 => Self::disasm_010011(x),
            0b010100..=0b011101 => Self::disasm_basic2(x),
            0b011111 => Self::disasm_011111(x),
            0b100000..=0b110111 => Self::disasm_basic3(x),
            0b111000..=0b111001 => Self::disasm_psq(x),
            0b111011 => Self::disasm_111011(x),
            0b111100..=0b111101 => Self::disasm_psq(x),
            0b111111 => Self::disasm_111111(x),
            _ => Self::illegal(),
        }
    }

    fn disasm_cl_ext(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key: u8 = bits(x, 26..31);
        match key {
            // AB cmp form
            0b00000 => {
                ins.op = match bits(x, 26..31) {
                    0b00000 => Opcode::PsCmpu0,
                    0b00001 => Opcode::PsCmpo0,
                    0b00010 => Opcode::PsCmpu1,
                    0b00011 => Opcode::PsCmpo1,
                    _ => Opcode::Illegal,
                };
                if !zero_bits(x, 9..11) || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            // ABwi form
            0b00110 | 0b00111 => {
                if !bit(x, 25) {
                    ins.op = match key {
                        0b00110 => Opcode::PsqLx,
                        0b00111 => Opcode::PsqStx,
                        _ => Opcode::Illegal,
                    };
                } else {
                    ins.op = match key {
                        0b00110 => Opcode::PsqLux,
                        0b00111 => Opcode::PsqStux,
                        _ => Opcode::Illegal,
                    };
                }
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            // ABC form
            0b01010 | 0b01011 | 0b01110 | 0b01111 | 0b11100..=0b11111 => {
                ins.op = match key {
                    0b01010 => Opcode::PsSum0,
                    0b01011 => Opcode::PsSum1,
                    0b01110 => Opcode::PsMadds0,
                    0b01111 => Opcode::PsMadds1,
                    0b10111 => Opcode::PsSel,
                    0b11100 => Opcode::PsMsub,
                    0b11101 => Opcode::PsMadd,
                    0b11110 => Opcode::PsNmsub,
                    0b11111 => Opcode::PsNmadd,
                    _ => disasm_unreachable!(x),
                };
            }
            // AC form
            0b01100 | 0b01101 | 0b11001 => {
                ins.op = match key {
                    0b01100 => Opcode::PsMuls0,
                    0b01101 => Opcode::PsMuls1,
                    0b11001 => Opcode::PsMul,
                    _ => disasm_unreachable!(x),
                };
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            // AB form
            0b10010 | 0b10100 | 0b10101 => {
                ins.op = match key {
                    0b10010 => Opcode::PsDiv,
                    0b10100 => Opcode::PsSub,
                    0b10101 => Opcode::PsAdd,
                    _ => disasm_unreachable!(x),
                };
                if bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            // B form
            0b11000 | 0b11010 => {
                ins.op = match key {
                    0b11000 => Opcode::PsRes,
                    0b11010 => Opcode::PsRsqrte,
                    _ => disasm_unreachable!(x),
                };
                if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            // B alt form
            0b01000 => {
                ins.op = match bits(x, 26..31) {
                    0b00001 => Opcode::PsNeg,
                    0b00010 => Opcode::PsMr,
                    0b00100 => Opcode::PsNabs,
                    0b01000 => Opcode::PsAbs,
                    _ => Opcode::Illegal,
                };
            }
            // AB alt form
            0b10000 => {
                ins.op = match bits(x, 26..31) {
                    0b10000 => Opcode::PsMerge00,
                    0b10001 => Opcode::PsMerge01,
                    0b10010 => Opcode::PsMerge10, // violates IBM user guide
                    0b10011 => Opcode::PsMerge11,
                    _ => Opcode::Illegal,
                }
            }
            // dcbz_l
            0b10110 => {
                ins.op = Opcode::DcbzL;
                if bits::<u8>(x, 11..16) != 0 {
                    ins.op = Opcode::Illegal; // reserved
                }
            }
            // Unknown paired-singles key.
            _ => ins.op = Opcode::Illegal,
        }
        ins
    }

    fn disasm_basic1(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits(x, 0..6);
        ins.op = match key {
            0b000111 => Opcode::Mulli,
            0b001000 => Opcode::Subfic,
            0b001010 | 0b001011 => {
                if bit(x, 9) {
                    Opcode::Illegal // reserved
                } else if key == 0b001010 {
                    Opcode::Cmpli
                } else if key == 0b001011 {
                    Opcode::Cmpi
                } else {
                    disasm_unreachable!(x)
                }
            }
            0b001100 => Opcode::Addic,
            0b001101 => Opcode::Addic_,
            0b001110 => Opcode::Addi,
            0b001111 => Opcode::Addis,
            _ => Opcode::Illegal,
        };
        ins
    }

    fn disasm_bc(x: u32) -> Self {
        let mut ins = Ins::new(x);
        ins.op = Opcode::Bc;
        ins
    }

    fn disasm_sc(x: u32) -> Self {
        let mut ins = Ins::new(x);
        ins.op = Opcode::Sc;
        if bits::<u32>(x, 6..32) != 0b10 {
            ins.op = Opcode::Illegal;
        }
        ins
    }

    fn disasm_b(x: u32) -> Self {
        let mut ins = Ins::new(x);
        ins.op = Opcode::B;
        ins
    }

    fn disasm_010011(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits(x, 21..27);
        let key2 = bits::<u8>(x, 27..31);
        match key {
            // mcrf
            0b000000 => {
                ins.op = Opcode::Mcrf;
                if ins.code & 0b11111_000_11_000_11_11111_1111111111_1
                    != 0b10011_000_00_000_00_00000_0000000000_0
                {
                    ins.op = Opcode::Illegal;
                }
            }
            // DA form
            0b000001 | 0b100001 => {
                ins.op = match key {
                    0b000001 => Opcode::Bclr,
                    0b100001 => Opcode::Bcctr,
                    _ => disasm_unreachable!(x),
                };
                if ins.code & 0b00000000_00000000_11111_000_00000000 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b000011 => {
                ins.op = Opcode::Rfi;
                if !zero_bits(x, 6..21) || bits::<u8>(x, 27..31) != 0b0010 || !bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b001001 => {
                ins.op = Opcode::Isync;
                if !zero_bits(x, 6..21) || bits::<u8>(x, 27..31) != 0b0110 || !bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            // DAB form
            0b000010 | 0b001000 | 0b001100..=0b011100 => {
                ins.op = match key {
                    0b000010 => Opcode::Crnor,
                    0b001000 => Opcode::Crandc,
                    0b001100 => Opcode::Crxor,
                    0b001110 => Opcode::Crnand,
                    0b010000 => Opcode::Crand,
                    0b010010 => Opcode::Creqv,
                    0b011010 => Opcode::Crorc,
                    0b011100 => Opcode::Cror,
                    _ => Opcode::Illegal,
                };
                if key2 != 0b0001 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            _ => (),
        }
        ins
    }

    fn disasm_basic2(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits(x, 0..6);
        ins.op = match key {
            0b10100 => Opcode::Rlwimi,
            0b10101 => Opcode::Rlwinm,
            0b10111 => Opcode::Rlwnm,
            0b11000 => Opcode::Ori,
            0b11001 => Opcode::Oris,
            0b11010 => Opcode::Xori,
            0b11011 => Opcode::Xoris,
            0b11100 => Opcode::Andi_,
            0b11101 => Opcode::Andis_,
            _ => Opcode::Illegal,
        };
        ins
    }

    fn disasm_011111(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits::<u32>(x, 21..31);
        // Super ugly switch statement.
        // Plenty to clean up here.
        match key {
            0b00_0000_0000 | 0b00_0010_0000 => {
                ins.op = match key {
                    0b00_0000_0000 => Opcode::Cmp,
                    0b00_0010_0000 => Opcode::Cmpl,
                    _ => disasm_unreachable!(x),
                };
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0000_0100 => {
                ins.op = Opcode::Tw;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0000_1000 => ins.op = Opcode::Subfc,
            0b00_0000_1010 => ins.op = Opcode::Addc,
            0b00_0000_1011 => ins.op = Opcode::Mulhwu,
            0b00_0001_0011 => {
                ins.op = Opcode::Mfcr;
                if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 21..26) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0001_0100 => {
                ins.op = Opcode::Lwarx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0001_0111 => {
                ins.op = Opcode::Lwzx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0001_1000 => ins.op = Opcode::Slw,
            0b00_0001_1010 => {
                ins.op = Opcode::Cntlzw;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0001_1100 => ins.op = Opcode::And,
            0b00_0010_1000 => ins.op = Opcode::Subf,
            0b00_0011_0110 => {
                ins.op = Opcode::Dcbst;
                if bits::<u8>(x, 11..16) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0011_0111 => {
                ins.op = Opcode::Lwzux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0011_1100 => ins.op = Opcode::Andc,
            0b00_0100_1101 => ins.op = Opcode::Mulhw,
            0b00_0101_0011 => {
                ins.op = Opcode::Mfmsr;
                if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 21..26) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0101_0110 => {
                ins.op = Opcode::Dcbf;
                if bits::<u8>(x, 11..16) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0101_0111 => {
                ins.op = Opcode::Lbzx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0110_1000 => {
                ins.op = Opcode::Neg;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0111_0111 => {
                ins.op = Opcode::Lbzux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0111_1100 => ins.op = Opcode::Nor,
            0b00_1000_1000 => ins.op = Opcode::Subfe,
            0b00_1000_1010 => ins.op = Opcode::Adde,
            0b00_1001_0000 => {
                ins.op = Opcode::Mtcrf;
                if bit(x, 11) || bit(x, 20) || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1001_0010 => {
                ins.op = Opcode::Mtmsr;
                if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 21..26) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1001_0110 => {
                ins.op = Opcode::Stwcx_;
                if !bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1001_0111 => {
                ins.op = Opcode::Stwx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1011_0111 => {
                ins.op = Opcode::Stwux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1100_1000 => {
                ins.op = Opcode::Subfze;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1100_1010 => {
                ins.op = Opcode::Addze;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1101_0010 => {
                ins.op = Opcode::Mtsr;
                if bit(x, 11) || bits::<u8>(x, 21..26) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1101_0111 => {
                ins.op = Opcode::Stbx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1110_1000 => {
                ins.op = Opcode::Subfme;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1110_1010 => {
                ins.op = Opcode::Addme;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1110_1011 => ins.op = Opcode::Mullw,
            0b00_1111_0010 => {
                ins.op = Opcode::Mtsrin;
                if bits::<u8>(x, 16..21) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1111_0110 => {
                ins.op = Opcode::Dcbtst;
                if bits::<u8>(x, 11..16) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1111_0111 => {
                ins.op = Opcode::Stbux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0000_1010 => ins.op = Opcode::Add,
            0b01_0000_0110 => {
                ins.op = Opcode::Dcbt;
                if bits::<u8>(x, 11..16) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0000_0111 => {
                ins.op = Opcode::Lhzx;
                if bits::<u8>(x, 11..16) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0001_1100 => ins.op = Opcode::Eqv,
            0b01_0011_0010 => {
                ins.op = Opcode::Tlbie;
                if bits::<u8>(x, 11..16) != 0 || bits::<u8>(x, 16..21) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0011_0110 => {
                ins.op = Opcode::Eciwx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0011_0111 => {
                ins.op = Opcode::Lhzux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0011_1100 => ins.op = Opcode::Xor,
            0b01_0101_0011 => {
                ins.op = Opcode::Mfspr;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0101_0111 => {
                ins.op = Opcode::Lhax;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0111_0011 => {
                ins.op = Opcode::Mftb;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0111_0111 => {
                ins.op = Opcode::Lhaux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_1001_0111 => {
                ins.op = Opcode::Sthx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_1001_1100 => ins.op = Opcode::Orc,
            0b01_1011_0110 => {
                ins.op = Opcode::Ecowx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_1011_0111 => {
                ins.op = Opcode::Sthux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_1011_1100 => ins.op = Opcode::Or,
            0b01_1100_1011 => ins.op = Opcode::Divwu,
            0b01_1101_0011 => {
                ins.op = Opcode::Mtspr;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_1101_0110 => {
                ins.op = Opcode::Dcbi;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_1101_1100 => ins.op = Opcode::Nand,
            0b01_1111_1011 => ins.op = Opcode::Divw,
            0b10_0000_0000 => {
                ins.op = Opcode::Mcrxr;
                if !zero_bits(x, 9..21) || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0001_0101 => {
                ins.op = Opcode::Lswx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0001_0110 => {
                ins.op = Opcode::Lwbrx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0001_0111 => {
                ins.op = Opcode::Lfsx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0001_1000 => ins.op = Opcode::Srw,
            0b10_0011_0110 => {
                ins.op = Opcode::Tlbsync;
                if bits::<u8>(x, 11..16) != 0
                    || bits::<u8>(x, 16..21) != 0
                    || bits::<u8>(x, 21..26) != 0
                    || bit(x, 31)
                {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0011_0111 => {
                ins.op = Opcode::Lfsux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0101_0011 => {
                ins.op = Opcode::Mfsr;
                if bit(x, 11) || bits::<u8>(x, 21..26) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0101_0101 => {
                ins.op = Opcode::Lswi;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0101_0110 => {
                ins.op = Opcode::Sync;
                if bits::<u8>(x, 11..16) != 0
                    || bits::<u8>(x, 16..21) != 0
                    || bits::<u8>(x, 21..26) != 0
                    || bit(x, 31)
                {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0101_0111 => {
                ins.op = Opcode::Lfdx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_0111_0111 => {
                ins.op = Opcode::Lfdux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1001_0011 => {
                ins.op = Opcode::Mfsrin;
                if bits::<u8>(x, 16..21) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1001_0101 => {
                ins.op = Opcode::Stswx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1001_0110 => {
                ins.op = Opcode::Stwbrx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1001_0111 => {
                ins.op = Opcode::Stfsx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1011_0111 => {
                ins.op = Opcode::Stfsux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1101_0101 => {
                ins.op = Opcode::Stswi;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1101_0111 => {
                ins.op = Opcode::Stfdx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10_1111_0111 => {
                ins.op = Opcode::Stfdux;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_0001_0110 => {
                ins.op = Opcode::Lhbrx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_0001_1000 => ins.op = Opcode::Sraw,
            0b11_0011_1000 => ins.op = Opcode::Srawi,
            0b11_0101_0110 => {
                ins.op = Opcode::Eieio;
                if bits::<u8>(x, 16..21) != 0
                    || bits::<u8>(x, 21..26) != 0
                    || bits::<u8>(x, 26..31) != 0
                    || bit(x, 31)
                {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1001_0110 => {
                ins.op = Opcode::Sthbrx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1001_1010 => {
                ins.op = Opcode::Extsh;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1011_1010 => {
                ins.op = Opcode::Extsb;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1101_0110 => {
                ins.op = Opcode::Icbi;
                if bits::<u8>(x, 11..16) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1101_0111 => {
                ins.op = Opcode::Stfiwx;
                if bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1111_0110 => {
                ins.op = Opcode::Dcbz;
                if bits::<u8>(x, 11..16) != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            _ => ins.op = Opcode::Illegal,
        }
        ins
    }

    fn disasm_basic3(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits(x, 0..6);
        ins.op = match key {
            0b100000 => Opcode::Lwz,
            0b100001 => Opcode::Lwzu,
            0b100010 => Opcode::Lbz,
            0b100011 => Opcode::Lbzu,
            0b100100 => Opcode::Stw,
            0b100101 => Opcode::Stwu,
            0b100110 => Opcode::Stb,
            0b100111 => Opcode::Stbu,
            0b101000 => Opcode::Lhz,
            0b101001 => Opcode::Lhzu,
            0b101010 => Opcode::Lha,
            0b101011 => Opcode::Lhau,
            0b101100 => Opcode::Sth,
            0b101101 => Opcode::Sthu,
            0b101110 => Opcode::Lmw,
            0b101111 => Opcode::Stmw,
            0b110000 => Opcode::Lfs,
            0b110001 => Opcode::Lfsu,
            0b110010 => Opcode::Lfd,
            0b110011 => Opcode::Lfdu,
            0b110100 => Opcode::Stfs,
            0b110101 => Opcode::Stfsu,
            0b110110 => Opcode::Stfd,
            0b110111 => Opcode::Stfdu,
            _ => disasm_unreachable!(x),
        };
        ins
    }

    fn disasm_psq(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits(x, 0..6);
        ins.op = match key {
            0b111000 => Opcode::PsqL,
            0b111001 => Opcode::PsqLu,
            0b111100 => Opcode::PsqSt,
            0b111101 => Opcode::PsqStu,
            _ => disasm_unreachable!(x),
        };
        ins
    }

    fn disasm_111011(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits(x, 26..31);
        match key {
            0b10010 => {
                ins.op = Opcode::Fdivs;
                if bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10100 => {
                ins.op = Opcode::Fsubs;
                if bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10101 => {
                ins.op = Opcode::Fadds;
                if bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11000 => {
                ins.op = Opcode::Fres;
                if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11001 => {
                ins.op = Opcode::Fmuls;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11100 => ins.op = Opcode::Fmsubs,
            0b11101 => ins.op = Opcode::Fmadds,
            0b11110 => ins.op = Opcode::Fnmsubs,
            0b11111 => ins.op = Opcode::Fnmadds,
            _ => (),
        }
        ins
    }

    fn disasm_111111(x: u32) -> Self {
        let mut ins = Ins::new(x);
        let key = bits::<u32>(x, 26..31);
        match key {
            0b00000 => match bits(x, 26..31) {
                0b00 => {
                    ins.op = Opcode::Fcmpu;
                    if !zero_bits(x, 9..11) || bit(x, 31) {
                        ins.op = Opcode::Illegal;
                    }
                }
                0b01 => {
                    ins.op = Opcode::Fcmpo;
                    if !zero_bits(x, 9..11) || bit(x, 31) {
                        ins.op = Opcode::Illegal;
                    }
                }
                0b10 => {
                    ins.op = Opcode::Mcrfs;
                    if !zero_bits(x, 9..11) || !zero_bits(x, 14..16) || bit(x, 31) {
                        ins.op = Opcode::Illegal;
                    }
                }
                _ => (),
            },
            0b00110 => {
                match bits(x, 26..31) {
                    0b001 => {
                        ins.op = Opcode::Mtfsb1;
                        if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 21..26) != 0 {
                            ins.op = Opcode::Illegal;
                        }
                    }
                    0b010 => {
                        ins.op = Opcode::Mtfsb0;
                        if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 21..26) != 0 {
                            ins.op = Opcode::Illegal;
                        }
                    }
                    0b100 => {
                        ins.op = Opcode::Mtfsfi;
                        if !zero_bits(x, 9..16) || bit(x, 20) {
                            ins.op = Opcode::Illegal;
                        }
                    }
                    _ => (),
                };
            }
            0b00111 => match bits(x, 26..31) {
                0b10010 => {
                    ins.op = Opcode::Mffs;
                    if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 21..26) != 0 {
                        ins.op = Opcode::Illegal;
                    }
                }
                0b10110 => {
                    ins.op = Opcode::Mtfsf;
                    if bit(x, 6) || bit(x, 16) {
                        ins.op = Opcode::Illegal;
                    }
                }
                _ => (),
            },
            0b01000 => {
                ins.op = match bits(x, 26..31) {
                    0b0001 => Opcode::Fneg,
                    0b0010 => Opcode::Fmr,
                    0b0100 => Opcode::Fnabs,
                    0b1000 => Opcode::Fabs,
                    _ => Opcode::Illegal,
                };
                if bits::<u8>(x, 16..21) != 0 {
                    ins.op = Opcode::Illegal
                }
            }
            0b01100 => {
                ins.op = Opcode::Frsp;
                if bits::<u8>(x, 16..21) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01110 => {
                ins.op = Opcode::Fctiw;
                if bits::<u8>(x, 16..21) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01111 => {
                ins.op = Opcode::Fctiwz;
                if bits::<u8>(x, 16..21) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10010 => {
                ins.op = Opcode::Fdiv;
                if bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10100 => {
                ins.op = Opcode::Fsub;
                if bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10101 => {
                ins.op = Opcode::Fadd;
                if bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10111 => ins.op = Opcode::Fsel,
            0b11001 => {
                ins.op = Opcode::Fmul;
                if bits::<u8>(x, 21..26) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11010 => {
                ins.op = Opcode::Frsqrte;
                if bits::<u8>(x, 16..21) != 0 || bits::<u8>(x, 26..31) != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11100 => ins.op = Opcode::Fmsub,
            0b11101 => ins.op = Opcode::Fmadd,
            0b11110 => ins.op = Opcode::Fnmsub,
            0b11111 => ins.op = Opcode::Fnmadd,
            _ => (),
        }
        ins
    }

    fn write_string_form_reg123(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Eciwx => "eciwx",
            Opcode::Ecowx => "ecowx",
            Opcode::Lhaux => "lhaux",
            Opcode::Lhax => "lhax",
            Opcode::Lbzux => "lbzux",
            Opcode::Lbzx => "lbzx",
            Opcode::Lhbrx => "lhbrx",
            Opcode::Lhzux => "lhzux",
            Opcode::Lhzx => "lhzx",
            Opcode::Lswx => "lswx",
            Opcode::Lwarx => "lwarx",
            Opcode::Lwbrx => "lwbrx",
            Opcode::Lwzx => "lwzx",
            Opcode::Lwzux => "lwzux",
            Opcode::Stbux => "stbux",
            Opcode::Stbx => "stbx",
            Opcode::Sthux => "sthux",
            Opcode::Sthx => "sthx",
            Opcode::Sthbrx => "sthbrx",
            Opcode::Stswx => "stswx",
            Opcode::Stwbrx => "stwbrx",
            Opcode::Stwcx_ => "stwcx.",
            Opcode::Stwx => "stwx",
            Opcode::Stwux => "stwux",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}, r{}", name, self.d(), self.a(), self.b())
    }

    fn write_string_form_reg123_rc(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::And => "and",
            Opcode::Andc => "andc",
            Opcode::Mulhw => "mulhw",
            Opcode::Mulhwu => "mulhwu",
            Opcode::Xor => "xor",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, r{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_form_reg123_oe_rc(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = match (self.oe(), self.rc()) {
            (false, false) => "",
            (false, true) => ".",
            (true, false) => "o",
            (true, true) => "o.",
        };
        let name = match self.op {
            Opcode::Add => "add",
            Opcode::Addc => "addc",
            Opcode::Adde => "adde",
            Opcode::Divw => "divw",
            Opcode::Divwu => "divwu",
            Opcode::Mullw => "mullw",
            Opcode::Subf => "subf",
            Opcode::Subfc => "subfc",
            Opcode::Subfe => "subfe",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, r{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_noargs(&self, out: &mut String) -> std::fmt::Result {
        *out = match self.op {
            Opcode::Eieio => "eieio",
            Opcode::Isync => "isync",
            Opcode::Rfi => "rfi",
            Opcode::Sc => "sc",
            Opcode::Sync => "sync",
            Opcode::Tlbsync => "tlbsync",
            _ => disasm_unreachable!(self.code),
        }
        .to_owned();
        Ok(())
    }

    fn write_string_form_reg12_simm(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Addi => "addi",
            Opcode::Addic => "addic",
            Opcode::Addic_ => "addic.",
            Opcode::Addis => "addis",
            Opcode::Mulli => "mulli",
            Opcode::Subfic => "subfic",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} r{}, r{}, {}",
            name,
            self.d(),
            self.a(),
            self.simm()
        )
    }

    fn write_string_form_reg12_uimm(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Andi_ => "andi.",
            Opcode::Andis_ => "andis.",
            Opcode::Ori => "ori",
            Opcode::Oris => "oris",
            Opcode::Xori => "xori",
            Opcode::Xoris => "xoris",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} r{}, r{}, {}",
            name,
            self.d(),
            self.a(),
            self.uimm()
        )
    }

    fn write_string_form_reg12_offset(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Lha => "lha",
            Opcode::Lhau => "lhau",
            Opcode::Lbz => "lbz",
            Opcode::Lbzu => "lbzu",
            Opcode::Lhz => "lhz",
            Opcode::Lhzu => "lhzu",
            Opcode::Lmw => "lmw",
            Opcode::Lwz => "lwz",
            Opcode::Lwzu => "lwzu",
            Opcode::Stb => "stb",
            Opcode::Stbu => "stbu",
            Opcode::Sth => "sth",
            Opcode::Sthu => "sthu",
            Opcode::Stmw => "stmw",
            Opcode::Stw => "stw",
            Opcode::Stwu => "stwu",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} r{}, {}(r{})",
            name,
            self.d(),
            self.simm(),
            self.a()
        )
    }

    fn write_string_form_fr1_reg2_offset(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Lfd => "lfd",
            Opcode::Lfdu => "lfdu",
            Opcode::Lfs => "lfs",
            Opcode::Lfsu => "lfsu",
            Opcode::Stfd => "stfd",
            Opcode::Stfdu => "stfdu",
            Opcode::Stfs => "stfs",
            Opcode::Stfsu => "stfsu",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} fr{}, {}(r{})",
            name,
            self.d(),
            self.simm(),
            self.a()
        )
    }

    fn write_string_form_fr1_reg23(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Lfdux => "lfdux",
            Opcode::Lfdx => "lfdx",
            Opcode::Lfsux => "lfsux",
            Opcode::Lfsx => "lfsx",
            Opcode::Stfdux => "stfdux",
            Opcode::Stfdx => "stfdx",
            Opcode::Stfiwx => "stfiwx",
            Opcode::Stfsux => "stfsux",
            Opcode::Stfsx => "stfsx",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} fr{}, r{}, r{}", name, self.d(), self.a(), self.b())
    }

    fn write_string_mtfsf(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mtfsf => "mtfsf",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} {}, fr{}", name, self.fm(), self.b())
    }

    fn write_string_mtfsfi(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mtfsfi => "mtfsfi",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crf{}, {}",
            name,
            self.crf_d(),
            bits::<u8>(self.code, 16..20)
        )
    }

    fn write_string_form_reg1(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mfcr => "mfcr",
            Opcode::Mfmsr => "mfmsr",
            Opcode::Mtmsr => "mtmsr",
            Opcode::Tlbie => "tblie",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}", name, self.d())
    }

    fn write_string_form_reg12_oe_rc(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = match (self.oe(), self.rc()) {
            (false, false) => "",
            (false, true) => ".",
            (true, false) => "o",
            (true, true) => "o.",
        };
        let name = match self.op {
            Opcode::Addme => "addme",
            Opcode::Addze => "addze",
            Opcode::Neg => "neg",
            Opcode::Subfme => "subfme",
            Opcode::Subfze => "subfze",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{}{} r{}, r{}", name, name_suffix, self.d(), self.a())
    }

    fn write_string_form_reg13(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mfsrin => "mfsrin",
            Opcode::Mtsrin => "mtsrin",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}", name, self.d(), self.b())
    }

    fn write_string_form_reg21_rc(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Cntlzw => "cntlzw",
            Opcode::Extsb => "extsb",
            Opcode::Extsh => "extsh",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{}{} r{}, r{}", name, name_suffix, self.a(), self.s())
    }

    fn write_string_form_fr1(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mffs => match self.rc() {
                false => "mffs",
                true => "mffs.",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} fr{}", name, self.d())
    }

    fn write_string_form_fr13(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Fabs => "fabs",
            Opcode::Fnabs => "fnabs",
            Opcode::Fmr => "fmr",
            Opcode::Fneg => "fneg",
            Opcode::Fres => "fres",
            Opcode::Frsp => "frsp",
            Opcode::Frsqrte => "frsqrte",
            Opcode::PsAbs => "ps_abs",
            Opcode::PsMr => "ps_mr",
            Opcode::PsNabs => "ps_nabs",
            Opcode::PsNeg => "ps_neg",
            Opcode::PsRes => "ps_res",
            Opcode::PsRsqrte => "ps_rsqrte",
            Opcode::PsSum0 => "ps_sum0",
            Opcode::PsSum1 => "ps_sum1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.b()
        )
    }

    fn write_string_form_fr123(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Fadd => "fadd",
            Opcode::Fadds => "fadds",
            Opcode::Fdiv => "fdiv",
            Opcode::Fdivs => "fdivs",
            Opcode::Fsub => "fsub",
            Opcode::Fsubs => "fsubs",
            Opcode::PsAdd => "ps_add",
            Opcode::PsDiv => "ps_div",
            Opcode::PsMerge00 => "ps_merge00",
            Opcode::PsMerge01 => "ps_merge01",
            Opcode::PsMerge10 => "ps_merge10",
            Opcode::PsMerge11 => "ps_merge11",
            Opcode::PsSub => "ps_sub",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_form_fr1243(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Fmadd => "fmadd",
            Opcode::Fmadds => "fmadds",
            Opcode::Fmsub => "fmsub",
            Opcode::Fmsubs => "fmsubs",
            Opcode::Fnmadd => "fnmadd",
            Opcode::Fnmadds => "fnmadds",
            Opcode::Fnmsub => "fnmsub",
            Opcode::Fnmsubs => "fnmsubs",
            Opcode::Fsel => "fsel",
            Opcode::PsMadd => "ps_madd",
            Opcode::PsMadds0 => "ps_madds0",
            Opcode::PsMadds1 => "ps_madds1",
            Opcode::PsMsub => "ps_msub",
            Opcode::PsNmadd => "ps_nmadd",
            Opcode::PsNmsub => "ps_nmsub",
            Opcode::PsSel => "ps_sel",
            Opcode::PsSum0 => "ps_sum0",
            Opcode::PsSum1 => "ps_sum1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}, fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.c(),
            self.b()
        )
    }

    fn write_string_form_fr124(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Fmul => "fmul",
            Opcode::Fmuls => "fmuls",
            Opcode::PsMul => "ps_mul",
            Opcode::PsMuls0 => "ps_muls0",
            Opcode::PsMuls1 => "ps_muls1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.c()
        )
    }

    fn write_string_form_condreg1_fr23(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Fcmpo => "fcmpo",
            Opcode::Fcmpu => "fcmpu",
            Opcode::PsCmpo0 => "ps_cmpo0",
            Opcode::PsCmpo1 => "ps_cmpo1",
            Opcode::PsCmpu0 => "ps_cmpu0",
            Opcode::PsCmpu1 => "ps_cmpu1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crf{}, fr{}, fr{}",
            name,
            self.crf_d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_form_condreg1_fr13_rc(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Fctiw => "fctiw",
            Opcode::Fctiwz => "fctiwz",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} crf{}, fr{}, fr{}",
            name,
            name_suffix,
            self.crf_d(),
            self.d(),
            self.b()
        )
    }

    fn write_string_b(&self, out: &mut String) -> std::fmt::Result {
        let name = match (self.aa(), self.lk()) {
            (false, false) => "b",
            (false, true) => "bl",
            (true, false) => "ba",
            (true, true) => "bla",
        };
        // TODO absolute address
        write!(out, "{} 0x{:x}", name, self.li())
    }

    fn write_string_bc(&self, out: &mut String) -> std::fmt::Result {
        let name = match (self.aa(), self.lk()) {
            (false, false) => "bc",
            (false, true) => "bcl",
            (true, false) => "bca",
            (true, true) => "bcla",
        };
        // TODO absolute address
        write!(
            out,
            "{} 0x{:x}, 0x{:x}, 0x{:x}",
            name,
            self.bo(),
            self.bi(),
            self.li()
        )
    }

    fn write_string_branch_cond_to_reg(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Bcctr => match self.lk() {
                false => "bcctr",
                true => "bcctrl",
            },
            Opcode::Bclr => match self.lk() {
                false => "bclr",
                true => "bclrl",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} 0x{:x}, 0x{:x}", name, self.bo(), self.bi())
    }

    fn write_string_cmp(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Cmp => "cmp",
            Opcode::Cmpl => "cmpl",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crf{}, {}, r{}, r{}",
            name,
            self.crf_d(),
            self.l() as u8,
            self.a(),
            self.b()
        )
    }

    fn write_string_cmp_simm(&self, out: &mut String) -> std::fmt::Result {
        let name = "cmpi";
        write!(
            out,
            "{} crf{}, {}, r{}, {}",
            name,
            self.crf_d(),
            self.l() as u8,
            self.a(),
            self.simm()
        )
    }

    fn write_string_cmp_uimm(&self, out: &mut String) -> std::fmt::Result {
        let name = "cmpli";
        write!(
            out,
            "{} crf{}, {}, r{}, {}",
            name,
            self.crf_d(),
            self.l() as u8,
            self.a(),
            self.uimm()
        )
    }

    fn write_string_form_condreg1(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mcrxr => "mcrxr",
            Opcode::Mtfsb0 => match self.rc() {
                false => "mtfsb0",
                true => "mtfsb0.",
            },
            Opcode::Mtfsb1 => match self.rc() {
                false => "mtfsb1",
                true => "mtfsb1.",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} crf{}", name, self.crf_d())
    }

    fn write_string_form_condreg12(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mcrf => "mcrf",
            Opcode::Mcrfs => "mcrfs",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} crf{}, crf{}", name, self.crf_d(), self.crf_s())
    }

    fn write_string_form_condreg123(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Crand => "crand",
            Opcode::Crandc => "crandc",
            Opcode::Creqv => "creqv",
            Opcode::Crnand => "crnand",
            Opcode::Crnor => "crnor",
            Opcode::Cror => "cror",
            Opcode::Crorc => "crorc",
            Opcode::Crxor => "crxor",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crb{}, crb{}, crb{}",
            name,
            self.crb_d(),
            self.crb_a(),
            self.crb_b()
        )
    }

    fn write_string_form_reg23(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Dcbf => "dcbf",
            Opcode::Dcbi => "dcbi",
            Opcode::Dcbst => "dcbst",
            Opcode::Dcbt => "dcbt",
            Opcode::Dcbtst => "dcbtst",
            Opcode::Dcbz => "dcbz",
            Opcode::DcbzL => "dcbz_l",
            Opcode::Icbi => "icbi",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}", name, self.a(), self.b())
    }

    fn write_string_form_reg213(&self, out: &mut String) -> std::fmt::Result {
        let name_suffix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Eqv => "eqv",
            Opcode::Nand => "nand",
            Opcode::Nor => "nor",
            Opcode::Or => "or",
            Opcode::Orc => "orc",
            Opcode::Slw => "slw",
            Opcode::Sraw => "sraw",
            Opcode::Srw => "srw",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, r{}",
            name,
            name_suffix,
            self.a(),
            self.s(),
            self.b()
        )
    }

    fn write_string_rlw_imm(&self, out: &mut String) -> std::fmt::Result {
        let name_prefix = if self.rc() { "." } else { "" };
        let name = match self.op {
            Opcode::Rlwimi => "rlwimi",
            Opcode::Rlwinm => "rlwinm",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, {}, {}, {}",
            name,
            name_prefix,
            self.a(),
            self.s(),
            self.sh(),
            self.mb(),
            self.me()
        )
    }

    fn write_string_rlw_reg(&self, out: &mut String) -> std::fmt::Result {
        assert_eq!(self.op, Opcode::Rlwnm);
        let name_prefix = if self.rc() { "." } else { "" };
        write!(
            out,
            "rlwnm{} r{}, r{}, r{}, {}, {}",
            name_prefix,
            self.a(),
            self.s(),
            self.b(),
            self.mb(),
            self.me()
        )
    }

    fn write_string_form_reg12_nb(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Lswi => "lswi",
            Opcode::Stswi => "stswi",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}, {}", name, self.d(), self.a(), self.b())
    }

    fn write_string_form_reg1_spr(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mfspr => "mfspr",
            Opcode::Mftb => "mftb",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, {}", name, self.d(), self.spr())
    }

    fn write_string_form_spr_reg1(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mtspr => "mtspr",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} {}, r{}", name, self.spr(), self.s())
    }

    fn write_string_form_reg1_sr(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mfsr => "mfsr",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, {}", name, self.d(), self.sr())
    }

    fn write_string_form_sr_reg1(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::Mtsr => "mtsr",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} {}, r{}", name, self.sr(), self.s())
    }

    fn write_string_mtcrf(&self, out: &mut String) -> std::fmt::Result {
        assert_eq!(self.op, Opcode::Mtcrf);
        write!(out, "mtcrf {} r{}", self.crm(), self.s())
    }

    fn write_string_srawi(&self, out: &mut String) -> std::fmt::Result {
        assert_eq!(self.op, Opcode::Srawi);
        let name_suffix = if self.rc() { "." } else { "" };
        write!(
            out,
            "srawi{} r{}, r{}, {}",
            name_suffix,
            self.s(),
            self.a(),
            self.sh()
        )
    }

    fn write_string_tw(&self, out: &mut String) -> std::fmt::Result {
        assert_eq!(self.op, Opcode::Tw);
        write!(out, "tw {}, r{}, r{}", self.to(), self.a(), self.b())
    }

    fn write_string_twi(&self, out: &mut String) -> std::fmt::Result {
        assert_eq!(self.op, Opcode::Twi);
        write!(out, "twi {}, r{}, {}", self.to(), self.a(), self.simm())
    }

    fn write_string_psq(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::PsqL => "psq_l",
            Opcode::PsqLu => "psq_lu",
            Opcode::PsqSt => "psq_st",
            Opcode::PsqStu => "psq_stu",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} fr{}, {}(r{}), {}, {}",
            name,
            self.d(),
            self.ps_d(),
            self.a(),
            self.w(),
            self.ps_l()
        )
    }

    fn write_string_psq_x(&self, out: &mut String) -> std::fmt::Result {
        let name = match self.op {
            Opcode::PsqLx => "psq_lx",
            Opcode::PsqLux => "psq_lux",
            Opcode::PsqStx => "psq_stx",
            Opcode::PsqStux => "psq_stux",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} fr{}, r{}, r{}, {}, {}",
            name,
            self.d(),
            self.a(),
            self.b(),
            self.w(),
            self.ps_l()
        )
    }

    pub fn write_string(&self, out: &mut String) -> std::fmt::Result {
        match self.op {
            Opcode::Illegal => write!(out, "<illegal>"),

            // Standalone instructions
            Opcode::Eieio
            | Opcode::Isync
            | Opcode::Rfi
            | Opcode::Sc
            | Opcode::Sync
            | Opcode::Tlbsync => self.write_string_noargs(out),

            // General purpose register only
            Opcode::Mfcr | Opcode::Mfmsr | Opcode::Mtmsr | Opcode::Tlbie => {
                self.write_string_form_reg1(out)
            }
            Opcode::Addme | Opcode::Addze | Opcode::Neg | Opcode::Subfme | Opcode::Subfze => {
                self.write_string_form_reg12_oe_rc(out)
            }
            Opcode::Mfsrin | Opcode::Mtsrin => self.write_string_form_reg13(out),
            Opcode::Cntlzw | Opcode::Extsb | Opcode::Extsh => self.write_string_form_reg21_rc(out),
            Opcode::Dcbf
            | Opcode::Dcbi
            | Opcode::Dcbst
            | Opcode::Dcbt
            | Opcode::Dcbtst
            | Opcode::Dcbz
            | Opcode::DcbzL
            | Opcode::Icbi => self.write_string_form_reg23(out),
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
            | Opcode::Stwux => self.write_string_form_reg123(out),
            Opcode::And | Opcode::Andc | Opcode::Mulhw | Opcode::Mulhwu | Opcode::Xor => {
                self.write_string_form_reg123_rc(out)
            }
            Opcode::Add
            | Opcode::Addc
            | Opcode::Adde
            | Opcode::Divw
            | Opcode::Divwu
            | Opcode::Mullw
            | Opcode::Subf
            | Opcode::Subfc
            | Opcode::Subfe => self.write_string_form_reg123_oe_rc(out),
            Opcode::Eqv
            | Opcode::Nand
            | Opcode::Nor
            | Opcode::Or
            | Opcode::Orc
            | Opcode::Slw
            | Opcode::Sraw
            | Opcode::Srw => self.write_string_form_reg213(out),

            // General purpose shifts
            Opcode::Rlwimi | Opcode::Rlwinm => self.write_string_rlw_imm(out),
            Opcode::Rlwnm => self.write_string_rlw_reg(out),

            // General purpose register misc
            Opcode::Addi
            | Opcode::Addic
            | Opcode::Addic_
            | Opcode::Addis
            | Opcode::Mulli
            | Opcode::Subfic => self.write_string_form_reg12_simm(out),
            Opcode::Andi_
            | Opcode::Andis_
            | Opcode::Ori
            | Opcode::Oris
            | Opcode::Xori
            | Opcode::Xoris => self.write_string_form_reg12_uimm(out),
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
            | Opcode::Stwu => self.write_string_form_reg12_offset(out),
            Opcode::Lswi | Opcode::Stswi => self.write_string_form_reg12_nb(out),
            Opcode::Mfspr | Opcode::Mftb => self.write_string_form_reg1_spr(out),
            Opcode::Mtspr => self.write_string_form_spr_reg1(out),
            Opcode::Mfsr => self.write_string_form_reg1_sr(out),
            Opcode::Mtsr => self.write_string_form_sr_reg1(out),
            Opcode::Mtcrf => self.write_string_mtcrf(out),
            Opcode::Srawi => self.write_string_srawi(out),
            Opcode::Tw => self.write_string_tw(out),
            Opcode::Twi => self.write_string_twi(out),

            // Branch instructions
            Opcode::B => self.write_string_b(out),
            Opcode::Bc => self.write_string_bc(out),
            Opcode::Bcctr | Opcode::Bclr => self.write_string_branch_cond_to_reg(out),

            // Compare instructions
            Opcode::Cmp | Opcode::Cmpl => self.write_string_cmp(out),
            Opcode::Cmpi => self.write_string_cmp_simm(out),
            Opcode::Cmpli => self.write_string_cmp_uimm(out),

            // Floating point register only instructions
            Opcode::Mffs => self.write_string_form_fr1(out),
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
            | Opcode::PsRsqrte => self.write_string_form_fr13(out),
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
            | Opcode::PsSub => self.write_string_form_fr123(out),
            Opcode::Fmul | Opcode::Fmuls | Opcode::PsMul | Opcode::PsMuls0 | Opcode::PsMuls1 => {
                self.write_string_form_fr124(out)
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
            | Opcode::PsSum1 => self.write_string_form_fr1243(out),

            // Floating point register misc instructions
            Opcode::Fctiw | Opcode::Fctiwz => self.write_string_form_condreg1_fr13_rc(out),
            Opcode::Fcmpo
            | Opcode::Fcmpu
            | Opcode::PsCmpo0
            | Opcode::PsCmpo1
            | Opcode::PsCmpu0
            | Opcode::PsCmpu1 => self.write_string_form_condreg1_fr23(out),
            Opcode::Lfd
            | Opcode::Lfdu
            | Opcode::Lfs
            | Opcode::Lfsu
            | Opcode::Stfd
            | Opcode::Stfdu
            | Opcode::Stfs
            | Opcode::Stfsu => self.write_string_form_fr1_reg2_offset(out),
            Opcode::Lfdux
            | Opcode::Lfdx
            | Opcode::Lfsux
            | Opcode::Lfsx
            | Opcode::Stfdux
            | Opcode::Stfdx
            | Opcode::Stfiwx
            | Opcode::Stfsux
            | Opcode::Stfsx => self.write_string_form_fr1_reg23(out),
            Opcode::Mtfsf => self.write_string_mtfsf(out),

            // Condition register only
            Opcode::Mcrxr | Opcode::Mtfsb0 | Opcode::Mtfsb1 => self.write_string_form_condreg1(out),
            Opcode::Mcrf | Opcode::Mcrfs => self.write_string_form_condreg12(out),
            Opcode::Crand
            | Opcode::Crandc
            | Opcode::Creqv
            | Opcode::Crnand
            | Opcode::Crnor
            | Opcode::Cror
            | Opcode::Crorc
            | Opcode::Crxor => self.write_string_form_condreg123(out),

            // Condition register misc
            Opcode::Mtfsfi => self.write_string_mtfsfi(out),

            // Paired-single instructions
            Opcode::PsqL | Opcode::PsqLu | Opcode::PsqSt | Opcode::PsqStu => {
                self.write_string_psq(out)
            }
            Opcode::PsqLx | Opcode::PsqLux | Opcode::PsqStx | Opcode::PsqStux => {
                self.write_string_psq_x(out)
            }
        }
    }
}

impl ToString for Ins {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.write_string(&mut s).unwrap();
        s
    }
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings, clippy::bool_assert_comparison)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        assert_eq!(
            bits::<u32>(0b00000101100000000000000000000000u32, 5..9),
            0b1011u32
        );
        assert_eq!(bit(0b00000101100000000000000000000000u32, 5), true);
    }

    #[test]
    fn test_opcodes() {
        // twi
        assert_eq!(
            Ins::disasm(0b000011_00000_00000_0000000000000000).op,
            Opcode::Twi
        );
        // ps_cmpu0
        assert_eq!(
            Ins::disasm(0b000100_00000_00000_00000_0000000000_0).op,
            Opcode::PsCmpu0
        );
        assert_eq!(
            Ins::disasm(0b000100_00000_00000_00000_0000000000_1).op,
            Opcode::Illegal
        );
        assert_eq!(
            Ins::disasm(0b000100_00001_00000_00000_0000000000_0).op,
            Opcode::Illegal
        );
        // psq_lx
        assert_eq!(
            Ins::disasm(0b000100_00001_00000_00000_0000000110_0).op,
            Opcode::PsqLx
        );
        assert_eq!(
            Ins::disasm(0b000100_00001_00000_00000_0000000110_1).op,
            Opcode::Illegal
        );
        assert_eq!(
            Ins::disasm(0b000100_00001_00000_00000_0000000111_0).op,
            Opcode::PsqStx
        );
        assert_eq!(
            Ins::disasm(0b000100_00001_00000_00000_0000000111_1).op,
            Opcode::Illegal
        );
        assert_eq!(Ins::disasm(0x7c000278).op, Opcode::Xor);
        // TODO more tests
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Ins::disasm(0x4c000000).to_string(), "mcrf crf0, crf0");
        assert_eq!(Ins::disasm(0x7c000278).to_string(), "xor r0, r0, r0");
        assert_eq!(
            Ins::disasm(0x10000014).to_string(),
            "ps_sum0 fr0, fr0, fr0, fr0"
        );
        assert_eq!(Ins::disasm(0x10000032).to_string(), "ps_mul fr0, fr0, fr0");
        assert_eq!(Ins::disasm(0x7c00052a).to_string(), "stswx r0, r0, r0");
    }
}
