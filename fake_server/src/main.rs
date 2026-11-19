use axum::{
    routing::get,
    Json,
    Router,
};
use serde::{
    Serialize,
};
use std::sync::{
    Arc,
    atomic::{
        AtomicU64, 
        Ordering,
    }
};

#[derive(Clone)]
struct AppState{
    counter: Arc<AtomicU64>
}

async fn get_logs() -> Json<serde_json::Value>
{
    Json(serde_json::json!([
        {
            "timestamp": "timestamp",
            "status": "status",
            "source": "source",
            "body": "body",
            "host_ip": "host_ip"
        }
    ]))
}

async fn hmi_status() -> Json<serde_json::Value>
{
    Json(serde_json::json!({
        "hmi": false
    }))
}

async fn hat_status() -> Json<serde_json::Value>
{
    Json(serde_json::json!({
        "hat": true
    }))
}

async fn nas_status() -> Json<serde_json::Value>
{
    Json(serde_json::json!({
        "nas": true
    }))
}

async fn plc_status() -> Json<serde_json::Value>
{
    Json(serde_json::json!({
        "plc": true
    }))
}

#[tokio::main]
async fn main(){
    let state = AppState{
        counter: Arc::new(AtomicU64::new(0))
    };

    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .route("/logs", get(get_logs))
        .route("/hmi/status", get(hmi_status))
        .route("/hat/status", get(hat_status))
        .route("/nas/status", get(nas_status))
        .route("/plc/status", get(plc_status))
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