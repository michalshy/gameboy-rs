use std::io::Stdout;
use std::env;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};
use crate::{cpu::decoder::{Opcode, OpcodeEntry}, debug::disasm::{self, disassemble}, emulator::Emulator};
use super::command::{Command, LoadRomCommand, ResetCommand};
enum UiMode {
    Debug,
    Shell,
}

#[derive(PartialEq)]
enum EmulatorMode {
    Step,
    Continuous,
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    mode: UiMode,
    command_buffer: String,
    command_history: Vec<String>,
    emulator_mode: EmulatorMode,
    pub advance: bool,
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
            emulator_mode: EmulatorMode::Step,
            advance: false,
        }
    }

    pub fn draw(&mut self, emulator: &Emulator) {
        match self.mode {
            UiMode::Debug => self.draw_debug(&emulator),
            UiMode::Shell => self.draw_shell(),
        }
    }

    pub fn continuous(&self) -> bool {
        self.emulator_mode == EmulatorMode::Continuous 
    }

    pub fn poll(&mut self, emulator: &mut Emulator) -> bool {
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match self.mode {
                        UiMode::Shell => {
                            match key.code {
                                KeyCode::Esc => self.mode = UiMode::Debug,
                                KeyCode::Enter => self.execute_command(emulator),
                                KeyCode::Backspace => { self.command_buffer.pop(); },
                                KeyCode::Char(c) => self.command_buffer.push(c),
                                _ => {}
                            }
                        }

                        UiMode::Debug => {
                            match key.code {
                                KeyCode::Esc => return false,
                                KeyCode::Char('`') => self.mode = UiMode::Shell,
                                KeyCode::Right => self.advance = true,
                                KeyCode::Tab => match self.emulator_mode {
                                    EmulatorMode::Continuous => self.emulator_mode = EmulatorMode::Step,
                                    EmulatorMode::Step => self.emulator_mode = EmulatorMode::Continuous,
                                }
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
            PC: 0x{:04X}\n\
            SP: 0x{:04X}\n\
            A:  0x{:02X}  F: 0x{:02X}\n\
            B:  0x{:02X}  C: 0x{:02X}\n\
            D:  0x{:02X}  E: 0x{:02X}\n\
            H:  0x{:02X}  L: 0x{:02X}\n\
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
                "Cartridge info\n\
                ----\n\
                ROM Size: {} KB\n\
                RAM Size: {} KB\n\
                MBC Type: {}\n",
                cart.rom.len() / 1024,
                cart.ram.len() / 1024,
                cart.mbc.name()
            )
        } else {
            "No cartridge loaded\n".to_string()
        };

        let instruction_info = if let Some(_cart) = &mmu.cartridge {
            format!(
                "Current instruction info\n\
                ----\n\
                Byte: 0x{:02X}\n\
                Byte + 1: 0x{:02X}\n\
                Byte + 2: 0x{:02X}\n\
                Opcode: {}\n\
                ",
                emulator.mmu.read_8(emulator.cpu.registers.pc),
                emulator.mmu.read_8(emulator.cpu.registers.pc + 1),
                emulator.mmu.read_8(emulator.cpu.registers.pc + 2),
                disassemble(&emulator.cpu.get_current_opcode(mmu).opcode)
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
            frame.render_widget(Paragraph::new(instruction_info), row1[1]);
            frame.render_widget(Paragraph::new(cartridge_info), row1[2]);
        }).unwrap();
    }

    pub fn draw_shell(&mut self) {
        self.terminal.draw(|frame| {
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
            let history_lines: Vec<Line> = self.command_history
                .iter()
                .map(|s| Line::from(s.clone()))
                .collect();

            frame.render_widget(Paragraph::new(history_lines), chunks[1]);

            // -------- INPUT LINE --------
            let input = Paragraph::new(format!("> {}", self.command_buffer))
                .style(Style::default().fg(Color::Yellow));

            frame.render_widget(input, chunks[2]);
        }).unwrap();
    }

    fn execute_command(&mut self, emulator: &mut Emulator) {
        let cmd = self.command_buffer.trim().to_string();
        self.command_history.push(format!("> {}", cmd));
        self.command_buffer.clear();

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() { return; }

        match parts[0] {
            "reset" => {
                self.command_history.push(ResetCommand.execute(emulator));
            }

            "load" if parts.len() == 2 => {
                let path = parts[1].to_string();
                self.command_history.push(LoadRomCommand{path}.execute(emulator));
            }

            _ => {
                self.command_history.push("Unknown command".into());
            }
        }
    }

    pub fn shutdown(&mut self) {
        disable_raw_mode().unwrap();
    }
}