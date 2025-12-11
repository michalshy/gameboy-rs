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
////////////////////////////////////////////////////////////////////////////////
pub struct NoMbc;
impl NoMbc {
    pub fn new() -> Self {
        Self{}
    }
}
impl Mbc for NoMbc {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => {
                let idx = addr as usize;
                rom.get(idx).copied().unwrap_or(0xFF)
            }
            _ => 0xFF,
        }
    }

    fn write_rom(&mut self, _: &mut [u8], _: u16, _: u8) {
        // ROM-only cartridges ignore writes in 0000..=7FFF
    }

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        if !(0xA000..=0xBFFF).contains(&addr) {
            return 0xFF;
        }
        let offset = (addr - 0xA000) as usize;
        ram.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, ram: &mut [u8], addr: u16, value: u8) {
        if !(0xA000..=0xBFFF).contains(&addr) {
            return;
        }
        let offset = (addr - 0xA000) as usize;
        if let Some(b) = ram.get_mut(offset) {
            *b = value;
        }
    }

    fn name(&self) -> &str { "NoMbc" }
}
////////////////////////////////////////////////////////////////////////////////

pub struct Mbc1 {
    ram_enabled: bool,
    rom_bank_low: u8,
    rom_bank_high: u8,
    mode: u8,
}
impl Mbc1 {
    pub fn new() -> Self {
        Self {
            ram_enabled: false,
            rom_bank_low: 1,  // Bank 1 is default
            rom_bank_high: 0,
            mode: 0,
        }
    }
    fn get_rom_bank(&self) -> u8 {
        let mut bank = match self.mode {
            0 => self.rom_bank_low | (self.rom_bank_high << 5),
            1 => self.rom_bank_low,
            _ => unreachable!(),
        };

        if bank & 0x1F == 0 {
            bank |= 1;
        }
        bank
    }
}
impl Mbc for Mbc1 {
    fn read_rom(&self, rom: &[u8], addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => rom[addr as usize],
            0x4000..=0x7FFF => {
                let bank = self.get_rom_bank();
                let offset = (bank as usize * 0x4000) + (addr as usize - 0x4000);
                rom.get(offset).copied().unwrap_or(0xFF)
            }
            _ => 0xFF,
        }
    }

    fn write_rom(&mut self, _rom: &mut [u8], addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A, // 0000–1FFF : RAM enable
            0x2000..=0x3FFF => { // 2000–3FFF : ROM bank low 5 bits
                self.rom_bank_low = value & 0x1F;
                if self.rom_bank_low == 0 {
                    self.rom_bank_low = 1;
                }
            },
            0x4000..=0x5FFF => self.rom_bank_high = value & 0x03, // 4000–5FFF : Upper ROM bits OR RAM bank, depending on mode
            0x6000..=0x7FFF => self.mode = value & 0x01, // 6000–7FFF : Mode select
            _ => {}
        }
    }

    fn read_ram(&self, ram: &[u8], addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let bank = match self.mode {
            0 => 0,
            1 => self.rom_bank_high,
            _ => 0,
        };

        let offset = (bank as usize * 0x2000) + (addr as usize - 0xA000);

        ram.get(offset).copied().unwrap_or(0xFF)
    }

    fn write_ram(&mut self, ram: &mut [u8], addr: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let bank = match self.mode {
            0 => 0,
            1 => self.rom_bank_high,
            _ => 0,
        };

        let offset = (bank as usize * 0x2000) + (addr as usize - 0xA000);

        if let Some(byte) = ram.get_mut(offset) {
            *byte = value;
        }
    }

    fn name(&self) -> &str { "Mbc1" }
}
////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////
