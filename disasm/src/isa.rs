#![allow(clippy::bad_bit_mask)]

use crate::{bit, bits};
use ppc750cl_macros::isa;

isa! {
    "add" & 0xfc0007fe == 0x7c000214;
    //"addc" & 0xfc0007fe == 0x7c00002a;
    "addc" & 0x0 == 0x0;
    "adde" & 0xfc0007fe == 0x7c000114;
    "addi" & 0xfc000000 == 0x38000000;
    "addic" & 0xfc000000 == 0x30000000;
    "addic." & 0xfc000000 == 0x34000000;
    "addis" & 0xfc000000 == 0x3c000000;
    "addme" & 0xfc00fbfe == 0x7c0001d4;
    "addze" & 0xfc00fbfe == 0x7c000194;
    "and" & 0xfc0007fe == 0x7c000038;
    "andc" & 0xfc0007fe == 0x7c000078;
    "andi." & 0xfc000000 == 0x70000000;
    "andis." & 0xfc000000 == 0x74000000;
    "b" & 0xfc000000 == 0x48000000;
    //"bc" & 0xfc000000 == 0x40000000;
    "bc" & 0x0 == 0x0; // TODO
    //"bcctr" & 0xfc00ffff == 0x4c000210;
    "bcctr" & 0x0 == 0x0; // TODO
    "bclr" & 0xfc00fffe == 0x4c000020;
    "cmp" & 0xfc4007ff == 0x7c000000;
    "cmpi" & 0xfc400000 == 0x2c000000;
    "cmpl" & 0xfc4007ff == 0x7c000040;
    "cmpli" & 0xfc400000 == 0x28000000;
    "cntlzw" & 0xfc00fffe == 0x7c000034;
    "crand" & 0xfc0007ff == 0x4c000202;
    "crandc" & 0xfc0007ff == 0x4c000102;
    "creqv" & 0xfc0007ff == 0x4c000242;
    "crnand" & 0xfc0007ff == 0x4c0001c2;
    "crnor" & 0xfc0007ff == 0x4c000042;
    "cror" & 0xfc0007ff == 0x4c000382;
    "crorc" & 0xfc0007ff == 0x4c000342;
    "crxor" & 0xfc0007ff == 0x4c000182;
    "dcbf" & 0xffe007ff == 0x7c0000ac;
    "dcbi" & 0xffe007ff == 0x7c0003ac;
    "dcbst" & 0xffe007ff == 0x7c00006c;
    "dcbt" & 0xffe007ff == 0x7c00022c;
    "dcbtst" & 0xffe007ff == 0x7c0001ec;
    "dcbz" & 0xffe007ff == 0x7c0007ec;
    "dcbz_l" & 0xffe007ff == 0x100007ec;
    "divw" & 0xfc0003fe == 0x7c0003d6;
    "divwu" & 0xfc0003fe == 0x7c000396;
    "eciwx" & 0xfc0003ff == 0x7c00026c;
    "ecowx" & 0xfc0003ff == 0x7c00036c;
    "eieio" & 0xffffffff == 0x7c0006ac;
    "eqv" & 0xfc0003fe == 0x7c000238;
    "extsb" & 0xfc00fffe == 0x7c000774;
    "extsh" & 0xfc00fffe == 0x7c000734;
    //"fabs" & 0xfc1f07fe == 0xfc000734;
    "fabs" & 0x0 == 0x0; // TODO
    "fadd" & 0xfc0007fe == 0xfc00002a;
    "fadds" & 0xfc0007fe == 0xec00002a;
    "fcmpo" & 0xfc6007ff == 0xfc000040;
    "fcmpu" & 0xfc6007ff == 0xfc000000;
    "fctiw" & 0xfc1f07fe == 0xfc00001c;
    "fctiwz" & 0xfc1f07fe == 0xfc00001e;
    "fdiv" & 0xfc0007fe == 0xfc000024;
    "fdivs" & 0xfc0007fe == 0xec000024;
    "fmadd" & 0xfc00003e == 0xfc00003a;
    "fmadds" & 0xfc00003e == 0xec00003a;
    "fmr" & 0xfc1f07fe == 0xfc000090;
    "fmsub" & 0xfc00003e == 0xfc000038;
    "fmsubs" & 0xfc00003e == 0xec000038;
    "fmul" & 0xfc00f83e == 0xfc000032;
    "fmuls" & 0xfc00f83e == 0xec000032;
    "fnabs" & 0xfc1f07fe == 0xfc000110;
    //"fneg" & 0xfc1f07fe == 0xfc000050;
    "fneg" & 0x0 == 0x0; // TODO
    "fnmadd" & 0xfc00003e == 0xfc00003e;
    "fnmadds" & 0xfc00003e == 0xec00003e;
    "fnmsub" & 0xfc00003e == 0xfc00003c;
    "fnmsubs" & 0xfc00003e == 0xec00003c;
    "fres" & 0xfc1f07fe == 0xec000030;
    "frsp" & 0xfc1f07fe == 0xfc000018;
    "frsqrte" & 0xfc1f07fe == 0xfc000034;
    "fsel" & 0xfc00003e == 0xfc00002e;
    "fsub" & 0xfc0007fe == 0xfc000028;
    "fsubs" & 0xfc0007fe == 0xec000028;
    "icbi" & 0xffe007ff == 0x7c0007ac;
    "isync" & 0xffffffff == 0x4c00012c;
    "lbz" & 0xfc000000 == 0x88000000;
    "lbzu" & 0xfc000000 == 0x8c000000;
    "lbzux" & 0xfc0007ff == 0x7c0000ee;
    "lbzx" & 0xfc0007ff == 0x7c0000ae;
    "lfd" & 0xfc000000 == 0xc8000000;
    "lfdu" & 0xfc000000 == 0xcc000000;
    "lfdux" & 0xfc0007ff == 0x7c0004ee;
    //"lfdx" & 0xfc0007ff == 0x7c00045e;
    "lfdx" & 0x0 == 0x0;
    "lfs" & 0xfc000000 == 0xc0000000;
    "lfsu" & 0xfc000000 == 0xc4000000;
    "lfsux" & 0xfc0007ff == 0x7c00046e;
    "lfsx" & 0xfc0007ff == 0x7c00042e;
    "lha" & 0xfc000000 == 0xa8000000;
    "lhau" & 0xfc000000 == 0xac000000;
    "lhaux" & 0xfc0007ff == 0x7c0002ee;
    "lhax" & 0xfc0007ff == 0x7c0002ae;
    "lhbrx" & 0xfc0007ff == 0x7c00062c;
    "lhz" & 0xfc000000 == 0xa0000000;
    "lhzu" & 0xfc000000 == 0xa4000000;
    "lhzux" & 0xfc0007ff == 0x7c00026e;
    "lhzx" & 0xfc0007ff == 0x7c00022e;
    "lmw" & 0xfc000000 == 0xb8000000;
    "lswi" & 0xfc0007ff == 0x7c0004aa;
    "lswx" & 0xfc0007ff == 0x7c00042a;
    "lwarx" & 0xfc0007ff == 0x7c000028;
    "lwbrx" & 0xfc0007ff == 0x7c00042c;
    "lwz" & 0xfc000000 == 0x80000000;
    "lwzu" & 0xfc000000 == 0x84000000;
    "lwzux" & 0xfc0007ff == 0x7c00006e;
    "lwzx" & 0xfc0007ff == 0x7c00002e;
    "mcrf" & 0xfc300fff == 0x4c000000;
    "mcrfs" & 0xfc30ffff == 0xfc000080;
    "mcrxr" & 0xfc30ffff == 0x7c000400;
    "mfcr" & 0xfc1fffff == 0x7c000026;
    //"mffs" & 0xfc1ffffe == 0x7c00048e;
    "mffs" & 0x0 == 0x0; // TODO
    "mfmsr" & 0xfc1fffff == 0x7c0000a6;
    "mfspr" & 0xfc0007ff == 0x7c0002a6;
    "mfsr" & 0xfc10ffff == 0x7c0004a6;
    "mfsrin" & 0xfc1f07ff == 0x7c000526;
    "mftb" & 0xfc0007ff == 0x7c0002e6;
    "mtcrf" & 0xfc100fff == 0x7c000120;
    "mtfsb0" & 0xfc1ffffe == 0xfc00008c;
    "mtfsb1" & 0xfc1ffffe == 0xfc00004c;
    "mtfsf" & 0xfe0107fe == 0xfc00058e;
    "mtfsfi" & 0xfc7f0ffe == 0xfc00010c;
    "mtmsr" & 0xfc1fffff == 0x7c000124;
    "mtspr" & 0xfc0007ff == 0x7c0003a6;
    "mtsr" & 0xfc10ffff == 0x7c0001a4;
    "mtsrin" & 0xfc1f07ff == 0x7c0001e4;
    //"mulhw" & 0xfc0007fe == 0x7c000096;
    "mulhw" & 0x0 == 0x0;
    //"mulhwu" & 0xfc0007fe == 0x7c000016;
    "mulhwu" & 0x0 == 0x0;
    "mulli" & 0xfc000000 == 0x1c000000;
    "mullw" & 0xfc0003fe == 0x7c0001d6;
    "nand" & 0xfc0007fe == 0x7c0003b8;
    "neg" & 0xfc00fffe == 0x7c0000d0;
    "nor" & 0xfc0007fe == 0x7c0000f8;
    "or" & 0xfc0007fe == 0x7c000378;
    "orc" & 0xfc0007fe == 0x7c000338;
    "ori" & 0xfc000000 == 0x60000000;
    "oris" & 0xfc000000 == 0x64000000;
    "psq_l" & 0xfc000000 == 0xe0000000;
    "psq_lu" & 0xfc000000 == 0xe4000000;
    "psq_lux" & 0xfc00007f == 0x1000004c;
    "psq_lx" & 0xfc00007f == 0x1000000c;
    "psq_st" & 0xfc000000 == 0xf0000000;
    "psq_stu" & 0xfc000000 == 0xf4000000;
    "psq_stux" & 0xfc00007f == 0x1000004e;
    "psq_stx" & 0xfc00007f == 0x1000000e;
    "ps_abs" & 0xfc1f07fe == 0x10000210;
    "ps_add" & 0xfc0007fe == 0x1000002a;
    "ps_cmpo0" & 0xfc6007ff == 0x10000040;
    "ps_cmpo1" & 0xfc6007ff == 0x100000c0;
    "ps_cmpu0" & 0xfc6007ff == 0x10000000;
    "ps_cmpu1" & 0xfc6007ff == 0x10000080;
    "ps_div" & 0xfc0007fe == 0x10000024;
    "ps_madd" & 0xfc00003e == 0x1000003a;
    "ps_madds0" & 0xfc00003e == 0x1000001c;
    "ps_madds1" & 0xfc00003e == 0x1000001e;
    "ps_merge00" & 0xfc0007fe == 0x10000420;
    "ps_merge01" & 0xfc0007fe == 0x10000460;
    "ps_merge10" & 0xfc0007fe == 0x100004a0;
    "ps_merge11" & 0xfc0007fe == 0x100004e0;
    "ps_mr" & 0xfc1f07fe == 0x10000090;
    "ps_msub" & 0xfc00003e == 0x10000038;
    "ps_mul" & 0xfc00f83e == 0x10000032;
    "ps_muls0" & 0xfc00f83e == 0x10000018;
    "ps_muls1" & 0xfc00f83e == 0x1000001a;
    "ps_nabs" & 0xfc1f07fe == 0x10000110;
    "ps_neg" & 0xfc1f07fe == 0x10000050;
    "ps_nmadd" & 0xfc00003e == 0x1000003e;
    "ps_nmsub" & 0xfc00003e == 0x1000003c;
    "ps_res" & 0xfc1f07fe == 0x10000030;
    "ps_rsqrte" & 0xfc1f07fe == 0x10000034;
    "ps_sel" & 0xfc00003e == 0x1000002e;
    "ps_sub" & 0xfc0007fe == 0x10000028;
    "ps_sum0" & 0xfc00003e == 0x10000014;
    "ps_sum1" & 0xfc00003e == 0x10000016;
    "rfi" & 0xfffff801 == 0x4c000000;
    "rlwimi" & 0xfc000000 == 0x50000000;
    "rlwinm" & 0xfc000000 == 0x54000000;
    "rlwnm" & 0xfc000000 == 0x5c000000;
    "sc" & 0xffffffff == 0x44000002;
    "slw" & 0xfc0007fe == 0x7c000030;
    "sraw" & 0xfc0007fe == 0x7c000630;
    "srawi" & 0xfc0007fe == 0x7c000670;
    "srw" & 0xfc0007fe == 0x7c000430;
    "stb" & 0xfc000000 == 0x98000000;
    "stbu" & 0xfc000000 == 0x9c000000;
    "stbux" & 0xfc0003ff == 0x7c0001ee;
    "stbx" & 0xfc0003ff == 0x7c0001ae;
    "stfd" & 0xfc000000 == 0xd8000000;
    "stfdu" & 0xfc000000 == 0xdc000000;
    "stfdux" & 0xfc0007ff == 0x7c0005ee;
    "stfdx" & 0xfc0007ff == 0x7c0005ae;
    "stfiwx" & 0xfc0007ff == 0x7c0007ae;
    "stfs" & 0xfc000000 == 0xd0000000;
    "stfsu" & 0xfc000000 == 0xd4000000;
    "stfsux" & 0xfc0007ff == 0x7c00056e;
    "stfsx" & 0xfc0007ff == 0x7c00052e;
    "sth" & 0xfc000000 == 0xb0000000;
    "sthbrx" & 0xfc0007ff == 0x7c00072c;
    "sthu" & 0xfc000000 == 0xb4000000;
    "sthux" & 0xfc0007ff == 0x7c00036e;
    "sthx" & 0xfc0007ff == 0x7c00032e;
    "stmw" & 0xfc000000 == 0xbc000000;
    "stswi" & 0xfc0007ff == 0x7c0005aa;
    "stswx" & 0xfc0007ff == 0x7c00052a;
    "stw" & 0xfc000000 == 0x90000000;
    "stwbrx" & 0xfc0007ff == 0x7c00052c;
    "stwcx." & 0xfc0007ff == 0x7c00012d;
    "stwu" & 0xfc000000 == 0x94000000;
    "stwux" & 0xfc0007ff == 0x7c00016e;
    "stwx" & 0xfc0007ff == 0x7c00012e;
    "subf" & 0xfc0003fe == 0x7c000050;
    "subfc" & 0xfc0003fe == 0x7c000010;
    "subfe" & 0xfc0003fe == 0x7c000110;
    "subfic" & 0xfc000000 == 0x20000000;
    "subfme" & 0xfc00fbfe == 0x7c0001d0;
    "subfze" & 0xfc00fbfe == 0x7c000190;
    "sync" & 0xffffffff == 0x7c0004ac;
    "tlbie" & 0xffff07ff == 0x7c000264;
    "tlbsync" & 0xffffffff == 0x7c00046c;
    "tw" & 0xfc0007ff == 0x7c000008;
    "twi" & 0xfc000000 == 0xc000000;
    "xor" & 0xfc0007fe == 0x7c000278;
    "xori" & 0xfc000000 == 0x68000000;
    "xoris" & 0xfc000000 == 0x6c000000;
}

