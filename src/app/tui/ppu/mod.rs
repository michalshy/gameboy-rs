use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use std::io::Stdout;

use crate::app::tui::View;
use crate::emulator::Emulator;

pub struct PpuView;

impl PpuView {
    pub fn new() -> Self {
        Self
    }
}

fn gb_color(pix: u8) -> Color {
    match pix & 0b11 {
        0 => Color::Rgb(224, 248, 208), // white
        1 => Color::Rgb(136, 192, 112), // light gray
        2 => Color::Rgb(52, 104, 86),   // dark gray
        _ => Color::Rgb(8, 24, 32),     // black
    }
}

impl View for PpuView {
    fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, emulator: &Emulator) {
        terminal
            .draw(|frame| {
                let area = frame.size();

                let block = Block::default().title(" PPU ").borders(Borders::ALL);

                let inner = block.inner(area);
                frame.render_widget(block, area);

                let fb = &emulator.mmu.ppu.framebuffer.pixels;

                let max_x = inner.width.min(160);
                let max_y = inner.height.min(72);

                let buf = frame.buffer_mut();

                for ty in 0..max_y {
                    let top_y = ty * 2;
                    let bot_y = top_y + 1;

                    for x in 0..max_x {
                        let idx_top = top_y * 160 + x;
                        let idx_bot = bot_y * 160 + x;

                        let top = fb[idx_top as usize];
                        let bottom = fb[idx_bot as usize];

                        let ch = if top == bottom { "█" } else { "▀" };
                        let fg = gb_color(top);
                        let bg = gb_color(bottom);

                        let px = inner.x + x as u16;
                        let py = inner.y + ty as u16;

                        buf.get_mut(px, py)
                            .set_symbol(ch)
                            .set_style(Style::default().fg(fg).bg(bg));
                    }
                }
            })
            .unwrap();
    }

    fn handle_key(&mut self, _key: crossterm::event::KeyEvent, _emu: &mut Emulator) -> bool {
        false
    }
}
