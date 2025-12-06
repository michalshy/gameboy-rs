pub mod tui;

use tui::Tui;

pub fn run() {

    let mut tui = Tui::new();

    loop {
        tui.draw();

        if !tui.poll()
        {
            break;
        }
    }

    tui.shutdown();
}