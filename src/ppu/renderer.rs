pub struct Framebuffer {
    pub pixels: [u8; 160 * 144],
}

impl Framebuffer {
    pub fn set(&mut self, x: usize, y: usize, v: u8) {
        self.pixels[y * 160 + x] = v;
    }
}
