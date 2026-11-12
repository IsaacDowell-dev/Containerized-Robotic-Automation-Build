use egui::Ui;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// --------------------------------------------------
// Configuration
// --------------------------------------------------

const MAX_TEMPERATURE_POINTS: usize = 100;

// --------------------------------------------------
// Temperature Graph
// --------------------------------------------------

struct GraphFrame {
    name: String,
    points: VecDeque<[f64; 2]>,
}

impl GraphFrame {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            points: VecDeque::new(),
        }
    }

    fn write_new(&mut self, x: f64, y: f64) {
        if self.points.len() >= MAX_TEMPERATURE_POINTS {
            self.points.pop_front();
        }

        self.points.push_back([x, y]);
    }

    fn show(&self, ui: &mut egui::Ui) {
        ui.heading(&self.name);

        let points: PlotPoints<'_> =
            self.points.iter().copied().collect();

        Plot::new("temperature_plot")
            .height(230.0)
            .x_axis_label("Time (seconds)")
            .y_axis_label("Temperature (°C)")
            .allow_zoom(true)
            .allow_drag(true)
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new("Temperature", points)
                        .width(2.0),
                );
            });
    }
}

// --------------------------------------------------
// Status Button
// --------------------------------------------------

struct StatusFrame {
    name: String,
    state: bool,
    counter: u64,
}

impl StatusFrame {
    fn new(name: &str, state: bool) -> Self {
        Self {
            name: name.to_owned(),
            state,
            counter: 0,
        }
    }

    fn set_state(&mut self, state: bool) {
        self.state = state;
    }

    fn toggle_state(&mut self) {
        self.state = !self.state;
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        let fill_color = if self.state {
            egui::Color32::from_rgb(30, 150, 30)
        } else {
            egui::Color32::from_rgb(150, 30, 30)
        };

        let label = if self.state {
            "ONLINE"
        } else {
            "OFFLINE"
        };

        let button = egui::Button::new(
            egui::RichText::new(
                format!("{}\n{}", self.name, label)
            )
            .color(egui::Color32::WHITE)
            .strong()
        )
        .fill(fill_color)
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::WHITE,
        ));

        if ui.add_sized([140.0, 85.0], button).clicked() {
            self.toggle_state();
        }
    }
}

// --------------------------------------------------
// Log Entries
// --------------------------------------------------

struct LogEntry {
    timestamp: String,
    status: String,
    source: String,
    body: String,
    host_ip: String,
}

impl LogEntry {
    fn new(
        timestamp: &str,
        status: &str,
        source: &str,
        body: &str,
        host_ip: &str,
    ) -> Self {
        Self {
            timestamp: timestamp.to_owned(),
            status: status.to_owned(),
            source: source.to_owned(),
            body: body.to_owned(),
            host_ip: host_ip.to_owned(),
        }
    }

    // Parses:
    // timestamp, status, source, body, host_ip
    //
    // The body may contain additional commas.
    fn from_csv(line: &str) -> Option<Self> {
        let mut parts = line.splitn(4, ',');

        let timestamp = parts.next()?.trim();
        let status = parts.next()?.trim();
        let source = parts.next()?.trim();
        let remainder = parts.next()?;

        let (body, host_ip) = remainder.rsplit_once(',')?;

        Some(Self::new(
            timestamp,
            status,
            source,
            body.trim(),
            host_ip.trim(),
        ))
    }
}

// --------------------------------------------------
// Main Application
// --------------------------------------------------

struct CrabGUI {
    name: String,

    // Status buttons
    statuses: Vec<StatusFrame>,

    // Temperature graph
    temperature: GraphFrame,
    start_time: Instant,
    last_sample: Instant,

    // Logs
    logs: VecDeque<LogEntry>,
}

