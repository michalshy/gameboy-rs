pub struct Interrupts {
    pub ime: bool,
    pub ime_scheduled: bool,
    pub halted: bool,
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            ime: false,
            ime_scheduled: false,
            halted: false,
        }
    }

    pub fn set_ime(&mut self) {
        self.ime = true;
        self.ime_scheduled = false;
    }

    pub fn reset_ime(&mut self) {
        self.ime = false;
    }
}
