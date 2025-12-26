use std::env;

pub mod command;
pub mod tui;
use crate::emulator::Emulator;
use tui::{EmulatorMode, Tui};

pub fn run() {
    let mut tui = Tui::new();
    let mut emulator = Emulator::new();

    handle_arguments(&mut emulator);

    loop {
        logic(&mut emulator, &mut tui);
        if draw(&mut emulator, &mut tui) {
            break;
        }
    }

    tui.shutdown();
}

pub fn logic(emulator: &mut Emulator, tui: &mut Tui) {
    match tui.mode() {
        EmulatorMode::Step => {
            if tui.advance {
                emulator.tick();
                tui.advance = false;
            }
        }
        EmulatorMode::Continuous => {
            if emulator.check_breakpoint() {
                tui.set_mode(EmulatorMode::Step);
            }
            emulator.tick();
        }
    }
}

pub fn draw(emulator: &mut Emulator, tui: &mut Tui) -> bool {
    match tui.mode() {
        EmulatorMode::Step => {
            tui.draw(&emulator);
            if !tui.poll(emulator) {
                return true
            }
        }
        EmulatorMode::Continuous => {
            if emulator.draw_call() {
                tui.draw(&emulator);
                if !tui.poll(emulator) {
                    return true
                }
            }
        }
    }
    false
}

pub fn handle_arguments(emulator: &mut Emulator) {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        emulator.load_rom(&args[1]).unwrap();
    }
}
