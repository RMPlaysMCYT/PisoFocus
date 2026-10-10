use crate::app::PisoAdminApp;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut PisoAdminApp) {
    ui.heading("Dashboard Page");
    ui.label("This is the dashboard page.");
}