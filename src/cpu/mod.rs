pub mod decoder;
pub mod instructions;
pub mod interrupts;
pub mod registers;

use decoder::decode;
use registers::Registers;
use std::collections::VecDeque;

use crate::{
    cpu::{decoder::OpcodeEntry, interrupts::Interrupts},
    debug::disasm::disassemble,
    mmu::Mmu,
};

const HISTORY_CAPACITY: usize = 25;

pub struct Cpu {
    pub registers: Registers,
    pub int: Interrupts,
    pub history: VecDeque<String>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            int: Interrupts::new(),
            history: VecDeque::with_capacity(HISTORY_CAPACITY),
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> u32 {
        let opcode_byte = mmu.read_8(self.registers.pc);

        let entry = decode(opcode_byte);

        self.execute_instruction(entry, mmu);
        
        if self.history.len() == HISTORY_CAPACITY {
            self.history.pop_front(); // remove oldest
        }
        self.history.push_back(disassemble(&entry.opcode));

        entry.cycles as u32
    }

    pub fn get_current_opcode(&self, mmu: &Mmu) -> &OpcodeEntry {
        let opcode_byte = mmu.read_8(self.registers.pc);
        decode(opcode_byte)
    }
}
