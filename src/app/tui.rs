use std::io::Stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};
use crate::{emulator::Emulator};

const MENU_ITEMS: &[&str] = &[
    "Load ROM",
    "Reset Emulator",
];

enum UiMode {
    Debug,
    Shell,
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    mode: UiMode,
    command_buffer: String,
    command_history: Vec<String>,
}

impl Tui {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Self {
            terminal,
            mode: UiMode::Debug,
            command_buffer: String::new(),
            command_history: Vec::new(),
        }
    }

    pub fn draw(&mut self, emulator: &Emulator) {
        match self.mode {
            UiMode::Debug => self.draw_debug(&emulator),
            UiMode::Shell => self.draw_shell(&emulator),
        }
    }

    pub fn poll(&mut self) -> bool {
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match self.mode {
                        UiMode::Shell => {
                            match key.code {
                                KeyCode::Esc => self.mode = UiMode::Debug,
                                KeyCode::Enter => self.execute_command(),
                                KeyCode::Backspace => { self.command_buffer.pop(); },
                                KeyCode::Char(c) => self.command_buffer.push(c),
                                _ => {}
                            }
                        }

                        UiMode::Debug => {
                            match key.code {
                                KeyCode::Char('q') => return false,
                                KeyCode::Char('`') => self.mode = UiMode::Shell,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        return true;
    }

    pub fn draw_debug(&mut self, emulator: &Emulator) {
        let cpu = &emulator.cpu;
        let mmu = &emulator.mmu;

        // CPU INFO
        let cpu_info = format!(
            "CPU\n\
            ----\n\
            PC: {:04X}\n\
            SP: {:04X}\n\
            A:  {:02X}  F: {:02X}\n\
            B:  {:02X}  C: {:02X}\n\
            D:  {:02X}  E: {:02X}\n\
            H:  {:02X}  L: {:02X}\n\
            \n",
            cpu.registers.pc,
            cpu.registers.sp,
            cpu.registers.a,
            cpu.registers.f,
            cpu.registers.b,
            cpu.registers.c,
            cpu.registers.d,
            cpu.registers.e,
            cpu.registers.h,
            cpu.registers.l,
        );

        // CARTRIDGE INFO
        let cartridge_info = if let Some(cart) = &mmu.cartridge {
            format!(
                "ROM\n\
                ----\n\
                Size: {} KB\n",
                cart.rom.len() / 1024,
            )
        } else {
            "No cartridge loaded\n".to_string()
        };

        self.terminal.draw(|frame| {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(frame.size());

            let row0 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ])
                .split(rows[0]);

            let row1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ])
                .split(rows[1]);

            frame.render_widget(Paragraph::new("Empty panel!"), row0[0]);
            frame.render_widget(Paragraph::new("Empty panel!"), row0[1]);
            frame.render_widget(Paragraph::new(cpu_info), row0[2]);
            
            frame.render_widget(Paragraph::new("Empty panel!"), row1[0]);
            frame.render_widget(Paragraph::new("Empty panel!"), row1[1]);
            frame.render_widget(Paragraph::new(cartridge_info), row1[2]);
        }).unwrap();
    }

    pub fn draw_shell(&mut self, emulator: &Emulator) {
        self.terminal.draw(|frame| {
            let area = frame.size();

            // Split into history (top) and input line (bottom)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(area);

            // HISTORY as Paragraph
            let lines: Vec<Line> = self.command_history
                .iter()
                .map(|s| Line::from(s.clone()))
                .collect();

            frame.render_widget(Paragraph::new(lines), chunks[0]);

            // INPUT LINE
            let input = Paragraph::new(format!("> {}", self.command_buffer))
                .style(Style::default().fg(Color::Yellow));

            frame.render_widget(input, chunks[1]);
        }).unwrap();
    }

    fn execute_command(&mut self) {
        let cmd = self.command_buffer.trim().to_string();
        self.command_history.push(cmd.clone());
        self.command_buffer.clear();

        match cmd.as_str() {
            "help" => self.command_history.push("Commands: help, load <file>, reset".into()),
            "reset" => self.command_history.push("Resetting emulator...".into()),
            other if other.starts_with("load ") => {
                let path = &other[5..];
                self.command_history.push(format!("Loading ROM: {}", path));
                // TODO: call emulator.load_rom(path)
            }
            _ => self.command_history.push("Unknown command".into()),
        }
    }

    pub fn shutdown(&mut self) {
        disable_raw_mode().unwrap();
    }
}