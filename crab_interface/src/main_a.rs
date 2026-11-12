use eframe::{egui::Vec2b, wgpu::VertexFormat::Float64};
// use egui_extras::{TableBuilder, Column};
use egui_plot;
// use egui_graph::{Graph};
use egui_graphs::{DefaultGraphView, Graph};
use queue::Queue;

// log format
// timestamp, status, source (name of program), log body, host ip

// figure out how to set up rust-ethernet-ip for direct hmi to gray plc communication

struct GraphFrame{
    name: String,
    xVals: Queue<f64>,
    yVals: Queue<f64>,
}

impl GraphFrame{
    fn new(name: &str) -> Self{
        Self{
            name: name.to_owned(),
            xVals: Queue::new(),
            yVals: Queue::new(),
        }
    }
    
    fn write_new(&mut self, x: f64, y: f64){
        self.xVals.dequeue();
        self.yVals.dequeue();

        self.xVals.queue(x);
        self.yVals.queue(y);
    }
}

impl Default for GraphFrame{
    fn default() -> Self {
        Self { 
            name: "New Graph".to_owned(), 
            xVals: Queue::new(), 
            yVals: Queue::new(), 
        }
    }
}


impl egui::Widget for GraphFrame{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response{
        
        
        // let graph = Graph::new();
        // graph.show(ui, &mut graph {
        //     ui.horizontal(|ui| {
        //         ui.label(&self.name);
        //     });
        // }).response
    }
}

// #[derive(Default)]
struct StatusFrame{
    frame: egui::Frame,
    name: String,
    state: bool,
    counter: u64,
}

impl StatusFrame{
    fn new(name: &str, state: bool) -> Self{
        Self{
            frame: egui::Frame::new()
                .inner_margin(20)
                .outer_margin(20)
                .corner_radius(10)
                .fill(egui::Color32::from_rgba_unmultiplied(0, 157, 84, 255))
                .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)),
            name: name.to_owned(),
            state,
            counter: 0,
        }
    }
    
    fn set_state(&mut self, state: bool){
        self.state = state;
    }

    fn toggle_state(&mut self){
        self.state = !self.state;
    }
}

impl Default for StatusFrame{
    fn default() -> Self {
        Self {
            frame: egui::Frame::new()
                .inner_margin(20)
                .outer_margin(20)
                .corner_radius(10)
                .fill(egui::Color32::from_rgba_unmultiplied(0, 157, 84, 255))
                .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE)),
            name: "StatusFrame".to_owned(),
            state: true,
            counter: 0
        }
    }
}

impl egui::Widget for StatusFrame{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response{
        // self.counter += 1;

        ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

        let fill_color = if self.state {
            egui::Color32::from_rgb(30, 150, 30)
        } else {
            egui::Color32::from_rgb(150, 30, 30)
        };

        // egui::frame::new(20, 20, 10,)

        let frame = egui::Frame::new()
            .inner_margin(20)
            .outer_margin(20)
            .corner_radius(10)
            .fill(fill_color)
            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.name);
                ui.separator();

                ui.label(if self.state { "ONLINE" } else { "OFFLINE" });
            });
        }).response

        // self.frame.show(ui, |ui| {
        //     ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

        //     ui.horizontal(|ui| {
        //         ui.label(&self.name);
        //         ui.separator();

                

        //         if self.state {
        //             ui.label("ONLINE");
        //         } else {
        //             ui.label("OFFLINE");
        //         }
        //     });
        // }).response
    }
}

struct CrabGUI{
    name: String,
    hatState: bool,
    nasState: bool,
    hmiState: bool,
}

impl Default for CrabGUI{
    fn default() -> Self{
        Self{
            name:"Containerized-Robotic-Automation-Build".to_owned(),
            hatState:false,
            nasState:false,
            hmiState:false,
        }
    }
}

impl eframe::App for CrabGUI{
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("CRAB: Containerized Robotic Automation Build");
            ui.add(StatusFrame::default());
            ui.add(StatusFrame::default());
            ui.add(StatusFrame::default());
            ui.add(StatusFrame::new("Test", false));
        });
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