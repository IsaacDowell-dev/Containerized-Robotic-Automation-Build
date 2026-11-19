use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_plot::{Plot, Line, PlotPoints};
use tokio;
use std::time::Instant;

use url::Url;
use std::env;

use serde_json;

use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

mod plc_comm;
use plc_comm::{read_plc_tags};

#[tokio::main]
async fn main() {
    read_plc_tags();

    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))))
        .expect("Failed to launch eframe");
}

enum PlcUpdate {
    Status { device: String, online: bool, delay: Duration },
    Temperature(f64),
    Logs(Vec<LogEntry>),
}

#[derive(Clone)]
#[derive(serde::Deserialize)]
struct LogEntry{
    timestamp: String,
    status: String,
    source: String,
    body: String,
    host_ip: String,
}

// #[derive(Default)]
struct MyEguiApp {
    hmi_online: bool,
    hat_online: bool,
    nas_online: bool,
    plc_online: bool,

    hmi_delay: Duration,
    hat_delay: Duration,
    nas_delay: Duration,
    plc_delay: Duration,

    temperature: f64,
    logs: Vec<LogEntry>,
    max_logs: usize,
    // termpature_logs: Vec<Vec<f64>>,
    update_reciever: Receiver<PlcUpdate>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (sender, reciever) = std::sync::mpsc::channel();
        let ctx = cc.egui_ctx.clone();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let client = reqwest::Client::new();
                
                loop {
                    let now = Instant::now();

                    if let Ok(resp) = client
                        .get("http://127.0.0.1:8000/hmi/status")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {

                            println!("{data}");

                            if let Some(hmi) = data["hmi"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "HMI".into(), online: hmi, delay: now.elapsed()});
                            } else {
                                println!("Error: Failed to find field 'hmi' in response from 'url'");
                            }
                        }
                    }

                    if let Ok(resp) = client
                        .get("http://127.0.0.1:8000/hat/status")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {   
                        if let Ok(data) = resp.json::<serde_json::Value>().await {

                            println!("{data}");

                            if let Some(hat) = data["hat"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "HAT".into(), online: hat, delay: now.elapsed()});
                            } else {
                                println!("Error: Failed to find field 'hat' in response from 'url'");
                            }
                        }
                    }

                    if let Ok(resp) = client
                        .get("http://127.0.0.1:8000/nas/status")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                        
                            if let Some(nas) = data["nas"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "NAS".into(), online: nas, delay: now.elapsed()});
                            } else {
                                println!("Error: Failed to find field 'nas' in response from 'url'");
                            }
                        }
                    }

                    if let Ok(resp) = client
                        .get("http://127.0.0.1:8000/plc/status")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                        
                            if let Some(plc) = data["plc"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "PLC".into(), online: plc, delay: now.elapsed()});
                            } else {
                                println!("Error: Failed to find field 'plc' in response from 'url'");
                            }
                        }
                    }

                    if let Ok(resp) = client
                        .get("http://127.0.0.1:8000/logs")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        let status = resp.status();
                        let text = match resp.text().await{
                            Ok(t) => t,
                            Err(e) => {
                                println!("Error: Failed to read body: {e}");
                                return;
                            }
                        };

                        match serde_json::from_str::<Vec<LogEntry>>(&text) {
                            Ok(logs) => {
                                let _ = sender.send(PlcUpdate::Logs(logs));
                            }
                            Err(e) => {
                                println!("Error: failed to deserialize logs: {e}");
                                println!("Raw body was: {text}");
                            }
                        }
                    }

                    ctx.request_repaint();
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            });
        });

        Self {
            hmi_online: false,
            hat_online: false,
            nas_online: false,
            plc_online: false,
            hmi_delay: Duration::new(0, 0),
            hat_delay: Duration::new(0, 0),
            nas_delay: Duration::new(0, 0),
            plc_delay: Duration::new(0, 0),
            temperature: 0.0,
            logs: Vec::new(),
            max_logs: 500,
            update_reciever: reciever,
        }
    }
}

fn status_box(ui: &mut egui::Ui, label: &str, status: bool, delay: Duration) -> egui::Response {
    let color = if status {egui::Color32::from_rgb(30, 150, 30)}
                else      {egui::Color32::from_rgb(150, 30, 30)};
    
    egui::Frame::new()
        .fill(color)
        .corner_radius(10)
        .inner_margin(50)
        .show(ui, |ui| {
            ui.heading(
                egui::RichText::new(label)
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(40.0)
            );
            ui.heading(
                egui::RichText::new(delay.as_secs().to_string())
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(40.0)
            );
    }).response
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        loop {
            match self.update_reciever.try_recv() {
                Ok(PlcUpdate::Status { device, online, delay }) => match device.as_str() {
                    "HMI" => {self.hmi_online = online; self.hmi_delay = delay},
                    "HAT" => {self.hat_online = online; self.hat_delay = delay},
                    "NAS" => {self.nas_online = online; self.nas_delay = delay},
                    "PLC" => {self.plc_online = online; self.plc_delay = delay},
                    _ => {}
                },
                Ok(PlcUpdate::Logs(incoming)) => {
                    self.logs.extend(incoming);
                    let cap = 500;
                    if self.logs.len() > cap {
                        let drop = self.logs.len() - cap;
                        self.logs.drain(0..drop);
                    }
                }
                Ok(PlcUpdate::Temperature(t)) => self.temperature = t,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        let sin: PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();
        let line = Line::new("sin", sin);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    status_box(ui, "HMI", self.hmi_online, self.hmi_delay);
                    status_box(ui, "HAT", self.hat_online, self.hat_delay);
                });
                ui.horizontal(|ui| {
                    status_box(ui, "NAS", self.nas_online, self.nas_delay);
                    status_box(ui, "PLC", self.plc_online, self.plc_delay);
                });
            });
            Plot::new("my_plot")
                    .view_aspect(2.0)
                    .width(600.0)
                    .height(300.0)
                    .show(ui, |plot_ui| plot_ui.line(line));

            ui.label(format!("Temperature: {:.1} °C", self.temperature));
        });

        ui.separator();
        ui.heading("Logs");

        egui::ScrollArea::vertical()
            .max_height(250.0)
            .stick_to_bottom(true)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(180.0).at_least(120.0))           // Timestamp
                    .column(Column::initial(80.0).at_least(60.0))            // Status
                    .column(Column::initial(90.0).at_least(60.0))           // Source
                    .column(Column::remainder().at_least(200.0))           // Body
                    .column(Column::initial(130.0).at_least(90.0))        // Host IP
                    .header(20.0, |mut header| {
                        header.col(|ui| { ui.strong("Timestamp"); });
                        header.col(|ui| { ui.strong("Status"); });
                        header.col(|ui| { ui.strong("Source"); });
                        header.col(|ui| { ui.strong("Body"); });
                        header.col(|ui| { ui.strong("Host IP"); });
                    })
                    .body(|mut body| {
                        for entry in &self.logs {
                            body.row(18.0, |mut row| {
                                row.col(|ui| { ui.label(&entry.timestamp); });
                                row.col(|ui| { ui.label(&entry.status); });
                                row.col(|ui| { ui.label(&entry.source); });
                                row.col(|ui| { ui.label(&entry.body); });
                                row.col(|ui| { ui.label(&entry.host_ip); });
                            });
                        }
                    });
            });
   }
}