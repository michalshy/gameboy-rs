use crate::cpu::{self, Cpu};
use crate::mmu::Mmu;
use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::interrupts::InterruptController;
use crate::joypad::Joypad;
use crate::serial::SerialPort;
use crate::apu::Apu;

pub struct Emulator {
    cpu: Cpu,
    mmu: Mmu,
    ppu: Ppu,
    timer: Timer,
    interrupt_controller: InterruptController,
    joypad: Joypad,
    serial: SerialPort,
    apu: Apu,
}

impl Emulator {
    pub fn new() -> Self {
        return Self { 
            cpu: Cpu::new(), 
            mmu: Mmu::new(), 
            ppu: Ppu::new(), 
            timer: Timer::new(), 
            interrupt_controller: InterruptController::new(), 
            joypad: Joypad::new(), 
            serial: SerialPort::new(), 
            apu: Apu::new() 
        }
    }

    pub fn tick(&mut self) -> u32 {
        let cycles = self.cpu.step(&self.mmu);

        // Todo: Tick other systems

        cycles
    }
}