impl Opcode {
    pub const BLR: u32 = 0x4c000020;

    pub fn from_code(x: u32) -> Self {
        let op = match bits(x, 0..6) {
            0b000011 => Opcode::Twi,
            0b000100 => Self::from_code_cl_ext(x),
            0b000111..=0b001111 => Self::from_code_basic1(x),
            0b010000 => Opcode::Bc,
            0b010001 => Opcode::Sc,
            0b010010 => Opcode::B,
            0b010011 => Self::from_code_010011(x),
            0b010100..=0b011101 => Self::from_code_basic2(x),
            0b011111 => Self::from_code_011111(x),
            0b100000..=0b110111 => Self::from_code_basic3(x),
            0b111000..=0b111001 => Self::from_code_psq(x),
            0b111011 => Self::from_code_111011(x),
            0b111100..=0b111101 => Self::from_code_psq(x),
            0b111111 => Self::from_code_111111(x),
            _ => Opcode::Illegal,
        };
        if !op.is_valid(x) {
            return Opcode::Illegal;
        }
        op
    }

    fn from_code_cl_ext(x: u32) -> Self {
        match bits(x, 26..31) {
            0b00000 => match bits(x, 21..26) {
                0b00000 => Opcode::PsCmpu0,
                0b00001 => Opcode::PsCmpo0,
                0b00010 => Opcode::PsCmpu1,
                0b00011 => Opcode::PsCmpo1,
                _ => Opcode::Illegal,
            },
            0b00110 => {
                if bit(x, 25) == 0 {
                    Opcode::PsqLx
                } else {
                    Opcode::PsqLux
                }
            }
            0b00111 => {
                if bit(x, 25) == 0 {
                    Opcode::PsqStx
                } else {
                    Opcode::PsqStux
                }
            }
            0b01010 => Opcode::PsSum0,
            0b01011 => Opcode::PsSum1,
            0b01110 => Opcode::PsMadds0,
            0b01111 => Opcode::PsMadds1,
            0b10111 => Opcode::PsSel,
            0b11100 => Opcode::PsMsub,
            0b11101 => Opcode::PsMadd,
            0b11110 => Opcode::PsNmsub,
            0b11111 => Opcode::PsNmadd,
            0b01100 => Opcode::PsMuls0,
            0b01101 => Opcode::PsMuls1,
            0b11001 => Opcode::PsMul,
            0b10010 => Opcode::PsDiv,
            0b10100 => Opcode::PsSub,
            0b10101 => Opcode::PsAdd,
            0b11000 => Opcode::PsRes,
            0b11010 => Opcode::PsRsqrte,
            0b01000 => match bits(x, 21..26) {
                0b00001 => Opcode::PsNeg,
                0b00010 => Opcode::PsMr,
                0b00100 => Opcode::PsNabs,
                0b01000 => Opcode::PsAbs,
                _ => Opcode::Illegal,
            },
            0b10000 => match bits(x, 21..26) {
                0b10000 => Opcode::PsMerge00,
                0b10001 => Opcode::PsMerge01,
                0b10010 => Opcode::PsMerge10,
                0b10011 => Opcode::PsMerge11,
                _ => Opcode::Illegal,
            },
            0b10110 => Opcode::DcbzL,
            // Unknown paired-singles key.
            _ => Opcode::Illegal,
        }
    }

