mod ppu;
mod shell;
mod debug;
use crossterm::event::KeyEvent;
use std::io::Stdout;
use std::time::Duration;
use crossterm::execute;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
};
use crate::app::tui::debug::DebugView;
use crate::{emulator::Emulator};
use shell::ShellView;

#[derive(PartialEq)]
pub enum EmulatorMode {
    Step,
    Continuous,
}

pub trait View {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        emulator: &Emulator,
    );

    fn handle_key(
        &mut self,
        _key: KeyEvent,
        _emulator: &mut Emulator,
    ) -> bool {
        false
    }
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    views: Vec<Box<dyn View>>,
    active: usize,
    emulator_mode: EmulatorMode,
    pub advance: bool,
}

impl Tui {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Self {
            terminal,
            views: vec![
                Box::new(DebugView::new()),
                Box::new(ShellView::new()),
            ],
            active: 0,
            emulator_mode: EmulatorMode::Step,
            advance: false,
        }
    }

    pub fn draw(&mut self, emulator: &Emulator) {
        self.views[self.active].draw(&mut self.terminal, emulator);
    }

    pub fn mode(&self) -> &EmulatorMode {
        &self.emulator_mode
    }

    pub fn poll(&mut self, emulator: &mut Emulator) -> bool {
        if !event::poll(Duration::from_millis(16)).unwrap() {
            return true;
        }

        let Event::Key(key) = event::read().unwrap() else {
            return true;
        };

        if key.kind != KeyEventKind::Press {
            return true;
        }

        if self.views[self.active].handle_key(key, emulator) {
            return true;
        }

        match key.code {
            KeyCode::F(1) => {
                self.emulator_mode = match &self.emulator_mode {
                    EmulatorMode::Continuous => EmulatorMode::Step,
                    EmulatorMode::Step => EmulatorMode::Continuous,
                }
            }
            KeyCode::Tab => {
                self.active = (self.active + 1) % self.views.len();
            }
            KeyCode::Esc => {
                return false;
            }
            KeyCode::Right => if self.emulator_mode == EmulatorMode::Step { self.advance = true }
            _ => {}
        }

        true
    }

    pub fn shutdown(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    }
}

