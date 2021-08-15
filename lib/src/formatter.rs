use std::fmt::{Display, LowerHex, UpperHex};
use std::io::Write;

use num_traits::PrimInt;

use crate::Ins;

type IOResult = std::io::Result<()>;

pub trait AsmFormatter<W>
where
    W: Write,
{
    /// Returns the underlying writer.
    fn writer(&mut self) -> &mut W;

    /// Callback for custom styling before writing an instruction.
    fn before_instruction(&mut self, _: &Ins) -> IOResult {
        Ok(())
    }
    /// Callback for custom styling after writing an instruction.
    fn after_instruction(&mut self, _: &Ins) -> IOResult {
        Ok(())
    }

    /// Writes the instruction mnemonic.
    fn write_mnemonic(&mut self, name: &str) -> IOResult {
        write!(self.writer(), "{}", name)
    }

    /// Separates the instruction mnemonic and arguments.
    fn write_opcode_separator(&mut self) -> IOResult {
        write!(self.writer(), " ")
    }

    /// Separates two instruction arguments (e.g. registers).
    fn write_operand_separator(&mut self) -> IOResult {
        write!(self.writer(), ", ")
    }

    /// Writes a general-purpose register argument.
    fn write_gpr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "r{}", reg)
    }

    /// Writes a floating point register argument.
    fn write_fpr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "f{}", reg)
    }

    /// Writes a condition register argument.
    fn write_cr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "cr{}", reg)
    }

    /// Writes a paired-singles quantization register argument.
    fn write_qr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "qr{}", reg)
    }

    fn write_sr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "{}", reg)
    }

    /// Sets the mnemonic 'o' suffix.
    fn write_oe(&mut self, oe: u8) -> IOResult {
        if oe != 0 {
            write!(self.writer(), "o")?;
        }
        Ok(())
    }

    /// Sets the mnemonic 'a' suffix.
    fn write_aa(&mut self, aa: u8) -> IOResult {
        if aa != 0 {
            write!(self.writer(), "a")?;
        }
        Ok(())
    }

    /// Sets the mnemonic 'l' suffix.
    fn write_lk(&mut self, lk: u8) -> IOResult {
        if lk != 0 {
            write!(self.writer(), "l")?;
        }
        Ok(())
    }

    /// Sets the mnemonic '.' suffix.
    fn write_rc(&mut self, rc: u8) -> IOResult {
        if rc != 0 {
            write!(self.writer(), ".")?;
        }
        Ok(())
    }

    /// Writes an unsigned immediate.
    fn write_uimm(&mut self, uimm: u16) -> IOResult {
        write!(self.writer(), "{:#x}", uimm)
    }

    /// Writes a signed immediate.
    fn write_simm(&mut self, simm: i16) -> IOResult {
        write!(self.writer(), "{:#x}", ReallySigned(simm))
    }

    /// Writes an instruction-specific field like the compare mode.
    fn write_mode<P: PrimInt + Display>(&mut self, mode: P) -> IOResult {
        write!(self.writer(), "{}", mode)
    }

    fn write_fm(&mut self, fm: u16) -> IOResult {
        write!(self.writer(), "{}", fm)
    }

    fn write_offset_unsigned_open(&mut self, offset: u16) -> IOResult {
        write!(self.writer(), "{:#x}(", offset)
    }

    /// Writes an offset prefix.
    ///
    /// The next write calls that follow should be:
    ///   - An operand (almost always a general-purpose register)
    ///   - `write_offset_close()`
    fn write_offset_open(&mut self, offset: i16) -> IOResult {
        write!(self.writer(), "{:#x}(", ReallySigned(offset))
    }

    /// Closes an offset prefix.
    fn write_offset_close(&mut self) -> IOResult {
        write!(self.writer(), ")")
    }

    /// Writes a branch target given the jump offset and current program counter.
    fn write_branch_target(&mut self, offset: u32, pc: u32) -> IOResult {
        write!(self.writer(), "{:#x}", offset + pc)
    }
}

pub struct SimpleFormatter<W: Write> {
    pub writer: W,
}

impl<W: Write> AsmFormatter<W> for SimpleFormatter<W> {
    fn writer(&mut self) -> &mut W {
        &mut self.writer
    }
}

pub struct DoldecompFormatter<W: Write> {
    pub writer: W,
}

impl<W: Write> AsmFormatter<W> for DoldecompFormatter<W> {
    fn writer(&mut self) -> &mut W {
        &mut self.writer
    }

    fn before_instruction(&mut self, ins: &Ins) -> IOResult {
        write!(
            &mut self.writer,
            "/* TODO */  {:X} {:X} {:X} {:X}\t",
            (ins.code >> 24) as u8,
            (ins.code >> 16) as u8,
            (ins.code >> 8) as u8,
            ins.code as u8
        )
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
