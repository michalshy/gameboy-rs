use crate::mmu::mbc::{Mbc, NoMbc};

pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub mbc: Box<dyn Mbc>
}

impl Cartridge {
    pub fn empty() -> Self {
        Self { rom: Vec::new(), ram: Vec::new(), mbc: Box::new(NoMbc), }
    }
}