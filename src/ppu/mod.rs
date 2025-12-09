pub mod registers;
pub mod renderer;

pub struct Ppu {
    
}

impl Ppu {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        0
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {

    }
}