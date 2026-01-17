// 🦀 RUST-AS-A-SERVICE: Load rustc_driver.so and charge for compilation
use axum::{extract::Query, http::StatusCode, response::{Json, IntoResponse}, routing::post, Router};
use libloading::Library;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileRequest {
    pub source_code: String,
    pub target: Option<String>,
    pub optimization: Option<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileResponse {
    pub success: bool,
    pub output: Option<String>,
    pub errors: Option<String>,
    pub cost_lamports: u64,
    pub execution_time_ms: u64,
    pub binary_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RustcMetrics {
    pub compilation_time: Duration,
    pub memory_usage: u64,
    pub binary_size: u64,
    pub optimization_level: String,
}

// Pricing model for Rust compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustPricing {
    pub base_cost: u64,           // Base cost per compilation
    pub per_line_cost: u64,       // Cost per line of code
    pub optimization_multiplier: f64, // Multiplier for optimization levels
    pub feature_cost: u64,        // Cost per feature flag
}

impl Default for RustPricing {
    fn default() -> Self {
        Self {
            base_cost: 1000,           // 1000 lamports base
            per_line_cost: 10,         // 10 lamports per line
            optimization_multiplier: 1.5, // 50% more for -O2/-O3
            feature_cost: 100,         // 100 lamports per feature
        }
    }
}

pub struct RustAsAService {
    rustc_lib: Arc<Library>,
    pricing: RustPricing,
    metrics: Arc<Mutex<HashMap<String, RustcMetrics>>>,
    zombie_driver_path: String,
}

impl RustAsAService {
    pub fn new(zombie_driver_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Load zombie_driver2 rustc capabilities
        let rustc_lib = unsafe { Library::new(zombie_driver_path)? };
        
        Ok(Self {
            rustc_lib: Arc::new(rustc_lib),
            pricing: RustPricing::default(),
            metrics: Arc::new(Mutex::new(HashMap::new())),
            zombie_driver_path: zombie_driver_path.to_string(),
        })
    }

    pub fn calculate_cost(&self, request: &CompileRequest) -> u64 {
        let line_count = request.source_code.lines().count() as u64;
        let mut cost = self.pricing.base_cost + (line_count * self.pricing.per_line_cost);
        
        // Add optimization costs
        if let Some(opt) = &request.optimization {
            if opt.contains("2") || opt.contains("3") {
                cost = (cost as f64 * self.pricing.optimization_multiplier) as u64;
            }
        }
        
        // Add feature costs
        cost += request.features.len() as u64 * self.pricing.feature_cost;
        
        cost
    }

    pub async fn compile_rust(&self, request: CompileRequest) -> Result<CompileResponse, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let cost = self.calculate_cost(&request);
        
        // Content-addressable build directory (like nix)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        request.source_code.hash(&mut hasher);
        request.target.hash(&mut hasher);
        request.optimization.hash(&mut hasher);
        request.features.hash(&mut hasher);
        let content_hash = format!("{:x}", hasher.finish());
        
        // Store in content-addressable location
        let build_dir = std::path::PathBuf::from("/tmp/rust-builds")
            .join(&content_hash[..2])  // First 2 chars for sharding
            .join(&content_hash);
        
        // Check if already built (cache hit!)
        let binary_path = build_dir.join("output");
        if binary_path.exists() {
            let binary_size = std::fs::metadata(&binary_path)?.len();
            return Ok(CompileResponse {
                success: true,
                output: Some(format!("Cache hit! Binary at: {}", binary_path.display())),
                errors: None,
                cost_lamports: 0, // Free for cached builds!
                execution_time_ms: 0,
                binary_size: Some(binary_size),
            });
        }
        
        // New build - create content-addressable directory
        std::fs::create_dir_all(&build_dir)?;
        let source_path = build_dir.join("main.rs");
        std::fs::write(&source_path, &request.source_code)?;
        
        // Build rustc command using zombie_driver2
        let mut cmd = std::process::Command::new(&self.zombie_driver_path);
        cmd.arg(&source_path);
        
        // Add target if specified
        if let Some(target) = &request.target {
            cmd.args(["--target", target]);
        }
        
        // Add optimization level
        if let Some(opt) = &request.optimization {
            cmd.arg(format!("-O{}", opt));
        }
        
        // Add features
        for feature in &request.features {
            cmd.args(["--cfg", &format!("feature=\"{}\"", feature)]);
        }
        
        // Execute compilation
        let output = cmd.output()?;
        let execution_time = start_time.elapsed();
        
        // Calculate binary size if successful and save to content-addressable location
        let binary_size = if output.status.success() {
            let output_binary = build_dir.join("main");
            let stored_binary = build_dir.join("output");
            if output_binary.exists() {
                // Move to canonical location for caching
                std::fs::rename(&output_binary, &stored_binary)?;
                Some(std::fs::metadata(&stored_binary)?.len())
            } else {
                None
            }
        } else {
            None
        };
        
        // Store metrics
        let metrics = RustcMetrics {
            compilation_time: execution_time,
            memory_usage: 0, // TODO: Implement memory tracking
            binary_size: binary_size.unwrap_or(0),
            optimization_level: request.optimization.unwrap_or_default(),
        };
        
        let mut metrics_map = self.metrics.lock().unwrap();
        metrics_map.insert(format!("compile_{}", chrono::Utc::now().timestamp()), metrics);
        
        Ok(CompileResponse {
            success: output.status.success(),
            output: if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                None
            },
            errors: if !output.status.success() {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            } else {
                None
            },
            cost_lamports: cost,
            execution_time_ms: execution_time.as_millis() as u64,
            binary_size,
        })
    }
}

