mod cpu;
mod mmu;
mod ppu;
mod timer;
mod interrupts;
mod joypad;
mod serial;
mod apu;
mod debug;
mod ui;

fn main() {
    ui::run();
}
