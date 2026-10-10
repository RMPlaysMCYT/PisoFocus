mod app;
mod pages;

use app::PisoFocusAdminApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "PisoFocus Admin",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(PisoFocusAdminApp::default()))
        }),
    )
}