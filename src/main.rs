mod app;
mod camera;
mod element;
mod fps;
mod ui;
mod world;

use app::SandboxApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([880.0, 660.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Powder Sandbox",
        options,
        Box::new(|_cc| Ok(Box::new(SandboxApp::new()))),
    )
}