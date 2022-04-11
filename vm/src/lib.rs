use ppc750cl::{Ins, Opcode};

/// Machine is a very basic PowerPC 750CL virtual machine.
///
/// It implements the bare minimum for CFA to work and
/// therefore makes no attempt at behaving correctly.
// TODO use sparse memory
// TODO mem cells
// TODO addends
// TODO separate stack
pub struct Machine {
    pub gpr: [u32; 32],
    pub fpr: [u32; 32],
    pub spr: [u32; 1024],
    pub mem1: [u8; 0x2000000],
}

impl Machine {
    const STACK_START: u32 = 0x80399180;
    const SDATA2_ADDR: u32 = 0x8038efa0;
    const SDATA_ADDR: u32 = 0x8038cc00;

    pub fn new() -> Self {
        let mut machine = Self {
            gpr: [0u32; 32],
            fpr: [0u32; 32],
            spr: [0u32; 1024],
            mem1: [0u8; 0x2000000],
        };
        machine.gpr[1] = Self::STACK_START;
        machine.gpr[2] = Self::SDATA2_ADDR;
        machine.gpr[13] = Self::SDATA_ADDR;
        machine
    }

    /// Executes a single instruction. Panics at will.
    pub fn step(&mut self, ins: &Ins) {
        match ins.op {
            Opcode::Addi => {
                self.gpr[ins.field_rD()] = if ins.field_rA() == 0 {
                    ins.field_uimm() as u32
                } else {
                    (self.gpr[ins.field_rA()] as i64 + ins.field_simm() as i64) as u32
                };
            }
            Opcode::Addis => {
                self.gpr[ins.field_rD()] = if ins.field_rA() == 0 {
                    (ins.field_uimm() as u32) << 16
                } else {
                    (self.gpr[ins.field_rA()] as i64 + ((ins.field_simm() as i64) << 16)) as u32
                };
            }
            Opcode::Or => {
                self.gpr[ins.field_rA()] = self.gpr[ins.field_rS()] | self.gpr[ins.field_rB()]
            }
            Opcode::Ori => {
                self.gpr[ins.field_rA()] = self.gpr[ins.field_rS()] | ins.field_uimm() as u32;
            }
            Opcode::Mfspr => self.gpr[ins.field_rD()] = self.spr[ins.field_spr()],
            Opcode::Mtspr => self.spr[ins.field_spr()] = self.gpr[ins.field_rS()],
            Opcode::Stw => {
                let ea = self.mem_access_ea(ins);
                self.store_u32(ea, self.gpr[ins.field_rS()]);
            }
            Opcode::Stwu => {
                let ea = self.mem_access_ea(ins);
                self.store_u32(ea, self.gpr[ins.field_rS()]);
                self.gpr[ins.field_rA()] = ea;
            }
            Opcode::Lwz => {
                let ea = self.mem_access_ea(ins);
                self.gpr[ins.field_rD()] = self.load_u32(ea);
            }
            _ => (),
        }
    }

    fn mem_access_ea(&self, ins: &Ins) -> u32 {
        ((self.gpr[ins.field_rA()] as isize) + ins.field_offset()) as u32
    }

    fn mem_ref(&self, at: usize) -> &[u8] {
        match at {
            0x80000000..=0x817FFFFF => &self.mem1[at - 0x80000000..],
            0xc0000000..=0xC17FFFFF => &self.mem1[at - 0xC0000000..],
            _ => &[],
        }
    }

    fn load_u32(&mut self, at: u32) -> u32 {
        match self.mem_ref(at as usize).try_into() {
            Ok(v) => u32::from_be_bytes(v),
            Err(_) => 0,
        }
    }

    fn mem_mut_ref(&mut self, at: usize) -> &mut [u8] {
        match at {
            0x80000000..=0x817FFFFF => &mut self.mem1[at - 0x80000000..],
            0xc0000000..=0xC17FFFFF => &mut self.mem1[at - 0xC0000000..],
            _ => &mut [],
        }
    }

    fn store_u32(&mut self, at: u32, val: u32) {
        let area = self.mem_mut_ref(at as usize);
        area[..4].copy_from_slice(&val.to_be_bytes());
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}
