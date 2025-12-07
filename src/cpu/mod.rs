
pub mod registers;
pub mod decoder;
pub mod instructions;

use registers::Registers;
use decoder::decode;

pub struct Cpu {
    registers: Registers
}

impl Cpu {
    pub fn new() -> Self {
        Self { registers: Registers::new() }
    }

    pub fn step(&mut self) {
        let opcode_byte = 0x00; // (temporary placeholder)
        
        let entry = decode(opcode_byte);

        self.execute_instruction(entry);
    }
}