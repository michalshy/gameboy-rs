
pub mod registers;
pub mod decoder;
pub mod instructions;

use registers::Registers;
use decoder::decode;

use crate::mmu::Mmu;

pub struct Cpu {
    pub registers: Registers
}

impl Cpu {
    pub fn new() -> Self {
        Self { registers: Registers::new() }
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> u32 {
        let opcode_byte = mmu.read_8(self.registers.pc);
        
        let entry = decode(opcode_byte);

        self.execute_instruction(entry);
    
        entry.cycles as u32
    }
}