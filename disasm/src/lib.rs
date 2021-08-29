use std::fmt::Formatter;
use std::ops::Range;

use num_traits::AsPrimitive;

use ppc750cl_macros::{fields, ins_impl, opcodes};

pub use crate::iter::{disasm_iter, DisasmIterator};

pub mod formatter;
mod iter;

pub mod prelude {
    pub use crate::formatter::FormattedIns;
    pub use crate::Field;
    pub use crate::Field::*;
    pub use crate::Ins;
    pub use crate::Opcode::*;
    pub use crate::{
        Bit, BranchDest, CRBit, CRField, Offset, OpaqueU, Simm, Uimm, FPR, GPR, GQR, SPR, SR,
    };
}

macro_rules! field_arg {
    ($name:ident, $typ:ident) => {
        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
        pub struct $name(pub $typ);
    };
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

// General-purpose register.
field_arg!(GPR, u8);
// Floating-point register (direct or paired-singles mode).
field_arg!(FPR, u8);
// Segment register.
field_arg!(SR, u8);
// Special-purpose register.
field_arg!(SPR, u16);
// Condition register field.
field_arg!(CRField, u8);
// Condition register bit (index + condition case).
field_arg!(CRBit, u8);
// Paired-single graphics quantization register
field_arg!(GQR, u8);
// Unsigned immediate.
field_arg!(Uimm, u16);
// Signed immediate.
field_arg!(Simm, i16);
// Offset for indirect memory reference.
field_arg!(Offset, u32);
// Branch destination.
field_arg!(BranchDest, u32);
// Opaque zero or one argument.
field_arg!(Bit, bool);
// Unsigned opaque argument.
field_arg!(OpaqueU, u32);

// Generate the Field enum and impls.
fields!();

#[derive(Debug, Default)]
pub struct Modifiers {
    pub oe: bool,
    pub rc: bool,
    pub lk: bool,
    pub aa: bool,
}

impl std::fmt::Display for Modifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.aa {
            write!(f, "a")?;
        }
        if self.lk {
            write!(f, "l")?;
        }
        if self.oe {
            write!(f, "o")?;
        }
        if self.rc {
            write!(f, ".")?;
        }
        Ok(())
    }
}

// Generate the Opcode enum and impls.
// TODO This could be made more readable with a derive over an empty enum.
opcodes!();

impl Opcode {
    /// Detects the opcode of a machine code instruction.
    pub fn detect(code: u32) -> Self {
        Self::_detect(code) // auto-generated
    }

    /// Prints the basic mnemonic of an opcode.
    pub fn mnemonic(self) -> &'static str {
        self._mnemonic() // auto-generated
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::Illegal
    }
}

impl std::string::ToString for Opcode {
    fn to_string(&self) -> String {
        let mnemonic = self.mnemonic();
        mnemonic.to_owned()
    }
}

/// A PowerPC 750CL instruction.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Ins {
    pub code: u32,
    pub addr: u32,
    pub op: Opcode,
}

impl Ins {
    /// Constructs an instruction from the given machine code and address.
    pub fn new(code: u32, addr: u32) -> Self {
        Self {
            code,
            addr,
            op: Opcode::detect(code),
        }
    }

    /// Gets the fields of an instruction.
    pub fn fields(&self) -> Vec<Field> {
        self._fields() // auto-generated
    }

    /// Gets the modifiers of an instruction.
    pub fn modifiers(&self) -> Modifiers {
        self._modifiers() // auto-generated
    }

    /// Gets the defs of an instruction.
    pub fn defs(&self) -> Vec<Field> {
        self._defs() // auto-generated
    }

    /// Gets the uses of an instruction.
    pub fn uses(&self) -> Vec<Field> {
        self._uses() // auto-generated
    }

    /// Gets the given bit from the machine code instruction.
    pub fn bit(&self, idx: usize) -> bool {
        bit(self.code, idx)
    }

    /// Gets the given range of btis from the machine code instruction.
    pub fn bits(&self, range: Range<usize>) -> u32 {
        bits(self.code, range)
    }
}

ins_impl!();
