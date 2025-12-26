use std::fmt::format;

use crate::{
    app::tui::debug::Widget, cpu::registers::Flags, debug::disasm::disassemble, emulator::Emulator,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct InfoView;

impl Widget for InfoView {
    fn draw_in(&mut self, frame: &mut Frame, area: Rect, emulator: &Emulator) {
        let cpu = &emulator.cpu;
        let mmu = &emulator.mmu;

        let cpu_info = format!(
            "PC: 0x{:04X}\n\
            SP: 0x{:04X}\n\
            A:  0x{:02X}  F: 0x{:02X}\n\
            B:  0x{:02X}  C: 0x{:02X}\n\
            D:  0x{:02X}  E: 0x{:02X}\n\
            H:  0x{:02X}  L: 0x{:02X}\n\
            Z({}), N({}), H({}), C({})",
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
            cpu.registers.get_flag(Flags::Z) as u8,
            cpu.registers.get_flag(Flags::N) as u8,
            cpu.registers.get_flag(Flags::H) as u8,
            cpu.registers.get_flag(Flags::C) as u8,
        );

        let instruction_info = if let Some(cart) = &mmu.cartridge {
            format!(
                "Byte:      0x{:02X}\n\
                Byte + 1:  0x{:02X}\n\
                Byte + 2:  0x{:02X}\n\
                Opcode:    {}\n\
                ROM Size: {} KB\n\
                RAM Size: {} KB\n\
                MBC Type: {}",
                mmu.read_8(cpu.registers.pc),
                mmu.read_8(cpu.registers.pc + 1),
                mmu.read_8(cpu.registers.pc + 2),
                disassemble(&cpu.get_current_opcode(mmu).opcode, mmu, cpu),
                cart.rom.len() / 1024,
                cart.ram.len() / 1024,
                cart.mbc.name()
            )
        } else {
            "No cartridge loaded".to_string()
        };

        let ppu_info = format!(
            "Complete: {}\n",
            mmu.ppu.complete
        );

        let serial_output = &mmu.serial.output;

        let outer = Block::default();

        frame.render_widget(&outer, area);
        let inner = outer.inner(area);

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        let row0 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[0]);

        let row1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[1]);

        frame.render_widget(
            Paragraph::new(serial_output.as_str())
                .wrap(Wrap { trim: false })
                .block(Block::default().title("Serial").borders(Borders::ALL)),
            row0[0],
        );

        frame.render_widget(
            Paragraph::new(cpu_info).block(Block::default().title("CPU").borders(Borders::ALL)),
            row0[1],
        );

        frame.render_widget(
            Paragraph::new(instruction_info)
                .block(Block::default().title("Instruction").borders(Borders::ALL)),
            row1[0],
        );

        frame.render_widget(
            Paragraph::new(ppu_info)
                .block(Block::default().title("PPU").borders(Borders::ALL)),
            row1[1],
        );
    }

    fn handle_key(&mut self, _key: crossterm::event::KeyEvent, _emulator: &mut Emulator) -> bool {
        false
    }
}

impl InfoView {
    pub fn new() -> Self {
        Self {}
    }
}