    fn from_code_basic1(x: u32) -> Self {
        match bits(x, 0..6) {
            0b000111 => Opcode::Mulli,
            0b001000 => Opcode::Subfic,
            0b001010 => Opcode::Cmpli,
            0b001011 => Opcode::Cmpi,
            0b001100 => Opcode::Addic,
            0b001101 => Opcode::Addic_,
            0b001110 => Opcode::Addi,
            0b001111 => Opcode::Addis,
            _ => Opcode::Illegal,
        }
    }

    fn from_code_010011(x: u32) -> Self {
        match bits(x, 21..27) {
            0b000000 => Opcode::Mcrf,
            0b000001 => Opcode::Bclr,
            0b100001 => Opcode::Bcctr,
            0b000011 => Opcode::Rfi,
            0b001001 => Opcode::Isync,
            0b000010 => Opcode::Crnor,
            0b001000 => Opcode::Crandc,
            0b001100 => Opcode::Crxor,
            0b001110 => Opcode::Crnand,
            0b010000 => Opcode::Crand,
            0b010010 => Opcode::Creqv,
            0b011010 => Opcode::Crorc,
            0b011100 => Opcode::Cror,
            _ => Opcode::Illegal,
        }
    }

    fn from_code_basic2(x: u32) -> Self {
        match bits(x, 0..6) {
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
        }
    }

