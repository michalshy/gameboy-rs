pub mod tui;

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
        if !tui.poll()
        {
            break;
        }
    }

    tui.shutdown();
}