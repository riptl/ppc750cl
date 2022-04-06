use std::fmt::{Display, Formatter, LowerHex, UpperHex, Write};
use std::ops::Range;

use num_traits::{AsPrimitive, PrimInt};

pub use crate::iter::{disasm_iter, DisasmIterator};

pub mod formatter;
mod generated;
mod iter;
use generated::*;

pub mod prelude {
    pub use crate::formatter::FormattedIns;
    pub use crate::Argument;
    pub use crate::Field::*;
    pub use crate::Ins;
    pub use crate::Modifiers;
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
field_arg!(FPR, u8, "fr{}");
// Segment register.
field_arg!(SR, u8);
// Special-purpose register.
field_arg!(SPR, u16);
// Condition register field.
field_arg!(CRField, u8, "crb{}");
// Condition register bit (index + condition case).
field_arg!(CRBit, u8, "crf{}");
// Paired-single graphics quantization register
field_arg!(GQR, u8);
// Unsigned immediate.
field_arg!(Uimm, u16, "{:#x}");
// Signed immediate.
field_arg!(Simm, i16, "{:#x}", ReallySigned);
// Offset for indirect memory reference.
field_arg!(Offset, i32, "{:#x}", ReallySigned);
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

impl Field {
    pub fn argument(&self) -> Option<Argument> {
        match self {
            Field::simm(x) => Some(Argument::Simm(*x)),
            Field::uimm(x) => Some(Argument::Uimm(*x)),
            Field::offset(x) => Some(Argument::Offset(*x)),
            Field::BO(x) => Some(Argument::OpaqueU(*x)),
            Field::BI(x) => Some(Argument::OpaqueU(*x)),
            Field::BD(x) => Some(Argument::BranchDest(*x)),
            Field::LI(x) => Some(Argument::BranchDest(*x)),
            Field::SH(x) => Some(Argument::OpaqueU(*x)),
            Field::MB(x) => Some(Argument::OpaqueU(*x)),
            Field::ME(x) => Some(Argument::OpaqueU(*x)),
            Field::rS(x) => Some(Argument::GPR(*x)),
            Field::rD(x) => Some(Argument::GPR(*x)),
            Field::rA(x) => Some(Argument::GPR(*x)),
            Field::rB(x) => Some(Argument::GPR(*x)),
            Field::rC(x) => Some(Argument::GPR(*x)),
            Field::sr(x) => Some(Argument::SR(*x)),
            Field::spr(x) => Some(Argument::SPR(*x)),
            Field::frS(x) => Some(Argument::FPR(*x)),
            Field::frD(x) => Some(Argument::FPR(*x)),
            Field::frA(x) => Some(Argument::FPR(*x)),
            Field::frB(x) => Some(Argument::FPR(*x)),
            Field::frC(x) => Some(Argument::FPR(*x)),
            Field::crbD(x) => Some(Argument::CRField(*x)),
            Field::crbA(x) => Some(Argument::CRField(*x)),
            Field::crbB(x) => Some(Argument::CRField(*x)),
            Field::crfD(x) => Some(Argument::CRBit(*x)),
            Field::crfS(x) => Some(Argument::CRBit(*x)),
            Field::crm(x) => Some(Argument::OpaqueU(*x)),
            Field::ps_l(x) => Some(Argument::GQR(*x)),
            Field::ps_W(x) => Some(Argument::OpaqueU(*x)),
            Field::NB(x) => Some(Argument::OpaqueU(*x)),
            Field::tbr(x) => Some(Argument::OpaqueU(*x)),
            Field::mtfsf_FM(x) => Some(Argument::OpaqueU(*x)),
            Field::mtfsf_IMM(x) => Some(Argument::OpaqueU(*x)),
            Field::TO(x) => Some(Argument::OpaqueU(*x)),
            _ => None,
        }
    }
}

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

    /// Returns the simplified representation of an instruction.
    pub fn simplified(self) -> SimplifiedIns {
        self._simplified() // auto-generated
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

/// A simplified PowerPC 750CL instruction.
pub struct SimplifiedIns {
    pub ins: Ins,
    pub mnemonic: &'static str,
    pub modifiers: Modifiers,
    pub args: Vec<Argument>,
}

impl Display for SimplifiedIns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} ", self.mnemonic, self.modifiers)?;
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