    fn from_code_011111(x: u32) -> Self {
        match bits::<u32>(x, 21..31) {
            0b00_0000_0000 => Opcode::Cmp,
            0b00_0010_0000 => Opcode::Cmpl,
            0b00_0000_0100 => Opcode::Tw,
            0b00_0000_1000 => Opcode::Subfc,
            0b00_0000_1010 => Opcode::Addc,
            0b00_0000_1011 => Opcode::Mulhwu,
            0b00_0001_0011 => Opcode::Mfcr,
            0b00_0001_0100 => Opcode::Lwarx,
            0b00_0001_0111 => Opcode::Lwzx,
            0b00_0001_1000 => Opcode::Slw,
            0b00_0001_1010 => Opcode::Cntlzw,
            0b00_0001_1100 => Opcode::And,
            0b00_0010_1000 => Opcode::Subf,
            0b00_0011_0110 => Opcode::Dcbst,
            0b00_0011_0111 => Opcode::Lwzux,
            0b00_0011_1100 => Opcode::Andc,
            0b00_0100_1011 => Opcode::Mulhw,
            0b00_0101_0011 => Opcode::Mfmsr,
            0b00_0101_0110 => Opcode::Dcbf,
            0b00_0101_0111 => Opcode::Lbzx,
            0b00_0110_1000 => Opcode::Neg,
            0b00_0111_0111 => Opcode::Lbzux,
            0b00_0111_1100 => Opcode::Nor,
            0b00_1000_1000 => Opcode::Subfe,
            0b00_1000_1010 => Opcode::Adde,
            0b00_1001_0000 => Opcode::Mtcrf,
            0b00_1001_0010 => Opcode::Mtmsr,
            0b00_1001_0110 => Opcode::Stwcx_,
            0b00_1001_0111 => Opcode::Stwx,
            0b00_1011_0111 => Opcode::Stwux,
            0b00_1100_1000 => Opcode::Subfze,
            0b00_1100_1010 => Opcode::Addze,
            0b00_1101_0010 => Opcode::Mtsr,
            0b00_1101_0111 => Opcode::Stbx,
            0b00_1110_1000 => Opcode::Subfme,
            0b00_1110_1010 => Opcode::Addme,
            0b00_1110_1011 => Opcode::Mullw,
            0b00_1111_0010 => Opcode::Mtsrin,
            0b00_1111_0110 => Opcode::Dcbtst,
            0b00_1111_0111 => Opcode::Stbux,
            0b01_0000_1010 => Opcode::Add,
            0b01_0001_0110 => Opcode::Dcbt,
            0b01_0001_0111 => Opcode::Lhzx,
            0b01_0001_1100 => Opcode::Eqv,
            0b01_0011_0010 => Opcode::Tlbie,
            0b01_0011_0110 => Opcode::Eciwx,
            0b01_0011_0111 => Opcode::Lhzux,
            0b01_0011_1100 => Opcode::Xor,
            0b01_0101_0011 => Opcode::Mfspr,
            0b01_0101_0111 => Opcode::Lhax,
            0b01_0111_0011 => Opcode::Mftb,
            0b01_0111_0111 => Opcode::Lhaux,
            0b01_1001_0111 => Opcode::Sthx,
            0b01_1001_1100 => Opcode::Orc,
            0b01_1011_0110 => Opcode::Ecowx,
            0b01_1011_0111 => Opcode::Sthux,
            0b01_1011_1100 => Opcode::Or,
            0b01_1100_1011 => Opcode::Divwu,
            0b01_1101_0011 => Opcode::Mtspr,
            0b01_1101_0110 => Opcode::Dcbi,
            0b01_1101_1100 => Opcode::Nand,
            0b01_1110_1011 => Opcode::Divw,
            0b10_0000_0000 => Opcode::Mcrxr,
            0b10_0001_0101 => Opcode::Lswx,
            0b10_0001_0110 => Opcode::Lwbrx,
            0b10_0001_0111 => Opcode::Lfsx,
            0b10_0001_1000 => Opcode::Srw,
            0b10_0011_0110 => Opcode::Tlbsync,
            0b10_0011_0111 => Opcode::Lfsux,
            0b10_0101_0011 => Opcode::Mfsr,
            0b10_0101_0101 => Opcode::Lswi,
            0b10_0101_0110 => Opcode::Sync,
            0b10_0101_0111 => Opcode::Lfdx,
            0b10_0111_0111 => Opcode::Lfdux,
            0b10_1001_0011 => Opcode::Mfsrin,
            0b10_1001_0101 => Opcode::Stswx,
            0b10_1001_0110 => Opcode::Stwbrx,
            0b10_1001_0111 => Opcode::Stfsx,
            0b10_1011_0111 => Opcode::Stfsux,
            0b10_1101_0101 => Opcode::Stswi,
            0b10_1101_0111 => Opcode::Stfdx,
            0b10_1111_0111 => Opcode::Stfdux,
            0b11_0001_0110 => Opcode::Lhbrx,
            0b11_0001_1000 => Opcode::Sraw,
            0b11_0011_1000 => Opcode::Srawi,
            0b11_0101_0110 => Opcode::Eieio,
            0b11_1001_0110 => Opcode::Sthbrx,
            0b11_1001_1010 => Opcode::Extsh,
            0b11_1011_1010 => Opcode::Extsb,
            0b11_1101_0110 => Opcode::Icbi,
            0b11_1101_0111 => Opcode::Stfiwx,
            0b11_1111_0110 => Opcode::Dcbz,
            _ => Opcode::Illegal,
        }
    }

