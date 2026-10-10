use eframe::egui;

pub fn stat_card(ui: &mut egui::Ui, title: &str, value: &str, subtitle: &str, accent_color: egui::Color32) {
    let width = (ui.available_width() - 10.0) / 2.0; // Two cards per row with 10px spacing

    egui::Frame::none()
        .fill(egui::Color32::from_rgb(32, 33, 36)) // Card background
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 50, 55))) // Border
        .rounding(8.0) // Corner rounding
        .inner_margin(12.0) // Padding inside the card
        .show(ui, |ui| {
            ui.set_width(width);
            ui.vertical(|ui| {
                // Title
                ui.label(
                    egui::RichText::new(title)
                        .size(13.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(4.0);

                // Main Stat Value
                ui.label(
                    egui::RichText::new(value)
                        .size(24.0)
                        .bold()
                        .color(accent_color),
                );
                ui.add_space(2.0);

                // Subtitle / Note
                ui.label(
                    egui::RichText::new(subtitle)
                        .size(11.0)
                        .color(egui::Color32::DARK_GRAY),
                );
            });
        });
}