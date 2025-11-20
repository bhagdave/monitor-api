use serde_json::json;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
fn handle_monitor_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let path = req.path();
    println!("Request path: {}", path);

    match path {
        "/metrics" => all_metrics(),
        "/metrics/health" => health_check(),
        _ => not_found(), 
    }
}

fn health_check() -> anyhow::Result<Response> {
    let response = json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });


    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response.to_string())
        .build())
}

fn not_found() -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body("Not Found")
        .build())
}

fn all_metrics() -> anyhow::Result<Response> {
    // Placeholder for actual metrics collection logic
    let metrics_data = "metric1 100\nmetric2 200\n";

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(metrics_data)
        .build())
}
