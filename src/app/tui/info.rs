use std::io::Stdout;

use ratatui::{Terminal, layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, widgets::Paragraph};

use super::Tui;
use crate::{app::tui::View, debug::disasm::disassemble, emulator::Emulator};

pub struct InfoView;

impl View for InfoView {
    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        emulator: &Emulator,
    ) {
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

        let serial_output = &mmu.serial.output;

        terminal.draw(|frame| {
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
            frame.render_widget(Paragraph::new(serial_output.as_str()), row0[1]);
            frame.render_widget(Paragraph::new(cpu_info), row0[2]);
            
            frame.render_widget(Paragraph::new("Empty panel!"), row1[0]);
            frame.render_widget(Paragraph::new(instruction_info), row1[1]);
            frame.render_widget(Paragraph::new(cartridge_info), row1[2]);
        }).unwrap();
    }
}

impl InfoView {
    pub fn new() -> Self {
        Self {}
    }
}