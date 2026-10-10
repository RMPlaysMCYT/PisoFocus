use crate::app::PisoFocusAdminApp;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, _app: &mut PisoFocusAdminApp) {
    ui.heading("Analytics Page");
    ui.label("This is the analytics page.");
}