#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod gui;
mod utils;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(400., 197.)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "Autoclicker",
        native_options,
        Box::new(|_cc| Box::new(app::AutoclickerApp::new())),
    )
}
