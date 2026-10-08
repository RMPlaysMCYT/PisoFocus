use eframe::egui;
use egui::TextStyle::Button;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "PisoFocus Admin",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    age: u32,
    counter: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Welcome to PisoFocus Admin!");
                ui.label("Main Menu");
                ui.add_space(10.0);

                let btn_size = egui::Vec2::new(120.0, 40.0);
                if ui.add_sized(btn_size, egui::Button::new("Dashboard")).clicked() {
                    // Handle Dashboard button click
                }
                if ui.add_sized(btn_size, egui::Button::new("Analytics")).clicked() {
                    // Handle Analytics button click
                }
                ui.add_space(10.0);
                if ui.add_sized(btn_size, egui::Button::new("Settings")).clicked() {
                    // Handle Settings button click
                }
                if ui.add_sized(btn_size, egui::Button::new("Quit")).clicked() {
                    // Handle Logout button click
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to PisoFocus Admin!");
        });
    }
}
