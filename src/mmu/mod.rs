pub mod cartridge;
pub mod mbc;
pub mod memory;
use std::io::Error;

use crate::{
    apu::Apu, interrupt_controller::InterruptController, joypad::Joypad, ppu::Ppu,
    serial::SerialPort, timer::Timer,
};
use cartridge::Cartridge;
use memory::Memory;

// Game Boy Memory Map
//
// 0000–3FFF   ROM Bank 0 (fixed)
// 4000–7FFF   ROM Bank N (switchable, via MBC)
// 8000–9FFF   VRAM (PPU)
// A000–BFFF   External RAM (Cartridge RAM)
// C000–CFFF   WRAM Bank 0
// D000–DFFF   WRAM Bank 1 (switchable on CGB)
// E000–FDFF   Echo RAM (mirror of C000–DDFF)
// FE00–FE9F   OAM (Sprite Attribute Table)
// FEA0–FEFF   Not usable
// FF00        Joypad
// FF01–FF02   Serial
// FF04–FF07   Timer
// FF0F        IF (Interrupt Flag)
// FF10–FF3F   APU
// FF40–FF4B   PPU Registers
// FF80–FFFE   HRAM
// FFFF        IE (Interrupt Enable)

pub const HIGH_RAM: u16 = 0xFF00;

pub struct Mmu {
    pub memory: Memory,
    pub cartridge: Option<Cartridge>,

    pub timer: Timer,
    pub ppu: Ppu,
    pub joypad: Joypad,
    pub interrupts: InterruptController,
    pub serial: SerialPort,
    pub apu: Apu,
}

#[allow(clippy::too_many_arguments)] // TODO: reconsider
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

    pub fn read_8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.read_rom(addr),
            0x8000..=0x9FFF => self.read_vram(addr),
            0xA000..=0xBFFF => self.read_cartridge_ram(addr),
            0xC000..=0xDFFF => self.read_wram(addr),
            0xE000..=0xFDFF => self.read_echo(addr),
            0xFE00..=0xFE9F => self.read_oam(addr),
            0xFF00 => self.joypad.read_reg(),
            0xFF01..=0xFF02 => self.serial.read_reg(addr),
            0xFF04..=0xFF07 => self.timer.read_reg(addr),
            0xFF0F => self.interrupts.iflag,
            0xFF10..=0xFF3F => self.apu.read_reg(addr),
            0xFF40..=0xFF4B => self.ppu.read_reg(addr),
            0xFF80..=0xFFFE => self.read_hram(addr),
            0xFFFF => self.interrupts.ie,
            _ => 0xFF,
        }
    }

    pub fn write_8(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => self.write_rom(addr, value),
            0x8000..=0x9FFF => self.write_vram(addr, value),
            0xA000..=0xBFFF => self.write_cartridge_ram(addr, value),
            0xC000..=0xDFFF => self.write_wram(addr, value),
            0xE000..=0xFDFF => self.write_echo(addr, value),
            0xFE00..=0xFE9F => self.write_oam(addr, value),
            0xFEA0..=0xFEFF => {}
            0xFF00 => self.joypad.write_reg(value),
            0xFF01..=0xFF02 => self.serial.write_reg(addr, value),
            0xFF04..=0xFF07 => self.timer.write_reg(addr, value),
            0xFF0F => self.interrupts.iflag = (value & 0x1F) | 0xE0,
            0xFF10..=0xFF3F => self.apu.write_reg(addr, value),
            0xFF40..=0xFF4B => self.ppu.write_reg(addr, value),
            0xFF68..=0xFF69 => self.ppu.write_reg(addr, value),
            0xFF4F => self.ppu.write_reg(addr, value),
            0xFF80..=0xFFFE => self.write_hram(addr, value),
            0xFFFF => self.interrupts.ie = value,
            _ => {
                // stub, TODO: add logging?
            }
        }
    }

    pub fn read_16(&self, addr: u16) -> u16 {
        let lo = self.read_8(addr) as u16;
        let hi = self.read_8(addr.wrapping_add(1)) as u16;
        hi << 8 | lo
    }

    pub fn tick(&mut self, cycles: u32) {
        self.ppu.tick(cycles, &self.memory.vram);
        self.timer.tick(cycles, &mut self.interrupts);
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Error> {
        let cartridge = Cartridge::new(path)?;
        self.cartridge = Some(cartridge);
        Ok(())
    }

    // Helper functions
    fn read_rom(&self, addr: u16) -> u8 {
        let cart = self.cartridge.as_ref().expect("Cartridge not loaded");
        cart.mbc.read_rom(&cart.rom, addr)
    }

    fn write_rom(&mut self, addr: u16, value: u8) {
        let cart = self.cartridge.as_mut().expect("Cartridge not loaded");
        cart.mbc.write_rom(&mut cart.rom, addr, value);
    }

    fn read_vram(&self, addr: u16) -> u8 {
        let offset = (addr - 0x8000) as usize;
        self.memory.vram[offset]
    }

    fn write_vram(&mut self, addr: u16, value: u8) {
        let offset = (addr - 0x8000) as usize;
        self.memory.vram[offset] = value;
    }

    fn read_cartridge_ram(&self, addr: u16) -> u8 {
        let cart = self.cartridge.as_ref().expect("Cartridge not loaded");
        cart.mbc.read_ram(&cart.ram, addr)
    }

    fn write_cartridge_ram(&mut self, addr: u16, value: u8) {
        let cart = self.cartridge.as_mut().expect("Cartridge not loaded");
        cart.mbc.write_ram(&mut cart.ram, addr, value);
    }

    fn read_wram(&self, addr: u16) -> u8 {
        let offset = addr - 0xC000;
        self.memory.wram[offset as usize]
    }

    fn write_wram(&mut self, addr: u16, value: u8) {
        let offset = addr - 0xC000;
        self.memory.wram[offset as usize] = value;
    }

    fn read_echo(&self, addr: u16) -> u8 {
        self.read_wram(addr - 0x2000)
    }

    fn write_echo(&mut self, addr: u16, value: u8) {
        self.write_wram(addr - 0x2000, value)
    }

    fn read_oam(&self, addr: u16) -> u8 {
        let offset = (addr - 0xFE00) as usize;
        self.memory.oam[offset]
    }

    fn write_oam(&mut self, addr: u16, value: u8) {
        let offset = (addr - 0xFE00) as usize;
        self.memory.oam[offset] = value;
    }

    fn read_hram(&self, addr: u16) -> u8 {
        let offset = addr - 0xFF80;
        self.memory.hram[offset as usize]
    }

    fn write_hram(&mut self, addr: u16, value: u8) {
        let offset = addr - 0xFF80;
        self.memory.hram[offset as usize] = value;
    }
}
