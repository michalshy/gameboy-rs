pub mod disasm;
pub mod logger;

pub struct Debug {
    pub log_cpu: bool,
    pub breakpoints: Vec<u16>,
}

impl Debug {
    pub fn new() -> Self {
        Self { log_cpu: false, breakpoints: Vec::new() }
    }

    pub fn add_breakpoint(&mut self, address: u16) -> String {
        if let Some(pos) = self.breakpoints.iter().position(|&x| x == address) {
            self.breakpoints.remove(pos);
            "Removed breakpoint".to_string()
        }
        else {
            self.breakpoints.push(address);
            "Added breakpoint".to_string()
        }
    }
}