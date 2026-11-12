use axum::{
    routing::get,
    Json,
    Router,
    extract::Query,
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering}
};
use chrono::{DateTime, Utc, format::Numeric::Timestamp};
use std::path::PathBuf;
use std::file;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader, Seek, SeekFrom};
use std::thread;

#[derive(Clone)]
struct AppState{
    counter: Arc<AtomicU64>
}

#[derive(Deserialize)]
struct LogQuery {
    limit: Option<usize>,
    start: Option<u64>,
}

fn read_logs(limit: usize, start: u64) -> io::Result<Vec<String>>{
    let mut logs: Vec<String> = Vec::new();
    if limit == 0 { return Ok(logs); }

    const BUFFER_SIZE: usize = 4096;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut file = File::open("logs/logs.txt")?;
    let mut position = file.seek(SeekFrom::End(0))?;
    let mut current_line: Vec<u8> = Vec::new();
    let mut done = false;

    while position > 0 && !done {
        let chunk_start = position.saturating_sub(BUFFER_SIZE as u64);
        let chunk_size = (position - chunk_start) as usize;

        file.seek(SeekFrom::Start(chunk_start))?;
        file.read_exact(&mut buffer[..chunk_size])?;

        for byte in buffer[..chunk_size].iter().rev() {
            if *byte == b'\n' {
                if let Some((id, line)) = parse_line(&mut current_line) {
                    if id < start {
                        done = true;
                        break;
                    }
                    logs.push(line);
                    if logs.len() >= limit {
                        done = true;
                        break;
                    }
                }
            } else {
                current_line.push(*byte);
            }
        }

        position = chunk_start;
    }

    // File doesn't start with \n
    if !done && !current_line.is_empty() && logs.len() < limit {
        if let Some((id, line)) = parse_line(&mut current_line) {
            if id >= start {
                logs.push(line);
            }
        }
    }

    logs.reverse();
    Ok(logs)
}

fn parse_line(current_line: &mut Vec<u8>) -> Option<(u64, String)> {
    if current_line.is_empty() {
        return None;
    }
    current_line.reverse();

    if current_line.last() == Some(&b'\r') {
        current_line.pop();
    }

    let line_content = String::from_utf8_lossy(current_line).into_owned();
    let line_id = line_content
        .split_once(char::is_whitespace)
        .and_then(|(first, _)| first.parse::<u64>().ok());
    current_line.clear();
    line_id.map(|id| (id, line_content))
}

async fn get_logs (
    Query(query): Query<LogQuery>,
    ) -> Result<Json<Vec<String>>, (StatusCode, String)>{

    let limit = query.limit.unwrap_or(100);
    let start = query.start.unwrap_or(0);

    let logs = tokio::task::spawn_blocking(move || {
        read_logs(limit, start)
    })
        .await
        .map_err(|e|(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        ))?
        .map_err(|e|(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        ))?;

    Ok(Json(logs))
}

#[tokio::main]
async fn main(){
    let state = AppState{
        counter: Arc::new(AtomicU64::new(0))
    };

    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .route("/logs", get(get_logs))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:8000")
        .await
        .unwrap();

    println!("fake_server running on http:127.0.0.1:8000");

    axum::serve(listener, app)
        .await
        .unwrap()
}

// Only here for testing graph readings. Need to set up parsing logs into graph data in client
// -Lawrence 10/30
#[derive(Serialize)]
struct Sample{
    x: u64,
    y: f64
}
async fn get_metrics(
    axum::extract::State(state): axum::extract::State<AppState>
    ) -> Json<serde_json::Value> {
    let x = state.counter.fetch_add(1, Ordering::Relaxed);

    let temperature = 65.0 + (x % 10) as f64;
    let speed = 65.0 + (x % 15) as f64;

    Json(serde_json::json!({
        "temperature": Sample{
            x,
            y: temperature
        },
        "speed": Sample{
            x,
            y: speed
        }
    }))
}