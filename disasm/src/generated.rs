use crate::prelude::*;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Opcode {
    Illegal = -1,
    Add,
    Addc,
    Adde,
    Addi,
    Addic,
    Addic_,
    Addis,
    Addme,
    Addze,
    And,
    Andc,
    Andi_,
    Andis_,
    B,
    Bc,
    Bcctr,
    Bclr,
    Cmp,
    Cmpi,
    Cmpl,
    Cmpli,
    Cntlzw,
    Crand,
    Crandc,
    Creqv,
    Crnand,
    Crnor,
    Cror,
    Crorc,
    Crxor,
    Dcbf,
    Dcbi,
    Dcbst,
    Dcbt,
    Dcbtst,
    Dcbz,
    DcbzL,
    Divw,
    Divwu,
    Eciwx,
    Ecowx,
    Eieio,
    Eqv,
    Extsb,
    Extsh,
    Fabs,
    Fadd,
    Fadds,
    Fcmpo,
    Fcmpu,
    Fctiw,
    Fctiwz,
    Fdiv,
    Fdivs,
    Fmadd,
    Fmadds,
    Fmr,
    Fmsub,
    Fmsubs,
    Fmul,
    Fmuls,
    Fnabs,
    Fneg,
    Fnmadd,
    Fnmadds,
    Fnmsub,
    Fnmsubs,
    Fres,
    Frsp,
    Frsqrte,
    Fsel,
    Fsub,
    Fsubs,
    Icbi,
    Isync,
    Lbz,
    Lbzu,
    Lbzux,
    Lbzx,
    Lfd,
    Lfdu,
    Lfdux,
    Lfdx,
    Lfs,
    Lfsu,
    Lfsux,
    Lfsx,
    Lha,
    Lhau,
    Lhaux,
    Lhax,
    Lhbrx,
    Lhz,
    Lhzu,
    Lhzux,
    Lhzx,
    Lmw,
    Lswi,
    Lswx,
    Lwarx,
    Lwbrx,
    Lwz,
    Lwzu,
    Lwzux,
    Lwzx,
    Mcrf,
    Mcrfs,
    Mcrxr,
    Mfcr,
    Mffs,
    Mfmsr,
    Mfspr,
    Mfsr,
    Mfsrin,
    Mftb,
    Mtcrf,
    Mtfsb0,
    Mtfsb1,
    Mtfsf,
    Mtfsfi,
    Mtmsr,
    Mtspr,
    Mtsr,
    Mtsrin,
    Mulhw,
    Mulhwu,
    Mulli,
    Mullw,
    Nand,
    Neg,
    Nor,
    Or,
    Orc,
    Ori,
    Oris,
    PsqL,
    PsqLu,
    PsqLux,
    PsqLx,
    PsqSt,
    PsqStu,
    PsqStux,
    PsqStx,
    PsAbs,
    PsAdd,
    PsCmpo0,
    PsCmpo1,
    PsCmpu0,
    PsCmpu1,
    PsDiv,
    PsMadd,
    PsMadds0,
    PsMadds1,
    PsMerge00,
    PsMerge01,
    PsMerge10,
    PsMerge11,
    PsMr,
    PsMsub,
    PsMul,
    PsMuls0,
    PsMuls1,
    PsNabs,
    PsNeg,
    PsNmadd,
    PsNmsub,
    PsRes,
    PsRsqrte,
    PsSel,
    PsSub,
    PsSum0,
    PsSum1,
    Rfi,
    Rlwimi,
    Rlwinm,
    Rlwnm,
    Sc,
    Slw,
    Sraw,
    Srawi,
    Srw,
    Stb,
    Stbu,
    Stbux,
    Stbx,
    Stfd,
    Stfdu,
    Stfdux,
    Stfdx,
    Stfiwx,
    Stfs,
    Stfsu,
    Stfsux,
    Stfsx,
    Sth,
    Sthbrx,
    Sthu,
    Sthux,
    Sthx,
    Stmw,
    Stswi,
    Stswx,
    Stw,
    Stwbrx,
    Stwcx_,
    Stwu,
    Stwux,
    Stwx,
    Subf,
    Subfc,
    Subfe,
    Subfic,
    Subfme,
    Subfze,
    Sync,
    Tlbie,
    Tlbsync,
    Tw,
    Twi,
    Xor,
    Xori,
    Xoris,
}
impl Opcode {
    pub(crate) fn _mnemonic(self) -> &'static str {
        match self {
            Opcode::Illegal => "<illegal>",
            Opcode::Add => "add",
            Opcode::Addc => "addc",
            Opcode::Adde => "adde",
            Opcode::Addi => "addi",
            Opcode::Addic => "addic",
            Opcode::Addic_ => "addic",
            Opcode::Addis => "addis",
            Opcode::Addme => "addme",
            Opcode::Addze => "addze",
            Opcode::And => "and",
            Opcode::Andc => "andc",
            Opcode::Andi_ => "andi",
            Opcode::Andis_ => "andis",
            Opcode::B => "b",
            Opcode::Bc => "bc",
            Opcode::Bcctr => "bcctr",
            Opcode::Bclr => "bclr",
            Opcode::Cmp => "cmp",
            Opcode::Cmpi => "cmpi",
            Opcode::Cmpl => "cmpl",
            Opcode::Cmpli => "cmpli",
            Opcode::Cntlzw => "cntlzw",
            Opcode::Crand => "crand",
            Opcode::Crandc => "crandc",
            Opcode::Creqv => "creqv",
            Opcode::Crnand => "crnand",
            Opcode::Crnor => "crnor",
            Opcode::Cror => "cror",
            Opcode::Crorc => "crorc",
            Opcode::Crxor => "crxor",
            Opcode::Dcbf => "dcbf",
            Opcode::Dcbi => "dcbi",
            Opcode::Dcbst => "dcbst",
            Opcode::Dcbt => "dcbt",
            Opcode::Dcbtst => "dcbtst",
            Opcode::Dcbz => "dcbz",
            Opcode::DcbzL => "dcbz_l",
            Opcode::Divw => "divw",
            Opcode::Divwu => "divwu",
            Opcode::Eciwx => "eciwx",
            Opcode::Ecowx => "ecowx",
            Opcode::Eieio => "eieio",
            Opcode::Eqv => "eqv",
            Opcode::Extsb => "extsb",
            Opcode::Extsh => "extsh",
            Opcode::Fabs => "fabs",
            Opcode::Fadd => "fadd",
            Opcode::Fadds => "fadds",
            Opcode::Fcmpo => "fcmpo",
            Opcode::Fcmpu => "fcmpu",
            Opcode::Fctiw => "fctiw",
            Opcode::Fctiwz => "fctiwz",
            Opcode::Fdiv => "fdiv",
            Opcode::Fdivs => "fdivs",
            Opcode::Fmadd => "fmadd",
            Opcode::Fmadds => "fmadds",
            Opcode::Fmr => "fmr",
            Opcode::Fmsub => "fmsub",
            Opcode::Fmsubs => "fmsubs",
            Opcode::Fmul => "fmul",
            Opcode::Fmuls => "fmuls",
            Opcode::Fnabs => "fnabs",
            Opcode::Fneg => "fneg",
            Opcode::Fnmadd => "fnmadd",
            Opcode::Fnmadds => "fnmadds",
            Opcode::Fnmsub => "fnmsub",
            Opcode::Fnmsubs => "fnmsubs",
            Opcode::Fres => "fres",
            Opcode::Frsp => "frsp",
            Opcode::Frsqrte => "frsqrte",
            Opcode::Fsel => "fsel",
            Opcode::Fsub => "fsub",
            Opcode::Fsubs => "fsubs",
            Opcode::Icbi => "icbi",
            Opcode::Isync => "isync",
            Opcode::Lbz => "lbz",
            Opcode::Lbzu => "lbzu",
            Opcode::Lbzux => "lbzux",
            Opcode::Lbzx => "lbzx",
            Opcode::Lfd => "lfd",
            Opcode::Lfdu => "lfdu",
            Opcode::Lfdux => "lfdux",
            Opcode::Lfdx => "lfdx",
            Opcode::Lfs => "lfs",
            Opcode::Lfsu => "lfsu",
            Opcode::Lfsux => "lfsux",
            Opcode::Lfsx => "lfsx",
            Opcode::Lha => "lha",
            Opcode::Lhau => "lhau",
            Opcode::Lhaux => "lhaux",
            Opcode::Lhax => "lhax",
            Opcode::Lhbrx => "lhbrx",
            Opcode::Lhz => "lhz",
            Opcode::Lhzu => "lhzu",
            Opcode::Lhzux => "lhzux",
            Opcode::Lhzx => "lhzx",
            Opcode::Lmw => "lmw",
            Opcode::Lswi => "lswi",
            Opcode::Lswx => "lswx",
            Opcode::Lwarx => "lwarx",
            Opcode::Lwbrx => "lwbrx",
            Opcode::Lwz => "lwz",
            Opcode::Lwzu => "lwzu",
            Opcode::Lwzux => "lwzux",
            Opcode::Lwzx => "lwzx",
            Opcode::Mcrf => "mcrf",
            Opcode::Mcrfs => "mcrfs",
            Opcode::Mcrxr => "mcrxr",
            Opcode::Mfcr => "mfcr",
            Opcode::Mffs => "mffs",
            Opcode::Mfmsr => "mfmsr",
            Opcode::Mfspr => "mfspr",
            Opcode::Mfsr => "mfsr",
            Opcode::Mfsrin => "mfsrin",
            Opcode::Mftb => "mftb",
            Opcode::Mtcrf => "mtcrf",
            Opcode::Mtfsb0 => "mtfsb0",
            Opcode::Mtfsb1 => "mtfsb1",
            Opcode::Mtfsf => "mtfsf",
            Opcode::Mtfsfi => "mtfsfi",
            Opcode::Mtmsr => "mtmsr",
            Opcode::Mtspr => "mtspr",
            Opcode::Mtsr => "mtsr",
            Opcode::Mtsrin => "mtsrin",
            Opcode::Mulhw => "mulhw",
            Opcode::Mulhwu => "mulhwu",
            Opcode::Mulli => "mulli",
            Opcode::Mullw => "mullw",
            Opcode::Nand => "nand",
            Opcode::Neg => "neg",
            Opcode::Nor => "nor",
            Opcode::Or => "or",
            Opcode::Orc => "orc",
            Opcode::Ori => "ori",
            Opcode::Oris => "oris",
            Opcode::PsqL => "psq_l",
            Opcode::PsqLu => "psq_lu",
            Opcode::PsqLux => "psq_lux",
            Opcode::PsqLx => "psq_lx",
            Opcode::PsqSt => "psq_st",
            Opcode::PsqStu => "psq_stu",
            Opcode::PsqStux => "psq_stux",
            Opcode::PsqStx => "psq_stx",
            Opcode::PsAbs => "ps_abs",
            Opcode::PsAdd => "ps_add",
            Opcode::PsCmpo0 => "ps_cmpo0",
            Opcode::PsCmpo1 => "ps_cmpo1",
            Opcode::PsCmpu0 => "ps_cmpu0",
            Opcode::PsCmpu1 => "ps_cmpu1",
            Opcode::PsDiv => "ps_div",
            Opcode::PsMadd => "ps_madd",
            Opcode::PsMadds0 => "ps_madds0",
            Opcode::PsMadds1 => "ps_madds1",
            Opcode::PsMerge00 => "ps_merge00",
            Opcode::PsMerge01 => "ps_merge01",
            Opcode::PsMerge10 => "ps_merge10",
            Opcode::PsMerge11 => "ps_merge11",
            Opcode::PsMr => "ps_mr",
            Opcode::PsMsub => "ps_msub",
            Opcode::PsMul => "ps_mul",
            Opcode::PsMuls0 => "ps_muls0",
            Opcode::PsMuls1 => "ps_muls1",
            Opcode::PsNabs => "ps_nabs",
            Opcode::PsNeg => "ps_neg",
            Opcode::PsNmadd => "ps_nmadd",
            Opcode::PsNmsub => "ps_nmsub",
            Opcode::PsRes => "ps_res",
            Opcode::PsRsqrte => "ps_rsqrte",
            Opcode::PsSel => "ps_sel",
            Opcode::PsSub => "ps_sub",
            Opcode::PsSum0 => "ps_sum0",
            Opcode::PsSum1 => "ps_sum1",
            Opcode::Rfi => "rfi",
            Opcode::Rlwimi => "rlwimi",
            Opcode::Rlwinm => "rlwinm",
            Opcode::Rlwnm => "rlwnm",
            Opcode::Sc => "sc",
            Opcode::Slw => "slw",
            Opcode::Sraw => "sraw",
            Opcode::Srawi => "srawi",
            Opcode::Srw => "srw",
            Opcode::Stb => "stb",
            Opcode::Stbu => "stbu",
            Opcode::Stbux => "stbux",
            Opcode::Stbx => "stbx",
            Opcode::Stfd => "stfd",
            Opcode::Stfdu => "stfdu",
            Opcode::Stfdux => "stfdux",
            Opcode::Stfdx => "stfdx",
            Opcode::Stfiwx => "stfiwx",
            Opcode::Stfs => "stfs",
            Opcode::Stfsu => "stfsu",
            Opcode::Stfsux => "stfsux",
            Opcode::Stfsx => "stfsx",
            Opcode::Sth => "sth",
            Opcode::Sthbrx => "sthbrx",
            Opcode::Sthu => "sthu",
            Opcode::Sthux => "sthux",
            Opcode::Sthx => "sthx",
            Opcode::Stmw => "stmw",
            Opcode::Stswi => "stswi",
            Opcode::Stswx => "stswx",
            Opcode::Stw => "stw",
            Opcode::Stwbrx => "stwbrx",
            Opcode::Stwcx_ => "stwcx",
            Opcode::Stwu => "stwu",
            Opcode::Stwux => "stwux",
            Opcode::Stwx => "stwx",
            Opcode::Subf => "subf",
            Opcode::Subfc => "subfc",
            Opcode::Subfe => "subfe",
            Opcode::Subfic => "subfic",
            Opcode::Subfme => "subfme",
            Opcode::Subfze => "subfze",
            Opcode::Sync => "sync",
            Opcode::Tlbie => "tlbie",
            Opcode::Tlbsync => "tlbsync",
            Opcode::Tw => "tw",
            Opcode::Twi => "twi",
            Opcode::Xor => "xor",
            Opcode::Xori => "xori",
            Opcode::Xoris => "xoris",
        }
    }
    pub(crate) fn _detect(code: u32) -> Self {
        if code & 0xfc0007fe == 0x7c000214 {
            return Opcode::Add;
        }
        if code & 0xfc0007fe == 0x7c000014 {
            return Opcode::Addc;
        }
        if code & 0xfc0007fe == 0x7c000114 {
            return Opcode::Adde;
        }
        if code & 0xfc000000 == 0x38000000 {
            return Opcode::Addi;
        }
        if code & 0xfc000000 == 0x30000000 {
            return Opcode::Addic;
        }
        if code & 0xfc000000 == 0x34000000 {
            return Opcode::Addic_;
        }
        if code & 0xfc000000 == 0x3c000000 {
            return Opcode::Addis;
        }
        if code & 0xfc00fbfe == 0x7c0001d4 {
            return Opcode::Addme;
        }
        if code & 0xfc00fbfe == 0x7c000194 {
            return Opcode::Addze;
        }
        if code & 0xfc0007fe == 0x7c000038 {
            return Opcode::And;
        }
        if code & 0xfc0007fe == 0x7c000078 {
            return Opcode::Andc;
        }
        if code & 0xfc000000 == 0x70000000 {
            return Opcode::Andi_;
        }
        if code & 0xfc000000 == 0x74000000 {
            return Opcode::Andis_;
        }
        if code & 0xfc000000 == 0x48000000 {
            return Opcode::B;
        }
        if code & 0xfc000000 == 0x40000000 {
            return Opcode::Bc;
        }
        if code & 0xfc00ffff == 0x4c000210 {
            return Opcode::Bcctr;
        }
        if code & 0xfc00fffe == 0x4c000020 {
            return Opcode::Bclr;
        }
        if code & 0xfc4007ff == 0x7c000000 {
            return Opcode::Cmp;
        }
        if code & 0xfc400000 == 0x2c000000 {
            return Opcode::Cmpi;
        }
        if code & 0xfc4007ff == 0x7c000040 {
            return Opcode::Cmpl;
        }
        if code & 0xfc400000 == 0x28000000 {
            return Opcode::Cmpli;
        }
        if code & 0xfc00fffe == 0x7c000034 {
            return Opcode::Cntlzw;
        }
        if code & 0xfc0007ff == 0x4c000202 {
            return Opcode::Crand;
        }
        if code & 0xfc0007ff == 0x4c000102 {
            return Opcode::Crandc;
        }
        if code & 0xfc0007ff == 0x4c000242 {
            return Opcode::Creqv;
        }
        if code & 0xfc0007ff == 0x4c0001c2 {
            return Opcode::Crnand;
        }
        if code & 0xfc0007ff == 0x4c000042 {
            return Opcode::Crnor;
        }
        if code & 0xfc0007ff == 0x4c000382 {
            return Opcode::Cror;
        }
        if code & 0xfc0007ff == 0x4c000342 {
            return Opcode::Crorc;
        }
        if code & 0xfc0007ff == 0x4c000182 {
            return Opcode::Crxor;
        }
        if code & 0xffe007ff == 0x7c0000ac {
            return Opcode::Dcbf;
        }
        if code & 0xffe007ff == 0x7c0003ac {
            return Opcode::Dcbi;
        }
        if code & 0xffe007ff == 0x7c00006c {
            return Opcode::Dcbst;
        }
        if code & 0xffe007ff == 0x7c00022c {
            return Opcode::Dcbt;
        }
        if code & 0xffe007ff == 0x7c0001ec {
            return Opcode::Dcbtst;
        }
        if code & 0xffe007ff == 0x7c0007ec {
            return Opcode::Dcbz;
        }
        if code & 0xffe007ff == 0x100007ec {
            return Opcode::DcbzL;
        }
        if code & 0xfc0003fe == 0x7c0003d6 {
            return Opcode::Divw;
        }
        if code & 0xfc0003fe == 0x7c000396 {
            return Opcode::Divwu;
        }
        if code & 0xfc0003ff == 0x7c00026c {
            return Opcode::Eciwx;
        }
        if code & 0xfc0003ff == 0x7c00036c {
            return Opcode::Ecowx;
        }
        if code & 0xffffffff == 0x7c0006ac {
            return Opcode::Eieio;
        }
        if code & 0xfc0003fe == 0x7c000238 {
            return Opcode::Eqv;
        }
        if code & 0xfc00fffe == 0x7c000774 {
            return Opcode::Extsb;
        }
        if code & 0xfc00fffe == 0x7c000734 {
            return Opcode::Extsh;
        }
        if code & 0xfc1f07fe == 0xfc000210 {
            return Opcode::Fabs;
        }
        if code & 0xfc0007fe == 0xfc00002a {
            return Opcode::Fadd;
        }
        if code & 0xfc0007fe == 0xec00002a {
            return Opcode::Fadds;
        }
        if code & 0xfc6007ff == 0xfc000040 {
            return Opcode::Fcmpo;
        }
        if code & 0xfc6007ff == 0xfc000000 {
            return Opcode::Fcmpu;
        }
        if code & 0xfc1f07fe == 0xfc00001c {
            return Opcode::Fctiw;
        }
        if code & 0xfc1f07fe == 0xfc00001e {
            return Opcode::Fctiwz;
        }
        if code & 0xfc0007fe == 0xfc000024 {
            return Opcode::Fdiv;
        }
        if code & 0xfc0007fe == 0xec000024 {
            return Opcode::Fdivs;
        }
        if code & 0xfc00003e == 0xfc00003a {
            return Opcode::Fmadd;
        }
        if code & 0xfc00003e == 0xec00003a {
            return Opcode::Fmadds;
        }
        if code & 0xfc1f07fe == 0xfc000090 {
            return Opcode::Fmr;
        }
        if code & 0xfc00003e == 0xfc000038 {
            return Opcode::Fmsub;
        }
        if code & 0xfc00003e == 0xec000038 {
            return Opcode::Fmsubs;
        }
        if code & 0xfc00f83e == 0xfc000032 {
            return Opcode::Fmul;
        }
        if code & 0xfc00f83e == 0xec000032 {
            return Opcode::Fmuls;
        }
        if code & 0xfc1f07fe == 0xfc000110 {
            return Opcode::Fnabs;
        }
        if code & 0xfc1f07fe == 0xfc000050 {
            return Opcode::Fneg;
        }
        if code & 0xfc00003e == 0xfc00003e {
            return Opcode::Fnmadd;
        }
        if code & 0xfc00003e == 0xec00003e {
            return Opcode::Fnmadds;
        }
        if code & 0xfc00003e == 0xfc00003c {
            return Opcode::Fnmsub;
        }
        if code & 0xfc00003e == 0xec00003c {
            return Opcode::Fnmsubs;
        }
        if code & 0xfc1f07fe == 0xec000030 {
            return Opcode::Fres;
        }
        if code & 0xfc1f07fe == 0xfc000018 {
            return Opcode::Frsp;
        }
        if code & 0xfc1f07fe == 0xfc000034 {
            return Opcode::Frsqrte;
        }
        if code & 0xfc00003e == 0xfc00002e {
            return Opcode::Fsel;
        }
        if code & 0xfc0007fe == 0xfc000028 {
            return Opcode::Fsub;
        }
        if code & 0xfc0007fe == 0xec000028 {
            return Opcode::Fsubs;
        }
        if code & 0xffe007ff == 0x7c0007ac {
            return Opcode::Icbi;
        }
        if code & 0xffffffff == 0x4c00012c {
            return Opcode::Isync;
        }
        if code & 0xfc000000 == 0x88000000 {
            return Opcode::Lbz;
        }
        if code & 0xfc000000 == 0x8c000000 {
            return Opcode::Lbzu;
        }
        if code & 0xfc0007ff == 0x7c0000ee {
            return Opcode::Lbzux;
        }
        if code & 0xfc0007ff == 0x7c0000ae {
            return Opcode::Lbzx;
        }
        if code & 0xfc000000 == 0xc8000000 {
            return Opcode::Lfd;
        }
        if code & 0xfc000000 == 0xcc000000 {
            return Opcode::Lfdu;
        }
        if code & 0xfc0007ff == 0x7c0004ee {
            return Opcode::Lfdux;
        }
        if code & 0xfc0007ff == 0x7c0004ae {
            return Opcode::Lfdx;
        }
        if code & 0xfc000000 == 0xc0000000 {
            return Opcode::Lfs;
        }
        if code & 0xfc000000 == 0xc4000000 {
            return Opcode::Lfsu;
        }
        if code & 0xfc0007ff == 0x7c00046e {
            return Opcode::Lfsux;
        }
        if code & 0xfc0007ff == 0x7c00042e {
            return Opcode::Lfsx;
        }
        if code & 0xfc000000 == 0xa8000000 {
            return Opcode::Lha;
        }
        if code & 0xfc000000 == 0xac000000 {
            return Opcode::Lhau;
        }
        if code & 0xfc0007ff == 0x7c0002ee {
            return Opcode::Lhaux;
        }
        if code & 0xfc0007ff == 0x7c0002ae {
            return Opcode::Lhax;
        }
        if code & 0xfc0007ff == 0x7c00062c {
            return Opcode::Lhbrx;
        }
        if code & 0xfc000000 == 0xa0000000 {
            return Opcode::Lhz;
        }
        if code & 0xfc000000 == 0xa4000000 {
            return Opcode::Lhzu;
        }
        if code & 0xfc0007ff == 0x7c00026e {
            return Opcode::Lhzux;
        }
        if code & 0xfc0007ff == 0x7c00022e {
            return Opcode::Lhzx;
        }
        if code & 0xfc000000 == 0xb8000000 {
            return Opcode::Lmw;
        }
        if code & 0xfc0007ff == 0x7c0004aa {
            return Opcode::Lswi;
        }
        if code & 0xfc0007ff == 0x7c00042a {
            return Opcode::Lswx;
        }
        if code & 0xfc0007ff == 0x7c000028 {
            return Opcode::Lwarx;
        }
        if code & 0xfc0007ff == 0x7c00042c {
            return Opcode::Lwbrx;
        }
        if code & 0xfc000000 == 0x80000000 {
            return Opcode::Lwz;
        }
        if code & 0xfc000000 == 0x84000000 {
            return Opcode::Lwzu;
        }
        if code & 0xfc0007ff == 0x7c00006e {
            return Opcode::Lwzux;
        }
        if code & 0xfc0007ff == 0x7c00002e {
            return Opcode::Lwzx;
        }
        if code & 0xfc300fff == 0x4c000000 {
            return Opcode::Mcrf;
        }
        if code & 0xfc30ffff == 0xfc000080 {
            return Opcode::Mcrfs;
        }
        if code & 0xfc30ffff == 0x7c000400 {
            return Opcode::Mcrxr;
        }
        if code & 0xfc1fffff == 0x7c000026 {
            return Opcode::Mfcr;
        }
        if code & 0xfc1ffffe == 0xfc00048e {
            return Opcode::Mffs;
        }
        if code & 0xfc1fffff == 0x7c0000a6 {
            return Opcode::Mfmsr;
        }
        if code & 0xfc0007ff == 0x7c0002a6 {
            return Opcode::Mfspr;
        }
        if code & 0xfc10ffff == 0x7c0004a6 {
            return Opcode::Mfsr;
        }
        if code & 0xfc1f07ff == 0x7c000526 {
            return Opcode::Mfsrin;
        }
        if code & 0xfc0007ff == 0x7c0002e6 {
            return Opcode::Mftb;
        }
        if code & 0xfc100fff == 0x7c000120 {
            return Opcode::Mtcrf;
        }
        if code & 0xfc1ffffe == 0xfc00008c {
            return Opcode::Mtfsb0;
        }
        if code & 0xfc1ffffe == 0xfc00004c {
            return Opcode::Mtfsb1;
        }
        if code & 0xfe0107fe == 0xfc00058e {
            return Opcode::Mtfsf;
        }
        if code & 0xfc7f0ffe == 0xfc00010c {
            return Opcode::Mtfsfi;
        }
        if code & 0xfc1fffff == 0x7c000124 {
            return Opcode::Mtmsr;
        }
        if code & 0xfc0007ff == 0x7c0003a6 {
            return Opcode::Mtspr;
        }
        if code & 0xfc10ffff == 0x7c0001a4 {
            return Opcode::Mtsr;
        }
        if code & 0xfc1f07ff == 0x7c0001e4 {
            return Opcode::Mtsrin;
        }
        if code & 0xfc0007fe == 0x7c000096 {
            return Opcode::Mulhw;
        }
        if code & 0xfc0007fe == 0x7c000016 {
            return Opcode::Mulhwu;
        }
        if code & 0xfc000000 == 0x1c000000 {
            return Opcode::Mulli;
        }
        if code & 0xfc0003fe == 0x7c0001d6 {
            return Opcode::Mullw;
        }
        if code & 0xfc0007fe == 0x7c0003b8 {
            return Opcode::Nand;
        }
        if code & 0xfc00fffe == 0x7c0000d0 {
            return Opcode::Neg;
        }
        if code & 0xfc0007fe == 0x7c0000f8 {
            return Opcode::Nor;
        }
        if code & 0xfc0007fe == 0x7c000378 {
            return Opcode::Or;
        }
        if code & 0xfc0007fe == 0x7c000338 {
            return Opcode::Orc;
        }
        if code & 0xfc000000 == 0x60000000 {
            return Opcode::Ori;
        }
        if code & 0xfc000000 == 0x64000000 {
            return Opcode::Oris;
        }
        if code & 0xfc000000 == 0xe0000000 {
            return Opcode::PsqL;
        }
        if code & 0xfc000000 == 0xe4000000 {
            return Opcode::PsqLu;
        }
        if code & 0xfc00007f == 0x1000004c {
            return Opcode::PsqLux;
        }
        if code & 0xfc00007f == 0x1000000c {
            return Opcode::PsqLx;
        }
        if code & 0xfc000000 == 0xf0000000 {
            return Opcode::PsqSt;
        }
        if code & 0xfc000000 == 0xf4000000 {
            return Opcode::PsqStu;
        }
        if code & 0xfc00007f == 0x1000004e {
            return Opcode::PsqStux;
        }
        if code & 0xfc00007f == 0x1000000e {
            return Opcode::PsqStx;
        }
        if code & 0xfc1f07fe == 0x10000210 {
            return Opcode::PsAbs;
        }
        if code & 0xfc0007fe == 0x1000002a {
            return Opcode::PsAdd;
        }
        if code & 0xfc6007ff == 0x10000040 {
            return Opcode::PsCmpo0;
        }
        if code & 0xfc6007ff == 0x100000c0 {
            return Opcode::PsCmpo1;
        }
        if code & 0xfc6007ff == 0x10000000 {
            return Opcode::PsCmpu0;
        }
        if code & 0xfc6007ff == 0x10000080 {
            return Opcode::PsCmpu1;
        }
        if code & 0xfc0007fe == 0x10000024 {
            return Opcode::PsDiv;
        }
        if code & 0xfc00003e == 0x1000003a {
            return Opcode::PsMadd;
        }
        if code & 0xfc00003e == 0x1000001c {
            return Opcode::PsMadds0;
        }
        if code & 0xfc00003e == 0x1000001e {
            return Opcode::PsMadds1;
        }
        if code & 0xfc0007fe == 0x10000420 {
            return Opcode::PsMerge00;
        }
        if code & 0xfc0007fe == 0x10000460 {
            return Opcode::PsMerge01;
        }
        if code & 0xfc0007fe == 0x100004a0 {
            return Opcode::PsMerge10;
        }
        if code & 0xfc0007fe == 0x100004e0 {
            return Opcode::PsMerge11;
        }
        if code & 0xfc1f07fe == 0x10000090 {
            return Opcode::PsMr;
        }
        if code & 0xfc00003e == 0x10000038 {
            return Opcode::PsMsub;
        }
        if code & 0xfc00f83e == 0x10000032 {
            return Opcode::PsMul;
        }
        if code & 0xfc00f83e == 0x10000018 {
            return Opcode::PsMuls0;
        }
        if code & 0xfc00f83e == 0x1000001a {
            return Opcode::PsMuls1;
        }
        if code & 0xfc1f07fe == 0x10000110 {
            return Opcode::PsNabs;
        }
        if code & 0xfc1f07fe == 0x10000050 {
            return Opcode::PsNeg;
        }
        if code & 0xfc00003e == 0x1000003e {
            return Opcode::PsNmadd;
        }
        if code & 0xfc00003e == 0x1000003c {
            return Opcode::PsNmsub;
        }
        if code & 0xfc1f07fe == 0x10000030 {
            return Opcode::PsRes;
        }
        if code & 0xfc1f07fe == 0x10000034 {
            return Opcode::PsRsqrte;
        }
        if code & 0xfc00003e == 0x1000002e {
            return Opcode::PsSel;
        }
        if code & 0xfc0007fe == 0x10000028 {
            return Opcode::PsSub;
        }
        if code & 0xfc00003e == 0x10000014 {
            return Opcode::PsSum0;
        }
        if code & 0xfc00003e == 0x10000016 {
            return Opcode::PsSum1;
        }
        if code & 0xfffff801 == 0x4c000000 {
            return Opcode::Rfi;
        }
        if code & 0xfc000000 == 0x50000000 {
            return Opcode::Rlwimi;
        }
        if code & 0xfc000000 == 0x54000000 {
            return Opcode::Rlwinm;
        }
        if code & 0xfc000000 == 0x5c000000 {
            return Opcode::Rlwnm;
        }
        if code & 0xffffffff == 0x44000002 {
            return Opcode::Sc;
        }
        if code & 0xfc0007fe == 0x7c000030 {
            return Opcode::Slw;
        }
        if code & 0xfc0007fe == 0x7c000630 {
            return Opcode::Sraw;
        }
        if code & 0xfc0007fe == 0x7c000670 {
            return Opcode::Srawi;
        }
        if code & 0xfc0007fe == 0x7c000430 {
            return Opcode::Srw;
        }
        if code & 0xfc000000 == 0x98000000 {
            return Opcode::Stb;
        }
        if code & 0xfc000000 == 0x9c000000 {
            return Opcode::Stbu;
        }
        if code & 0xfc0003ff == 0x7c0001ee {
            return Opcode::Stbux;
        }
        if code & 0xfc0003ff == 0x7c0001ae {
            return Opcode::Stbx;
        }
        if code & 0xfc000000 == 0xd8000000 {
            return Opcode::Stfd;
        }
        if code & 0xfc000000 == 0xdc000000 {
            return Opcode::Stfdu;
        }
        if code & 0xfc0007ff == 0x7c0005ee {
            return Opcode::Stfdux;
        }
        if code & 0xfc0007ff == 0x7c0005ae {
            return Opcode::Stfdx;
        }
        if code & 0xfc0007ff == 0x7c0007ae {
            return Opcode::Stfiwx;
        }
        if code & 0xfc000000 == 0xd0000000 {
            return Opcode::Stfs;
        }
        if code & 0xfc000000 == 0xd4000000 {
            return Opcode::Stfsu;
        }
        if code & 0xfc0007ff == 0x7c00056e {
            return Opcode::Stfsux;
        }
        if code & 0xfc0007ff == 0x7c00052e {
            return Opcode::Stfsx;
        }
        if code & 0xfc000000 == 0xb0000000 {
            return Opcode::Sth;
        }
        if code & 0xfc0007ff == 0x7c00072c {
            return Opcode::Sthbrx;
        }
        if code & 0xfc000000 == 0xb4000000 {
            return Opcode::Sthu;
        }
        if code & 0xfc0007ff == 0x7c00036e {
            return Opcode::Sthux;
        }
        if code & 0xfc0007ff == 0x7c00032e {
            return Opcode::Sthx;
        }
        if code & 0xfc000000 == 0xbc000000 {
            return Opcode::Stmw;
        }
        if code & 0xfc0007ff == 0x7c0005aa {
            return Opcode::Stswi;
        }
        if code & 0xfc0007ff == 0x7c00052a {
            return Opcode::Stswx;
        }
        if code & 0xfc000000 == 0x90000000 {
            return Opcode::Stw;
        }
        if code & 0xfc0007ff == 0x7c00052c {
            return Opcode::Stwbrx;
        }
        if code & 0xfc0007ff == 0x7c00012d {
            return Opcode::Stwcx_;
        }
        if code & 0xfc000000 == 0x94000000 {
            return Opcode::Stwu;
        }
        if code & 0xfc0007ff == 0x7c00016e {
            return Opcode::Stwux;
        }
        if code & 0xfc0007ff == 0x7c00012e {
            return Opcode::Stwx;
        }
        if code & 0xfc0003fe == 0x7c000050 {
            return Opcode::Subf;
        }
        if code & 0xfc0003fe == 0x7c000010 {
            return Opcode::Subfc;
        }
        if code & 0xfc0003fe == 0x7c000110 {
            return Opcode::Subfe;
        }
        if code & 0xfc000000 == 0x20000000 {
            return Opcode::Subfic;
        }
        if code & 0xfc00fbfe == 0x7c0001d0 {
            return Opcode::Subfme;
        }
        if code & 0xfc00fbfe == 0x7c000190 {
            return Opcode::Subfze;
        }
        if code & 0xffffffff == 0x7c0004ac {
            return Opcode::Sync;
        }
        if code & 0xffff07ff == 0x7c000264 {
            return Opcode::Tlbie;
        }
        if code & 0xffffffff == 0x7c00046c {
            return Opcode::Tlbsync;
        }
        if code & 0xfc0007ff == 0x7c000008 {
            return Opcode::Tw;
        }
        if code & 0xfc000000 == 0xc0000000 {
            return Opcode::Twi;
        }
        if code & 0xfc0007fe == 0x7c000278 {
            return Opcode::Xor;
        }
        if code & 0xfc000000 == 0x68000000 {
            return Opcode::Xori;
        }
        if code & 0xfc000000 == 0x6c000000 {
            return Opcode::Xoris;
        }
        Opcode::Illegal
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Field {
    simm(Simm),
    uimm(Uimm),
    offset(Offset),
    BO(OpaqueU),
    BI(OpaqueU),
    BD(BranchDest),
    LI(BranchDest),
    SH(OpaqueU),
    MB(OpaqueU),
    ME(OpaqueU),
    rS(GPR),
    rD(GPR),
    rA(GPR),
    rB(GPR),
    rC(GPR),
    sr(SR),
    spr(SPR),
    frS(FPR),
    frD(FPR),
    frA(FPR),
    frB(FPR),
    frC(FPR),
    crbD(CRField),
    crbA(CRField),
    crbB(CRField),
    crfD(CRBit),
    crfS(CRBit),
    crm(OpaqueU),
    ps_l(GQR),
    ps_W(OpaqueU),
    NB(OpaqueU),
    tbr(OpaqueU),
    mtfsf_FM(OpaqueU),
    mtfsf_IMM(OpaqueU),
    tw_TO(OpaqueU),
    xer,
    ctr,
    lr,
}
impl Ins {
    pub(crate) fn _fields(&self) -> Vec<Field> {
        match self.op {
            Opcode::Illegal => vec![],
            Opcode::Add => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Addc => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Adde => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Addi => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::simm(Simm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Addic => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::simm(Simm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Addic_ => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::simm(Simm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Addis => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Addme => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Addze => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::And => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Andc => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Andi_ => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Andis_ => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::B => vec![Field::LI(BranchDest(
                (((self.code) >> (32 - 30u8)) & ((1 << 24usize) - 1)) as _,
            ))],
            Opcode::Bc => vec![
                Field::BO(OpaqueU(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::BI(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::BD(BranchDest(
                    (((self.code) >> (32 - 30u8)) & ((1 << 14usize) - 1)) as _,
                )),
            ],
            Opcode::Bcctr => vec![
                Field::BO(OpaqueU(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::BI(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Bclr => vec![
                Field::BO(OpaqueU(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::BI(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Cmp => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Cmpi => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::simm(Simm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Cmpl => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Cmpli => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Cntlzw => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Crand => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Crandc => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Creqv => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Crnand => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Crnor => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Cror => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Crorc => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Crxor => vec![
                Field::crbD(CRField(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbA(CRField(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::crbB(CRField(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Dcbf => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Dcbi => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Dcbst => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Dcbt => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Dcbtst => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Dcbz => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::DcbzL => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Divw => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Divwu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Eciwx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Ecowx => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Eieio => vec![],
            Opcode::Eqv => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Extsb => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Extsh => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fabs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fadd => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fadds => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fcmpo => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fcmpu => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fctiw => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fctiwz => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fdiv => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fdivs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fmadd => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fmadds => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fmr => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fmsub => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fmsubs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fmul => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fmuls => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fnabs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fneg => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fnmadd => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fnmadds => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fnmsub => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fnmsubs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fres => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Frsp => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Frsqrte => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fsel => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fsub => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Fsubs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Icbi => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Isync => vec![],
            Opcode::Lbz => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lbzu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lbzux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lbzx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfd => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfdu => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfdux => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfdx => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfsu => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfsux => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfsx => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lha => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhau => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhaux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhax => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhbrx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhz => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhzu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhzux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhzx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lmw => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lswi => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::NB(OpaqueU(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Lswx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwarx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwbrx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwz => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwzu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwzux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwzx => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mcrf => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::crfS(CRBit(
                    (((self.code) >> (32 - 14u8)) & ((1 << 3usize) - 1)) as _,
                )),
            ],
            Opcode::Mcrfs => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::crfS(CRBit(
                    (((self.code) >> (32 - 14u8)) & ((1 << 3usize) - 1)) as _,
                )),
            ],
            Opcode::Mcrxr => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mfcr => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mffs => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mfmsr => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mfspr => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::spr(SPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 10usize) - 1)) as _
                )),
            ],
            Opcode::Mfsr => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::sr(SR((((self.code) >> (32 - 16u8)) & ((1 << 4usize) - 1)) as _)),
            ],
            Opcode::Mfsrin => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mftb => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::tbr(OpaqueU(
                    (((self.code) >> (32 - 21u8)) & ((1 << 10usize) - 1)) as _,
                )),
            ],
            Opcode::Mtcrf => vec![
                Field::crm(OpaqueU(
                    (((self.code) >> (32 - 20u8)) & ((1 << 8usize) - 1)) as _,
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mtfsb0 => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mtfsb1 => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mtfsf => vec![
                Field::mtfsf_FM(OpaqueU(
                    (((self.code) >> (32 - 15u8)) & ((1 << 8usize) - 1)) as _,
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mtfsfi => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::mtfsf_IMM(OpaqueU(
                    (((self.code) >> (32 - 20u8)) & ((1 << 4usize) - 1)) as _,
                )),
            ],
            Opcode::Mtmsr => vec![Field::rS(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mtspr => vec![
                Field::spr(SPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 10usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mtsr => vec![
                Field::sr(SR((((self.code) >> (32 - 16u8)) & ((1 << 4usize) - 1)) as _)),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mtsrin => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mulhw => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mulhwu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Mulli => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::simm(Simm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Mullw => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Nand => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Neg => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Nor => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Or => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Orc => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Ori => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Oris => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::PsqL => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsqLu => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsqLux => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsqLx => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsqSt => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsqStu => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsqStux => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsqStx => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::ps_W(OpaqueU(
                    (((self.code) >> (32 - 16u8)) & ((1 << 0usize) - 1)) as _,
                )),
                Field::ps_l(GQR(
                    (((self.code) >> (32 - 20u8)) & ((1 << 3usize) - 1)) as _
                )),
            ],
            Opcode::PsAbs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsAdd => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsCmpo0 => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsCmpo1 => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsCmpu0 => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsCmpu1 => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsDiv => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMadd => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMadds0 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMadds1 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMerge00 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMerge01 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMerge10 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMerge11 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMr => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMsub => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMul => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMuls0 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsMuls1 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsNabs => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsNeg => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsNmadd => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsNmsub => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsRes => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsRsqrte => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsSel => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsSub => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsSum0 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsSum1 => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frA(FPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frC(FPR(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::frB(FPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Rfi => vec![],
            Opcode::Rlwimi => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::SH(OpaqueU(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::MB(OpaqueU(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::ME(OpaqueU(
                    (((self.code) >> (32 - 31u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Rlwinm => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::SH(OpaqueU(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::MB(OpaqueU(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::ME(OpaqueU(
                    (((self.code) >> (32 - 31u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Rlwnm => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::MB(OpaqueU(
                    (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::ME(OpaqueU(
                    (((self.code) >> (32 - 31u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Sc => vec![],
            Opcode::Slw => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Sraw => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Srawi => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::SH(OpaqueU(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Srw => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stb => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stbu => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stbux => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stbx => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfd => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfdu => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfdux => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfdx => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfiwx => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfs => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfsu => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfsux => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stfsx => vec![
                Field::frS(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Sth => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Sthbrx => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Sthu => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Sthux => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Sthx => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stmw => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stswi => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::NB(OpaqueU(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                )),
            ],
            Opcode::Stswx => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stw => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stwbrx => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stwcx_ => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stwu => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stwux => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Stwx => vec![
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Subf => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Subfc => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Subfe => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Subfic => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::simm(Simm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Subfme => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Subfze => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Sync => vec![],
            Opcode::Tlbie => vec![Field::rB(GPR(
                (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Tlbsync => vec![],
            Opcode::Tw => vec![
                Field::tw_TO(OpaqueU(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Twi => vec![
                Field::tw_TO(OpaqueU(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::simm(Simm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Xor => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rB(GPR(
                    (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Xori => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
            Opcode::Xoris => vec![
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rS(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::uimm(Uimm(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                )),
            ],
        }
    }
    #[allow(unused_mut)]
    pub(crate) fn _defs(&self) -> Vec<Field> {
        match self.op {
            Opcode::Illegal => vec![],
            Opcode::Add => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Addc => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Adde => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Addi => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Addic => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Addic_ => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Addis => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Addme => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Addze => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::And => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Andc => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Andi_ => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Andis_ => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::B => vec![],
            Opcode::Bc => vec![],
            Opcode::Bcctr => vec![],
            Opcode::Bclr => vec![],
            Opcode::Cmp => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Cmpi => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Cmpl => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Cmpli => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Cntlzw => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Crand => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Crandc => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Creqv => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Crnand => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Crnor => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Cror => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Crorc => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Crxor => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Dcbf => vec![],
            Opcode::Dcbi => vec![],
            Opcode::Dcbst => vec![],
            Opcode::Dcbt => vec![],
            Opcode::Dcbtst => vec![],
            Opcode::Dcbz => vec![],
            Opcode::DcbzL => vec![],
            Opcode::Divw => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Divwu => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Eciwx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Ecowx => vec![],
            Opcode::Eieio => vec![],
            Opcode::Eqv => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Extsb => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Extsh => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fabs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fadd => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fadds => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fcmpo => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Fcmpu => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Fctiw => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fctiwz => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fdiv => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fdivs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fmadd => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fmadds => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fmr => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fmsub => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fmsubs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fmul => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fmuls => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fnabs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fneg => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fnmadd => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fnmadds => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fnmsub => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fnmsubs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fres => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Frsp => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Frsqrte => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Fsel => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fsub => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Fsubs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Icbi => vec![],
            Opcode::Isync => vec![],
            Opcode::Lbz => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lbzu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lbzux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lbzx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lfd => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lfdu => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfdux => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfdx => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lfs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lfsu => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfsux => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lfsx => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lha => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lhau => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhaux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhax => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lhbrx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lhz => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lhzu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhzux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lhzx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lmw => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lswi => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lswx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lwarx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lwbrx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lwz => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Lwzu => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwzux => vec![
                Field::rD(GPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::Lwzx => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mcrf => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mcrfs => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mcrxr => vec![
                Field::crfD(CRBit(
                    (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
                )),
                Field::xer,
            ],
            Opcode::Mfcr => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mffs => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mfmsr => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mfspr => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mfsr => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mfsrin => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Mftb => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mtcrf => vec![],
            Opcode::Mtfsb0 => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mtfsb1 => vec![Field::crbD(CRField(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mtfsf => vec![],
            Opcode::Mtfsfi => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::Mtmsr => vec![],
            Opcode::Mtspr => vec![],
            Opcode::Mtsr => vec![],
            Opcode::Mtsrin => vec![],
            Opcode::Mulhw => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mulhwu => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Mulli => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Mullw => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Nand => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Neg => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Nor => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Or => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Orc => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Ori => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Oris => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::PsqL => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::PsqLu => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsqLux => vec![
                Field::frD(FPR(
                    (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                )),
                Field::rA(GPR(
                    (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                )),
            ],
            Opcode::PsqLx => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsqSt => vec![],
            Opcode::PsqStu => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsqStux => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsqStx => vec![],
            Opcode::PsAbs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsAdd => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsCmpo0 => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::PsCmpo1 => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::PsCmpu0 => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::PsCmpu1 => vec![Field::crfD(CRBit(
                (((self.code) >> (32 - 9u8)) & ((1 << 3usize) - 1)) as _,
            ))],
            Opcode::PsDiv => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMadd => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMadds0 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMadds1 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMerge00 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMerge01 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMerge10 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMerge11 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMr => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::PsMsub => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMul => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMuls0 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsMuls1 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsNabs => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsNeg => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsNmadd => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsNmsub => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsRes => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsRsqrte => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsSel => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsSub => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsSum0 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::PsSum1 => vec![Field::frD(FPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Rfi => vec![],
            Opcode::Rlwimi => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Rlwinm => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Rlwnm => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Sc => vec![],
            Opcode::Slw => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Sraw => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Srawi => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Srw => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Stb => vec![],
            Opcode::Stbu => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Stbux => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Stbx => vec![],
            Opcode::Stfd => vec![],
            Opcode::Stfdu => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Stfdux => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Stfdx => vec![],
            Opcode::Stfiwx => vec![],
            Opcode::Stfs => vec![],
            Opcode::Stfsu => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Stfsux => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Stfsx => vec![],
            Opcode::Sth => vec![],
            Opcode::Sthbrx => vec![],
            Opcode::Sthu => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Sthux => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Sthx => vec![],
            Opcode::Stmw => vec![],
            Opcode::Stswi => vec![],
            Opcode::Stswx => vec![],
            Opcode::Stw => vec![],
            Opcode::Stwbrx => vec![],
            Opcode::Stwcx_ => vec![],
            Opcode::Stwu => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Stwux => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Stwx => vec![],
            Opcode::Subf => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Subfc => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Subfe => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Subfic => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Subfme => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Subfze => vec![Field::rD(GPR(
                (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
            ))],
            Opcode::Sync => vec![],
            Opcode::Tlbie => vec![],
            Opcode::Tlbsync => vec![],
            Opcode::Tw => vec![],
            Opcode::Twi => vec![],
            Opcode::Xor => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Xori => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
            Opcode::Xoris => vec![Field::rA(GPR(
                (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
            ))],
        }
    }
    #[allow(unused_mut)]
    pub(crate) fn _uses(&self) -> Vec<Field> {
        match self.op {
            Opcode::Illegal => vec![],
            Opcode::Add => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Addc => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Adde => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Addi => {
                let mut uses = vec![];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Addic => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Addic_ => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Addis => {
                let mut uses = vec![];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Addme => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Addze => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::And => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Andc => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Andi_ => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Andis_ => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::B => {
                let mut uses = vec![];
                uses
            }
            Opcode::Bc => {
                let mut uses = vec![];
                uses
            }
            Opcode::Bcctr => {
                let mut uses = vec![Field::ctr];
                uses
            }
            Opcode::Bclr => {
                let mut uses = vec![Field::lr];
                uses
            }
            Opcode::Cmp => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Cmpi => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Cmpl => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Cmpli => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Cntlzw => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Crand => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Crandc => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Creqv => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Crnand => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Crnor => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Cror => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Crorc => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Crxor => {
                let mut uses = vec![
                    Field::crbA(CRField(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                    Field::crbB(CRField(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Dcbf => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Dcbi => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Dcbst => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Dcbt => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Dcbtst => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Dcbz => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::DcbzL => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Divw => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Divwu => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Eciwx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Ecowx => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Eieio => {
                let mut uses = vec![];
                uses
            }
            Opcode::Eqv => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Extsb => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Extsh => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fabs => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fadd => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fadds => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fcmpo => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fcmpu => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fctiw => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fctiwz => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fdiv => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fdivs => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fmadd => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fmadds => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fmr => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fmsub => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fmsubs => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fmul => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fmuls => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fnabs => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fneg => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fnmadd => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fnmadds => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fnmsub => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fnmsubs => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fres => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Frsp => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Frsqrte => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Fsel => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fsub => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Fsubs => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Icbi => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Isync => {
                let mut uses = vec![];
                uses
            }
            Opcode::Lbz => {
                let mut uses = vec![Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                ))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lbzu => {
                let mut uses = vec![
                    Field::offset(Offset(
                        (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lbzux => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lbzx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lfd => {
                let mut uses = vec![Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                ))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lfdu => {
                let mut uses = vec![
                    Field::offset(Offset(
                        (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lfdux => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lfdx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lfs => {
                let mut uses = vec![Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                ))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lfsu => {
                let mut uses = vec![
                    Field::offset(Offset(
                        (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lfsux => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lfsx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lha => {
                let mut uses = vec![Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                ))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lhau => {
                let mut uses = vec![
                    Field::offset(Offset(
                        (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lhaux => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lhax => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lhbrx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lhz => {
                let mut uses = vec![Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                ))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lhzu => {
                let mut uses = vec![
                    Field::offset(Offset(
                        (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lhzux => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lhzx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lmw => {
                let mut uses = vec![Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                ))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lswi => {
                let mut uses = vec![];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lswx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lwarx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lwbrx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lwz => {
                let mut uses = vec![Field::offset(Offset(
                    (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                ))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Lwzu => {
                let mut uses = vec![
                    Field::offset(Offset(
                        (((self.code) >> (32 - 32u8)) & ((1 << 16usize) - 1)) as _,
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lwzux => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Lwzx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Mcrf => {
                let mut uses = vec![Field::crfS(CRBit(
                    (((self.code) >> (32 - 14u8)) & ((1 << 3usize) - 1)) as _,
                ))];
                uses
            }
            Opcode::Mcrfs => {
                let mut uses = vec![Field::crfS(CRBit(
                    (((self.code) >> (32 - 14u8)) & ((1 << 3usize) - 1)) as _,
                ))];
                uses
            }
            Opcode::Mcrxr => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mfcr => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mffs => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mfmsr => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mfspr => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mfsr => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mfsrin => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Mftb => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mtcrf => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Mtfsb0 => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mtfsb1 => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mtfsf => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Mtfsfi => {
                let mut uses = vec![];
                uses
            }
            Opcode::Mtmsr => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Mtspr => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Mtsr => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Mtsrin => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Mulhw => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Mulhwu => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Mulli => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Mullw => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Nand => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Neg => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Nor => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Or => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Orc => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Ori => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Oris => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::PsqL => {
                let mut uses = vec![];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsqLu => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::PsqLux => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsqLx => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsqSt => {
                let mut uses = vec![Field::frS(FPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsqStu => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsqStux => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsqStx => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsAbs => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsAdd => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsCmpo0 => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsCmpo1 => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsCmpu0 => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsCmpu1 => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::PsDiv => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMadd => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMadds0 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMadds1 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMerge00 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMerge01 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMerge10 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMerge11 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMr => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::PsMsub => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMul => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMuls0 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsMuls1 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsNabs => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::PsNeg => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::PsNmadd => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsNmsub => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsRes => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::PsRsqrte => {
                let mut uses = vec![Field::frB(FPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::PsSel => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsSub => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsSum0 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::PsSum1 => {
                let mut uses = vec![
                    Field::frA(FPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frC(FPR(
                        (((self.code) >> (32 - 26u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::frB(FPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Rfi => {
                let mut uses = vec![];
                uses
            }
            Opcode::Rlwimi => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::SH(OpaqueU(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Rlwinm => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::SH(OpaqueU(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _,
                    )),
                ];
                uses
            }
            Opcode::Rlwnm => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Sc => {
                let mut uses = vec![];
                uses
            }
            Opcode::Slw => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Sraw => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Srawi => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Srw => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stb => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stbu => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stbux => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stbx => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stfd => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stfdu => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stfdux => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stfdx => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stfiwx => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stfs => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stfsu => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stfsux => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stfsx => {
                let mut uses = vec![
                    Field::frS(FPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Sth => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Sthbrx => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Sthu => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Sthux => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Sthx => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stmw => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stswi => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stswx => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stw => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stwbrx => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stwcx_ => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Stwu => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stwux => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Stwx => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                if (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) != 0 {
                    uses.push(Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _,
                    )));
                }
                uses
            }
            Opcode::Subf => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Subfc => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Subfe => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Subfic => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Subfme => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Subfze => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Sync => {
                let mut uses = vec![];
                uses
            }
            Opcode::Tlbie => {
                let mut uses = vec![Field::rB(GPR((((self.code) >> (32 - 21u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Tlbsync => {
                let mut uses = vec![];
                uses
            }
            Opcode::Tw => {
                let mut uses = vec![
                    Field::rA(GPR(
                        (((self.code) >> (32 - 16u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Twi => {
                let mut uses = vec![Field::rA(GPR((((self.code) >> (32 - 16u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Xor => {
                let mut uses = vec![
                    Field::rS(GPR(
                        (((self.code) >> (32 - 11u8)) & ((1 << 5usize) - 1)) as _
                    )),
                    Field::rB(GPR(
                        (((self.code) >> (32 - 21u8)) & ((1 << 5usize) - 1)) as _
                    )),
                ];
                uses
            }
            Opcode::Xori => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
            Opcode::Xoris => {
                let mut uses = vec![Field::rS(GPR((((self.code) >> (32 - 11u8))
                    & ((1 << 5usize) - 1))
                    as _))];
                uses
            }
        }
    }
    #[allow(unused_mut)]
    pub(crate) fn _modifiers(&self) -> Modifiers {
        match self.op {
            Opcode::Illegal => Modifiers::default(),
            Opcode::Add => {
                let mut m = Modifiers::default();
                m.oe = self.bit(21);
                m.rc = self.bit(31);
                m
            }
            Opcode::Addc => {
                let mut m = Modifiers::default();
                m.oe = self.bit(21);
                m.rc = self.bit(31);
                m
            }
            Opcode::Adde => {
                let mut m = Modifiers::default();
                m.oe = self.bit(21);
                m.rc = self.bit(31);
                m
            }
            Opcode::Addi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Addic => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Addic_ => {
                let mut m = Modifiers::default();
                m.rc = true;
                m
            }
            Opcode::Addis => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Addme => {
                let mut m = Modifiers::default();
                m.oe = self.bit(21);
                m.rc = self.bit(31);
                m
            }
            Opcode::Addze => {
                let mut m = Modifiers::default();
                m.oe = self.bit(21);
                m.rc = self.bit(31);
                m
            }
            Opcode::And => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Andc => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Andi_ => {
                let mut m = Modifiers::default();
                m.rc = true;
                m
            }
            Opcode::Andis_ => {
                let mut m = Modifiers::default();
                m.rc = true;
                m
            }
            Opcode::B => {
                let mut m = Modifiers::default();
                m.aa = self.bit(30);
                m.lk = self.bit(31);
                m
            }
            Opcode::Bc => {
                let mut m = Modifiers::default();
                m.aa = self.bit(30);
                m.lk = self.bit(31);
                m
            }
            Opcode::Bcctr => {
                let mut m = Modifiers::default();
                m.lk = self.bit(31);
                m
            }
            Opcode::Bclr => {
                let mut m = Modifiers::default();
                m.lk = self.bit(31);
                m
            }
            Opcode::Cmp => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Cmpi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Cmpl => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Cmpli => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Cntlzw => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Crand => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Crandc => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Creqv => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Crnand => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Crnor => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Cror => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Crorc => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Crxor => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Dcbf => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Dcbi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Dcbst => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Dcbt => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Dcbtst => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Dcbz => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::DcbzL => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Divw => {
                let mut m = Modifiers::default();
                m.oe = self.bit(21);
                m.rc = self.bit(31);
                m
            }
            Opcode::Divwu => {
                let mut m = Modifiers::default();
                m.oe = self.bit(21);
                m.rc = self.bit(31);
                m
            }
            Opcode::Eciwx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Ecowx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Eieio => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Eqv => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Extsb => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Extsh => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fabs => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fadd => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fadds => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fcmpo => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Fcmpu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Fctiw => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fctiwz => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fdiv => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fdivs => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fmadd => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fmadds => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fmr => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fmsub => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fmsubs => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fmul => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fmuls => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fnabs => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fneg => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fnmadd => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fnmadds => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fnmsub => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fnmsubs => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fres => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Frsp => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Frsqrte => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fsel => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fsub => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Fsubs => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Icbi => {
                let mut m = Modifiers::default();
                m.rc = self.bit(31);
                m
            }
            Opcode::Isync => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lbz => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lbzu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lbzux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lbzx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfd => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfdu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfdux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfdx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfs => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfsu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfsux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lfsx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lha => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhau => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhaux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhax => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhbrx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhz => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhzu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhzux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lhzx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lmw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lswi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lswx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lwarx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lwbrx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lwz => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lwzu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lwzux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Lwzx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mcrf => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mcrfs => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mcrxr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mfcr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mffs => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mfmsr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mfspr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mfsr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mfsrin => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mftb => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtcrf => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtfsb0 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtfsb1 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtfsf => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtfsfi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtmsr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtspr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtsr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mtsrin => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mulhw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mulhwu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mulli => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Mullw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Nand => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Neg => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Nor => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Or => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Orc => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Ori => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Oris => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqL => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqLu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqLux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqLx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqSt => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqStu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqStux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsqStx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsAbs => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsAdd => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsCmpo0 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsCmpo1 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsCmpu0 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsCmpu1 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsDiv => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMadd => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMadds0 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMadds1 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMerge00 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMerge01 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMerge10 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMerge11 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMr => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMsub => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMul => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMuls0 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsMuls1 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsNabs => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsNeg => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsNmadd => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsNmsub => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsRes => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsRsqrte => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsSel => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsSub => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsSum0 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::PsSum1 => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Rfi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Rlwimi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Rlwinm => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Rlwnm => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sc => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Slw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sraw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Srawi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Srw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stb => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stbu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stbux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stbx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfd => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfdu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfdux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfdx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfiwx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfs => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfsu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfsux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stfsx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sth => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sthbrx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sthu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sthux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sthx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stmw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stswi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stswx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stwbrx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stwcx_ => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stwu => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stwux => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Stwx => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Subf => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Subfc => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Subfe => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Subfic => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Subfme => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Subfze => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Sync => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Tlbie => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Tlbsync => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Tw => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Twi => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Xor => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Xori => {
                let mut m = Modifiers::default();
                m
            }
            Opcode::Xoris => {
                let mut m = Modifiers::default();
                m
            }
        }
    }
    pub(crate) fn _simplified(self) -> SimplifiedIns {
        SimplifiedIns {
            mnemonic: self.op.mnemonic(),
            modifiers: self._modifiers(),
            args: vec![],
            ins: self,
        }
    }
}
