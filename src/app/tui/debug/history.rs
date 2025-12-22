use crate::{app::tui::debug::Widget, emulator::Emulator};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

const HISTORY: usize = 35;

pub struct HistoryView;

impl Widget for HistoryView {
    fn draw_in(&mut self, frame: &mut Frame, area: Rect, emulator: &Emulator) {
        let cpu = &emulator.cpu;

        let outer = Block::default();

        frame.render_widget(&outer, area);

        let mut history = format!(
            "NO ({}):\n\
            NOI ({}):\n\
            ------------",
            cpu.history.len(),
            cpu.instruction_number);
        for (i, e) in cpu.history.iter().rev().take(HISTORY).rev().enumerate() {
            history.push_str(&format!("{:02}: {}\n", i, e));
        }

        frame.render_widget(Paragraph::new(history).block(Block::default()), area);
    }

    fn handle_key(&mut self, _key: crossterm::event::KeyEvent, _emulator: &mut Emulator) -> bool {
        false
    }
}

impl HistoryView {
    pub fn new() -> Self {
        Self {}
    }
}
