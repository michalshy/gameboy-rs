pub mod registers;
pub mod renderer;

use registers::PpuRegisters;
use renderer::Framebuffer;

pub struct Ppu {
    dot_counter: u32,
    pub ly: u8,
    pub registers: PpuRegisters,
    pub framebuffer: Framebuffer,
    frame_complete: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            dot_counter: 0,
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
        self.dot_counter += cycles;

        while self.dot_counter >= 456 {
            self.dot_counter -= 456;

            // Visible scanlines
            if self.ly < 144 {
                self.render_scanline(vram);
                self.render_sprites_scanline(vram, oam);
            }

            self.ly += 1;

            // End of frame
            if self.ly == 154 {
                self.ly = 0;
                self.frame_complete = true;
            }
        }
    }

    pub fn frame_ready(&self) -> bool {
        self.frame_complete
    }

    fn render_scanline(&mut self, vram: &[u8]) {
        if self.registers.lcdc & 0x01 == 0 {
            return;
        }

        let y = self.ly as usize;

        let tile_map_base = if self.registers.lcdc & 0x08 != 0 {
            0x9C00
        } else {
            0x9800
        };

        for x in 0usize..160 {
            let sx = x.wrapping_add(self.registers.scx as usize);
            let sy = y.wrapping_add(self.registers.scy as usize);

            let tile_x = (sx / 8) & 31;
            let tile_y = (sy / 8) & 31;

            let map_index = tile_y * 32 + tile_x;
            let tile_id = vram[(tile_map_base + map_index as u16 - 0x8000) as usize];

            let tile_addr = if self.registers.lcdc & 0x10 != 0 {
                0x8000 + tile_id as u16 * 16
            } else {
                (0x9000i32 + (tile_id as i8 as i32 * 16)) as u16
            };

            let row = (sy & 7) as u16;
            let lo = vram[(tile_addr + row * 2 - 0x8000) as usize];
            let hi = vram[(tile_addr + row * 2 + 1 - 0x8000) as usize];

            let bit = 7 - (sx & 7);
            let color = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);
            let shade = (self.registers.bgp >> (color * 2)) & 0x03;

            self.framebuffer.pixels[y * 160 + x] = shade;
        }
    }

    fn render_sprites_scanline(&mut self, vram: &[u8], oam: &[u8]) {
        if self.registers.lcdc & 0x02 == 0 {
            return;
        }

        let sprite_height = if self.registers.lcdc & 0x04 != 0 {
            16
        } else {
            8
        };
        let y = self.ly as i16;
        let mut drawn = 0;

        for i in 0..40 {
            if drawn == 10 {
                break;
            }

            let base = i * 4;
            let sy = oam[base] as i16 - 16;
            let sx = oam[base + 1] as i16 - 8;

            if y < sy || y >= sy + sprite_height {
                continue;
            }

            drawn += 1;
            let tile = oam[base + 2];
            let attr = oam[base + 3];

            let mut row = (y - sy) as u8;
            if attr & 0x40 != 0 {
                row = sprite_height as u8 - 1 - row;
            }

            let tile = if sprite_height == 16 {
                tile & 0xFE
            } else {
                tile
            };
            let tile_addr = 0x8000 + tile as u16 * 16;
            let row = row & 7;

            let lo = vram[(tile_addr + row as u16 * 2 - 0x8000) as usize];
            let hi = vram[(tile_addr + row as u16 * 2 + 1 - 0x8000) as usize];

            let palette = if attr & 0x10 != 0 {
                self.registers.obp1
            } else {
                self.registers.obp0
            };

            for px in 0..8 {
                let bit = if attr & 0x20 != 0 { px } else { 7 - px };
                let color = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);
                if color == 0 {
                    continue;
                }

                let x = sx + px as i16;
                if x < 0 || x >= 160 {
                    continue;
                }

                let shade = (palette >> (color * 2)) & 0x03;
                self.framebuffer.pixels[y as usize * 160 + x as usize] = shade;
            }
        }
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
            0xFF40 => self.registers.lcdc = value,

            // bits 0–2 are read-only (mode + coincidence)
            0xFF41 => self.registers.stat = (self.registers.stat & 0x07) | (value & 0xF8),

            // Scroll
            0xFF42 => self.registers.scy = value,
            0xFF43 => self.registers.scx = value,

            // LY is read-only
            0xFF44 => {}

            // LYC
            0xFF45 => self.registers.lyc = value,

            // Palettes
            0xFF47 => self.registers.bgp = value,
            0xFF48 => self.registers.obp0 = value,
            0xFF49 => self.registers.obp1 = value,

            // Window
            0xFF4A => self.registers.wy = value,
            0xFF4B => self.registers.wx = value,

            _ => {}
        }
    }
}
