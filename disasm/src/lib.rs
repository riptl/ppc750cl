//pub mod formatter;
mod iter;

pub mod prelude {
    pub use crate::Field::*;
    pub use crate::Ins;
    pub use crate::Opcode::*;
}

use ppc750cl_macros::{fields, ins_impl, opcodes};

//pub use crate::formatter::AsmFormatter;
//use crate::formatter::SimpleFormatter;
pub use crate::iter::{disasm_iter, DisasmIterator};

macro_rules! field_arg {
    ($name:ident, $typ:ident) => {
        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
        pub struct $name(pub $typ);
    };
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

// Generate the Opcode enum and impls.
// TODO This could be made more readable with a derive over an empty enum.
opcodes!();

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
}

ins_impl!();

/*
impl ToString for Ins {
    fn to_string(&self) -> String {
        let buf = Vec::<u8>::new();
        let mut formatter = SimpleFormatter::new(buf);
        self.write_string(&mut formatter).unwrap();
        unsafe { String::from_utf8_unchecked(formatter.writer) }
    }
}
*/
