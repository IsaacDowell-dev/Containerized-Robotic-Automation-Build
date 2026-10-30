use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Sample {
    pub x: u64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
pub struct MetricsResponse {
    pub temperature: Sample,
    pub speed: Sample,
}

pub fn get_metrics() -> Result<MetricsResponse, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(
        "http://127.0.0.1:8000/metrics"
    )?;

    let response = response.error_for_status()?;

    let metrics: MetricsResponse = response.json()?;

    Ok(metrics)
}

struct Metric{
    
}

pub fn launch(){
    
    

}

fn mount_files(){



}

fn spawn_threads(){
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000)); 

    });
}