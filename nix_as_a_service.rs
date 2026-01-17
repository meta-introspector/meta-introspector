// 🔥 NIX-AS-A-SERVICE: Load any nix flake, wrap .so files, expose via MCP with Solana CA
use axum::{extract::Query, http::StatusCode, response::Json, routing::post, Router};
use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct FlakeLoadRequest {
    pub flake_url: String,           // GitHub URL or local path
    pub flake_ref: Option<String>,   // Branch/tag/commit
    pub outputs: Vec<String>,        // Which outputs to load
    pub payment_lamports: u64,       // Payment for loading
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlakeLoadResponse {
    pub success: bool,
    pub content_address: String,     // Solana CA hash
    pub loaded_libraries: Vec<String>,
    pub mcp_endpoints: Vec<String>,
    pub cost_lamports: u64,
    pub flake_info: FlakeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakeInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub outputs: HashMap<String, String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub method: String,
    pub params: serde_json::Value,
    pub content_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResponse {
    pub result: serde_json::Value,
    pub cost_lamports: u64,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct NixPricing {
    pub flake_load_base: u64,        // Base cost to load a flake
    pub per_output_cost: u64,        // Cost per flake output
    pub library_load_cost: u64,      // Cost per .so library
    pub mcp_call_cost: u64,          // Cost per MCP method call
    pub storage_cost_per_mb: u64,    // Storage cost per MB
}

impl Default for NixPricing {
    fn default() -> Self {
        Self {
            flake_load_base: 5000,      // 5000 lamports base
            per_output_cost: 1000,      // 1000 per output
            library_load_cost: 2000,    // 2000 per .so
            mcp_call_cost: 100,         // 100 per MCP call
            storage_cost_per_mb: 500,   // 500 per MB storage
        }
    }
}

pub struct LoadedFlake {
    pub content_address: String,
    pub flake_info: FlakeInfo,
    pub libraries: HashMap<String, Arc<Library>>,
    pub store_path: PathBuf,
    pub mcp_methods: HashMap<String, String>,
}

pub struct NixAsAService {
    pricing: NixPricing,
    loaded_flakes: Arc<Mutex<HashMap<String, LoadedFlake>>>,
    nix_store_path: PathBuf,
}

impl NixAsAService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            pricing: NixPricing::default(),
            loaded_flakes: Arc::new(Mutex::new(HashMap::new())),
            nix_store_path: PathBuf::from("/nix/store"),
        })
    }

    pub fn calculate_flake_cost(&self, request: &FlakeLoadRequest) -> u64 {
        let mut cost = self.pricing.flake_load_base;
        cost += request.outputs.len() as u64 * self.pricing.per_output_cost;
        cost
    }

    pub fn generate_content_address(&self, flake_url: &str, outputs: &[String]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(flake_url.as_bytes());
        hasher.update(outputs.join(",").as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)[..16].to_string()
    }

    pub async fn load_nix_flake(&self, request: FlakeLoadRequest) -> Result<FlakeLoadResponse, Box<dyn std::error::Error>> {
        let cost = self.calculate_flake_cost(&request);
        
        // Verify payment
        if request.payment_lamports < cost {
            return Ok(FlakeLoadResponse {
                success: false,
                content_address: String::new(),
                loaded_libraries: vec![],
                mcp_endpoints: vec![],
                cost_lamports: cost,
                flake_info: FlakeInfo {
                    name: "payment_required".to_string(),
                    version: "0.0.0".to_string(),
                    description: Some("Insufficient payment".to_string()),
                    outputs: HashMap::new(),
                    dependencies: vec![],
                },
            });
        }

        let content_address = self.generate_content_address(&request.flake_url, &request.outputs);
        
        // Check if already loaded
        {
            let flakes = self.loaded_flakes.lock().unwrap();
            if let Some(existing) = flakes.get(&content_address) {
                return Ok(FlakeLoadResponse {
                    success: true,
                    content_address: content_address.clone(),
                    loaded_libraries: existing.libraries.keys().cloned().collect(),
                    mcp_endpoints: existing.mcp_methods.keys().map(|k| format!("/mcp/{}/{}", content_address, k)).collect(),
                    cost_lamports: 0, // Already loaded, no cost
                    flake_info: existing.flake_info.clone(),
                });
            }
        }

        // Build nix flake
        let flake_ref = if let Some(ref_str) = &request.flake_ref {
            format!("{}#{}", request.flake_url, ref_str)
        } else {
            request.flake_url.clone()
        };

        println!("🔥 Loading nix flake: {}", flake_ref);
        
        // Use nix build to build the flake
        let mut nix_cmd = Command::new("nix");
        nix_cmd.args(&["build", "--json", "--no-link"]);
        
        // Add specific outputs if requested
        for output in &request.outputs {
            nix_cmd.arg(format!("{}#{}", flake_ref, output));
        }
        
        let nix_output = nix_cmd.output()?;
        
        if !nix_output.status.success() {
            return Err(format!("Nix build failed: {}", String::from_utf8_lossy(&nix_output.stderr)).into());
        }

        // Parse nix build output to get store paths
        let build_results: Vec<serde_json::Value> = serde_json::from_slice(&nix_output.stdout)?;
        
        let mut loaded_libraries = Vec::new();
        let mut libraries = HashMap::new();
        let mut mcp_methods = HashMap::new();
        let mut store_path = PathBuf::new();

        // Scan for .so files in build outputs
        for result in &build_results {
            if let Some(path_str) = result["outputs"]["out"].as_str() {
                let path = PathBuf::from(path_str);
                store_path = path.clone();
                
                // Find all .so files recursively
                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if let Some(ext) = entry_path.extension() {
                            if ext == "so" {
                                let lib_name = entry_path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("unknown")
                                    .to_string();
                                
                                // Load the library
                                match unsafe { Library::new(&entry_path) } {
                                    Ok(lib) => {
                                        loaded_libraries.push(lib_name.clone());
                                        
                                        // Auto-discover MCP methods by looking for common patterns
                                        self.discover_mcp_methods(&lib, &lib_name, &mut mcp_methods);
                                        
                                        libraries.insert(lib_name, Arc::new(lib));
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to load library {}: {}", entry_path.display(), e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Extract flake info
        let flake_info = self.extract_flake_info(&flake_ref).await?;

        // Create loaded flake entry
        let loaded_flake = LoadedFlake {
            content_address: content_address.clone(),
            flake_info: flake_info.clone(),
            libraries,
            store_path,
            mcp_methods: mcp_methods.clone(),
        };

        // Store in loaded flakes
        {
            let mut flakes = self.loaded_flakes.lock().unwrap();
            flakes.insert(content_address.clone(), loaded_flake);
        }

        Ok(LoadFlakeResponse {
            success: true,
            content_address: content_address.clone(),
            loaded_libraries,
            mcp_endpoints: mcp_methods.keys().map(|k| format!("/mcp/{}/{}", content_address, k)).collect(),
            cost_lamports: cost,
            flake_info,
        })
    }

    pub async fn call_mcp_method(&self, request: MCPRequest) -> Result<MCPResponse, Box<dyn std::error::Error>> {
            flake_info,
        })
    }

    fn discover_mcp_methods(&self, lib: &Library, lib_name: &str, methods: &mut HashMap<String, String>) {
        // Common MCP method patterns to look for
        let common_methods = [
            "mcp_list_tools",
            "mcp_call_tool", 
            "mcp_get_schema",
            "mcp_initialize",
            "list_resources",
            "read_resource",
            "call_function",
            "get_completion",
        ];

        for method in &common_methods {
            // Try to find the symbol
            let symbol_name = format!("{}_{}", lib_name, method);
            if unsafe { lib.get::<fn()>(symbol_name.as_bytes()) }.is_ok() {
                methods.insert(method.to_string(), symbol_name);
            }
        }
    }

    async fn extract_flake_info(&self, flake_ref: &str) -> Result<FlakeInfo, Box<dyn std::error::Error>> {
        // Use nix flake show to get flake metadata
        let output = Command::new("nix")
            .args(&["flake", "show", "--json", flake_ref])
            .output()?;

        if output.status.success() {
            let flake_data: serde_json::Value = serde_json::from_slice(&output.stdout)?;
            
            Ok(FlakeInfo {
                name: flake_data["name"].as_str().unwrap_or("unknown").to_string(),
                version: flake_data["version"].as_str().unwrap_or("0.0.0").to_string(),
                description: flake_data["description"].as_str().map(|s| s.to_string()),
                outputs: HashMap::new(), // TODO: Parse outputs
                dependencies: vec![], // TODO: Parse dependencies
            })
        } else {
            Ok(FlakeInfo {
                name: "unknown".to_string(),
                version: "0.0.0".to_string(),
                description: None,
                outputs: HashMap::new(),
                dependencies: vec![],
            })
        }
    }

    pub async fn call_mcp_method(&self, request: MCPRequest) -> Result<MCPResponse, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        
        let flakes = self.loaded_flakes.lock().unwrap();
        let flake = flakes.get(&request.content_address)
            .ok_or("Flake not loaded")?;

        // Find the method in the loaded libraries
        let method_symbol = flake.mcp_methods.get(&request.method)
            .ok_or("Method not found")?;

        // TODO: Implement actual MCP method calling via FFI
        // This is a simplified version - real implementation would need proper FFI handling
        
        let result = serde_json::json!({
            "method": request.method,
            "content_address": request.content_address,
            "status": "success",
            "message": "MCP method called successfully"
        });

        Ok(MCPResponse {
            result,
            cost_lamports: self.pricing.mcp_call_cost,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }

// REST API handlers
pub async fn load_flake_endpoint(Json(request): Json<FlakeLoadRequest>) -> Result<Json<FlakeLoadResponse>, StatusCode> {
    let service = match NixAsAService::new() {
        Ok(s) => s,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    match service.load_nix_flake(request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn mcp_call_endpoint(Json(request): Json<MCPRequest>) -> Result<Json<MCPResponse>, StatusCode> {
    let service = match NixAsAService::new() {
        Ok(s) => s,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    match service.call_mcp_method(request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_flakes_endpoint() -> Json<HashMap<String, String>> {
    // TODO: Implement flake listing
    let mut flakes = HashMap::new();
    flakes.insert("status".to_string(), "nix-as-a-service".to_string());
    Json(flakes)
}

pub async fn pricing_endpoint() -> Json<NixPricing> {
    Json(NixPricing::default())
}

pub fn create_nix_service_router() -> Router {
    Router::new()
        .route("/load", post(load_flake_endpoint))
        .route("/mcp", post(mcp_call_endpoint))
        .route("/flakes", axum::routing::get(list_flakes_endpoint))
        .route("/pricing", axum::routing::get(pricing_endpoint))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 NIX-AS-A-SERVICE: Starting with MCP + Solana CA integration");
    
    let app = create_nix_service_router();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    println!("🚀 Nix-as-a-Service running on http://0.0.0.0:8081");
    println!("📊 Endpoints:");
    println!("   POST /load - Load nix flake with Solana CA");
    println!("   POST /mcp - Call MCP methods on loaded libraries");
    println!("   GET  /flakes - List loaded flakes");
    println!("   GET  /pricing - View pricing model");
    
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_content_addressing() {
        let service = NixAsAService::new().unwrap();
        
        let ca1 = service.generate_content_address("github:nixos/nixpkgs", &["hello"]);
        let ca2 = service.generate_content_address("github:nixos/nixpkgs", &["hello"]);
        let ca3 = service.generate_content_address("github:nixos/nixpkgs", &["world"]);
        
        assert_eq!(ca1, ca2); // Same inputs = same CA
        assert_ne!(ca1, ca3); // Different inputs = different CA
    }
}
