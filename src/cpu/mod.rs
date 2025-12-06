use crate::cpu::registers::Registers;

pub mod registers;
pub mod decoder;
pub mod instructions;

pub struct Cpu {
    registers: Registers
}

impl Cpu {
    pub fn new() -> Self {
        Self { registers: Registers::new() }
    }

    pub fn step(&mut self) {

    }
}