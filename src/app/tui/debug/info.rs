use crate::{app::tui::debug::Widget, debug::disasm::disassemble, emulator::Emulator};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
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
            H:  0x{:02X}  L: 0x{:02X}",
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

        let cartridge_info = if let Some(cart) = &mmu.cartridge {
            format!(
                "ROM Size: {} KB\n\
                RAM Size: {} KB\n\
                MBC Type: {}",
                cart.rom.len() / 1024,
                cart.ram.len() / 1024,
                cart.mbc.name()
            )
        } else {
            "No cartridge loaded".to_string()
        };

        let instruction_info = if mmu.cartridge.is_some() {
            format!(
                "Byte:      0x{:02X}\n\
                Byte + 1:  0x{:02X}\n\
                Byte + 2:  0x{:02X}\n\
                Opcode:    {}",
                mmu.read_8(cpu.registers.pc),
                mmu.read_8(cpu.registers.pc + 1),
                mmu.read_8(cpu.registers.pc + 2),
                disassemble(&cpu.get_current_opcode(mmu).opcode),
            )
        } else {
            "No cartridge loaded".to_string()
        };

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
            Paragraph::new(cartridge_info)
                .block(Block::default().title("Cartridge").borders(Borders::ALL)),
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
