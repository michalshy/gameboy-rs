pub mod registers;
pub mod renderer;

use renderer::Framebuffer;

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
    pub framebuffer: Framebuffer,
    pub complete: bool,
    pub scanline_rendered: bool,
    pub was_complete: bool,
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
            framebuffer: Framebuffer {
                pixels: [0u8; 160 * 144],
            },
            complete: false,
            scanline_rendered: false,
            was_complete: false,
        }
    }

    pub fn tick(&mut self, cycles: u32, vram: &[u8]) {
        self.was_complete = self.complete;
        self.dot_counter += cycles;

        while self.dot_counter >= 456 {
            self.dot_counter -= 456;

            self.ly = self.ly.wrapping_add(1);
            self.scanline_rendered = false;

            if self.ly == 144 {
                self.complete = true; // enter VBlank
            }

            if self.ly >= 154 {
                self.ly = 0;
                self.complete = false;
            }
        }

        if self.ly >= 144 {
            self.set_mode(1); // VBlank
            return;
        }

        match self.dot_counter {
            0..=79 => {
                self.set_mode(2); // OAM
            }
            80..=251 => {
                self.set_mode(3); // Drawing
                if !self.scanline_rendered {
                    self.render_scanline(vram);
                    self.scanline_rendered = true;
                }
            }
            _ => {
                self.set_mode(0); // HBlank
            }
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => 0xFF,
        }
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = (self.stat & 0x07) | (value & 0xF8), // keep bits 0-2
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

    pub fn render_scanline(&mut self, vram: &[u8]) {
        // BG disabled
        if self.lcdc & 0x01 == 0 {
            return;
        }

        let y = self.ly as usize;

        // Select BG tile map
        let tile_map_base = if self.lcdc & 0x08 != 0 {
            0x9C00
        } else {
            0x9800
        };

        for x in 0..160 {
            let scrolled_x = x + self.scx as usize;
            let scrolled_y = y.wrapping_add(self.scy as usize);

            let tile_x = scrolled_x / 8;
            let tile_y = scrolled_y / 8;

            let tile_map_index = tile_y * 32 + tile_x;
            let tile_index_addr = tile_map_base + tile_map_index as u16;

            let tile_index = vram[(tile_index_addr - 0x8000) as usize];

            // Handle signed tile indices (0x8800 mode)
            let tile_addr = if self.lcdc & 0x10 != 0 {
                0x8000u16 + (tile_index as u16) * 16
            } else {
                let signed = tile_index as i8 as i32;
                (0x9000i32 + signed * 16) as u16
            };

            let row = scrolled_y % 8;
            let lo = vram[(tile_addr + row as u16 * 2 - 0x8000) as usize];
            let hi = vram[(tile_addr + row as u16 * 2 + 1 - 0x8000) as usize];

            let bit = 7 - (scrolled_x % 8);
            let color = (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1);

            let shade = (self.bgp >> (color * 2)) & 0b11;
            self.framebuffer.pixels[y * 160 + x] = shade;
        }
    }

    fn set_mode(&mut self, mode: u8) {
        self.stat = (self.stat & !0x03) | (mode & 0x03);
    }
}
