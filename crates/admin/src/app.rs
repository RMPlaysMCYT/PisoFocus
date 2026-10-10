use crate::pages::{analytics, dashboard, settings};
use eframe::egui;
use egui::TextStyle::Button;

#[derive(Default, PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Page {
    Dashboard,
    Analytics,
    Settings,
}

#[derive(Default)]
pub struct PisoFocusAdminApp {
    pub current_page: Page,
    pub config: crate::AppConfig,
}

impl eframe::App for PisoFocusAdminApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Welcome to PisoFocus Admin!");
                ui.label("Main Menu");
                ui.add_space(10.0);

                let btn_size = egui::Vec2::new(120.0, 40.0);
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::selectable(self.current_page == Page::Dashboard, "Dashboard"),
                    )
                    .clicked()
                {
                    self.current_page = Page::Dashboard;
                }
                if ui
                    .add_sized(btn_size, egui::Button::selectable(self.current_page == Page::Analytics, "Analytics"))
                    .clicked()
                {
                    self.current_page = Page::Analytics;
                }
                ui.add_space(10.0);
                if ui
                    .add_sized(btn_size, egui::Button::selectable(self.current_page == Page::Settings, "Settings"))
                    .clicked()
                {
                    self.current_page = Page::Settings;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.current_page {
            Page::Dashboard => dashboard::show(ui, self),
            Page::Analytics => analytics::show(ui, self),
            Page::Settings => settings::show(ui, self), 
        });
    }
}
