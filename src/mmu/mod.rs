pub mod memory;
pub mod mbc;
pub mod cartridge;
use crate::{apu::Apu, interrupts::InterruptController, joypad::Joypad, ppu::Ppu, serial::SerialPort, timer::Timer};
use memory::Memory;
use cartridge::Cartridge;

pub struct Mmu {
    pub memory: Memory,
    pub cartridge: Cartridge,

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
        cartridge: Cartridge,
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
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_mmu() -> Mmu {
        Mmu::new(
            Memory::new(),
            Cartridge::empty(),
            Timer::new(),
            Ppu::new(),
            Joypad::new(),
            InterruptController::new(),
            SerialPort::new(),
            Apu::new(),
        )
    }

    #[test]
    fn test_write_read_wram_8bit() {
        let mut mmu = create_test_mmu();
        mmu.write_8(0xC000, 0xAB);
        assert_eq!(mmu.read_8(0xC000), 0xAB);
    }

    #[test]
    fn test_write_read_wram_16bit() {
        let mut mmu = create_test_mmu();
        mmu.write_16(0xC000, 0x1234);
        assert_eq!(mmu.read_16(0xC000), 0x1234);
    }

    #[test]
    fn test_write_8bit_read_16bit() {
        let mut mmu = create_test_mmu();
        mmu.write_8(0xC000, 0x34);
        mmu.write_8(0xC001, 0x12);
        assert_eq!(mmu.read_16(0xC000), 0x1234);
    }

    #[test]
    fn test_write_16bit_read_8bit() {
        let mut mmu = create_test_mmu();
        mmu.write_16(0xC000, 0x5678);
        assert_eq!(mmu.read_8(0xC000), 0x78);
        assert_eq!(mmu.read_8(0xC001), 0x56);
    }

    #[test]
    fn test_write_read_hram_8bit() {
        let mut mmu = create_test_mmu();
        mmu.write_8(0xFF80, 0xCD);
        assert_eq!(mmu.read_8(0xFF80), 0xCD);
    }

    #[test]
    fn test_write_read_hram_16bit() {
        let mut mmu = create_test_mmu();
        mmu.write_16(0xFF80, 0xABCD);
        assert_eq!(mmu.read_16(0xFF80), 0xABCD);
    }

    #[test]
    fn test_interrupt_enable_register() {
        let mut mmu = create_test_mmu();
        mmu.write_8(0xFFFF, 0x1F);
        assert_eq!(mmu.read_8(0xFFFF), 0x1F);
    }

    #[test]
    fn test_multiple_writes_sequential() {
        let mut mmu = create_test_mmu();
        for i in 0..10 {
            mmu.write_8(0xC000 + i as u16, (i * 0x11) as u8);
        }
        for i in 0..10 {
            assert_eq!(mmu.read_8(0xC000 + i as u16), (i * 0x11) as u8);
        }
    }
}
