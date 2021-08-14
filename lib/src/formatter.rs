use crate::Ins;
use std::io::Write;
use num_traits::PrimInt;
use std::fmt::{LowerHex, UpperHex};

type IOResult = std::io::Result<()>;

pub trait AsmFormatter<W>
where
    W: Write,
{
    fn writer(&mut self) -> &mut W;

    fn before_instruction(&mut self, _: &Ins) -> IOResult {
        Ok(())
    }
    fn after_instruction(&mut self, _: &Ins) -> IOResult {
        Ok(())
    }

    fn write_mnemonic(&mut self, name: &str) -> IOResult {
        write!(self.writer(), "{}", name)
    }

    fn write_opcode_separator(&mut self) -> IOResult {
        write!(self.writer(), " ")
    }

    fn write_operand_separator(&mut self) -> IOResult {
        write!(self.writer(), ", ")
    }

    fn write_gpr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "r{}", reg)
    }

    fn write_fpr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "f{}", reg)
    }

    fn write_cr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "cr{}", reg)
    }

    fn write_qr(&mut self, reg: u8) -> IOResult {
        write!(self.writer(), "qr{}", reg)
    }

    fn write_oe(&mut self, oe: u8) -> IOResult {
        if oe != 0 {
            write!(self.writer(), "o")?;
        }
        Ok(())
    }

    fn write_aa(&mut self, aa: u8) -> IOResult {
        if aa != 0 {
            write!(self.writer(), "a")?;
        }
        Ok(())
    }

    fn write_lk(&mut self, lk: u8) -> IOResult {
        if lk != 0 {
            write!(self.writer(), "l")?;
        }
        Ok(())
    }

    fn write_rc(&mut self, rc: u8) -> IOResult {
        if rc != 0 {
            write!(self.writer(), ".")?;
        }
        Ok(())
    }

    fn write_uimm(&mut self, uimm: u16) -> IOResult {
        write!(self.writer(), "{:#x}", uimm)
    }

    fn write_simm(&mut self, simm: i16) -> IOResult {
        write!(self.writer(), "{:#x}", ReallySigned(simm))
    }

    fn write_fm(&mut self, fm: u16) -> IOResult {
        write!(self.writer(), "{}", fm)
    }

    fn write_offset_unsigned_open(&mut self, offset: u16) -> IOResult {
        write!(self.writer(), "{:#x}(", offset)
    }

    fn write_offset_open(&mut self, offset: i16) -> IOResult {
        write!(self.writer(), "{:#x}(", ReallySigned(offset))
    }

    fn write_offset_close(&mut self) -> IOResult {
        write!(self.writer(), ")")
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
