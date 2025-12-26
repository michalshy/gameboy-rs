use std::env;

pub mod command;
pub mod tui;
use crate::emulator::Emulator;
use tui::{EmulatorMode, Tui};
use eframe::{egui, App};

struct MyApp {
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
    }
}

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([160.0, 144.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "gameboy-rs",
        options,
        Box::new(|cc| {
            Ok(Box::<MyApp>::default())
        }),
    );

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
