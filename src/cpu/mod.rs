
pub mod registers;
pub mod decoder;
pub mod instructions;

use registers::Registers;
use decoder::decode;
use instructions::execute_instruction;

pub struct Cpu {
    registers: Registers
}

impl Cpu {
    pub fn new() -> Self {
        Self { registers: Registers::new() }
    }

    pub fn step(&mut self) {
        // Fetch
        let dummy_byte: u8 = 0x00; 
        // Execute
        execute_instruction(decode(dummy_byte));
    }
}