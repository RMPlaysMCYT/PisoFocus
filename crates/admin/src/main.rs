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
                if ui.button("Dashboard").clicked() {
                    // Handle Dashboard button click
                }
                if ui.button("Analytics").clicked() {
                    // Handle Analytics button click
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to PisoFocus Admin!");
        });
    }
}
