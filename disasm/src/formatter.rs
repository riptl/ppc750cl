use std::fmt::{Display, Formatter};

use crate::prelude::*;

pub struct FormattedIns(pub Ins);

impl Display for FormattedIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} ", self.0.op.mnemonic(), self.0.modifiers())?;
        let fields = self.0.fields();
        let mut writing_offset = false;
        for (i, field) in fields.iter().enumerate() {
            if let Some(argument) = field.argument() {
                write!(f, "{}", argument)?;
            }
            if let offset(_) = field {
                write!(f, "(")?;
                writing_offset = true;
                continue;
            }
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
}
