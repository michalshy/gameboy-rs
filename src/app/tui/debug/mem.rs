
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Paragraph}};

use crate::{app::tui::{debug::Widget}, emulator::Emulator};

pub struct MemoryWidget {
    pub start_addr: u16,   // first visible address
    pub bytes_per_row: u16,
    pub rows: u16,
}

impl Widget for MemoryWidget {
    fn draw_in(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        emulator: &Emulator,
    ) {
        let mmu = &emulator.mmu;
        let pc = emulator.cpu.registers.pc;

        let mut lines = Vec::new();
        let mut addr = self.start_addr;

        for _ in 0..self.rows {
            let mut spans = Vec::new();

            spans.push(Span::styled(
                format!("{:04X}: ", addr),
                Style::default().fg(Color::DarkGray),
            ));

            for i in 0..self.bytes_per_row {
                let a = addr.wrapping_add(i);
                let v = mmu.read_8(a);

                spans.push(if a == pc {
                    Span::styled(
                        format!("{:02X} ", v),
                        Style::default()
                            .add_modifier(Modifier::REVERSED),
                    )
                } else {
                    Span::raw(format!("{:02X} ", v))
                });
            }

            lines.push(Line::from(spans));
            addr = addr.wrapping_add(self.bytes_per_row);
        }

        let widget = Paragraph::new(lines)
            .block(Block::default());

        frame.render_widget(widget, area);
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        _emulator: &mut Emulator,
    ) -> bool {
        match key.code {
            KeyCode::Up => self.scroll(-1),
            KeyCode::Down => self.scroll(1),
            KeyCode::PageUp => self.scroll(-16),
            KeyCode::PageDown => self.scroll(16),
            _ => false,
        }
    }
}

impl MemoryWidget {
    pub fn new() -> Self {
        Self {
            start_addr: 0x0000,
            bytes_per_row: 16,
            rows: 32,
        }
    }

    pub fn scroll(&mut self, delta: i16) -> bool {
        let new = self.start_addr as i32 + delta as i32 * self.bytes_per_row as i32;
        self.start_addr = new.clamp(0, 0xFFFF) as u16;
        true
    }
}