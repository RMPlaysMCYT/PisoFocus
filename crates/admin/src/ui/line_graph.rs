use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
pub fn line_graph(ui:&mut egui::Ui, title:&str, data:Vec<(f64, f64)>, color:egui::Color32) {
    let points: PlotPoints = data.into_iter().collect();
    let line = Line::new(points).color(color);
    Plot::new(title)
        .view_aspect(2.0)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
}