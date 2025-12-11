pub mod memory;
pub mod mbc;
pub mod cartridge;
use std::io::Error;

use crate::{apu::Apu, interrupts::InterruptController, joypad::Joypad, ppu::Ppu, serial::SerialPort, timer::Timer};
use memory::Memory;
use cartridge::Cartridge;

pub struct Mmu {
    pub memory: Memory,
    pub cartridge: Option<Cartridge>,

    pub timer: Timer,
    pub ppu: Ppu,
    pub joypad: Joypad,
    pub interrupts: InterruptController,
    pub serial: SerialPort,
    pub apu: Apu
}

impl Mmu {
    pub fn new(
        memory: Memory,
        cartridge: Option<Cartridge>,
        timer: Timer,
        ppu: Ppu,
        joypad: Joypad,
        interrupts: InterruptController,
        serial: SerialPort,
        apu: Apu,
    ) -> Self {
        Self {
            memory,
            cartridge,
            timer,
            ppu,
            joypad,
            interrupts,
            serial,
            apu,
        }
    }
    pub fn read_8(&mut self, addr: u16) -> u8 {
        match addr {
            0x000..=0x7FFF => self.read_rom(addr),
            0x8000..=0x9FFF  => self.read_vram(addr),
            0xA000..=0xBFFF => self.read_cartridge_ram(addr),
            0xC000..=0xDFFF => self.read_wram(addr),
            0xE000..=0xFDFF => self.read_echo(addr),
            0xFE00..=0xFE9F => self.read_oam(addr),
            0xFF00 => self.joypad.read_reg(),
            0xFF04..=0xFF07 => self.timer.read_reg(addr),
            0xFF40..=0xFF4B => self.ppu.read_reg(addr),
            0xFF80..=0xFFFE => self.read_hram(addr),
            0xFFFF => self.interrupts.ie,
            _ => { panic!("Memory access violation!"); }
        }
    }
    pub fn write_8(&mut self, addr: u16, value: u8) {
        match addr {
            0x000..=0x7FFF => self.write_rom(addr, value),
            0x8000..=0x9FFF  => self.write_vram(addr, value),
            0xA000..=0xBFFF => self.write_cartridge_ram(addr, value),
            0xC000..=0xDFFF => self.write_wram(addr, value),
            0xE000..=0xFDFF => self.write_echo(addr, value),
            0xFE00..=0xFE9F => self.write_oam(addr, value),
            0xFF00 => self.joypad.write_reg(value),
            0xFF04..=0xFF07 => self.timer.write_reg(addr, value),
            0xFF40..=0xFF4B => self.ppu.write_reg(addr, value),
            0xFF80..=0xFFFE => self.write_hram(addr, value),
            0xFFFF => self.interrupts.ie = value,
            _ => { panic!("Memory access violation!"); }
        }
    }

    pub fn read_16(&mut self, addr: u16) -> u16 {
        let lo = self.read_8(addr) as u16;
        let hi = self.read_8(addr.wrapping_add(1)) as u16;
        hi << 8 | lo
    }

    pub fn write_16(&mut self, addr: u16, value: u16) {
        let lo = (value & 0x00FF) as u8;
        let hi = (value >> 8) as u8;
        self.write_8(addr, lo);
        self.write_8(addr.wrapping_add(1), hi);
    }

    pub fn tick(&mut self, cycles: u32) {
        // TODO:
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(),Error> {
        let cartridge = Cartridge::new(path)?;
        self.cartridge = Some(cartridge);
        Ok(())
    }

    // Helper functions
    fn read_rom(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_rom(&mut self, addr: u16, value: u8) {
    }

    fn read_vram(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_vram(&mut self, addr: u16, value: u8) {
    }

    fn read_cartridge_ram(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_cartridge_ram(&mut self, addr: u16, value: u8) {
    }

    fn read_wram(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_wram(&mut self, addr: u16, value: u8) {
    }

    fn read_echo(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_echo(&mut self, addr: u16, value: u8) {
    }

    fn read_oam(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_oam(&mut self, addr: u16, value: u8) {
    }

    fn read_hram(&mut self, addr: u16) -> u8 {
        0
    }

    fn write_hram(&mut self, addr: u16, value: u8) {
    }
}