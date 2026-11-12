use serde::{Serialize, Deserialize};
use reqwest;

#[derive(Deserialize)]
pub struct LogQuery{
    limit: Option<usize>,
    start: u64,
}

#[derive(Debug, Deserialize)]
pub struct LogQueryResponse{
    pub logs: Vec<Log>,
}

#[derive(Debug, Deserialize)]
pub struct Log{
    pub log: String,
}

pub fn request_logs(limit: usize, start: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {    
    let response = reqwest::blocking::Client::new()
        .get("http://127.0.0.1:8000/logs")
        .query(&[
            ("limit", limit),
            ("start", start),
        ])
        .send()?
        .error_for_status()?;
    let logs: Vec<String> = response.json()?;

    Ok(logs)
}