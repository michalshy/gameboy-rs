pub trait Mbc {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8;
    fn write_rom(&mut self, rom: &mut [u8], addr: u16, value: u8);

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8;
    fn write_ram(&mut self, ram: &mut [u8], addr: u16, value: u8);
}

pub struct NoMbc;

impl Mbc for NoMbc {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        // If rom is empty, return 0xFF
        rom.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _: &mut [u8], _: u16, _: u8) {}

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        ram.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, _: &mut [u8], _: u16, _: u8) {}
}