mod info;
mod mem;
mod history;

use info::InfoView;
use mem::MemoryWidget;
use history::HistoryView;

use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, Terminal, layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, style::{Color, Modifier, Style}, widgets::{Block, Borders}};

use crate::{app::tui::{View}, emulator::Emulator};

#[derive(PartialEq)]
enum Focus {
    Info,
    Memory,
    History,
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
    history: HistoryView,
    focus: Focus,
}

impl DebugView {
    pub fn new() -> Self {
        Self {
            info: InfoView::new(),
            memory: MemoryWidget::new(),
            history: HistoryView::new(),
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
        let info_style = if self.focus == Focus::Info {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let memory_style = if self.focus == Focus::Memory {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let history_style = if self.focus == Focus::History {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let info_block = Block::default()
            .title("Info")
            .borders(Borders::ALL)
            .border_style(info_style);

        let mem_block = Block::default()
            .title("Memory")
            .borders(Borders::ALL)
            .border_style(memory_style);

        let history_block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_style(history_style);

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Percentage(50),
                    Constraint::Percentage(10),
                ])
                .split(frame.size());

            // INFO
            frame.render_widget(info_block.clone(), chunks[0]);
            let info_inner = info_block.inner(chunks[0]);
            self.info.draw_in(frame, info_inner, emulator);

            // MEMORY
            frame.render_widget(mem_block.clone(), chunks[1]);
            let mem_inner = mem_block.inner(chunks[1]);
            self.memory.draw_in(frame, mem_inner, emulator);

            // HISTORY
            frame.render_widget(history_block.clone(), chunks[2]);
            let history_inner = history_block.inner(chunks[2]);
            self.history.draw_in(frame, history_inner, emulator);
        }).unwrap();
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        emulator: &mut Emulator,
    ) -> bool {
        match key.code {
            KeyCode::Char('f') => {
                self.focus = match self.focus {
                    Focus::Info => Focus::Memory,
                    Focus::Memory => Focus::History,
                    Focus::History => Focus::Info,
                };
                return true;
            }
            _ => {}
        }

        match self.focus {
            Focus::Info => self.info.handle_key(key, emulator),
            Focus::Memory => self.memory.handle_key(key, emulator),
            Focus::History => self.history.handle_key(key, emulator),
        }
    }
}