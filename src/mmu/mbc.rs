pub enum Mbcs {
    NoMbc,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5
}

pub trait Mbc {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8;
    fn write_rom(&mut self, rom: &mut [u8], addr: u16, value: u8);

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8;
    fn write_ram(&mut self, ram: &mut [u8], addr: u16, value: u8);

    fn name(&self) -> &str;
}

pub struct NoMbc;
impl NoMbc {
    pub fn new() -> Self {
        Self{}
    }
}
impl Mbc for NoMbc {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        // If rom is empty, return 0xFF
        rom.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        ram.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn name(&self) -> &str { "NoMbc" }
}
pub struct Mbc1;
impl Mbc1 {
    pub fn new() -> Self {
        Self{}
    }
}
impl Mbc for Mbc1 {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        // If rom is empty, return 0xFF
        rom.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        ram.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn name(&self) -> &str { "Mbc1" }
}
pub struct Mbc2;
impl Mbc2 {
    pub fn new() -> Self {
        Self{}
    }
}
impl Mbc for Mbc2 {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        // If rom is empty, return 0xFF
        rom.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        ram.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn name(&self) -> &str { "Mbc2" }
}
pub struct Mbc3;
impl Mbc3 {
    pub fn new() -> Self {
        Self{}
    }
}
impl Mbc for Mbc3 {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        // If rom is empty, return 0xFF
        rom.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        ram.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn name(&self) -> &str { "Mbc3" }
}
pub struct Mbc5;
impl Mbc5 {
    pub fn new() -> Self {
        Self{}
    }
}
impl Mbc for Mbc5 {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        // If rom is empty, return 0xFF
        rom.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        ram.get(addr as usize).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, _: &mut [u8], _: u16, _: u8) {

    }

    fn name(&self) -> &str { "Mbc5" }
}