mod app;
mod apu;
mod cpu;
mod debug;
mod emulator;
mod interrupt_controller;
mod joypad;
mod mmu;
mod ppu;
mod serial;
mod timer;
mod utils;

fn main() {
    app::run();
}
