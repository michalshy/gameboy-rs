use std::io::Stdout;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>
}

impl Tui {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Self { terminal }
    }

    pub fn draw(&mut self) {
        self.terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default()
                .title("GameBoy Emulator")
                .borders(Borders::ALL);
            let text = Paragraph::new(
                "Cpu state: TODO\n"
            )
            .block(block)
            .alignment(Alignment::Left);
            frame.render_widget(text, size);
        }).unwrap();
    }

    pub fn poll(&mut self) -> bool {
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn shutdown(&mut self) {
        disable_raw_mode().unwrap();
    }
}