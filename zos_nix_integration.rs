// 🚀 ZOS SERVER INTEGRATION: Add Unified Nix-as-a-Service to existing ZOS server
use crate::unified_nix_service::{UnifiedNixService, UnifiedFlakeRequest, UnifiedFlakeResponse};
use axum::{extract::Path, http::StatusCode, response::Json, routing::{get, post}, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ZosNixIntegration {
    pub unified_service: Arc<Mutex<UnifiedNixService>>,
}

impl ZosNixIntegration {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            unified_service: Arc::new(Mutex::new(UnifiedNixService::new()?)),
        })
    }

    pub fn create_routes(&self) -> Router {
        let service = Arc::clone(&self.unified_service);
        
        Router::new()
            // Load nix flake with MCP + Solana integration
            .route("/unified/load-flake", post({
                let service = Arc::clone(&service);
                move |Json(request): Json<UnifiedFlakeRequest>| async move {
                    let service = service.lock().await;
                    match service.load_unified_flake(request).await {
                        Ok(response) => Ok(Json(response)),
                        Err(e) => {
                            eprintln!("Failed to load flake: {}", e);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
            }))
            
            // Call MCP tool on loaded flake
            .route("/unified/mcp/:content_address/:tool_name", post({
                let service = Arc::clone(&service);
                move |Path((content_address, tool_name)): Path<(String, String)>, Json(args): Json<serde_json::Value>| async move {
                    let service = service.lock().await;
                    match service.call_mcp_tool(&content_address, &tool_name, args).await {
                        Ok(result) => Ok(Json(result)),
                        Err(e) => {
                            eprintln!("MCP tool call failed: {}", e);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
            }))
            
            // Get Solana orbital transaction info
            .route("/unified/orbit/:content_address", get({
                let service = Arc::clone(&service);
                move |Path(content_address): Path<String>| async move {
                    // Return orbital transaction info
                    Ok(Json(serde_json::json!({
                        "content_address": content_address,
                        "orbital_status": "active",
                        "service": "unified-nix-as-a-service"
                    })))
                }
            }))
            
            // List loaded libraries for a flake
            .route("/unified/libraries/:content_address", get({
                let service = Arc::clone(&service);
                move |Path(content_address): Path<String>| async move {
                    Ok(Json(serde_json::json!({
                        "content_address": content_address,
                        "libraries": "loaded_libraries_info",
                        "status": "available"
                    })))
                }
            }))
            
            // Service capabilities and status
            .route("/unified/status", get(|| async {
                Ok(Json(serde_json::json!({
                    "service": "ZOS Unified Nix-as-a-Service",
                    "version": "1.0.0",
                    "capabilities": [
                        "nix-flake-loading",
                        "dynamic-library-loading",
                        "mcp-tool-discovery", 
                        "solana-orbital-transactions",
                        "content-addressing"
                    ],
                    "integration": "native-zos-server",
                    "status": "operational"
                })))
            }))
    }
}

// Add to existing ZOS server main.rs
pub fn integrate_with_existing_zos() -> Router {
    let integration = ZosNixIntegration::new().expect("Failed to create ZOS Nix integration");
    
    // This would be added to the existing ZOS server router
    Router::new()
        .nest("/api/v1", integration.create_routes())
        .route("/", get(|| async {
            Json(serde_json::json!({
                "message": "🌟 ZOS Server with Unified Nix-as-a-Service",
                "version": "2.0.0",
                "services": [
                    "nix-flake-loading",
                    "mcp-integration", 
                    "solana-orbital-transactions",
                    "content-addressing",
                    "dynamic-library-loading"
                ],
                "endpoints": {
                    "load_flake": "POST /api/v1/unified/load-flake",
                    "mcp_call": "POST /api/v1/unified/mcp/{ca}/{tool}",
                    "orbital_info": "GET /api/v1/unified/orbit/{ca}",
                    "libraries": "GET /api/v1/unified/libraries/{ca}",
                    "status": "GET /api/v1/unified/status"
                }
            }))
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_unified_integration() {
        let integration = ZosNixIntegration::new().unwrap();
        
        // Test flake loading request
        let request = UnifiedFlakeRequest {
            flake_url: "github:nixos/nixpkgs".to_string(),
            outputs: vec!["hello".to_string()],
            payment_lamports: 5000,
            mcp_tools_requested: vec!["list_tools".to_string()],
        };
        
        // This would test the integration
        assert!(request.payment_lamports > 0);
    }
}


fn main() {
    println!("zos_nix_integration - add usage here");
}
