use eframe::egui;

struct StatusFrame {
    name: String,
    state: bool,
    counter: u64,
}

impl StatusFrame {
    fn new(name: &str, state: bool) -> Self {
        Self { name: name.to_owned(), state, counter: 0 }
    }

    fn toggle(&mut self) {
        self.state = !self.state;
    }
}

impl Default for StatusFrame {
    fn default() -> Self {
        Self { name: "StatusFrame".to_owned(), state: false, counter: 0 }
    }
}

// A lightweight view that borrows the model.
struct StatusFrameView<'a> {
    model: &'a mut StatusFrame,
}

impl<'a> egui::Widget for StatusFrameView<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let StatusFrame { name, state, counter } = self.model;

        let fill_color = if *state {
            egui::Color32::from_rgb(30, 150, 30)
        } else {
            egui::Color32::from_rgb(150, 30, 30)
        };

        let frame = egui::Frame::new()
            .inner_margin(20)
            .outer_margin(20)
            .corner_radius(10)
            .fill(fill_color)
            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE));

        frame
            .show(ui, |ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

                ui.horizontal(|ui| {
                    ui.label(name.as_str());
                    ui.separator();

                    // Toggle button instead of plain label
                    if ui.button(if *state { "ONLINE" } else { "OFFLINE" }).clicked() {
                        *state = !*state;
                    }

                    ui.separator();
                    ui.label(format!("count: {counter}"));
                });
            })
            .response
    }
}

struct CrabGUI {
    name: String,
    hat: StatusFrame,
    nas: StatusFrame,
    hmi: StatusFrame,
    test: StatusFrame,
}

impl Default for CrabGUI {
    fn default() -> Self {
        Self {
            name: "Containerized-Robotic-Automation-Build".to_owned(),
            hat: StatusFrame::new("HAT", false),
            nas: StatusFrame::new("NAS", false),
            hmi: StatusFrame::new("HMI", false),
            test: StatusFrame::new("Test", false),
        }
    }
}

impl eframe::App for CrabGUI {
    // 0.37.0 requires ui, not update
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Counters still increment once per frame here
        self.hat.counter += 1;
        self.nas.counter += 1;
        self.hmi.counter += 1;
        self.test.counter += 1;

        // No longer need CentralPanel::default().show(ctx, |ui| ...)
        // Draw directly inside the given ui
        ui.heading("CRAB: Containerized Robotic Automation Build");
        ui.heading(&self.name);

        ui.add(StatusFrameView { model: &mut self.hat });
        ui.add(StatusFrameView { model: &mut self.nas });
        ui.add(StatusFrameView { model: &mut self.hmi });
        ui.add(StatusFrameView { model: &mut self.test });

        if ui.button("Toggle all").clicked() {
            self.hat.toggle();
            self.nas.toggle();
            self.hmi.toggle();
            self.test.toggle();
        }

        // Request a repaint to keep the counters updating
        ui.ctx().request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("CRAB Dashboard"),
        ..Default::default()
    };

    eframe::run_native(
        "CRAB Dashboard",
        options,
        Box::new(|_cc| Ok(Box::new(CrabGUI::default()))),
    )
}