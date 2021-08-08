#![feature(bench_black_box)]

use num_traits::cast::cast;
use num_traits::PrimInt;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};
use std::hint::black_box;

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
    Fdivsx,
    Fsubsx,
    Fadds,
    Fresx,
    Fmulsx,
    Fmsubsx,
    Fmaddsx,
    Fnmsubsx,
    Fnmaddsx,
    PsqSt,
    PsqStu,
    Fcmpu,
    Frspx,
    Fctiw,
    Fctiwz,
    Fdivx,
    Fsubx,
    Fadd,
    Fselx,
    Fmulx,
    Frsqrte,
    Fmsubx,
    Fmaddx,
    Fnmsubx,
    Fnmaddx,
    Fcmpo,
    Mtfsb1,
    Fnegx,
    Mcrfs,
    Mtfsb0,
    Fmrx,
    Mtfsfi,
    Fnabsx,
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

impl Opcode {
    fn name(self) -> &'static str {
        match self {
            Self::Illegal => "<illegal>",
            Self::Twi => "twi",
            Self::PsCmpu0 => "ps_cmpu0",
            Self::PsqLx => "psq_lx",
            Self::PsqStx => "psq_stx",
            Self::PsSum0 => "ps_sum0",
            Self::PsSum1 => "ps_sum1",
            Self::PsMuls0 => "ps_muls0",
            Self::PsMuls1 => "ps_muls1",
            Self::PsMadds0 => "ps_madds0",
            Self::PsMadds1 => "ps_madds1",
            Self::PsDiv => "ps_div",
            Self::PsSub => "ps_sub",
            Self::PsAdd => "ps_add",
            Self::PsSel => "ps_sel",
            Self::PsRes => "ps_res",
            Self::PsMul => "ps_mul",
            Self::PsRsqrte => "ps_rsqrte",
            Self::PsMsub => "ps_msub",
            Self::PsMadd => "ps_madd",
            Self::PsNmsub => "ps_nmsub",
            Self::PsNmadd => "ps_nmadd",
            Self::PsCmpo0 => "ps_cmpo0",
            Self::PsqLux => "psq_lux",
            Self::PsqStux => "psq_stux",
            Self::PsNeg => "ps_neg",
            Self::PsCmpu1 => "ps_cmpu1",
            Self::PsMr => "ps_mr",
            Self::PsCmpo1 => "ps_cmpo1",
            Self::PsNabs => "ps_nabs",
            Self::PsAbs => "ps_abs",
            Self::PsMerge00 => "ps_merge00",
            Self::PsMerge01 => "ps_merge01",
            Self::PsMerge10 => "ps_merge10",
            Self::PsMerge11 => "ps_merge11",
            Self::DcbzL => "dcbz_l",
            Self::Mulli => "mulli",
            Self::Subfic => "subfic",
            Self::Cmpli => "cmpli",
            Self::Cmpi => "cmpi",
            Self::Addic => "addic",
            Self::Addic_ => "addic.",
            Self::Addi => "addi",
            Self::Addis => "addis",
            Self::Bc => "bc",
            Self::Sc => "sc",
            Self::B => "b",
            Self::Mcrf => "mcrf",
            Self::Bclr => "bclr",
            Self::Crnor => "crnor",
            Self::Rfi => "rfi",
            Self::Crandc => "crandc",
            Self::Isync => "isync",
            Self::Crxor => "crxor",
            Self::Crnand => "crnand",
            Self::Crand => "crand",
            Self::Creqv => "creqv",
            Self::Crorc => "crorc",
            Self::Cror => "cror",
            Self::Bcctr => "bcctr",
            Self::Rlwimi => "rlwimi",
            Self::Rlwinm => "rlwinm",
            Self::Rlwnm => "rlwnm",
            Self::Ori => "ori",
            Self::Oris => "oris",
            Self::Xori => "xori",
            Self::Xoris => "xoris",
            Self::Andi_ => "andi.",
            Self::Andis_ => "andis.",
            Self::Cmp => "cmp",
            Self::Tw => "tw",
            Self::Subfc => "subfc",
            Self::Addc => "addc",
            Self::Mulhwu => "mulhwu",
            Self::Mfcr => "mfcr",
            Self::Lwarx => "lwarx",
            Self::Lwzx => "lwzx",
            Self::Slw => "slw",
            Self::Cntlzw => "cntlzw",
            Self::And => "and",
            Self::Cmpl => "cmpl",
            Self::Subf => "subf",
            Self::Dcbst => "dcbst",
            Self::Lwzux => "lwzux",
            Self::Andc => "andc",
            Self::Mulhw => "mulhw",
            Self::Mfmsr => "mfmsr",
            Self::Dcbf => "dcbf",
            Self::Lbzx => "lbzx",
            Self::Neg => "neg",
            Self::Lbzux => "lbzux",
            Self::Nor => "nor",
            Self::Subfe => "subfe",
            Self::Adde => "adde",
            Self::Mtcrf => "mtcrf",
            Self::Mtmsr => "mtmsr",
            Self::Stwcx_ => "stwcx_",
            Self::Stwx => "stwx",
            Self::Stwux => "stwux",
            Self::Subfze => "subfze",
            Self::Addze => "addze",
            Self::Mtsr => "mtsr",
            Self::Stbx => "stbx",
            Self::Subfme => "subfme",
            Self::Addme => "addme",
            Self::Mullw => "mullw",
            Self::Mtsrin => "mtsrin",
            Self::Dcbtst => "dcbtst",
            Self::Stbux => "stbux",
            Self::Add => "add",
            Self::Dcbt => "dcbt",
            Self::Lhzx => "lhzx",
            Self::Eqv => "eqv",
            Self::Tlbie => "tlbie",
            Self::Eciwx => "eciwx",
            Self::Lhzux => "lhzux",
            Self::Xor => "xor",
            Self::Mfspr => "mfspr",
            Self::Lhax => "lhax",
            Self::Mftb => "mftb",
            Self::Lhaux => "lhaux",
            Self::Sthx => "sthx",
            Self::Orc => "orc",
            Self::Ecowx => "ecowx",
            Self::Sthux => "sthux",
            Self::Or => "or",
            Self::Divwu => "divwu",
            Self::Mtspr => "mtspr",
            Self::Dcbi => "dcbi",
            Self::Nand => "nand",
            Self::Divw => "divw",
            Self::Mcrxr => "mcrxr",
            Self::Lswx => "lswx",
            Self::Lwbrx => "lwbrx",
            Self::Lfsx => "lfsx",
            Self::Srw => "srw",
            Self::Tlbsync => "tlbsync",
            Self::Lfsux => "lfsux",
            Self::Mfsr => "mfsr",
            Self::Lswi => "Lswi",
            Self::Sync => "sync",
            Self::Lfdx => "lfdx",
            Self::Lfdux => "lfdux",
            Self::Mfsrin => "mfsrin",
            Self::Stswx => "stswx",
            Self::Stwbrx => "stwbrx",
            Self::Stfsx => "stfsx",
            Self::Stfsux => "stfsux",
            Self::Stswi => "stswi",
            Self::Stfdx => "stfdx",
            Self::Stfdux => "stfdux",
            Self::Lhbrx => "Lhbrx",
            Self::Sraw => "sraw",
            Self::Srawi => "srawi",
            Self::Eieio => "eieio",
            Self::Sthbrx => "sthbrx",
            Self::Extsh => "extsh",
            Self::Extsb => "extsb",
            Self::Icbi => "icbi",
            Self::Stfiwx => "stfiwx",
            Self::Dcbz => "dcbz",
            Self::Lwz => "lwz",
            Self::Lwzu => "lwzu",
            Self::Lbz => "lbz",
            Self::Lbzu => "lbzu",
            Self::Stw => "stw",
            Self::Stwu => "stwu",
            Self::Stb => "stb",
            Self::Stbu => "stbu",
            Self::Lhz => "lhz",
            Self::Lhzu => "lhzu",
            Self::Lha => "lha",
            Self::Lhau => "lhau",
            Self::Sth => "sth",
            Self::Sthu => "sthu",
            Self::Lmw => "lmw",
            Self::Stmw => "stmw",
            Self::Lfs => "lfs",
            Self::Lfsu => "lfsu",
            Self::Lfd => "lfd",
            Self::Lfdu => "lfdu",
            Self::Stfs => "stfs",
            Self::Stfsu => "stfsu",
            Self::Stfd => "stfd",
            Self::Stfdu => "stfdu",
            Self::PsqL => "psq_l",
            Self::PsqLu => "psq_lu",
            Self::Fdivsx => "fdivsx",
            Self::Fsubsx => "fsubsx",
            Self::Fadds => "fadds",
            Self::Fresx => "fresx",
            Self::Fmulsx => "fmulsx",
            Self::Fmsubsx => "fmsubsx",
            Self::Fmaddsx => "fmaddsx",
            Self::Fnmsubsx => "fnmsubsx",
            Self::Fnmaddsx => "fnmaddsx",
            Self::PsqSt => "psq_st",
            Self::PsqStu => "psq_stu",
            Self::Fcmpu => "fcmpu",
            Self::Frspx => "frspx",
            Self::Fctiw => "fctiw",
            Self::Fctiwz => "fctiwz",
            Self::Fdivx => "fdivx",
            Self::Fsubx => "fsubx",
            Self::Fadd => "fadd",
            Self::Fselx => "fselx",
            Self::Fmulx => "fmulx",
            Self::Frsqrte => "frsqrte",
            Self::Fmsubx => "fmsubx",
            Self::Fmaddx => "fmaddx",
            Self::Fnmsubx => "fnmsubx",
            Self::Fnmaddx => "fnmaddx",
            Self::Fcmpo => "fcmpo",
            Self::Mtfsb1 => "mtfsb1",
            Self::Fnegx => "fnegx",
            Self::Mcrfs => "mcrfs",
            Self::Mtfsb0 => "mtfsb0",
            Self::Fmrx => "fmrx",
            Self::Mtfsfi => "mtfsfi",
            Self::Fnabsx => "fnabsx",
            Self::Fabs => "fabs",
            Self::Mffs => "mffs",
            Self::Mtfsf => "mtfsf",
        }
    }
}

