use crate::interrupt_controller::{Interrupt, InterruptController};

fn tima_period(tac: u8) -> Option<u32> {
    if tac & 0x04 == 0 {
        return None; // timer disabled
    }

    Some(match tac & 0x03 {
        0b00 => 1024, // 4096 Hz
        0b01 => 16,   // 262144 Hz
        0b10 => 64,   // 65536 Hz
        0b11 => 256,  // 16384 Hz
        _ => unreachable!(),
    })
}

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,

    tima_counter: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0x0000,
            tima: 0x00,
            tma: 0x00,
            tac: 0x00,
            tima_counter: 0,
        }
    }

    pub fn tick(&mut self, cycles: u32, interrupts: &mut InterruptController) {
        self.div = self.div.wrapping_add(cycles as u16);

        let Some(period) = tima_period(self.tac) else {
            return;
        };

        self.tima_counter += cycles;

        while self.tima_counter >= period {
            self.tima_counter -= period;

            if self.tima == 0xFF {
                self.tima = self.tma;
                interrupts.request(Interrupt::Timer);
            } else {
                self.tima += 1;
            }
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac | 0xF8, // upper bits read as 1
            _ => 0xFF,
        }
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF04 => {
                self.div = 0;
            }
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => {
                self.tac = value & 0x07;
            }
            _ => {}
        }
    }

    pub fn reset_div(&mut self) {
        self.div = 0;
    }
}
