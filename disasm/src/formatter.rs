use std::fmt::{Display, LowerHex, UpperHex};
use std::io::Write;

use num_traits::PrimInt;

use crate::Ins;

/*
type IOResult = std::io::Result<()>;

pub trait AsmFormatter<W>
where
    W: Write,
{
    fn write_ins(ins: &Ins);
}

pub struct SimpleFormatter();

impl<W: Write> SimpleFormatter {
    /// Writes the instruction mnemonic.
    fn write_mnemonic(writer: &mut W, name: &str) -> IOResult {
        write!(writer, "{}", name)
    }

    /// Separates the instruction mnemonic and arguments.
    fn write_opcode_separator(writer: &mut W) -> IOResult {
        write!(writer, " ")
    }

    /// Separates two instruction arguments (e.g. registers).
    fn write_operand_separator(writer: &mut W) -> IOResult {
        write!(writer, ", ")
    }

    /// Writes a general-purpose register argument.
    fn write_gpr(writer: &mut W, reg: u8) -> IOResult {
        write!(writer, "r{}", reg)
    }

    /// Writes a nullable general-purpose register argument.
    fn write_gpr0(writer: &mut W, reg: u8) -> IOResult {
        if reg != 0 {
            Self::write_gpr(writer, reg)
        } else {
            write!(writer, "0")
        }
    }

    /// Writes a floating point register argument.
    fn write_fpr(writer: &mut W, reg: u8) -> IOResult {
        write!(writer, "f{}", reg)
    }

    /// Writes a condition register argument.
    fn write_cr(writer: &mut W, reg: u8) -> IOResult {
        write!(writer, "cr{}", reg)
    }

    /// Writes a paired-singles quantization register argument.
    fn write_qr(writer: &mut W, reg: u8) -> IOResult {
        write!(writer, "qr{}", reg)
    }

    fn write_sr(writer: &mut W, reg: u8) -> IOResult {
        write!(writer, "{}", reg)
    }

    /// Sets the mnemonic 'o' suffix.
    fn write_oe(writer: &mut W, oe: u8) -> IOResult {
        if oe != 0 {
            write!(writer, "o")?;
        }
        Ok(())
    }

    /// Sets the mnemonic 'a' suffix.
    fn write_aa(writer: &mut W, aa: u8) -> IOResult {
        if aa != 0 {
            write!(writer, "a")?;
        }
        Ok(())
    }

    /// Sets the mnemonic 'l' suffix.
    fn write_lk(writer: &mut W, lk: u8) -> IOResult {
        if lk != 0 {
            write!(writer, "l")?;
        }
        Ok(())
    }

    /// Sets the mnemonic '.' suffix.
    fn write_rc(writer: &mut W, rc: u8) -> IOResult {
        if rc != 0 {
            write!(writer, ".")?;
        }
        Ok(())
    }
}

impl<W: Write> AsmFormatter<W> for SimpleFormatter<W> {
    fn write_ins(ins: &Ins) {
        todo!()
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
*/
