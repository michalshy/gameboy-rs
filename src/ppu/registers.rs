pub struct PpuRegisters {
    pub lcdc: u8, // FF40
    pub stat: u8, // FF41
    pub scy: u8,  // FF42
    pub scx: u8,  // FF43
    pub lyc: u8,  // FF45
    pub bgp: u8,  // FF47
    pub obp0: u8, // FF48
    pub obp1: u8, // FF49
    pub wy: u8,   // FF4A
    pub wx: u8,   // FF4B
}

impl PpuRegisters {
    pub fn new() -> Self {
        Self {
            lcdc: 0x00, // LCD on, BG on (safe default)
            stat: 0x00,
            scy: 0,
            scx: 0,
            lyc: 0,
            bgp: 0xFC, // DMG default palette
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
        }
    }
}
