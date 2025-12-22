pub mod registers;
pub mod renderer;

pub struct Ppu {
    pub ly: u8,
    pub lyc: u8,
    dot_counter: u32,

    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wy: u8,
    pub wx: u8,
    pub bcps: u8,
    pub bcpd: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ly: 0,
            lyc: 0,
            dot_counter: 0,
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            bcps: 0,
            bcpd: 0,
        }
    }

    pub fn tick(&mut self, cycles: &u32) {
        self.dot_counter += cycles;

        if self.dot_counter >= 456 {
            self.dot_counter -= 456;
            self.ly += 1;

            if self.ly == 154 {
                self.ly = 0;
            }
        }
    }

    pub fn read_reg(&self, _addr: u16) -> u8 {
        self.ly
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => {}
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF68 => self.bcps = value,
            0xFF69 => self.bcpd = value,
            _ => {}
        }
    }
}
