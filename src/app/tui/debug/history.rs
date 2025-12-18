use ratatui::{Frame, layout::{Rect}, widgets::{Block, Paragraph}};
use crate::{app::tui::{debug::Widget}, emulator::Emulator};

pub struct HistoryView;

impl Widget for HistoryView {
    fn draw_in(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        emulator: &Emulator,
    ) {
        let cpu = &emulator.cpu;

        let outer = Block::default();

        frame.render_widget(&outer, area);

        let mut history = String::new();
        for e in &cpu.history {
            history = [history, e.to_owned()].join("\n");
        }

        frame.render_widget(
            Paragraph::new(history)
                .block(Block::default()),
            area,
        );
    }

    fn handle_key(
            &mut self,
            _key: crossterm::event::KeyEvent,
            _emulator: &mut Emulator,
        ) -> bool {
        false
    }
}

impl HistoryView {
    pub fn new() -> Self {
        Self {}
    }
}