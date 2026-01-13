use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::solfunmeme_ca_service::SolfunmemeCAService;

pub struct AppState {
    pub solfunmeme_service: Arc<Mutex<SolfunmemeCAService>>,
}

pub fn create_solfunmeme_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/meme/:ca_address", get(get_meme))
        .route("/memes", get(list_memes))
        .route("/introspect", post(run_introspection))
        .route("/reload", post(reload_meme))
        .route("/", get(index))
        .layer(CorsLayer::permissive())
}

async fn index() -> Json<Value> {
    Json(json!({
        "service": "ZOS Server - SOLFUNMEME CA Meme Endpoint",
        "emoji": "🔄📜🔍💬🧠",
        "version": "v2.0.0-automorphic",
        "endpoints": {
            "/meme/<ca_address>": "Get content addressable meme",
            "/memes": "List all CA memes",
            "/introspect": "Run SOLFUNMEME introspection",
            "/reload": "Reload SOLFUNMEME meme"
        },
        "status": "ready"
    }))
}

async fn get_meme(
    Path(ca_address): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let service = state.solfunmeme_service.lock().await;
    
    if let Some(meme) = service.get_meme(&ca_address) {
        Ok(Json(json!({
            "ca_address": ca_address,
            "meme": meme,
            "status": "found"
        })))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn list_memes(State(state): State<Arc<AppState>>) -> Json<Value> {
    let service = state.solfunmeme_service.lock().await;
    let memes = service.list_memes();
    
    Json(json!({
        "total_memes": memes.len(),
        "memes": memes
    }))
}

async fn run_introspection(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    let mut service = state.solfunmeme_service.lock().await;
    
    match service.load_solfunmeme_introspection().await {
        Ok(ca_address) => Ok(Json(json!({
            "status": "success",
            "ca_address": ca_address,
            "emoji": "🔄📜🔍💬🧠",
            "message": "SOLFUNMEME introspection complete",
            "access_url": format!("/meme/{}", ca_address)
        }))),
        Err(e) => {
            eprintln!("❌ Introspection error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn reload_meme(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    let mut service = state.solfunmeme_service.lock().await;
    
    match service.load_solfunmeme_introspection().await {
        Ok(ca_address) => Ok(Json(json!({
            "status": "reloaded",
            "ca_address": ca_address,
            "emoji": "🔄📜🔍💬🧠",
            "message": "SOLFUNMEME meme reloaded successfully"
        }))),
        Err(e) => {
            eprintln!("❌ Reload error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
