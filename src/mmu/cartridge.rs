use std::io::Error;
use crate::mmu::mbc::{Mbc, Mbc1, Mbc2, Mbc3, Mbc5, Mbcs, NoMbc};

pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub mbc: Box<dyn Mbc>
}

fn detect_mbc(rom: &Vec<u8>) -> Result<Mbcs, Error> {
    match rom[0x147] {
        0x00 => Ok(Mbcs::NoMbc),
        0x01..=0x03 => Ok(Mbcs::Mbc1),
        0x05..=0x06 => Ok(Mbcs::Mbc2),
        0x0F..=0x13 => Ok(Mbcs::Mbc3),
        0x19..=0x1E => Ok(Mbcs::Mbc5),
        _ => Err(Error::new(std::io::ErrorKind::InvalidData, "Mbc type invalid!"))
    }
}

fn detect_ram_size(rom: &Vec<u8>) -> Result<usize, Error> {
    match rom[0x149] {
        0x00 => Ok(0),
        0x01 => Ok(2),
        0x02 => Ok(8),
        0x03 => Ok(32),
        0x04 => Ok(128),
        0x05 => Ok(64),
        _ => Err(Error::new(std::io::ErrorKind::InvalidData, "RAM size invalid!"))
    }
}

impl Cartridge {
    pub fn new(path: &str) -> Result<Cartridge, Error> {
        let rom = std::fs::read(path)?;
        if rom.len() < 0x150 {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "ROM size invalid!"));
        }

        let mbc: Box<dyn Mbc> = match detect_mbc(&rom)? {
            Mbcs::NoMbc => Box::new(NoMbc::new()),
            Mbcs::Mbc1 => Box::new(Mbc1::new()),
            Mbcs::Mbc2 => Box::new(Mbc2::new()),
            Mbcs::Mbc3 => Box::new(Mbc3::new()),
            Mbcs::Mbc5 => Box::new(Mbc5::new()),
        };
        let ram = vec![0u8; detect_ram_size(&rom)?];
        
       Ok(Self { rom, ram, mbc })
    }

    
}