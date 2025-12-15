use std::env;
use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Paragraph;
use ratatui::{
    prelude::*,
};
use crate::app::command::{Command, LoadRomCommand, ResetCommand};
use crate::app::tui::{View};
use crate::emulator::Emulator;


pub struct ShellView {
    buffer: String,
    history: Vec<String>,
}

impl View for ShellView {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        _emulator: &Emulator,
    ) {
        terminal.draw(|frame| {
            let area = frame.size();

            // Split vertically: header | history | input
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),      // header (cwd + line)
                    Constraint::Percentage(80), // history
                    Constraint::Percentage(10), // input
                ])
                .split(area);

            // -------- HEADER --------
            let cwd = env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or("<unknown>".into());

            let separator = "─".repeat(area.width as usize);
            let header_lines = vec![
                Line::from(cwd),
                Line::from(separator),
            ];

            frame.render_widget(Paragraph::new(header_lines), chunks[0]);

            // -------- HISTORY --------
            let history_lines: Vec<Line> = self.history
                .iter()
                .map(|s| Line::from(s.clone()))
                .collect();

            frame.render_widget(Paragraph::new(history_lines), chunks[1]);

            // -------- INPUT LINE --------
            let input = Paragraph::new(format!("> {}", self.buffer))
                .style(Style::default().fg(Color::Yellow));

            frame.render_widget(input, chunks[2]);
        }).unwrap();
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        emulator: &mut Emulator,
    ) -> bool {
        match key.code {
            KeyCode::Char(c) => {
                self.buffer.push(c);
                true
            }
            KeyCode::Backspace => {
                self.buffer.pop();
                true
            }
            KeyCode::Enter => {
                self.execute_command(emulator);
                true
            }
            _ => false,
        }
    }
}


impl ShellView {
    pub fn new() -> Self {
        Self { 
            buffer: String::new(), 
            history: vec![] 
        }
    }

    pub fn execute_command(&mut self, emulator: &mut Emulator) {
        let cmd = self.buffer.trim().to_string();
        self.history.push(format!("> {}", cmd));
        self.buffer.clear();

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() { return; }

        match parts[0] {
            "reset" => {
                self.history.push(ResetCommand.execute(emulator));
            }

            "load" if parts.len() == 2 => {
                let path = parts[1].to_string();
                self.history.push(LoadRomCommand{path}.execute(emulator));
            }

            _ => {
                self.history.push("Unknown command".into());
            }
        }
    }
}