#[derive(Default, Clone)]
struct Ins {
    code: u32, // 0..32
    op: Opcode,
    // TODO move these struct members out to functions
    li: u32,  // 6..30
    imm: u16, // 16..32
    bd: u16,  // 16..30
    p2: u8,   // 6..11: S, D, TO, crbD, BO
    p3: u8,   // 11..16: A, crbA, BI
    p4: u8,   // 16..21: B, crbB, SH
    p5: u8,   // 21..26: C, MB
    p6: u8,   // 26..31: ME
    i: u8,    // 22..25
    crfd: u8, // 6..9
}

fn bit(x: u32, idx: usize) -> bool {
    ((x >> (32 - idx - 1)) & 1) == 1
}

fn bits<F>(x: u32, range: Range<usize>) -> F
where
    F: PrimInt,
{
    cast((x >> (32 - range.end)) & ((1 << range.len()) - 1)).unwrap()
}

macro_rules! disasm_unreachable {
    ($msg:expr $(,)?) => ({
        panic!("internal error: entered unreachable code disassembling instruction 0x{:08x}", $msg)
    });
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

    // TODO These can be macros

    fn w(&self) -> bool {
        bit(self.code, 21)
    }

    fn rc(&self) -> bool {
        bit(self.code, 31)
    }

    fn aa(&self) -> bool {
        bit(self.code, 30)
    }

    fn lk(&self) -> bool {
        bit(self.code, 31)
    }

    fn l(&self) -> bool {
        bit(self.code, 10)
    }

    fn oe(&self) -> bool {
        bit(self.code, 21)
    }

    fn crm(&self) -> u32 {
        bits(self.code, 12..20)
    }

    fn sr(&self) -> u32 {
        bits(self.code, 12..16)
    }

    fn spr(&self) -> u32 {
        bits(self.code, 11..21)
    }

    fn fm(&self) -> u32 {
        bits(self.code, 7..15)
    }

    fn crf_d(&self) -> u8 {
        bits(self.code, 6..9)
    }

    fn crf_s(&self) -> u8 {
        bits(self.code, 11..14)
    }

    fn disasm(x: u32) -> Self {
        let family = bits(x, 0..6);
        match family {
            0b000011 => {
                let mut ins = Ins::new(x);
                ins.op = Opcode::Twi;
                ins.p2 = bits(x, 6..11);
                ins.imm = bits(x, 16..32);
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
                if bits::<u32>(x, 9..11) != 0 || bit(x, 31) {
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
        ins.imm = bits(x, 16..32);
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
        ins.li = bits(x, 6..30);
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
        ins.imm = bits(x, 16..32);
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
                if bits::<u32>(x, 9..21) != 0 || bit(x, 31) {
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
                ins.op = Opcode::Fdivsx;
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10100 => {
                ins.op = Opcode::Fsubsx;
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
                ins.op = Opcode::Fresx;
                if ins.p3 != 0 || ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11001 => {
                ins.op = Opcode::Fmulsx;
                if ins.p4 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b11100 => ins.op = Opcode::Fmsubsx,
            0b11101 => ins.op = Opcode::Fmaddsx,
            0b11110 => ins.op = Opcode::Fnmsubsx,
            0b11111 => ins.op = Opcode::Fnmaddsx,
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
                    if bits::<u32>(x, 9..11) != 0 || bit(x, 31) {
                        ins.op = Opcode::Illegal;
                    }
                }
                0b01 => {
                    ins.op = Opcode::Fcmpo;
                    if bits::<u32>(x, 9..11) != 0 || bit(x, 31) {
                        ins.op = Opcode::Illegal;
                    }
                }
                0b10 => {
                    ins.op = Opcode::Mcrfs;
                    if bits::<u32>(x, 9..11) != 0 || bits::<u32>(x, 14..16) != 0 || bit(x, 31) {
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
                        if bits::<u32>(x, 9..16) != 0 || bit(x, 20) {
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
                    0b0001 => Opcode::Fnegx,
                    0b0010 => Opcode::Fmrx,
                    0b0100 => Opcode::Fnabsx,
                    0b1000 => Opcode::Fabs,
                    _ => Opcode::Illegal,
                };
                if ins.p3 != 0 {
                    ins.op = Opcode::Illegal
                }
            }
            0b01100 => {
                ins.op = Opcode::Frspx;
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
                ins.op = Opcode::Fdivx;
                if ins.p5 != 0 {
                    ins.op = Opcode::Illegal;
                }
            }
            0b10100 => {
                ins.op = Opcode::Fsubx;
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
            0b10111 => ins.op = Opcode::Fselx,
            0b11001 => {
                ins.op = Opcode::Fmulx;
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
            0b11100 => ins.op = Opcode::Fmsubx,
            0b11101 => ins.op = Opcode::Fmaddx,
            0b11110 => ins.op = Opcode::Fnmsubx,
            0b11111 => ins.op = Opcode::Fnmaddx,
            _ => (),
        }
        ins
    }

    fn format_codewarrior(self) -> String {
        format!("{} todo", self.op.name())
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
    for x in 0u32..0xFFFF_FFFFu32 {
        black_box(Ins::disasm(x));
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
        assert_eq!(bits::<u32>(0b00000101100000000000000000000000u32, 5..9), 0b1011u32);
        assert_eq!(bit(0b00000101100000000000000000000000u32, 5), true);
    }

    #[test]
    fn test_opcodes() {
        // twi
        assert_eq!(Ins::disasm(0b000011_00000_00000_0000000000000000).op, Opcode::Twi);
        // ps_cmpu0
        assert_eq!(Ins::disasm(0b000100_00000_00000_00000_0000000000_0).op, Opcode::PsCmpu0);
        assert_eq!(Ins::disasm(0b000100_00000_00000_00000_0000000000_1).op, Opcode::Illegal);
        assert_eq!(Ins::disasm(0b000100_00001_00000_00000_0000000000_0).op, Opcode::Illegal);
        // psq_lx
        assert_eq!(Ins::disasm(0b000100_00001_00000_00000_0000000110_0).op, Opcode::PsqLx);
        assert_eq!(Ins::disasm(0b000100_00001_00000_00000_0000000110_1).op, Opcode::Illegal);
        assert_eq!(Ins::disasm(0b000100_00001_00000_00000_0000000111_0).op, Opcode::PsqStx);
        assert_eq!(Ins::disasm(0b000100_00001_00000_00000_0000000111_1).op, Opcode::Illegal);
        // TODO more tests
    }
}
