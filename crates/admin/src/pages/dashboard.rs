use crate::app::PisoFocusAdminApp;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, _app: &mut PisoFocusAdminApp) {
    ui.heading("Dashboard Page");
    ui.label("This is the dashboard page.");
}