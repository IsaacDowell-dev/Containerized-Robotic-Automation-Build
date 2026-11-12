use eframe::egui::{Vec2b};
use egui_extras::{TableBuilder, Column};
use egui_plot;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread;

mod api_call;
use api_call::{get_metrics};

mod api;
use api::{request_logs};

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

        // let col_count = 2;

        TableBuilder::new(ui)
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .column(Column::auto().resizable(true))
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Timestamp");
                });
                header.col(|ui|{
                    ui.heading("State");
                });
                header.col(|ui| {
                    ui.heading("Source");
                });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Hello,");
                    });
                    row.col(|ui| {
                        ui.label("World!");
                    });
                    row.col(|ui| {
                        ui.label("World!");
                    });
                })
            });
    }
}

fn write_new_data(){

     match get_metrics() {
        Ok(metrics) => {
            let mut file = OpenOptions::new()
                .append(true)
                .open("temperature.txt")
                .expect("Failed to open data.txt");

            writeln!(file, "{},{}", metrics.temperature.x, metrics.temperature.y)
                .expect("Failed to write to temperature.txt");

            file = OpenOptions::new()
                .append(true)
                .open("speed.txt")
                .expect("Failed to open speed.txt");

            writeln!(file, "{},{}", metrics.speed.x, metrics.speed.y)
                .expect("Failed to write to temperature.txt");
        }
        Err(e) => {
            eprintln!("Failed to retrieve metrics: {e}");
        }
    }
}

fn clear_data() -> std::io::Result<()> {
    fs::write("temperature.txt", "")?;
    fs::write("speed.txt", "")?;
    Ok(())
}

fn main(){
    let logs = request_logs(7, 0)
        .expect("Failed to request logs");

    for log in logs{
        println!("{}", log);
    }
}

// fn main() -> eframe::Result<()> {
    // clear_data().expect("Failed to clear data");

    // let temperature_name = String::from("temperature");
    // let temperature_filename = "temperature.txt";
    // let temperature  = Metric::new(
    //     temperature_name,
    //     temperature_filename
    // ).unwrap();

    // let speed_name = String::from("speed");
    // let speed_filename = "speed.txt";
    // let speed  = Metric::new(
    //     speed_name,
    //     speed_filename
    // ).unwrap();

    // thread::spawn(move || {
    //     loop{
    //         thread::sleep(Duration::from_millis(100));
    //         write_new_data();
    //     }
    // });

    // let mut metrics: Vec<Metric> = Vec::new();
    // metrics.push(temperature);
    // metrics.push(speed);

    // let app = Graph::new(metrics)
    //     .expect("Failed to make Graph::new");

    // eframe::run_native(
    //     "Live Graph",
    //     eframe::NativeOptions::default(),
    //     Box::new(|_cc| Ok(Box::new(app))),
    // )
// }