impl Default for CrabGUI {
    fn default() -> Self {
        let now = Instant::now();

        let mut app = Self {
            name: "CRAB: Containerized Robotic Automation Build"
                .to_owned(),

            statuses: vec![
                StatusFrame::new("HAT", false),
                StatusFrame::new("NAS", false),
                StatusFrame::new("HMI", false),
                StatusFrame::new("PLC", false),
            ],

            temperature: GraphFrame::new("Temperature"),
            start_time: now,
            last_sample: now,

            logs: VecDeque::new(),
        };

        // Example log entries
        app.add_log(LogEntry::new(
            "2026-11-12 10:00:00",
            "INFO",
            "CRAB",
            "Application started",
            "127.0.0.1",
        ));

        app.add_log(LogEntry::new(
            "2026-11-12 10:00:01",
            "INFO",
            "PLC",
            "PLC connection initialized",
            "192.168.1.100",
        ));

        app.add_log(LogEntry::new(
            "2026-11-12 10:00:02",
            "WARN",
            "HMI",
            "Waiting for device response",
            "192.168.1.101",
        ));

        app
    }
}

impl CrabGUI {
    fn add_log(&mut self, entry: LogEntry) {
        self.logs.push_back(entry);

        // Prevent unlimited log growth.
        if self.logs.len() > 500 {
            self.logs.pop_front();
        }
    }

    fn update_temperature(&mut self) {
        let now = Instant::now();

        if now.duration_since(self.last_sample)
            >= Duration::from_secs(1)
        {
            self.last_sample = now;

            let elapsed = now
                .duration_since(self.start_time)
                .as_secs_f64();

            // Placeholder temperature data.
            // Replace this with actual sensor readings.
            let temperature =
                65.0 + (elapsed * 0.5).sin() * 5.0;

            self.temperature.write_new(
                elapsed,
                temperature,
            );
        }
    }

    fn show_header(&self, ui: &mut egui::Ui) {
        ui.heading(&self.name);
        ui.separator();
    }

    fn show_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {

            // Top left: Temperature graph
            ui.vertical(|ui| {
                self.temperature.show(ui);
            });

            ui.separator();

            // Top right: Status buttons
            ui.vertical(|ui| {
                ui.heading("System Status");

                egui::Grid::new("status_grid")
                    .num_columns(2)
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        for (index, status) in
                            self.statuses.iter_mut().enumerate()
                        {
                            status.show(ui);

                            if index % 2 == 1 {
                                ui.end_row();
                            }
                        }
                    });
            });
        });
    }

    fn show_logs(&mut self, ui: &mut egui::Ui) {
        ui.heading("System Logs");

        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(250.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {

                egui::Grid::new("log_table")
                    .striped(true)
                    .min_col_width(80.0)
                    .show(ui, |ui| {

                        // Table headers
                        ui.strong("Timestamp");
                        ui.strong("Status");
                        ui.strong("Source");
                        ui.strong("Log Body");
                        ui.strong("Host IP");

                        ui.end_row();

                        for log in &self.logs {
                            let status_color =
                                match log.status.as_str() {
                                    "ERROR" => egui::Color32::RED,
                                    "WARN" => egui::Color32::YELLOW,
                                    "INFO" => egui::Color32::GREEN,
                                    _ => egui::Color32::WHITE,
                                };

                            ui.label(&log.timestamp);

                            ui.colored_label(
                                status_color,
                                &log.status,
                            );

                            ui.label(&log.source);

                            ui.label(&log.body);

                            ui.label(&log.host_ip);

                            ui.end_row();
                        }
                    });
            });
    }
}

impl eframe::App for CrabGUI {
    fn ui(
        &mut self,
        ctx: &mut Ui,
        _frame: &mut eframe::Frame,
    ) {
        self.update_temperature();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_header(ui);

            // Top section
            self.show_dashboard(ui);

            ui.add_space(15.0);
            ui.separator();

            // Bottom section
            self.show_logs(ui);
        });

        // Keep the application refreshing.
        ctx.request_repaint_after(
            Duration::from_millis(100),
        );
    }
}

// --------------------------------------------------
// Main
// --------------------------------------------------

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("CRAB Dashboard"),

        ..Default::default()
    };

    eframe::run_native(
        "CRAB Dashboard",
        options,
        Box::new(|_cc| {
            Ok(Box::new(CrabGUI::default()))
        }),
    )
}
