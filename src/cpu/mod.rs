pub mod decoder;
pub mod instructions;
pub mod interrupts;
pub mod registers;

use decoder::decode;
use registers::Registers;
use std::io::Write;
use std::{fs::File, io::Error};

use crate::{
    cpu::{decoder::OpcodeEntry, interrupts::Interrupts},
    debug::{Debug, disasm::disassemble},
    interrupt_controller::Interrupt,
    mmu::Mmu,
};

pub struct Cpu {
    pub registers: Registers,
    pub interrupts: Interrupts,
    pub history: Vec<String>,
    pub instruction_number: u128,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            interrupts: Interrupts::new(),
            history: Vec::new(),
            instruction_number: 0,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu, debug: &Debug) -> u32 {
        if self.interrupts.ime_scheduled {
            self.interrupts.set_ime();
        }

        if self.interrupts.halted {
            if mmu.interrupts.pending_mask() != 0 {
                self.interrupts.halted = false;
            }

            return 4;
        }

        if self.interrupts.ime
            && let Some(irq) = mmu.interrupts.highest()
        {
            mmu.interrupts.ack(irq);
            self.service_interrupt(irq, mmu);
            return 20;
        }

        let opcode_byte = mmu.read_8(self.registers.pc);
        let entry = decode(opcode_byte);

        if debug.log_cpu {
            self.history.push(disassemble(&entry.opcode, mmu, self));
        }

        if self.execute_instruction(entry, mmu, true) {
            self.increment_pc(entry.length as u16);
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

    fn service_interrupt(&mut self, irq: Interrupt, mmu: &mut Mmu) {
        self.interrupts.ime = false;
        self.interrupts.ime_scheduled = false;
        self.interrupts.halted = false;

        let pc = self.registers.pc;
        self.push_u16(mmu, pc);
        self.registers.pc = irq.vector();
    }
}
