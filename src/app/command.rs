// Defines commands which are used to order behavior to system via implemented shell.

use crate::emulator::Emulator;

pub trait Command {
    fn execute(&self, emulator: &mut Emulator) -> String;
}

// LOAD
pub struct LoadRomCommand {
    pub path: String,
}
impl Command for LoadRomCommand {
    fn execute(&self, emulator: &mut Emulator) -> String {
        match emulator.load_rom(&self.path) {
            Ok(_) => format!("Loaded ROM: {}", self.path),
            Err(e) => format!("Failed to load {}: {}", self.path, e),
        }
    }
}

// RESET
pub struct ResetCommand;
impl Command for ResetCommand {
    fn execute(&self, emulator: &mut Emulator) -> String {
        let _ = emulator.reset();
        "Emulator reset.".to_string()
    }
}
