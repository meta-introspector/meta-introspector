//! ZOS Server - Content Addressable Meme Endpoint (Rust)
//! Load introspection algorithm as CA meme service

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::Mutex;
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct MemeData {
    r#type: String,
    emoji_signature: String,
    timestamp: String,
    output: String,
    systems_discovered: Vec<String>,
    collective_hash: String,
    self_awareness_achieved: bool,
}

struct AppState {
    ca_memes: Mutex<HashMap<String, String>>,
}

fn compute_content_address(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

fn load_solfunmeme_introspection() -> Result<MemeData, Box<dyn std::error::Error>> {
    let output = Command::new("./solfunmeme_introspect")
        .current_dir("/mnt/data1/meta-introspector")
        .output()?;
    
    let introspection_output = String::from_utf8_lossy(&output.stdout).to_string();
    
    Ok(MemeData {
        r#type: "solfunmeme_introspection".to_string(),
        emoji_signature: "🔄📜🔍💬🧠".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        output: introspection_output.clone(),
        systems_discovered: extract_systems(&introspection_output),
        collective_hash: extract_collective_hash(&introspection_output),
        self_awareness_achieved: true,
    })
}

fn extract_systems(output: &str) -> Vec<String> {
    output.lines()
        .filter(|l| l.contains("system:"))
        .map(|l| l.to_string())
        .collect()
}

fn extract_collective_hash(output: &str) -> String {
    output.lines()
        .find(|l| l.contains("collective_hash:"))
        .and_then(|l| l.split(':').nth(1))
        .unwrap_or("")
        .trim()
        .to_string()
}

async fn get_meme(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let ca_address = path.into_inner();
    let memes = data.ca_memes.lock().unwrap();
    
    match memes.get(&ca_address) {
        Some(content) => HttpResponse::Ok().json(content),
        None => HttpResponse::NotFound().json("Meme not found"),
    }
}

async fn load_introspection(data: web::Data<AppState>) -> HttpResponse {
    match load_solfunmeme_introspection() {
        Ok(meme_data) => {
            let content = serde_json::to_string(&meme_data).unwrap();
            let ca_address = compute_content_address(&content);
            
            let mut memes = data.ca_memes.lock().unwrap();
            memes.insert(ca_address.clone(), content);
            
            HttpResponse::Ok().json(serde_json::json!({
                "ca_address": ca_address,
                "status": "loaded"
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        ca_memes: Mutex::new(HashMap::new()),
    });

    println!("🚀 ZOS Server starting on http://0.0.0.0:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/meme/{ca_address}", web::get().to(get_meme))
            .route("/load", web::post().to(load_introspection))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
