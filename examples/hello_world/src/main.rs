#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

/* ### MAIN ### */
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello World!!!",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}

/* ### APP ### */
#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Hello World");
            });
            ui.centered_and_justified(|ui| {
                ui.label("This is center");
            });
        });
    }
}
