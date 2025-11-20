use serde_json::json;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use sysinfo::System;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/metrics", get(all_metrics))
        .route("/metrics/cpu", get(cpu_metrics))
        .route("/metrics/memory", get(memory_metrics))
        .route("/metrics/disk", get(disk_metrics))
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    println!("Metrics service listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn all_metrics() -> axum::Json<serde_json::Value> {
    let mut sys = System::new_all();
    sys.refresh_all();

    axum::Json(json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "cpu": {
            "global_usage": sys.global_cpu_info().cpu_usage(),
            "cores": sys.cpus().iter().map(|cpu| {
                json!({
                    "name": cpu.name(),
                    "usage": cpu.cpu_usage()
                })
            }).collect::<Vec<_>>()
        },
        "memory": {
            "total_mb": sys.total_memory() / 1024,
            "used_mb": sys.used_memory() / 1024,
            "available_mb": sys.available_memory() / 1024,
            "usage_percent": (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
        },
        "disk": {
            "disks": sysinfo::Disks::new_with_refreshed_list().iter().map(|disk| {
                json!({
                    "mount_point": disk.mount_point().to_string_lossy(),
                    "total_gb": disk.total_space() / 1_000_000_000,
                    "available_gb": disk.available_space() / 1_000_000_000,
                    "usage_percent": ((disk.total_space() - disk.available_space()) as f64 / disk.total_space() as f64) * 100.0
                })
            }).collect::<Vec<_>>()
        }
    }))
}

async fn cpu_metrics() -> axum::Json<serde_json::Value> {
    let mut sys = System::new_all();
    sys.refresh_all();

    axum::Json(json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "global_usage": sys.global_cpu_info().cpu_usage(),
        "cores": sys.cpus().iter().map(|cpu| {
            json!({
                "name": cpu.name(),
                "usage": cpu.cpu_usage()
            })
        }).collect::<Vec<_>>()
    }))
}

async fn memory_metrics() -> axum::Json<serde_json::Value> {
    let mut sys = System::new_all();
    sys.refresh_memory();

    axum::Json(json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_mb": sys.total_memory() / 1024,
        "used_mb": sys.used_memory() / 1024,
        "available_mb": sys.available_memory() / 1024,
        "usage_percent": (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
    }))
}

async fn disk_metrics() -> axum::Json<serde_json::Value> {
    let disks = sysinfo::Disks::new_with_refreshed_list();

    axum::Json(json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "disks": disks.iter().map(|disk| {
            json!({
                "mount_point": disk.mount_point().to_string_lossy(),
                "total_gb": disk.total_space() / 1_000_000_000,
                "used_gb": (disk.total_space() - disk.available_space()) / 1_000_000_000,
                "available_gb": disk.available_space() / 1_000_000_000,
                "usage_percent": ((disk.total_space() - disk.available_space()) as f64 / disk.total_space() as f64) * 100.0
            })
        }).collect::<Vec<_>>()
    }))
}
