use std::io::Stdout;

use crate::app::tui::View;
use crate::emulator::Emulator;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

fn pixel_to_char(v: u8) -> char {
    match v {
        0 => ' ',
        1 => '░',
        2 => '▒',
        3 => '▓',
        _ => '?',
    }
}

pub struct PpuView;

impl View for PpuView {
    fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, emulator: &Emulator) {
        let fb = &emulator.mmu.ppu.framebuffer.pixels;

        terminal
            .draw(|f| {
                let area = f.size();

                let mut lines: Vec<Line> = Vec::with_capacity(144);

                for y in 0..144 {
                    let mut line = String::with_capacity(160);

                    for x in 0..160 {
                        let pixel = fb[y * 160 + x];
                        line.push(pixel_to_char(pixel));
                    }

                    lines.push(Line::from(line));
                }

                let paragraph = Paragraph::new(lines)
                    .block(Block::default().title("PPU").borders(Borders::ALL));

                f.render_widget(paragraph, area);
            })
            .unwrap();
    }

    fn handle_key(&mut self, _key: KeyEvent, _emulator: &mut Emulator) -> bool {
        false
    }
}

impl PpuView {
    pub fn new() -> Self {
        Self {}
    }
}
