#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    VBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn bit(self) -> u8 {
        match self {
            Interrupt::VBlank => 1 << 0,
            Interrupt::Stat => 1 << 1,
            Interrupt::Timer => 1 << 2,
            Interrupt::Serial => 1 << 3,
            Interrupt::Joypad => 1 << 4,
        }
    }
    pub fn vector(self) -> u16 {
        match self {
            Interrupt::VBlank => 0x0040,
            Interrupt::Stat => 0x0048,
            Interrupt::Timer => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Joypad => 0x0060,
        }
    }
}

pub struct InterruptController {
    pub iflag: u8,
    pub ie: u8,
}

impl InterruptController {
    pub fn new() -> Self {
        Self { iflag: 0xE1, ie: 0 }
    }

    pub fn highest(&self) -> Option<Interrupt> {
        let pending = self.iflag & self.ie & 0x1F;
        if pending == 0 {
            return None;
        }

        if pending & 0x01 != 0 {
            Some(Interrupt::VBlank)
        } else if pending & 0x02 != 0 {
            Some(Interrupt::Stat)
        } else if pending & 0x04 != 0 {
            Some(Interrupt::Timer)
        } else if pending & 0x08 != 0 {
            Some(Interrupt::Serial)
        } else if pending & 0x10 != 0 {
            Some(Interrupt::Joypad)
        } else {
            None
        }
    }

    pub fn ack(&mut self, irq: Interrupt) {
        self.iflag = (self.iflag & !irq.bit()) | 0xE0;
    }

    pub fn request(&mut self, irq: Interrupt) {
        self.iflag |= irq.bit();
        self.iflag |= 0xE0;
    }

    pub fn pending_mask(&self) -> u8 {
        self.iflag & self.ie & 0x1F
    }
}
