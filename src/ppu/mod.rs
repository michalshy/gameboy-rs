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
    mode: PpuMode,
    stat_irq_line: bool,
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
            mode: PpuMode::OamSearch,
            stat_irq_line: false,
        }
    }

    pub fn tick(&mut self, cycles: u32, vram: &[u8], oam: &[u8]) {
        for _ in 0..cycles {
            self.tick_dot(vram, oam);
        }
    }

    fn tick_dot(&mut self, vram: &[u8], oam: &[u8]) {
        self.frame_complete = false;

        // resetting
        if self.registers.lcdc & 0x80 == 0 {
            self.mode = PpuMode::HBlank;
            self.ly = 0;
        } else {
            if self.ly == 0 {}
        }

        self.line_dots = self.line_dots.wrapping_add(1);

        match self.mode {
            PpuMode::OamSearch => {
                if self.line_dots == 80 {
                    self.mode = PpuMode::PixelTransfer;
                    self.px_x = 0;
                }
            }

            PpuMode::PixelTransfer => {
                // THIS is where pixels are produced
                self.render_pixel(vram);

                if self.px_x == 160 {
                    self.mode = PpuMode::HBlank;
                }
            }

            PpuMode::HBlank => {
                if self.line_dots == 456 {
                    self.end_scanline();
                }
            }

            PpuMode::VBlank => {
                if self.line_dots == 456 {
                    self.end_scanline();
                }
            }
        }
    }

    fn end_scanline(&mut self) {
        self.line_dots = 0;

        self.ly = self.ly.wrapping_add(1);

        if self.ly == 144 {
            self.mode = PpuMode::VBlank;
            self.frame_complete = true; // ONE frame completed here
        } else if self.ly > 153 {
            self.ly = 0;
            self.mode = PpuMode::OamSearch;
        } else {
            self.mode = PpuMode::OamSearch;
        }
    }

    pub fn render_pixel(&mut self, vram: &[u8]) {
        let x = self.px_x as usize;
        let y = self.ly as usize;
        if x < 160 && y < 144 {
            let color = self.bg_pixel(self.px_x, self.ly, vram);
            self.framebuffer.pixels[y * 160 + x] = color;
        }

        self.px_x = self.px_x.wrapping_add(1);
    }

    fn bg_pixel(&self, x: u8, y: u8, vram: &[u8]) -> u8 {
        if self.registers.lcdc & 0x01 == 0 {
            return 0;
        }
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
}
