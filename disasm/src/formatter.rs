use std::fmt::{Display, Formatter};

use crate::prelude::*;

pub struct FormattedIns(pub Ins);

impl Display for FormattedIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let simple = self.0.clone().simplified();
        write!(f, "{}{}", simple.mnemonic, simple.ins.suffix())?;
        let mut writing_offset = false;
        for (i, arg) in simple.args.iter().enumerate() {
            if i == 0 {
                write!(f, " ")?;
            }
            if i > 0 && !writing_offset {
                write!(f, ", ")?;
            }
            if let Argument::Offset(val) = arg {
                write!(f, "{}(", val)?;
                writing_offset = true;
                continue;
            } else {
                write!(f, "{}", arg)?;
            }
            if writing_offset {
                write!(f, ")")?;
                writing_offset = false;
            }
        }
        Ok(())
    }
}
