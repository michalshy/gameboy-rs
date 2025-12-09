pub struct InterruptController {
    pub ie: u8,
    pub _if: u8
}

impl InterruptController {
    pub fn new() -> Self {
        Self { ie: 0, _if: 0 }
    }
}