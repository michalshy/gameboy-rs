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

    pub fn tick(&mut self, cycles: u32, vram: &[u8], oam: &[u8]) {
        self.was_complete = self.complete;
        self.dot_counter += cycles;

        while self.dot_counter >= 456 {
            self.dot_counter -= 456;

            self.ly = self.ly.wrapping_add(1);
            self.scanline_rendered = false;

            if self.ly >= 144 && self.ly < 154 {
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
                    self.render_sprites_scanline(vram, oam);
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
        if self.lcdc & 0x01 == 0 {
            return;
        }

        let y = self.ly as usize;

        let tile_map_base = if self.lcdc & 0x08 != 0 {
            0x9C00
        } else {
            0x9800
        };

        for x in 0..160 {
            let scrolled_x = x + self.scx as usize;
            let scrolled_y = y.wrapping_add(self.scy as usize);

            let tile_x = (scrolled_x / 8) & 31;
            let tile_y = (scrolled_y / 8) & 31;

            let tile_map_index = tile_y * 32 + tile_x;
            let tile_index_addr = tile_map_base + tile_map_index as u16;

            let tile_index = vram[(tile_index_addr - 0x8000) as usize];

            let tile_addr = if self.lcdc & 0x10 != 0 {
                0x8000u16 + (tile_index as u16) * 16
            } else {
                let index = tile_index as i8 as i16;
                (0x9000i32 + ((index * 16) as i32)) as u16
            };

            let row = (scrolled_y & 7) as u16;
            let lo = vram[(tile_addr + row as u16 * 2 - 0x8000) as usize];
            let hi = vram[(tile_addr + row as u16 * 2 + 1 - 0x8000) as usize];

            let bit = 7 - (scrolled_x & 7);
            let color = (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1);

            let shade = (self.bgp >> (color * 2)) & 0b11;
            self.framebuffer.pixels[y * 160 + x] = shade;
        }
    }

    pub fn render_sprites_scanline(&mut self, vram: &[u8], oam: &[u8]) {
        if self.lcdc & 0x02 == 0 {
            return; // OBJ disabled
        }

        let sprite_height = if self.lcdc & 0x04 != 0 { 16 } else { 8 };
        let y = self.ly as i16;

        let mut sprites_drawn = 0;

        for i in 0..40 {
            if sprites_drawn >= 10 {
                break; // hardware limit
            }

            let base = i * 4;
            let oam_y = oam[base] as i16;
            let oam_x = oam[base + 1] as i16;
            let tile = oam[base + 2];
            let attr = oam[base + 3];

            let sprite_y = oam_y - 16;
            let sprite_x = oam_x - 8;

            if y < sprite_y || y >= sprite_y + sprite_height {
                continue;
            }

            sprites_drawn += 1;

            let mut row = (y - sprite_y) as u8;

            let flip_y = attr & 0x40 != 0;
            if flip_y {
                row = (sprite_height as u8 - 1) - row;
            }

            let tile_index = if sprite_height == 16 {
                tile & 0xFE
            } else {
                tile
            };

            let tile_index = tile_index + if sprite_height == 16 && row >= 8 { 1 } else { 0 };
            let row = row & 7;

            let tile_addr = 0x8000 + tile_index as u16 * 16;
            let lo = vram[(tile_addr + row as u16 * 2 - 0x8000) as usize];
            let hi = vram[(tile_addr + row as u16 * 2 + 1 - 0x8000) as usize];

            let flip_x = attr & 0x20 != 0;
            let palette = if attr & 0x10 != 0 { self.obp1 } else { self.obp0 };
            let bg_priority = attr & 0x80 != 0;

            for px in 0..8 {
                let bit = if flip_x { px } else { 7 - px };
                let color = (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1);

                if color == 0 {
                    continue; // transparent
                }

                let screen_x = sprite_x + px as i16;
                if screen_x < 0 || screen_x >= 160 {
                    continue;
                }

                let idx = y as usize * 160 + screen_x as usize;

                if bg_priority && self.framebuffer.pixels[idx] != 0 {
                    continue;
                }

                let shade = (palette >> (color * 2)) & 0b11;
                self.framebuffer.pixels[idx] = shade;
            }
        }
    }

    fn set_mode(&mut self, mode: u8) {
        self.stat = (self.stat & !0x03) | (mode & 0x03);
    }
}
