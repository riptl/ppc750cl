use crate::Ins;

/// Returns an iterator of instructions in the given byte slice.
pub fn disasm_iter(code: &[u8], addr: u32) -> DisasmIterator {
    DisasmIterator { code, addr }
}

pub struct DisasmIterator<'a> {
    code: &'a [u8],
    addr: u32,
}

impl<'a> Iterator for DisasmIterator<'a> {
    type Item = Ins;

    fn next(&mut self) -> Option<Self::Item> {
        if self.code.len() < 4 {
            return None;
        }
        let code = ((self.code[0] as u32) << 24)
            | ((self.code[1] as u32) << 16)
            | ((self.code[2] as u32) << 8)
            | (self.code[3] as u32);
        self.code = &self.code[4..];
        let addr = self.addr;
        self.addr += 4;
        Some(Ins::new(code, addr))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.code.len() / 4;
        (count, Some(count))
    }
}
