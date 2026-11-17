use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_plot::{Plot, Line, PlotPoints};
use tokio;

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
    Status { device: String, online: bool },
    Temperature(f64),
}

// #[derive(Default)]
struct MyEguiApp {
    hmi_online: bool,
    hat_online: bool,
    nas_online: bool,
    plc_online: bool,
    temperature: f64,
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
                    if let Ok(resp) = client
                        .get("127.0.0.1")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                        
                            if let Some(hmi) = data["hmi"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "HMI".into(), online: hmi});
                            } else {
                                println!("Error: Failed to find field 'hmi' in response from 'url'");
                            }
                        }
                    }

                    if let Ok(resp) = client
                        .get("127.0.0.1")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                        
                            if let Some(rpi) = data["rpi"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "RPI".into(), online: rpi});
                            } else {
                                println!("Error: Failed to find field 'rpi' in response from 'url'");
                            }
                        }
                    }

                    if let Ok(resp) = client
                        .get("127.0.0.1")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                        
                            if let Some(nas) = data["nas"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "NAS".into(), online: nas});
                            } else {
                                println!("Error: Failed to find field 'nas' in response from 'url'");
                            }
                        }
                    }

                    if let Ok(resp) = client
                        .get("127.0.0.1")
                        .timeout(Duration::from_secs(2))
                        .send()
                        .await
                    {
                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                        
                            if let Some(plc) = data["plc"].as_bool() {
                                let _ = sender.send(PlcUpdate::Status { device: "PLC".into(), online: plc});
                            } else {
                                println!("Error: Failed to find field 'plc' in response from 'url'");
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
            temperature: 0.0,
            update_reciever: reciever,
        }
    }
}

fn status_box(ui: &mut egui::Ui, label: &str, status: bool) -> egui::Response {
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
    }).response
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        loop {
            match self.update_reciever.try_recv() {
                Ok(PlcUpdate::Status { device, online }) => match device.as_str() {
                    "HMI" => self.hmi_online = online,
                    "HAT" => self.hat_online = online,
                    "NAS" => self.nas_online = online,
                    "PLC" => self.plc_online = online,
                    _ => {}
                },
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
                    status_box(ui, "HMI", self.hmi_online);
                    status_box(ui, "HAT", self.hat_online);
                });
                ui.horizontal(|ui| {
                    status_box(ui, "NAS", self.nas_online);
                    status_box(ui, "PLC", self.plc_online);
                });
                
                Plot::new("my_plot")
                    .view_aspect(2.0)
                    .width(600.0)
                    .height(300.0)
                    .show(ui, |plot_ui| plot_ui.line(line));

                ui.label(format!("Temperature: {:.1} °C", self.temperature));
            });
        });
   }
}