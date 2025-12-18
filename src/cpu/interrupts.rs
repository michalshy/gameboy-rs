pub struct Interrupts {
    pub ime: bool,
    pub ime_scheduled: bool,
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            ime: false,
            ime_scheduled: false,
        }
    }
}
