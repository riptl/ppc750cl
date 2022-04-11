use dol::{Dol, DolSection};
use ppc750cl::{disasm_iter, Ins, Opcode};
use ppc750cl_analysis::{BasicSlices, IntrinsicsLocs};
use std::fmt::{Debug, Formatter};

use crate::Result;

pub struct TextInfo {
    pub section_info: DolSection,
    pub instruction_count: usize,
    pub intrinsics_locs: IntrinsicsLocs,
    pub basic_slices: BasicSlices,
}

impl TextInfo {
    /// Analyzes the .text section given the DOL and an arbitrary address within the text section.
    pub fn from_dol(dol: &Dol, arb_text_addr: u32) -> Result<Self> {
        let section = dol
            .header
            .section_at(arb_text_addr)
            .ok_or("cannot find .text section")?;

        // Disassemble everything.
        let data = dol.section_data(section);
        let code: Vec<Ins> = disasm_iter(data, section.target).collect();
        // Control-flow analysis.
        let intrinsics_locs = IntrinsicsLocs::from_code(&code);
        let basic_slices = BasicSlices::from_code(&code);

        Ok(Self {
            section_info: section.clone(),
            instruction_count: code.iter().filter(|ins| ins.op != Opcode::Illegal).count(),
            intrinsics_locs,
            basic_slices,
        })
    }
}

impl Debug for TextInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".text:")?;
        writeln!(
            f,
            "  Section: {:#08x}..{:#08x}",
            self.section_info.target,
            self.section_info.target + self.section_info.size
        )?;
        if let Some(s) = self.intrinsics_locs.save_fpr_sled {
            writeln!(f, "  Save FPR sled: {:#08x}", s)?;
        }
        if let Some(s) = self.intrinsics_locs.rest_fpr_sled {
            writeln!(f, "  Rest FPR sled: {:#08x}", s)?;
        }
        if let Some(s) = self.intrinsics_locs.save_gpr_sled {
            writeln!(f, "  Save GPR sled: {:#08x}", s)?;
        }
        if let Some(s) = self.intrinsics_locs.rest_gpr_sled {
            writeln!(f, "  Rest GPR sled: {:#08x}", s)?;
        }
        writeln!(f, "  Instruction count: {}", self.instruction_count)?;
        writeln!(f, "  Function count: {}", self.basic_slices.funcs.len())?;
        writeln!(
            f,
            "  Basic block count: {}",
            self.basic_slices.cuts.len() - 1
        )?;
        Ok(())
    }
}
