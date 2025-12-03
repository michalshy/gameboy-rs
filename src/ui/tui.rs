use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

pub fn start() {
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();

    let backend = ratatui::backend::CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default()
                .title("GameBoy Emulator")
                .borders(Borders::ALL);
            frame.render_widget(block, size);
        }).unwrap();

        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode().unwrap();
}