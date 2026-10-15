use crate::app::PisoFocusAdminApp;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, _app: &mut PisoFocusAdminApp) {
    ui.heading(egui::RichText::new("Dashboard").size(20.0).color(egui::Color32::WHITE));
    ui.add_space(10.0);
    ui.label("This is the dashboard page.");
}