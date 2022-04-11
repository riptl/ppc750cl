use std::fmt::{Display, Formatter, LowerHex, UpperHex, Write};
use std::ops::Range;

use num_traits::{AsPrimitive, PrimInt};

pub use crate::iter::{disasm_iter, DisasmIterator};

pub mod formatter;
mod generated;
mod iter;
pub use generated::*;

pub mod prelude {
    pub use crate::formatter::FormattedIns;
    pub use crate::Argument;
    pub use crate::Field::*;
    pub use crate::Ins;
    pub use crate::Opcode::*;
    pub use crate::SimplifiedIns;
    pub use crate::{
        Bit, BranchDest, CRBit, CRField, Offset, OpaqueU, Simm, Uimm, FPR, GPR, GQR, SPR, SR,
    };
}

macro_rules! field_arg_no_display {
    ($name:ident, $typ:ident) => {
        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
        pub struct $name(pub $typ);
        impl std::convert::From<$name> for Argument {
            fn from(x: $name) -> Argument {
                Argument::$name(x)
            }
        }
    };
}

macro_rules! field_arg {
    ($name:ident, $typ:ident) => {
        field_arg_no_display!($name, $typ);
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    ($name:ident, $typ:ident, $format:literal) => {
        field_arg_no_display!($name, $typ);
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, $format, self.0)
            }
        }
    };
    ($name:ident, $typ:ident, $format:literal, $format_arg:expr) => {
        field_arg_no_display!($name, $typ);
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, $format, $format_arg(self.0))
            }
        }
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

// General-purpose register.
field_arg!(GPR, u8, "r{}");
// Floating-point register (direct or paired-singles mode).
field_arg!(FPR, u8, "f{}");
// Segment register.
field_arg!(SR, u8);
// Special-purpose register.
field_arg!(SPR, u16);
// Condition register field.
field_arg!(CRField, u8, "cr{}");
// Condition register bit (index + condition case).
field_arg!(CRBit, u8, "{}");
// Paired-single graphics quantization register
field_arg!(GQR, u8, "qr{}");
// Unsigned immediate.
field_arg!(Uimm, u16, "{:#x}");
// Signed immediate.
field_arg!(Simm, i16, "{:#x}", ReallySigned);
// Offset for indirect memory reference.
field_arg!(Offset, i16, "{:#x}", ReallySigned);
// Branch destination.
field_arg!(BranchDest, i32, "{:#x}", ReallySigned);
// Opaque zero or one argument.
field_arg_no_display!(Bit, bool);
impl Display for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(if self.0 { '1' } else { '0' })
    }
}
// Unsigned opaque argument.
field_arg!(OpaqueU, u32);

const SPR_LR: usize = 16;

#[derive(Debug, Clone)]
pub enum Argument {
    GPR(GPR),
    FPR(FPR),
    SR(SR),
    SPR(SPR),
    CRField(CRField),
    CRBit(CRBit),
    GQR(GQR),
    Uimm(Uimm),
    Simm(Simm),
    Offset(Offset),
    BranchDest(BranchDest),
    Bit(Bit),
    OpaqueU(OpaqueU),
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::GPR(x) => x.fmt(f),
            Argument::FPR(x) => x.fmt(f),
            Argument::SR(x) => x.fmt(f),
            Argument::SPR(x) => x.fmt(f),
            Argument::CRField(x) => x.fmt(f),
            Argument::CRBit(x) => x.fmt(f),
            Argument::GQR(x) => x.fmt(f),
            Argument::Uimm(x) => x.fmt(f),
            Argument::Simm(x) => x.fmt(f),
            Argument::Offset(x) => x.fmt(f),
            Argument::BranchDest(x) => x.fmt(f),
            Argument::Bit(x) => x.fmt(f),
            Argument::OpaqueU(x) => x.fmt(f),
        }
    }
}

