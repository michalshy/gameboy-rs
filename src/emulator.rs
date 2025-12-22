use std::io::Error;

use crate::apu::Apu;
use crate::cpu::Cpu;
use crate::debug::Debug;
use crate::interrupts::InterruptController;
use crate::joypad::Joypad;
use crate::mmu::Mmu;
use crate::mmu::memory::Memory;
use crate::ppu::Ppu;
use crate::serial::SerialPort;
use crate::timer::Timer;

pub struct Emulator {
    pub cpu: Cpu,
    pub mmu: Mmu,
    pub debug: Debug,
}

impl Emulator {
    pub fn new() -> Self {
        let memory = Memory::new();

        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let timer = Timer::new();
        let interrupt_controller = InterruptController::new();
        let joypad = Joypad::new();
        let serial = SerialPort::new();
        let apu = Apu::new();
        let mmu = Mmu::new(
            memory,
            None,
            timer,
            ppu,
            joypad,
            interrupt_controller,
            serial,
            apu,
        );

        Self {
            cpu,
            mmu,
            debug: Debug::new(),
        }
    }

    pub fn tick(&mut self) -> u32 {
        let cycles = self.cpu.step(&mut self.mmu, &self.debug);

        self.mmu.tick(&cycles);

        cycles
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Error> {
        self.mmu.load_rom(path)
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        self.cpu.registers.reset();
        Ok(())
    }

    pub fn dump_history(&mut self, path: &String) -> Result<(), Error> {
        self.cpu.dump_history(path)
    }

    pub fn toggle_log(&mut self) {
        self.debug.log_cpu = !self.debug.log_cpu;
    }

    pub fn add_breakpoint(&mut self, address: u16) -> String {
        self.debug.add_breakpoint(address)
    }

    pub fn check_breakpoint(&self) -> bool {
        for address in &self.debug.breakpoints {
            if self.cpu.registers.pc == *address {
                return true;
            }
        }
        false
    }
}
