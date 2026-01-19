//! # File Index HTTP Server
//! 
//! HTTP API for file index service that shell scripts can query.
//!
//! ## Endpoints
//! 
//! - `GET /query/ext/:ext` - Find files by extension
//! - `GET /query/name/:name` - Find files by name
//! - `GET /query/pattern?q=pattern` - Find files by pattern
//! - `GET /priority?limit=100` - Get top priority files
//! - `GET /predict` - Get predicted next queries
//! - `GET /stats` - Get index statistics
//! - `POST /refresh` - Refresh index from filesystem

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

mod file_index_service;
use file_index_service::{FileIndexService, FileEntry, IndexStats};

/// Shared application state
type AppState = Arc<RwLock<FileIndexService>>;

/// Query parameters for pattern search
#[derive(Deserialize)]
struct PatternQuery {
    q: String,
}

/// Query parameters for limit
#[derive(Deserialize)]
struct LimitQuery {
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    100
}

/// Response wrapper
#[derive(Serialize)]
struct Response<T> {
    success: bool,
    data: T,
    count: usize,
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting File Index Server...");
    
    // Initialize service
    let cache_dir = std::path::PathBuf::from("data/file_index_cache");
    let roots = vec![
        std::path::PathBuf::from("."),
        // Add more roots as needed
    ];
    
    let mut service = FileIndexService::new(cache_dir, roots);
    service.initialize().expect("Failed to initialize service");
    
    // Pre-fetch predicted queries
    service.prefetch_predicted();
    
    let state = Arc::new(RwLock::new(service));
    
    // Build router
    let app = Router::new()
        .route("/query/ext/:ext", get(query_by_extension))
        .route("/query/name/:name", get(query_by_name))
        .route("/query/pattern", get(query_by_pattern))
        .route("/priority", get(get_priority))
        .route("/predict", get(get_predictions))
        .route("/stats", get(get_stats))
        .route("/refresh", post(refresh_index))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    // Start server
    let addr = "127.0.0.1:3030";
    println!("✅ Server listening on http://{}", addr);
    println!("\n📚 Available endpoints:");
    println!("  GET  /query/ext/:ext          - Find by extension");
    println!("  GET  /query/name/:name        - Find by name");
    println!("  GET  /query/pattern?q=...     - Find by pattern");
    println!("  GET  /priority?limit=100      - Top priority files");
    println!("  GET  /predict                 - Predicted queries");
    println!("  GET  /stats                   - Index statistics");
    println!("  POST /refresh                 - Refresh index");
    println!("  GET  /health                  - Health check");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Query files by extension
async fn query_by_extension(
    State(state): State<AppState>,
    Path(ext): Path<String>,
) -> Json<Response<Vec<FileEntry>>> {
    let service = state.read().await;
    let results = service.query_by_extension(&ext);
    let count = results.len();
    
    Json(Response {
        success: true,
        data: results,
        count,
    })
}

/// Query files by name
async fn query_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Json<Response<Vec<FileEntry>>> {
    let service = state.read().await;
    let results = service.query_by_name(&name);
    let count = results.len();
    
    Json(Response {
        success: true,
        data: results,
        count,
    })
}

/// Query files by pattern
async fn query_by_pattern(
    State(state): State<AppState>,
    Query(params): Query<PatternQuery>,
) -> Json<Response<Vec<FileEntry>>> {
    let service = state.read().await;
    let results = service.query_by_pattern(&params.q);
    let count = results.len();
    
    Json(Response {
        success: true,
        data: results,
        count,
    })
}

/// Get top priority files
async fn get_priority(
    State(state): State<AppState>,
    Query(params): Query<LimitQuery>,
) -> Json<Response<Vec<FileEntry>>> {
    let service = state.read().await;
    let results = service.get_top_priority(params.limit);
    let count = results.len();
    
    Json(Response {
        success: true,
        data: results,
        count,
    })
}

/// Get predicted queries
async fn get_predictions(
    State(state): State<AppState>,
) -> Json<Response<Vec<String>>> {
    let service = state.read().await;
    let predictions = service.predict_next_queries(10);
    let count = predictions.len();
    
    Json(Response {
        success: true,
        data: predictions,
        count,
    })
}

/// Get index statistics
async fn get_stats(
    State(state): State<AppState>,
) -> Json<IndexStats> {
    let service = state.read().await;
    Json(service.stats())
}

/// Refresh index from filesystem
async fn refresh_index(
    State(state): State<AppState>,
) -> Result<Json<Response<String>>, StatusCode> {
    let mut service = state.write().await;
    
    match service.initialize() {
        Ok(_) => Ok(Json(Response {
            success: true,
            data: "Index refreshed successfully".to_string(),
            count: 1,
        })),
        Err(e) => {
            eprintln!("Failed to refresh index: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Health check
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "file-index-server",
        "version": "1.0.0"
    }))
}
