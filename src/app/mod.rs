use std::env;

pub mod command;
pub mod tui;
use crate::emulator::Emulator;
use tui::{EmulatorMode, Tui};

pub fn run() {
    let instructions_per_tick: usize = 10_000;
    let mut tui = Tui::new();
    let mut emulator = Emulator::new();

    handle_arguments(&mut emulator);

    loop {
        logic(&mut emulator, &mut tui, instructions_per_tick);

        tui.draw(&emulator);
        if !tui.poll(&mut emulator) {
            break;
        }
    }

    tui.shutdown();
}

pub fn logic(emulator: &mut Emulator, tui: &mut Tui, per_tick: usize) {
    match tui.mode() {
        EmulatorMode::Step => {
            if tui.advance {
                emulator.tick();
                tui.advance = false;
            }
        }
        EmulatorMode::Continuous => {
            for _ in 0..per_tick {
                if emulator.check_breakpoint() {
                    tui.set_mode(EmulatorMode::Step);
                    break;
                }
                emulator.tick();
            }
        }
    }
}

pub fn handle_arguments(emulator: &mut Emulator) {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        emulator.load_rom(&args[1]).unwrap();
    }
}
