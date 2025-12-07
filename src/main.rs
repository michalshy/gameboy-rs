mod cpu;
mod mmu;
mod ppu;
mod timer;
mod interrupts;
mod joypad;
mod serial;
mod apu;
mod debug;
mod app;
mod emulator;

fn main() {
    app::run();
}
