pub mod decoder;
pub mod instructions;
pub mod interrupts;
pub mod registers;

use decoder::decode;
use registers::Registers;
use std::{fs::File, io::Error};
use std::io::Write;

use crate::{
    cpu::{decoder::OpcodeEntry, interrupts::Interrupts},
    debug::{Debug, disasm::disassemble},
    mmu::Mmu,
};

pub struct Cpu {
    pub registers: Registers,
    pub int: Interrupts,
    pub history: Vec<String>,
    pub instruction_number: u128,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            int: Interrupts::new(),
            history: Vec::new(),
            instruction_number: 0,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, debug: &Debug) -> u32 {
        let opcode_byte = mmu.read_8(self.registers.pc);

        let entry = decode(opcode_byte);

        self.execute_instruction(entry, mmu, true);

        if debug.log_cpu {
            self.history.push(disassemble(&entry.opcode));
        }

        entry.cycles as u32
    }

    pub fn get_current_opcode(&self, mmu: &Mmu) -> &OpcodeEntry {
        let opcode_byte = mmu.read_8(self.registers.pc);
        decode(opcode_byte)
    }

    pub fn dump_history(&mut self, path: &String) -> Result<(), Error> {
        let mut file = File::create(path)?;

        for line in &self.history {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
}
