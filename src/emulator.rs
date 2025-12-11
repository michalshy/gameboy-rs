use std::io::Error;

use crate::cpu::Cpu;
use crate::mmu::memory::Memory;
use crate::mmu::{Mmu};
use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::interrupts::InterruptController;
use crate::joypad::Joypad;
use crate::serial::SerialPort;
use crate::apu::Apu;

pub struct Emulator {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

impl Emulator {
    pub fn new() -> Self {
            let memory = Memory::new();

            let cpu = Cpu::new(); 
            let ppu = Ppu::new();
            let timer =Timer::new();
            let interrupt_controller = InterruptController::new(); 
            let joypad = Joypad::new();
            let serial = SerialPort::new(); 
            let apu = Apu::new();
            let mmu = Mmu::new(memory, None, timer, ppu, joypad, interrupt_controller, serial, apu);

        return Self { 
            cpu,
            mmu
        }
    }

    pub fn tick(&mut self) -> u32 {
        let cycles = self.cpu.step(&mut self.mmu);

        self.mmu.tick(cycles);

        cycles
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(),Error> {
        self.mmu.load_rom(path)
    }

    pub fn reset(&mut self) -> Result<(),Error> {
        Ok(())
    }
}