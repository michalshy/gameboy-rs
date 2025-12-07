pub mod memory;
pub mod mbc;
pub mod cartridge;

pub struct Mmu {

}

impl Mmu {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn read_bye() -> u8 {
        0x00
    }
}