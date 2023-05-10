#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use eframe::egui::{ProgressBar, Color32};

/* ### MAIN ### */
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Progress Bar",
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

            let time = ui.input(|i| i.time) as f32 % 10.0 / 10.0;

            // これを付けないとアニメーションがギクシャクする
            ui.ctx().request_repaint();

            let progress_bar = ProgressBar::new(time)
                .fill(Color32::from_rgb(200, 150, 100))
                .animate(true)
                .show_percentage();

            ui.centered_and_justified(|ui| {
                ui.add(progress_bar);
            });
        });
    }
}

