pub struct Memory {
    pub vram: [u8; 0x2000],
    pub wram: [u8; 0x2000],
    pub hram: [u8; 0x7F],
    pub oam: [u8; 0xA0],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            hram: [0; 0x7F],
            oam: [0; 0xA0],
        }
    }
}