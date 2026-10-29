use eframe::egui::{Vec2b};
use egui_plot;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread;

struct LiveGraph
{
    file: File,
    points: Vec<[f64; 2]>,
    pending: String,
    error: Option<String>
}

struct Metric
{
    name: String,
    file: File,
    points: Vec<[f64; 2]>,
    pending: String,
    error: Option<String>
}

impl Metric{
    fn new(name: String, filename: &str) -> std::io::Result<Self>{
        let path = PathBuf::from(filename);
        let mut file = File::open(path)?;

        file.seek(SeekFrom::Start(0))?;

        Ok(Self{
            name,
            file,
            points: Vec::new(),
            pending: String::new(),
            error: None 
        })
    }

    fn read_new_data(&mut self){
        let mut buffer = [0u8, 255];

        loop{
            match self.file.read(&mut buffer){
                Ok(0) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buffer[..n]);
                    self.pending.push_str(&text);

                    while let Some(pos) = self.pending.find('\n'){
                        let line: String = self.pending.drain(..=pos).collect();
                        let line = line.trim();

                        if line.is_empty() {continue;}

                        if self.points.len() > 100 {
                            self.points.remove(0);
                        }

                        let mut values = line.split(',');

                        let x = values
                            .next()
                            .and_then(|s| s.trim().parse::<f64>().ok());

                        let y = values
                            .next()
                            .and_then(|s| s.trim().parse::<f64>().ok());

                        if let (Some(x), Some(y)) = (x, y){
                            if x.is_finite() && y.is_finite() {
                                self.points.push([x, y]);
                            }
                        }
                    }
                }
                Err(e) => {
                    self.error = Some(e.to_string());
                    break;
                }
            }
        }        
    }
}

struct Graph
{
    metrics: Vec<Metric>,
}

impl Graph{
    fn new(metrics: Vec<Metric>) -> std::io::Result<Self>{
        Ok(Self{
            metrics,
        })
    }
}

impl LiveGraph{
    fn new() -> std::io::Result<Self> {
        let path = PathBuf::from("data.txt");
        let mut file = File::open(path)?;

        file.seek(SeekFrom::Start(0))?;

        Ok(Self{
            file,
            points: Vec::new(),
            pending: String::new(),
            error: None
        })
    }

    fn read_new_data(&mut self){
        let mut buffer = [0u8, 255];

        loop{
            match self.file.read(&mut buffer){
                Ok(0) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buffer[..n]);
                    self.pending.push_str(&text);

                    while let Some(pos) = self.pending.find('\n'){
                        let line: String = self.pending.drain(..=pos).collect();
                        let line = line.trim();

                        if line.is_empty() {continue;}

                        if self.points.len() > 100 {
                            self.points.remove(0);
                        }

                        let mut values = line.split(',');

                        let x = values
                            .next()
                            .and_then(|s| s.trim().parse::<f64>().ok());

                        let y = values
                            .next()
                            .and_then(|s| s.trim().parse::<f64>().ok());

                        if let (Some(x), Some(y)) = (x, y){
                            if x.is_finite() && y.is_finite() {
                                self.points.push([x, y]);
                            }
                        }
                    }
                }
                Err(e) => {
                    self.error = Some(e.to_string());
                    break;
                }
            }
        }
    }
}

impl eframe::App for Graph{
    fn logic(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame
    ){
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame){
        for metric in self.metrics.iter_mut(){
            metric.read_new_data();
        }
    
        egui_plot::Plot::new("live_plot")
            .width(800.0)
            .height(400.0)
            .allow_zoom(Vec2b::new(false, false))
            .allow_scroll(Vec2b::new(false, false))
            .default_y_bounds(45.0, 85.0)
            .show(ui, |plot_ui|{
                for metric in &self.metrics{
                    let points: egui_plot::PlotPoints<'_> =
                        metric.points.iter().copied().collect();
                
                    let line = egui_plot::Line::new(
                        &metric.name,
                        points
                    );

                    plot_ui.line(line);
                }
            });
    }
}

impl eframe::App for LiveGraph {
    fn logic(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        // self.read_new_data();

        ctx.request_repaint_after(
            std::time::Duration::from_millis(100),
        );

        // if let Some(point) = self.points.last(){
        //     let x = point[0] + 1.0;
        //     let y = rand::random_range(1..5);
        //     let data = format!("{x},{y}");

        //     let mut file = OpenOptions::new()
        //         .append(true)
        //         .open("data.txt")
        //         .expect("Failed to open data.txt");
            
        //     writeln!(file, "{data}")
        //         .expect("Failed to write to data.txt");
        // }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.read_new_data();

        ui.heading("Live File Graph");

        let points: egui_plot::PlotPoints<'_> =
            self.points.iter().copied().collect();
        let points1: egui_plot::PlotPoints<'_> =
            self.points.iter().copied().collect();

        let line = egui_plot::Line::new("Measurements", points);
        let line1 = egui_plot::Line::new("Measurements", points1);

        if ui.button("Increment").clicked() {
            
        }

        ui.horizontal(|ui| {
            let plot = egui_plot::Plot::new("live_plot")
                .width(800.0)
                .height(400.0)
                .allow_zoom(Vec2b::new(false, false))
                .allow_scroll(Vec2b::new(false, false))
                .default_y_bounds(45.0, 85.0);
            
            plot.show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        });

        ui.horizontal(|ui| {
            egui_plot::Plot::new("live_plot1")
                .show(ui, |plot_ui1| {
                    plot_ui1.line(line1);
                });
        });

        if ui.button("Increment").clicked() {
            
        }
    }
}

fn write_new_data(x: i32, y: i32) {
    // let path = PathBuf::from("data.txt");
    // let mut file = File::open(path);

    // let y = rand::random_range(1..5);
    let mut data = format!("{x},{y}");

    let mut file = OpenOptions::new()
        .append(true)
        .open("temperature.txt")
        .expect("Failed to open data.txt");

    writeln!(file, "{data}")
        .expect("Failed to write to temperature.txt");

    file = OpenOptions::new()
        .append(true)
        .open("speed.txt")
        .expect("Failed to open speed.txt");

    let z = y + rand::random_range(0..2);
    
    data = format!("{x},{z}");

    writeln!(file, "{data}")
        .expect("Failed to write to speed.txt");
}

fn clear_data() -> std::io::Result<()> {
    // fs::write("data.txt", "")?;
    fs::write("temperature.txt", "")?;
    fs::write("speed.txt", "")?;
    Ok(())
}

fn main() -> eframe::Result<()> {
    let temperature_name = String::from("temperature");
    let temperature_filename = "temperature.txt";
    let temperature  = Metric::new(
        temperature_name,
        temperature_filename
    ).unwrap();

    let speed_name = String::from("speed");
    let speed_filename = "speed.txt";
    let speed  = Metric::new(
        speed_name,
        speed_filename
    ).unwrap();

    if let Err(e) = clear_data() {
        eprintln!("Error clearing data from data.txt");
    }

    thread::spawn(move || {
        let mut x = 0;
        let mut y = 65;
        loop{
            thread::sleep(Duration::from_millis(100));
            write_new_data(x, y);
            x += 1;
            y += rand::random_range(-1..2);
            y = y.clamp(50, 80);
        }
    });

    let mut metrics: Vec<Metric> = Vec::new();
    metrics.push(temperature);
    metrics.push(speed);

    // let app = LiveGraph::new()
        // .expect("Could not open data.txt");

    let app = Graph::new(metrics)
        .expect("Failed to make Graph::new");

    eframe::run_native(
        "Live Graph",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}