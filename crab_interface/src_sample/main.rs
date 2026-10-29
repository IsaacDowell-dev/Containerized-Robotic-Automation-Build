#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

mod log_reader;

use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use log_reader::read_contents;

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    read_contents();

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<CrabInterface>::default())
        }),
    )


}

struct CrabInterface {
    name: String,
    age: u32,
}

impl Default for CrabInterface {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for CrabInterface {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let sin_points: PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();

        let line = Line::new("sin curve", sin_points);

        Plot::new("my_sin_plot")
            .view_aspect(2.0)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("CRAB");





            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}