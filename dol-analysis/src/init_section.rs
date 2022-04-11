use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};

use dol::{Dol, DolSection};
use ppc750cl::{disasm_iter, Ins};

use crate::Result;

pub struct InitInfo<'a> {
    pub section_info: DolSection,
    pub trk_signature: &'a str,
    pub entry_point: u32,
    pub code_start: u32,
    pub code_stop: u32,
    pub text_call_targets: Vec<u32>, // not empty
}

impl<'a> InitInfo<'a> {
    /// Analyzes an .init section and returns recovered info.
    pub fn from_dol(dol: &'a Dol) -> Result<Self> {
        let section = dol
            .header
            .section_at(dol.header.entry_point)
            .ok_or("cannot find init section")?;
        let data = dol.section_data(section);

        // Basic facts.
        let mut this = Self {
            section_info: section.clone(),
            trk_signature: Self::find_trk_signature(data)?,
            entry_point: dol.header.entry_point,
            code_start: dol.header.entry_point,
            code_stop: Self::find_section_table(section, data)
                .ok_or("Failed to find .init section table")?,
            text_call_targets: Vec::new(),
        };
        // The entry point might be in the middle of the "code" part of the .init section.
        let function_call_targets = this.find_code_before_entry_point(data)?;
        // Assume any function call targets outside the .init section belong to .text.
        this.text_call_targets = function_call_targets
            .iter()
            .flat_map(|target| {
                if *target > this.code_stop {
                    Some(*target)
                } else {
                    None
                }
            })
            .collect();
        Ok(this)
    }

    /// Returns the signature string at the very beginning of the .init section.
    fn find_trk_signature(init_data: &[u8]) -> Result<&str> {
        let signature = &init_data[..0x100];
        let pos = signature
            .iter()
            .position(|b| *b == 0x00)
            .ok_or("Failed to find TRK signature")?;
        Ok(std::str::from_utf8(&signature[..pos]).map_err(|_| "Invalid TRK signature")?)
    }

    /// Returns the virtual address of the section table.
    /// It also marks the end of the entry point code area.
    fn find_section_table(section: &DolSection, haystack: &[u8]) -> Option<u32> {
        // The section table in DOL begins with a descriptor of the init section itself.
        let mut needle = [0u8; 8];
        needle[0..4].copy_from_slice(&section.target.to_be_bytes());
        needle[4..8].copy_from_slice(&section.target.to_be_bytes());
        let offset = twoway::find_bytes(haystack, &needle)?;
        // Return virtual address of section table.
        Some(section.target + offset as u32)
    }

    /// Returns the currently discovered instructions of the entry point area.
    fn code(&self, data: &'a [u8]) -> impl Iterator<Item = Ins> + 'a {
        let start_offset = (self.code_start - self.section_info.target) as usize;
        let stop_offset = (self.code_stop - self.section_info.target) as usize;
        disasm_iter(&data[start_offset..stop_offset], self.code_start)
    }

    /// Returns all function call targets in the currently known code part of the init section.
    fn function_call_targets(&self, data: &[u8]) -> BTreeSet<u32> {
        let mut funcs = BTreeSet::new();
        for ins in self.code(data) {
            if ins.is_direct_branch() && ins.field_LK() {
                funcs.insert(ins.branch_dest().unwrap());
            }
        }
        funcs
    }

    /// The entry point area might have code preceding the entry point itself.
    ///
    /// So we repeatedly check if any calls are being made to instructions
    /// that precede the currently known entry point area.
    ///
    /// Returns the result from the last call to `self.function_call_targets()`.
    fn find_code_before_entry_point(&mut self, data: &[u8]) -> Result<BTreeSet<u32>> {
        loop {
            let call_targets = self.function_call_targets(data);
            let first_call_target = *call_targets
                .iter()
                .next()
                .ok_or("Found no function calls in .init")?;
            if first_call_target < self.code_start {
                self.code_start = first_call_target;
            } else {
                return Ok(call_targets);
            }
        }
    }
}

impl<'a> Debug for InitInfo<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".init:")?;
        writeln!(
            f,
            "  Section: {:#08x}..{:#08x}",
            self.section_info.target,
            self.section_info.target + self.section_info.size
        )?;
        writeln!(
            f,
            "  Code: {:#08x}..{:#08x}",
            self.code_start, self.code_stop
        )?;
        writeln!(f, "  Entrypoint: {:#08x}", self.entry_point)?;
        writeln!(f, "  TRK signature: {}", self.trk_signature)?;
        writeln!(f, "  Calls to .text:")?;
        for call in &self.text_call_targets {
            writeln!(f, "  ~> {:#08x}", *call)?;
        }
        Ok(())
    }
}