    fn from_code_basic3(x: u32) -> Self {
        match bits(x, 0..6) {
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
        }
    }

    fn from_code_psq(x: u32) -> Self {
        match bits(x, 0..6) {
            0b111000 => Opcode::PsqL,
            0b111001 => Opcode::PsqLu,
            0b111100 => Opcode::PsqSt,
            0b111101 => Opcode::PsqStu,
            _ => disasm_unreachable!(x),
        }
    }

    fn from_code_111011(x: u32) -> Self {
        match bits(x, 26..31) {
            0b10010 => Opcode::Fdivs,
            0b10100 => Opcode::Fsubs,
            0b10101 => Opcode::Fadds,
            0b11000 => Opcode::Fres,
            0b11001 => Opcode::Fmuls,
            0b11100 => Opcode::Fmsubs,
            0b11101 => Opcode::Fmadds,
            0b11110 => Opcode::Fnmsubs,
            0b11111 => Opcode::Fnmadds,
            _ => Opcode::Illegal,
        }
    }

    fn from_code_111111(x: u32) -> Self {
        match bits::<u32>(x, 26..31) {
            0b00000 => match bits(x, 24..26) {
                0b00 => Opcode::Fcmpu,
                0b01 => Opcode::Fcmpo,
                0b10 => Opcode::Mcrfs,
                _ => Opcode::Illegal,
            },
            0b00110 => match bits(x, 23..26) {
                0b001 => Opcode::Mtfsb1,
                0b010 => Opcode::Mtfsb0,
                0b100 => Opcode::Mtfsfi,
                _ => Opcode::Illegal,
            },
            0b00111 => match bits(x, 21..26) {
                0b10010 => Opcode::Mffs,
                0b10110 => Opcode::Mtfsf,
                _ => Opcode::Illegal,
            },
            0b01000 => match bits(x, 22..26) {
                0b0001 => Opcode::Fneg,
                0b0010 => Opcode::Fmr,
                0b0100 => Opcode::Fnabs,
                0b1000 => Opcode::Fabs,
                _ => Opcode::Illegal,
            },
            0b01100 => Opcode::Frsp,
            0b01110 => Opcode::Fctiw,
            0b01111 => Opcode::Fctiwz,
            0b10010 => Opcode::Fdiv,
            0b10100 => Opcode::Fsub,
            0b10101 => Opcode::Fadd,
            0b10111 => Opcode::Fsel,
            0b11001 => Opcode::Fmul,
            0b11010 => Opcode::Frsqrte,
            0b11100 => Opcode::Fmsub,
            0b11101 => Opcode::Fmadd,
            0b11110 => Opcode::Fnmsub,
            0b11111 => Opcode::Fnmadd,
            _ => Opcode::Illegal,
        }
    }
}
