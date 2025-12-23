pub struct InterruptController {
    pub iflag: u8,
    pub ie: u8,
}

impl InterruptController {
    pub fn new() -> Self {
        Self { iflag: 0, ie: 0 }
    }
}