// REST API handlers
pub async fn compile_endpoint(
    Query(_params): Query<HashMap<String, String>>,
    Json(request): Json<CompileRequest>,
) -> Result<Json<CompileResponse>, StatusCode> {
    // Get zombie_driver2 path from environment or default
    let zombie_path = std::env::var("ZOMBIE_DRIVER_PATH")
        .unwrap_or_else(|_| "/home/mdupont/zombie_driver2/target/debug/zombie-rustc".to_string());
    
    let service = match RustAsAService::new(&zombie_path) {
        Ok(s) => s,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    match service.compile_rust(request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn pricing_endpoint() -> impl IntoResponse {
    Json(RustPricing::default())
}

pub async fn metrics_endpoint() -> impl IntoResponse {
    let mut metrics = HashMap::new();
    metrics.insert("service".to_string(), "rust-as-a-service".to_string());
    metrics.insert("version".to_string(), "1.0.0".to_string());
    metrics.insert("zombie_driver".to_string(), "enabled".to_string());
    Json(metrics)
}

pub fn create_rust_service_router() -> Router {
    Router::new()
        .route("/compile", post(compile_endpoint))
        .route("/pricing", axum::routing::get(pricing_endpoint))
        .route("/metrics", axum::routing::get(metrics_endpoint))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 RUST-AS-A-SERVICE: Starting devnet compilation service");
    
    let app = create_rust_service_router();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("🚀 Rust compilation service running on http://0.0.0.0:8080");
    println!("📊 Endpoints:");
    println!("   POST /compile - Compile Rust code (pay per compilation)");
    println!("   GET  /pricing - View pricing model");
    println!("   GET  /metrics - Service metrics");
    
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_pricing_calculation() {
        let service = RustAsAService::new("/fake/path").unwrap_or_else(|_| {
            // Mock service for testing
            RustAsAService {
                rustc_lib: Arc::new(unsafe { Library::new("libdl.so").unwrap() }),
                pricing: RustPricing::default(),
                metrics: Arc::new(Mutex::new(HashMap::new())),
                zombie_driver_path: "/fake/path".to_string(),
            }
        });
        
        let request = CompileRequest {
            source_code: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
            target: None,
            optimization: Some("2".to_string()),
            features: vec!["serde".to_string()],
        };
        
        let cost = service.calculate_cost(&request);
        
        // Base (1000) + 2 lines (20) + optimization (50% more) + 1 feature (100)
        // = (1020 * 1.5) + 100 = 1530 + 100 = 1630
        assert!(cost > 1500);
    }
}
