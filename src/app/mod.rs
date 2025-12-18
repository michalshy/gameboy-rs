use std::env;

pub mod tui;
pub mod command;
use tui::{Tui, EmulatorMode};
use crate::emulator::Emulator;

const INSTRUCTIONS_PER_TICK: usize = 10_000;

pub fn run() {

    let mut tui = Tui::new();
    let mut emulator = Emulator::new();

    handle_arguments(&mut emulator);

    loop {
        logic(&mut emulator, &mut tui);
        tui.draw(&emulator);
        if !tui.poll(&mut emulator)
        {
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
        },
        EmulatorMode::Continuous => {
            for _ in 0..INSTRUCTIONS_PER_TICK {
                emulator.tick();
            }
        },
    }
}

pub fn handle_arguments(emulator: &mut Emulator) {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        emulator.load_rom(&args[1]).unwrap();
    }
}