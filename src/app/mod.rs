use std::env;

pub mod command;
pub mod tui;
use crate::emulator::Emulator;
use tui::{EmulatorMode, Tui};

struct EmulatorApp {
    tui: Tui,
    emulator: Emulator,
}

pub fn handle_arguments(emulator: &mut Emulator) {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        emulator.load_rom(&args[1]).unwrap();
    }
}

impl EmulatorApp {
    fn new() -> Self {
        let tui = Tui::new();
        let mut emulator = Emulator::new();
        handle_arguments(&mut emulator);
        Self { tui, emulator }
    }
}

impl Drop for EmulatorApp {
    fn drop(&mut self) {
        self.tui.shutdown();
    }
}

pub fn run() {
    let mut emulator = EmulatorApp::new();

    loop {
        emulator.logic();
        emulator.draw();

        if emulator.poll() {
            break;
        }
    }
}

impl EmulatorApp {
    pub fn logic(&mut self) {
        match self.tui.mode() {
            EmulatorMode::Step => {
                if self.tui.advance {
                    self.emulator.tick();
                    self.tui.advance = false;
                }
            }
            EmulatorMode::Continuous => {
                if self.emulator.check_breakpoint() {
                    self.tui.set_mode(EmulatorMode::Step);
                }
                self.emulator.tick();
            }
        }
    }

    pub fn draw(&mut self) {
        match self.tui.mode() {
            EmulatorMode::Step => {
                self.tui.draw(&self.emulator);
            }
            EmulatorMode::Continuous => {
                if self.emulator.draw_call() {
                    self.tui.draw(&self.emulator);
                }
            }
        }
    }

    pub fn poll(&mut self) -> bool {
        match self.tui.mode() {
            EmulatorMode::Step => {
                if !self.tui.poll(&mut self.emulator) {
                    return true;
                }
            }
            EmulatorMode::Continuous => {
                if self.emulator.draw_call() {
                    if !self.tui.poll(&mut self.emulator) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
