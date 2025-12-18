pub struct SerialPort {
    pub sb: u8,
    pub sc: u8,
    pub output: String,
}

impl SerialPort {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0,
            output: String::new(),
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            _ => 0xFF,
        }
    }

    pub fn write_reg(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF01 => self.sb = value,
            0xFF02 => {
                self.sc = value;
                if value == 0x81 {
                    self.output.push(self.sb as char);
                    self.sc = 0x01;
                }
            }
            _ => (),
        }
    }
}
