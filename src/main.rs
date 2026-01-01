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

use std::panic;

fn print_panic(err: Box<dyn std::any::Any + Send>) {
    if let Some(s) = err.downcast_ref::<&str>() {
        eprintln!("panic: {}", s);
    } else if let Some(s) = err.downcast_ref::<String>() {
        eprintln!("panic: {}", s);
    } else {
        eprintln!("panic occurred (unknown type)");
    }
}

fn main() {
    let result = panic::catch_unwind(|| {
        app::run();
    });

    if let Err(err) = result {
        print_panic(err);
    }
}
