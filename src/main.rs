#![feature(bench_black_box)]

use num_traits::cast::cast;
use num_traits::PrimInt;
use std::hint::black_box;
use std::ops::Range;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
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
struct Ins {
    code: u32, // 0..32
    op: Opcode,
    // TODO move these struct members out to functions
    bd: u16,  // 16..30
    p2: u8,   // 6..11: S, D, TO, crbD, BO
    p3: u8,   // 11..16: A, crbA, BI
    p4: u8,   // 16..21: B, crbB, SH
    p5: u8,   // 21..26: C, MB
    p6: u8,   // 26..31: ME
    i: u8,    // 22..25
    crfd: u8, // 6..9
}

#[inline(always)]
fn bit(x: u32, idx: usize) -> bool {
    ((x >> (32 - idx - 1)) & 1) == 1
}

#[inline(always)]
fn bits<F>(x: u32, range: Range<usize>) -> F
where
    F: PrimInt,
{
    cast((x >> (32 - range.end)) & ((1 << range.len()) - 1)).expect("extracted bits do not fit")
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
    //ins_field!(bd, u16, 16..30);
    ins_field!(li, u32, 6..30);

    fn disasm(x: u32) -> Self {
        let family = bits(x, 0..6);
        match family {
            0b000011 => {
                let mut ins = Ins::new(x);
                ins.op = Opcode::Twi;
                ins.p2 = bits(x, 6..11);
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
        ins.p2 = bits(x, 6..11);
        ins.p3 = bits(x, 11..16);
        ins.p4 = bits(x, 16..21);
        ins.p5 = bits(x, 21..26);
        ins.crfd = bits(x, 6..9);
        let key: u8 = bits(x, 26..31);
        match key {
            // AB cmp form
            0b00000 => {
                ins.op = match ins.p5 {
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
                ins.i = bits(x, 22..25);
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
                if ins.p4 != 0 {
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
                if ins.p5 != 0 {
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
                if ins.p3 != 0 || ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            // B alt form
            0b01000 => {
                ins.op = match ins.p5 {
                    0b00001 => Opcode::PsNeg,
                    0b00010 => Opcode::PsMr,
                    0b00100 => Opcode::PsNabs,
                    0b01000 => Opcode::PsAbs,
                    _ => Opcode::Illegal,
                };
            }
            // AB alt form
            0b10000 => {
                ins.op = match ins.p5 {
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
                if ins.p2 != 0 {
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
        ins.p2 = bits(x, 6..11);
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
        ins.p2 = bits(x, 6..11);
        ins.p3 = bits(x, 11..16);
        ins.bd = bits(x, 16..30);
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
                if ins.p2 & 0b11 != 0
                    || ins.p3 & 0b11 != 0
                    || ins.p4 != 0
                    || key2 != 0
                    || bit(x, 31)
                {
                    ins.op = Opcode::Illegal;
                }
                ins.p2 >>= 2;
                ins.p3 >>= 2;
            }
            // DA form
            0b000001 | 0b100001 => {
                ins.op = match key {
                    0b000001 => Opcode::Bclr,
                    0b100001 => Opcode::Bcctr,
                    _ => disasm_unreachable!(x),
                };
                if ins.p4 != 0 {
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
        ins.p2 = bits(x, 6..11);
        ins.p3 = bits(x, 11..16);
        ins.p4 = bits(x, 16..21);
        ins.p5 = bits(x, 21..26);
        ins.p6 = bits(x, 26..31);
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
                ins.p2 >>= 2;
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
                if ins.p3 != 0 || ins.p4 != 0 || bit(x, 31) {
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
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0001_1100 => ins.op = Opcode::And,
            0b00_0010_1000 => ins.op = Opcode::Subf,
            0b00_0011_0110 => {
                ins.op = Opcode::Dcbst;
                if ins.p2 != 0 || bit(x, 31) {
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
                if ins.p3 != 0 || ins.p4 != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_0101_0110 => {
                ins.op = Opcode::Dcbf;
                if ins.p2 != 0 || bit(x, 31) {
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
                if ins.p4 != 0 {
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
                if ins.p3 != 0 || ins.p4 != 0 || bit(x, 31) {
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
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1100_1010 => {
                ins.op = Opcode::Addze;
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1101_0010 => {
                ins.op = Opcode::Mtsr;
                if bit(x, 11) || ins.p4 != 0 || bit(x, 31) {
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
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1110_1010 => {
                ins.op = Opcode::Addme;
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1110_1011 => ins.op = Opcode::Mullw,
            0b00_1111_0010 => {
                ins.op = Opcode::Mtsrin;
                if ins.p3 != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b00_1111_0110 => {
                ins.op = Opcode::Dcbtst;
                if ins.p2 != 0 || bit(x, 31) {
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
                if ins.p2 != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0000_0111 => {
                ins.op = Opcode::Lhzx;
                if ins.p2 != 0 || bit(x, 31) {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01_0001_1100 => ins.op = Opcode::Eqv,
            0b01_0011_0010 => {
                ins.op = Opcode::Tlbie;
                if ins.p2 != 0 || ins.p3 != 0 || bit(x, 31) {
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
                ins.p3 >>= 2;
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
                if ins.p2 != 0 || ins.p3 != 0 || ins.p4 != 0 || bit(x, 31) {
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
                if bit(x, 11) || ins.p4 != 0 || bit(x, 31) {
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
                if ins.p2 != 0 || ins.p3 != 0 || ins.p4 != 0 || bit(x, 31) {
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
                if ins.p3 != 0 || bit(x, 31) {
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
                if ins.p3 != 0 || ins.p4 != 0 || ins.p5 != 0 || bit(x, 31) {
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
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1011_1010 => {
                ins.op = Opcode::Extsb;
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11_1101_0110 => {
                ins.op = Opcode::Icbi;
                if ins.p2 != 0 || bit(x, 31) {
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
                if ins.p2 != 0 || bit(x, 31) {
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
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10100 => {
                ins.op = Opcode::Fsubs;
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10101 => {
                ins.op = Opcode::Fadds;
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11000 => {
                ins.op = Opcode::Fres;
                if ins.p3 != 0 || ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11001 => {
                ins.op = Opcode::Fmuls;
                if ins.p4 != 0 {
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
            0b00000 => match ins.p5 {
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
                match ins.p5 {
                    0b001 => {
                        ins.op = Opcode::Mtfsb1;
                        if ins.p3 != 0 || ins.p4 != 0 {
                            ins.op = Opcode::Illegal;
                        }
                    }
                    0b010 => {
                        ins.op = Opcode::Mtfsb0;
                        if ins.p3 != 0 || ins.p4 != 0 {
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
            0b00111 => match ins.p5 {
                0b10010 => {
                    ins.op = Opcode::Mffs;
                    if ins.p3 != 0 || ins.p4 != 0 {
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
                ins.op = match ins.p5 {
                    0b0001 => Opcode::Fneg,
                    0b0010 => Opcode::Fmr,
                    0b0100 => Opcode::Fnabs,
                    0b1000 => Opcode::Fabs,
                    _ => Opcode::Illegal,
                };
                if ins.p3 != 0 {
                    ins.op = Opcode::Illegal
                }
            }
            0b01100 => {
                ins.op = Opcode::Frsp;
                if ins.p3 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01110 => {
                ins.op = Opcode::Fctiw;
                if ins.p3 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b01111 => {
                ins.op = Opcode::Fctiwz;
                if ins.p3 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10010 => {
                ins.op = Opcode::Fdiv;
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10100 => {
                ins.op = Opcode::Fsub;
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10101 => {
                ins.op = Opcode::Fadd;
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10111 => ins.op = Opcode::Fsel,
            0b11001 => {
                ins.op = Opcode::Fmul;
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11010 => {
                ins.op = Opcode::Frsqrte;
                if ins.p3 != 0 || ins.p5 != 0 {
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

    fn to_string_form_reg123(&self) -> String {
        let name = match self.op {
            Opcode::Add => match (self.oe(), self.rc()) {
                (false, false) => "add",
                (false, true) => "add.",
                (true, false) => "addo",
                (true, true) => "addo.",
            },
            Opcode::Addc => match (self.oe(), self.rc()) {
                (false, false) => "addc",
                (false, true) => "addc.",
                (true, false) => "addco",
                (true, true) => "addco.",
            },
            Opcode::Adde => match (self.oe(), self.rc()) {
                (false, false) => "adde",
                (false, true) => "adde.",
                (true, false) => "addeo",
                (true, true) => "addeo.",
            },
            Opcode::And => match self.rc() {
                false => "and",
                true => "and.",
            },
            Opcode::Andc => match self.rc() {
                false => "andc",
                true => "andc.",
            },
            Opcode::Divw => match (self.oe(), self.rc()) {
                (false, false) => "divw",
                (false, true) => "divw.",
                (true, false) => "divwo",
                (true, true) => "divwo.",
            },
            Opcode::Divwu => match (self.oe(), self.rc()) {
                (false, false) => "divwu",
                (false, true) => "divwu.",
                (true, false) => "divwuo",
                (true, true) => "divwuo.",
            },
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
            Opcode::Mulhw => match self.rc() {
                false => "mulhw",
                true => "mulhw.",
            },
            Opcode::Mulhwu => match self.rc() {
                false => "mulhwu",
                true => "mulhwu.",
            },
            Opcode::Mullw => match (self.oe(), self.rc()) {
                (false, false) => "mullwu",
                (false, true) => "mullwu.",
                (true, false) => "mullwuo",
                (true, true) => "mullwuo.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}, r{}", name, self.d(), self.a(), self.b())
    }

    fn to_string_noargs(&self) -> String {
        match self.op {
            Opcode::Eieio => "eieio",
            Opcode::Isync => "isync",
            _ => disasm_unreachable!(self.code),
        }
        .to_owned()
    }

    fn to_string_form_reg12_simm(&self) -> String {
        let name = match self.op {
            Opcode::Addi => "addi",
            Opcode::Addic => "addic",
            Opcode::Addic_ => "addic.",
            Opcode::Addis => "addis",
            Opcode::Mulli => "mulli",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}, {}", name, self.d(), self.a(), self.simm())
    }

    fn to_string_form_reg12_uimm(&self) -> String {
        let name = match self.op {
            Opcode::Andi_ => "andi.",
            Opcode::Andis_ => "andis.",
            Opcode::Ori => "ori",
            Opcode::Oris => "oris",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}, {}", name, self.d(), self.a(), self.uimm())
    }

    fn to_string_form_reg12_offset(&self) -> String {
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
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, {}(r{})", name, self.d(), self.simm(), self.a())
    }

    fn to_string_form_fr1_reg2_offset(&self) -> String {
        let name = match self.op {
            Opcode::Lfd => "lfd",
            Opcode::Lfdu => "lfdu",
            Opcode::Lfs => "lfs",
            Opcode::Lfsu => "lfsu",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} fr{}, {}(r{})", name, self.d(), self.simm(), self.a())
    }

    fn to_string_form_fr1_reg23(&self) -> String {
        let name = match self.op {
            Opcode::Lfdux => "lfdux",
            Opcode::Lfdx => "lfdx",
            Opcode::Lfsux => "lfsux",
            Opcode::Lfsx => "lfsx",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} fr{}, r{}, r{}", name, self.d(), self.a(), self.b())
    }

    fn to_string_mtfsf(&self) -> String {
        let name = match self.op {
            Opcode::Mtfsf => "mtfsf",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} {}, fr{}", name, self.fm(), self.b())
    }

    fn to_string_mtfsfi(&self) -> String {
        let name = match self.op {
            Opcode::Mtfsfi => "mtfsfi",
            _ => disasm_unreachable!(self.code),
        };
        format!(
            "{} crf{}, {}",
            name,
            self.crf_d(),
            bits::<u8>(self.code, 16..20)
        )
    }

    fn to_string_form_reg1(&self) -> String {
        let name = match self.op {
            Opcode::Mfcr => "mfcr",
            Opcode::Mfmsr => "mfmsr",
            Opcode::Mtmsr => "mtmsr",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}", name, self.d())
    }

    fn to_string_form_reg12(&self) -> String {
        let name = match self.op {
            Opcode::Addme => match (self.oe(), self.rc()) {
                (false, false) => "addme",
                (false, true) => "addme.",
                (true, false) => "addmeo",
                (true, true) => "addmeo.",
            },
            Opcode::Addze => match (self.oe(), self.rc()) {
                (false, false) => "addze",
                (false, true) => "addze.",
                (true, false) => "addzeo",
                (true, true) => "addzeo.",
            },
            Opcode::Neg => match (self.oe(), self.rc()) {
                (false, false) => "neg",
                (false, true) => "neg.",
                (true, false) => "nego",
                (true, true) => "nego.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}", name, self.d(), self.a())
    }

    fn to_string_form_reg13(&self) -> String {
        let name = match self.op {
            Opcode::Mfsrin => "mfsrin",
            Opcode::Mtsrin => "mtsrin",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}", name, self.d(), self.b())
    }

    fn to_string_form_reg21(&self) -> String {
        let name = match self.op {
            Opcode::Cntlzw => match self.rc() {
                false => "cntlzw",
                true => "cntlzw.",
            },
            Opcode::Extsb => match self.rc() {
                false => "extsb",
                true => "extsb.",
            },
            Opcode::Extsh => match self.rc() {
                false => "extsh",
                true => "extsh.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}", name, self.a(), self.s())
    }

    fn to_string_form_fr1(&self) -> String {
        let name = match self.op {
            Opcode::Mffs => match self.rc() {
                false => "mffs",
                true => "mffs.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} fr{}", name, self.d())
    }

    fn to_string_form_fr13(&self) -> String {
        let name = match self.op {
            Opcode::Fabs => match self.rc() {
                false => "fabs",
                true => "fabs.",
            },
            Opcode::Fnabs => match self.rc() {
                false => "fnabs",
                true => "fnabs.",
            },
            Opcode::Fmr => match self.rc() {
                false => "fmr",
                true => "fmr.",
            },
            Opcode::Fneg => match self.rc() {
                false => "fneg",
                true => "fneg.",
            },
            Opcode::Fres => match self.rc() {
                false => "fres",
                true => "fres.",
            },
            Opcode::Frsp => match self.rc() {
                false => "frsp",
                true => "frsp.",
            },
            Opcode::Frsqrte => match self.rc() {
                false => "frsqrte",
                true => "frsqrte.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} fr{}, fr{}", name, self.d(), self.b())
    }

    fn to_string_form_fr123(&self) -> String {
        let name = match self.op {
            Opcode::Fadd => match self.rc() {
                false => "fadd",
                true => "fadd.",
            },
            Opcode::Fadds => match self.rc() {
                false => "fadds",
                true => "fadds.",
            },
            Opcode::Fdiv => match self.rc() {
                false => "fdiv",
                true => "fdiv.",
            },
            Opcode::Fdivs => match self.rc() {
                false => "fdivs",
                true => "fdivs.",
            },
            Opcode::Fsub => match self.rc() {
                false => "fsub",
                true => "fsub.",
            },
            Opcode::Fsubs => match self.rc() {
                false => "fsubs",
                true => "fsubs.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} fr{}, fr{}, fr{}", name, self.d(), self.a(), self.b())
    }

    fn to_string_form_fr1243(&self) -> String {
        let name = match self.op {
            Opcode::Fmadd => match self.rc() {
                false => "fmadd",
                true => "fmadd.",
            },
            Opcode::Fmadds => match self.rc() {
                false => "fmadds",
                true => "fmadds.",
            },
            Opcode::Fmsub => match self.rc() {
                false => "fmsub",
                true => "fmsub.",
            },
            Opcode::Fmsubs => match self.rc() {
                false => "fmsubs",
                true => "fmsubs.",
            },
            Opcode::Fnmadd => match self.rc() {
                false => "fnmadd",
                true => "fnmadd.",
            },
            Opcode::Fnmadds => match self.rc() {
                false => "fnmadds",
                true => "fnmadds.",
            },
            Opcode::Fnmsub => match self.rc() {
                false => "fnmsub",
                true => "fnmsub.",
            },
            Opcode::Fnmsubs => match self.rc() {
                false => "fnmsubs",
                true => "fnmsubs.",
            },
            Opcode::Fsel => match self.rc() {
                false => "fsel",
                true => "fsel.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!(
            "{} fr{}, fr{}, fr{}, fr{}",
            name,
            self.d(),
            self.a(),
            self.c(),
            self.b()
        )
    }

    fn to_string_form_fr124(&self) -> String {
        let name = match self.op {
            Opcode::Fmul => match self.rc() {
                false => "fmul",
                true => "fmul.",
            },
            Opcode::Fmuls => match self.rc() {
                false => "fmuls",
                true => "fmuls.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} fr{}, fr{}, fr{}", name, self.d(), self.a(), self.c())
    }

    fn to_string_form_condreg1_fr23(&self) -> String {
        let name = match self.op {
            Opcode::Fcmpo => "fcmpo",
            Opcode::Fcmpu => "fcmpu",
            _ => disasm_unreachable!(self.code),
        };
        format!(
            "{} crf{}, fr{}, fr{}",
            name,
            self.crf_d(),
            self.a(),
            self.b()
        )
    }

    fn to_string_form_condreg1_fr13(&self) -> String {
        let name = match self.op {
            Opcode::Fctiw => match self.rc() {
                false => "fctiw",
                true => "fctiw.",
            },
            Opcode::Fctiwz => match self.rc() {
                false => "fctiwz",
                true => "fctiwz.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!(
            "{} crf{}, fr{}, fr{}",
            name,
            self.crf_d(),
            self.d(),
            self.b()
        )
    }

    fn to_string_b(&self) -> String {
        let name = match (self.aa(), self.lk()) {
            (false, false) => "b",
            (false, true) => "bl",
            (true, false) => "ba",
            (true, true) => "bla",
        };
        // TODO absolute address
        format!("{} 0x{:x}", name, self.li())
    }

    fn to_string_bc(&self) -> String {
        let name = match (self.aa(), self.lk()) {
            (false, false) => "bc",
            (false, true) => "bcl",
            (true, false) => "bca",
            (true, true) => "bcla",
        };
        // TODO absolute address
        format!(
            "{} 0x{:x}, 0x{:x}, 0x{:x}",
            name,
            self.bo(),
            self.bi(),
            self.li()
        )
    }

    fn to_string_branch_cond_to_reg(&self) -> String {
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
        format!("{} 0x{:x}, 0x{:x}", name, self.bo(), self.bi())
    }

    fn to_string_cmp(&self) -> String {
        let name = match self.op {
            Opcode::Cmp => "cmp",
            Opcode::Cmpl => "cmpl",
            _ => disasm_unreachable!(self.code),
        };
        format!(
            "{} crf{}, {}, r{}, r{}",
            name,
            self.crfd,
            self.l() as u8,
            self.a(),
            self.b()
        )
    }

    fn to_string_cmp_simm(&self) -> String {
        let name = "cmpi";
        format!(
            "{} crf{}, {}, r{}, {}",
            name,
            self.crfd,
            self.l() as u8,
            self.a(),
            self.simm()
        )
    }

    fn to_string_cmp_uimm(&self) -> String {
        let name = "cmpli";
        format!(
            "{} crf{}, {}, r{}, {}",
            name,
            self.crfd,
            self.l() as u8,
            self.a(),
            self.uimm()
        )
    }

    fn to_string_form_condreg1(&self) -> String {
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
        format!("{} crf{}", name, self.crf_d())
    }

    fn to_string_form_condreg12(&self) -> String {
        let name = match self.op {
            Opcode::Mcrf => "mcrf",
            Opcode::Mcrfs => "mcrfs",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} crf{}, crf{}", name, self.crf_d(), self.crf_s())
    }

    fn to_string_form_condreg123(&self) -> String {
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
        format!(
            "{} crb{}, crb{}, crb{}",
            name,
            self.crb_d(),
            self.crb_a(),
            self.crb_b()
        )
    }

    fn to_string_form_reg23(&self) -> String {
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
        format!("{} r{}, r{}", name, self.a(), self.b())
    }

    fn to_string_form_reg213(&self) -> String {
        let name = match self.op {
            Opcode::Eqv => match self.rc() {
                false => "eqv",
                true => "eqv.",
            },
            Opcode::Nand => match self.rc() {
                false => "nand",
                true => "nand.",
            },
            Opcode::Nor => match self.rc() {
                false => "nor",
                true => "nor.",
            },
            Opcode::Or => match self.rc() {
                false => "or",
                true => "or.",
            },
            Opcode::Orc => match self.rc() {
                false => "orc",
                true => "orc.",
            },
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}, r{}", name, self.a(), self.s(), self.b())
    }

    fn to_string_form_reg12_nb(&self) -> String {
        let name = match self.op {
            Opcode::Lswi => "lswi",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, r{}, {}", name, self.d(), self.a(), self.b())
    }

    fn to_string_form_reg1_spr(&self) -> String {
        let name = match self.op {
            Opcode::Mfspr => "mfspr",
            Opcode::Mftb => "mftb",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, {}", name, self.d(), self.spr())
    }

    fn to_string_form_spr_reg1(&self) -> String {
        let name = match self.op {
            Opcode::Mtspr => "mtspr",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} {}, r{}", name, self.spr(), self.s())
    }

    fn to_string_form_reg1_sr(&self) -> String {
        let name = match self.op {
            Opcode::Mfsr => "mfsr",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} r{}, {}", name, self.d(), self.sr())
    }

    fn to_string_form_sr_reg1(&self) -> String {
        let name = match self.op {
            Opcode::Mtsr => "mtsr",
            _ => disasm_unreachable!(self.code),
        };
        format!("{} {}, r{}", name, self.sr(), self.s())
    }

    fn to_string_mtcrf(&self) -> String {
        assert_eq!(self.op, Opcode::Mtcrf);
        format!("mtcrf {} r{}", self.crm(), self.s())
    }
}

impl ToString for Ins {
    fn to_string(&self) -> String {
        match self.op {
            Opcode::Illegal => "<illegal>".to_string(),

            // Standalone instructions
            Opcode::Eieio | Opcode::Isync => self.to_string_noargs(),

            // General purpose register only
            Opcode::Mfcr | Opcode::Mfmsr | Opcode::Mtmsr => self.to_string_form_reg1(),
            Opcode::Addme | Opcode::Addze | Opcode::Neg => self.to_string_form_reg12(),
            Opcode::Mfsrin | Opcode::Mtsrin => self.to_string_form_reg13(),
            Opcode::Cntlzw | Opcode::Extsb | Opcode::Extsh => self.to_string_form_reg21(),
            Opcode::Dcbf
            | Opcode::Dcbi
            | Opcode::Dcbst
            | Opcode::Dcbt
            | Opcode::Dcbtst
            | Opcode::Dcbz
            | Opcode::DcbzL
            | Opcode::Icbi => self.to_string_form_reg23(),
            Opcode::Add
            | Opcode::Addc
            | Opcode::Adde
            | Opcode::And
            | Opcode::Andc
            | Opcode::Divw
            | Opcode::Divwu
            | Opcode::Eciwx
            | Opcode::Ecowx
            | Opcode::Lbzux
            | Opcode::Lbzx
            | Opcode::Lhaux
            | Opcode::Lhax
            | Opcode::Lhbrx
            | Opcode::Lhzux
            | Opcode::Lhzx
            | Opcode::Lswx
            | Opcode::Lwarx
            | Opcode::Lwbrx
            | Opcode::Lwzux
            | Opcode::Lwzx
            | Opcode::Mulhw
            | Opcode::Mulhwu
            | Opcode::Mullw => self.to_string_form_reg123(),
            Opcode::Eqv | Opcode::Nand | Opcode::Nor | Opcode::Or | Opcode::Orc => {
                self.to_string_form_reg213()
            }

            // General purpose register misc
            Opcode::Addi | Opcode::Addic | Opcode::Addic_ | Opcode::Addis | Opcode::Mulli => {
                self.to_string_form_reg12_simm()
            }
            Opcode::Andi_ | Opcode::Andis_ | Opcode::Ori | Opcode::Oris => {
                self.to_string_form_reg12_uimm()
            }
            Opcode::Lbz
            | Opcode::Lbzu
            | Opcode::Lha
            | Opcode::Lhau
            | Opcode::Lhz
            | Opcode::Lhzu
            | Opcode::Lmw
            | Opcode::Lwz
            | Opcode::Lwzu => self.to_string_form_reg12_offset(),
            Opcode::Lswi => self.to_string_form_reg12_nb(),
            Opcode::Mfspr | Opcode::Mftb => self.to_string_form_reg1_spr(),
            Opcode::Mtspr => self.to_string_form_spr_reg1(),
            Opcode::Mfsr => self.to_string_form_reg1_sr(),
            Opcode::Mtsr => self.to_string_form_sr_reg1(),
            Opcode::Mtcrf => self.to_string_mtcrf(),

            // Branch instructions
            Opcode::B => self.to_string_b(),
            Opcode::Bc => self.to_string_bc(),
            Opcode::Bcctr | Opcode::Bclr => self.to_string_branch_cond_to_reg(),

            // Compare instructions
            Opcode::Cmp | Opcode::Cmpl => self.to_string_cmp(),
            Opcode::Cmpi => self.to_string_cmp_simm(),
            Opcode::Cmpli => self.to_string_cmp_uimm(),

            // Floating point register only instructions
            Opcode::Mffs => self.to_string_form_fr1(),
            Opcode::Fabs
            | Opcode::Fmr
            | Opcode::Fnabs
            | Opcode::Fneg
            | Opcode::Fres
            | Opcode::Frsp
            | Opcode::Frsqrte => self.to_string_form_fr13(),
            Opcode::Fadd
            | Opcode::Fadds
            | Opcode::Fdiv
            | Opcode::Fdivs
            | Opcode::Fsub
            | Opcode::Fsubs => self.to_string_form_fr123(),
            Opcode::Fmul | Opcode::Fmuls => self.to_string_form_fr124(),
            Opcode::Fmadd
            | Opcode::Fmadds
            | Opcode::Fmsub
            | Opcode::Fmsubs
            | Opcode::Fnmadd
            | Opcode::Fnmadds
            | Opcode::Fnmsub
            | Opcode::Fnmsubs
            | Opcode::Fsel => self.to_string_form_fr1243(),

            // Floating point register misc instructions
            Opcode::Fctiw | Opcode::Fctiwz => self.to_string_form_condreg1_fr13(),
            Opcode::Fcmpo | Opcode::Fcmpu => self.to_string_form_condreg1_fr23(),
            Opcode::Lfd | Opcode::Lfdu | Opcode::Lfs | Opcode::Lfsu => {
                self.to_string_form_fr1_reg2_offset()
            }
            Opcode::Lfdux | Opcode::Lfdx | Opcode::Lfsux | Opcode::Lfsx => {
                self.to_string_form_fr1_reg23()
            }
            Opcode::Mtfsf => self.to_string_mtfsf(),

            // Condition register only
            Opcode::Mcrxr | Opcode::Mtfsb0 | Opcode::Mtfsb1 => self.to_string_form_condreg1(),
            Opcode::Mcrf | Opcode::Mcrfs => self.to_string_form_condreg12(),
            Opcode::Crand
            | Opcode::Crandc
            | Opcode::Creqv
            | Opcode::Crnand
            | Opcode::Crnor
            | Opcode::Cror
            | Opcode::Crorc
            | Opcode::Crxor => self.to_string_form_condreg123(),

            // Condition register misc
            Opcode::Mtfsfi => self.to_string_mtfsfi(),

            _ => todo!("can't format instruction 0x{:x}", self.code),
        }
    }
}

fn main() {
    // We don't have much to do. For now, just run a dumb microbenchmark / fuzzer.
    let start = Instant::now();
    let counter = Arc::new(AtomicU32::new(0));
    let counter_inner = Arc::clone(&counter);
    std::thread::spawn(move || {
        let mut last = 0u32;
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let now = counter_inner.load(Ordering::Relaxed);
            let per_second = now - last;
            last = now;
            let progress = 100f32 * ((now as f32) / (0x1_0000_0000u64 as f32));
            println!("{}/s\t{:05.2}%\tn=0x{:08x}", per_second, progress, now);
        }
    });
    for x in 0x4800_0000u32..0xFFFF_FFFFu32 {
        black_box(Ins::disasm(x).to_string());
        if x % (1 << 19) == 0 {
            counter.store(x, Ordering::Relaxed);
        }
    }
    println!("Finished in {:.2}s", start.elapsed().as_secs_f32());
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
        // TODO more tests
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Ins::disasm(0x4c000000).to_string(), "mcrf crf0, crf0");
    }
}
