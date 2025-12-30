pub mod registers;
pub mod renderer;

use registers::PpuRegisters;
use renderer::Framebuffer;

#[derive(Copy, Clone, PartialEq)]
enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamSearch = 2,
    PixelTransfer = 3,
}

pub struct Ppu {
    line_dots: u16,
    px_x: u8, // 0..159
    pub ly: u8,
    pub registers: PpuRegisters,
    pub framebuffer: Framebuffer,
    frame_complete: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            line_dots: 0,
            px_x: 0,
            ly: 0,
            registers: PpuRegisters::new(),
            framebuffer: Framebuffer {
                pixels: [0; 160 * 144],
            },
            frame_complete: false,
        }
    }

    pub fn tick(&mut self, cycles: u32, vram: &[u8], oam: &[u8]) {
        self.frame_complete = false;

        for _ in 0..cycles {}

        // Debug
        self.line_dots += cycles as u16;
        if self.line_dots > 4000 {
            self.render_frame(vram);
            self.line_dots = 0;
        }
    }

    pub fn render_frame(&mut self, vram: &[u8]) {
        // BG disabled → clear screen
        if self.registers.lcdc & 0x01 == 0 {
            self.framebuffer.pixels.fill(0);
            return;
        }

        for y in 0..144 {
            for x in 0..160 {
                let color = self.bg_pixel(x as u8, y as u8, vram);
                self.framebuffer.pixels[y * 160 + x] = color;
            }
        }

        self.frame_complete = true;
    }

    fn bg_pixel(&self, x: u8, y: u8, vram: &[u8]) -> u8 {
        // Tile coordinates
        let sx = x.wrapping_add(self.registers.scx);
        let sy = y.wrapping_add(self.registers.scy);

        let tile_x = (sx / 8) as u16;
        let tile_y = (sy / 8) as u16;
        let tile_index = tile_y * 32 + tile_x;

        // BG tile map base
        let tile_map_base = if self.registers.lcdc & 0x08 == 0 {
            0x9800
        } else {
            0x9C00
        };

        let tile_id = vram[(tile_map_base + tile_index - 0x8000) as usize];

        // Tile data base
        let tile_addr = if self.registers.lcdc & 0x10 != 0 {
            0x8000 + tile_id as u16 * 16
        } else {
            (0x9000i32 + (tile_id as i8 as i32 * 16)) as u16
        };

        let row = (sy % 8) as u16;
        let lo = vram[(tile_addr + row * 2 - 0x8000) as usize];
        let hi = vram[(tile_addr + row * 2 + 1 - 0x8000) as usize];

        let bit = 7 - (sx & 7);
        let color = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);

        // Palette lookup
        (self.registers.bgp >> (color * 2)) & 0x03
    }

    pub fn frame_ready(&self) -> bool {
        self.frame_complete
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.registers.lcdc,
            0xFF41 => self.registers.stat,
            0xFF42 => self.registers.scy,
            0xFF43 => self.registers.scx,
            0xFF44 => self.ly,
            0xFF45 => self.registers.lyc,
            0xFF47 => self.registers.bgp,
            0xFF48 => self.registers.obp0,
            0xFF49 => self.registers.obp1,
            0xFF4A => self.registers.wy,
            0xFF4B => self.registers.wx,
            _ => 0xFF,
        }
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {
        match addr {
            // on/off
            0xFF40 => {
                self.registers.lcdc = value;
            }
            0xFF41 => self.registers.stat = (self.registers.stat & 0x07) | (value & 0xF8),
            0xFF42 => self.registers.scy = value,
            0xFF43 => self.registers.scx = value,
            0xFF44 => {}
            0xFF45 => self.registers.lyc = value,
            0xFF47 => self.registers.bgp = value,
            0xFF48 => self.registers.obp0 = value,
            0xFF49 => self.registers.obp1 = value,
            0xFF4A => self.registers.wy = value,
            0xFF4B => self.registers.wx = value,

            _ => {}
        }
    }

    fn set_mode(&mut self, mode: PpuMode) {
        self.registers.stat = (self.registers.stat & !0x03) | (mode as u8);
    }
}
