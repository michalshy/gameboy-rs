pub mod tui;
pub mod command;

use tui::Tui;
use crate::emulator::Emulator;

pub fn run() {

    let mut tui = Tui::new();
    let mut emulator = Emulator::new();

    loop {
        // logic
        emulator.tick();

        // draw
        tui.draw(&emulator);

        // events
        if !tui.poll(&mut emulator)
        {
            break;
        }
    }

    tui.shutdown();
}