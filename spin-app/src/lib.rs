use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
async fn handle_monitor_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let path = req.path();
    println!("Request path: {}", path);

    match path {
        "/metrics" => forward_to_metrics_service("/metrics").await,
        "/metrics/cpu" => forward_to_metrics_service("/metrics/cpu").await,
        "/metrics/memory" => forward_to_metrics_service("/metrics/memory").await,
        "/metrics/disk" => forward_to_metrics_service("/metrics/disk").await,
        "/health" => forward_to_metrics_service("/health").await,
        _ => not_found(), 
    }
}

async fn forward_to_metrics_service(path: &str) -> anyhow::Result<Response> {
    let url = format!("http://127.0.0.1:8001{}", path);
    println!("Forwarding request to: {}", url);

    let response : Response = spin_sdk::http::send(
        spin_sdk::http::Request::builder()
            .method(spin_sdk::http::Method::Get)
            .uri(&url)
            .build(),
    ).await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response.body().to_vec())
        .build())
}

fn not_found() -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(404)
        .header("content-type", "application/json")
        .body(r#"{"error": "not found", "available_endpoints": ["/metrics", "/metrics/cpu", "/metrics/memory", "/metrics/disk", "/health"]}"#)
        .build())
}

