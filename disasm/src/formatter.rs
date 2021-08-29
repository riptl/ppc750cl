use std::fmt::{Display, Formatter, LowerHex, UpperHex};

use num_traits::PrimInt;

use crate::prelude::*;

pub struct FormattedIns(pub Ins);

impl Display for FormattedIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_ins(f)
    }
}

impl FormattedIns {
    fn fmt_ins(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} ", self.0.op.mnemonic(), self.0.modifiers())?;
        let fields = self.0.fields();
        let mut writing_offset = false;
        for (i, field) in fields.iter().enumerate() {
            if let offset(o) = field {
                writing_offset = true;
                write!(f, "{:#x}(", ReallySigned(o.0))?;
                continue;
            }
            Self::fmt_field(field, f)?;
            if writing_offset {
                write!(f, ")")?;
                writing_offset = false;
            }
            if i != fields.len() - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }

    fn fmt_field(field: &Field, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match field {
            simm(s) => Self::fmt_simm(*s, f),
            uimm(u) => Self::fmt_uimm(*u, f),
            BO(o) => Self::fmt_opaque_u(*o, f),
            BI(o) => Self::fmt_opaque_u(*o, f),
            BD(bd) => Self::fmt_branch_dest(*bd, f),
            LI(bd) => Self::fmt_branch_dest(*bd, f),
            SH(o) => Self::fmt_opaque_u(*o, f),
            MB(o) => Self::fmt_opaque_u(*o, f),
            ME(o) => Self::fmt_opaque_u(*o, f),
            rS(gpr) => Self::fmt_gpr(*gpr, f),
            rD(gpr) => Self::fmt_gpr(*gpr, f),
            rA(gpr) => Self::fmt_gpr(*gpr, f),
            rB(gpr) => Self::fmt_gpr(*gpr, f),
            rC(gpr) => Self::fmt_gpr(*gpr, f),
            sr(s) => Self::fmt_sr(*s, f),
            spr(s) => Self::fmt_spr(*s, f),
            frS(fpr) => Self::fmt_fpr(*fpr, f),
            frD(fpr) => Self::fmt_fpr(*fpr, f),
            frA(fpr) => Self::fmt_fpr(*fpr, f),
            frB(fpr) => Self::fmt_fpr(*fpr, f),
            frC(fpr) => Self::fmt_fpr(*fpr, f),
            crbD(crb) => Self::fmt_crb(*crb, f),
            crbA(crb) => Self::fmt_crb(*crb, f),
            crbB(crb) => Self::fmt_crb(*crb, f),
            crfD(crf) => Self::fmt_crf(*crf, f),
            crfS(crf) => Self::fmt_crf(*crf, f),
            crm(o) => Self::fmt_opaque_u(*o, f),
            ps_l(gqr) => Self::fmt_gqr(*gqr, f),
            ps_W(o) => Self::fmt_opaque_u(*o, f),
            NB(o) => Self::fmt_opaque_u(*o, f),
            tbr(o) => Self::fmt_opaque_u(*o, f),
            mtfsf_FM(o) => Self::fmt_opaque_u(*o, f),
            mtfsf_IMM(o) => Self::fmt_opaque_u(*o, f),
            tw_TO(o) => Self::fmt_opaque_u(*o, f),
            _ => Ok(()),
        }
    }

    fn fmt_gpr(gpr: GPR, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", gpr.0)
    }

    fn fmt_fpr(gpr: FPR, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fr{}", gpr.0)
    }

    fn fmt_opaque_u(u: OpaqueU, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u.0)
    }

    fn fmt_gqr(gqr: GQR, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", gqr.0)
    }

    fn fmt_spr(s: SPR, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", s.0)
    }

    fn fmt_sr(s: SR, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", s.0)
    }

    fn fmt_crb(crb: CRField, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crb.0)
    }

    fn fmt_crf(crf: CRBit, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crf.0)
    }

    fn fmt_branch_dest(bd: BranchDest, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", ReallySigned(bd.0))
    }

    fn fmt_uimm(u: Uimm, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", u.0)
    }

    fn fmt_simm(s: Simm, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", ReallySigned(s.0))
    }
}

// https://stackoverflow.com/questions/44711012/how-do-i-format-a-signed-integer-to-a-sign-aware-hexadecimal-representation
struct ReallySigned<N: PrimInt>(N);

impl<N: PrimInt> LowerHex for ReallySigned<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let num = self.0.to_i32().unwrap();
        let prefix = if f.alternate() { "0x" } else { "" };
        let bare_hex = format!("{:x}", num.abs());
        f.pad_integral(num >= 0, prefix, &bare_hex)
    }
}

impl<N: PrimInt> UpperHex for ReallySigned<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let num = self.0.to_i32().unwrap();
        let prefix = if f.alternate() { "0x" } else { "" };
        let bare_hex = format!("{:X}", num.abs());
        f.pad_integral(num >= 0, prefix, &bare_hex)
    }
}
