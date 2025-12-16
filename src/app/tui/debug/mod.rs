mod info;
mod mem;

use info::InfoView;
use mem::MemoryWidget;

use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, Terminal, layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend};

use crate::{app::tui::View, emulator::Emulator};

enum Focus {
    Info,
    Memory,
}

pub trait Widget {
    fn draw_in(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        emulator: &Emulator,
    );

    fn handle_key(
        &mut self,
        key: KeyEvent,
        _emulator: &mut Emulator,
    ) -> bool;
}


pub struct DebugView {
    info: InfoView,
    memory: MemoryWidget,
    focus: Focus,
}

impl DebugView {
    pub fn new() -> Self {
        Self {
            info: InfoView::new(),
            memory: MemoryWidget::new(),
            focus: Focus::Memory,
        }
    }
}

impl View for DebugView {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        emulator: &Emulator,
    ) {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Percentage(65),
                ])
                .split(frame.size());

            self.info.draw_in(frame, chunks[0], emulator);
            self.memory.draw_in(frame, chunks[1], emulator);
        }).unwrap();
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        emulator: &mut Emulator,
    ) -> bool {
        match key.code {
            KeyCode::Char('F') => {
                self.focus = match self.focus {
                    Focus::Info => Focus::Memory,
                    Focus::Memory => Focus::Info,
                };
                return true;
            }
            _ => {}
        }

        match self.focus {
            Focus::Info => self.info.handle_key(key, emulator),
            Focus::Memory => self.memory.handle_key(key, emulator),
        }
    }
}