impl From<Argument> for i64 {
    fn from(arg: Argument) -> Self {
        match arg {
            Argument::GPR(x) => x.0 as i64,
            Argument::FPR(x) => x.0 as i64,
            Argument::SR(x) => x.0 as i64,
            Argument::SPR(x) => x.0 as i64,
            Argument::CRField(x) => x.0 as i64,
            Argument::CRBit(x) => x.0 as i64,
            Argument::GQR(x) => x.0 as i64,
            Argument::Uimm(x) => x.0 as i64,
            Argument::Simm(x) => x.0 as i64,
            Argument::Offset(x) => x.0 as i64,
            Argument::BranchDest(x) => x.0 as i64,
            Argument::Bit(x) => x.0 as i64,
            Argument::OpaqueU(x) => x.0 as i64,
        }
    }
}

impl TryInto<Argument> for &Field {
    type Error = ();
    fn try_into(self) -> Result<Argument, Self::Error> {
        self.argument().ok_or(())
    }
}

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
    const BLR: u32 = 0x4e800020;

    /// Constructs an instruction from the given machine code and address.
    pub fn new(code: u32, addr: u32) -> Self {
        Self {
            code,
            addr,
            op: Opcode::detect(code),
        }
    }

    /// Returns the simplified representation of an instruction.
    pub fn simplified(self) -> SimplifiedIns {
        self._simplified() // auto-generated
    }

    /// Gets the fields of an instruction.
    pub fn fields(&self) -> Vec<Field> {
        self._fields() // auto-generated
    }

    /// Gets the suffix of an instruction mnemonic.
    pub fn suffix(&self) -> String {
        self._suffix() // auto-generated
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

    pub fn branch_offset(&self) -> Option<i32> {
        match self.op {
            Opcode::B => Some(self.field_LI() as i32),
            Opcode::Bc | Opcode::Bcctr | Opcode::Bclr => Some(self.field_BD() as i32),
            _ => None,
        }
    }

    pub fn branch_dest(&self) -> Option<u32> {
        // FIXME absolute branches not supported
        self.branch_offset().and_then(|offset| {
            if offset < 0 {
                self.addr.checked_sub((-offset) as u32)
            } else {
                self.addr.checked_add(offset as u32)
            }
        })
    }

    pub fn is_branch(&self) -> bool {
        matches!(self.op, Opcode::B | Opcode::Bc | Opcode::Bcctr | Opcode::Bclr)
    }

    pub fn is_direct_branch(&self) -> bool {
        matches!(self.op, Opcode::B | Opcode::Bc)
    }

    pub fn is_unconditional_branch(&self) -> bool {
        match self.op {
            Opcode::B => true,
            Opcode::Bc | Opcode::Bcctr | Opcode::Bclr => {
                self.field_BO() == 20 && self.field_BI() == 0
            }
            _ => false,
        }
    }

    pub fn is_conditional_branch(&self) -> bool {
        self.is_branch() && !self.is_unconditional_branch()
    }

    #[inline]
    pub fn is_blr(&self) -> bool {
        // self.op == Opcode::Bclr && self.is_unconditional_branch() && !self.field_LK()
        self.code == Ins::BLR
    }
}

/// A simplified PowerPC 750CL instruction.
pub struct SimplifiedIns {
    pub ins: Ins,
    pub mnemonic: &'static str,
    pub args: Vec<Argument>,
}

impl Display for SimplifiedIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} ", self.mnemonic, self.ins.suffix())?;
        let mut writing_offset = false;
        for (i, argument) in self.args.iter().enumerate() {
            write!(f, "{}", argument)?;
            if let Argument::Offset(_) = argument {
                write!(f, "(")?;
                writing_offset = true;
                continue;
            }
            if writing_offset {
                write!(f, ")")?;
                writing_offset = false;
            }
            if i != self.args.len() - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

impl SimplifiedIns {
    pub(crate) fn basic_form(ins: Ins) -> Self {
        Self {
            mnemonic: ins.op.mnemonic(),
            args: ins
                .fields()
                .iter()
                .flat_map(|field| field.argument())
                .collect(),
            ins,
        }
    }
}
