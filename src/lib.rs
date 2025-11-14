use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
fn handle_monitor_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let path = req.path();
    println!("Request path: {}", path);

    match path {
        "/metrics/health" => health_check(),
        _ => not_found(), 
    }
}

fn health_check() -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Monitor Api is Up and running")
        .build())
}

fn not_found() -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body("Not Found")
        .build())
}
