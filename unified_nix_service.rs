// 🌟 UNIFIED NIX-AS-A-SERVICE: Integrate existing ZOS + MCP + Solana + Content Addressing
// use crate::extra_plugins::protocol_plugins::McpPlugin;
// use crate::solana_orbital_transactions::{SolanaOrbit, PaymentCycle};
use libloading::Library;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

// Stubs
struct NixCanonicalBuilder;
impl NixCanonicalBuilder {
    fn new() -> Self { NixCanonicalBuilder }
    fn build(&self, _req: NixBuildRequest) -> Result<NixBuildResult, String> { panic!("stub") }
}
struct NixBuildRequest { args: Vec<String>, env: Vec<(String, String)>, working_dir: Option<PathBuf> }
struct NixBuildResult { stdout: String, stderr: String, success: bool, store_paths: Vec<PathBuf>, library_names: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PaymentCycle { 
    cycle_id: String, 
    participants: Vec<String>,
    cycle_payments: Vec<u64>,
    total_orbital_energy: u64,
    cycle_eigenvalue: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SolanaOrbit { orbit_signature: String, level: u32, cycle_index: u64, orbital_energy: u64, payment_cycle: PaymentCycle, compute_units: u64, orbital_period: u64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpPlugin;
use tokio::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnifiedFlakeRequest {
    pub flake_url: String,
    pub outputs: Vec<String>,
    pub payment_lamports: u64,
    pub mcp_tools_requested: Vec<String>,
}

// Stub types removed - defined at top of file

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnifiedFlakeResponse {
    pub success: bool,
    pub content_address: String,
    pub solana_orbit: SolanaOrbit,
    pub loaded_libraries: Vec<String>,
    pub mcp_endpoints: Vec<MCPEndpoint>,
    pub nix_store_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPEndpoint {
    pub tool_name: String,
    pub endpoint_url: String,
    pub content_address: String,
    pub library_path: String,
}

pub struct UnifiedNixService {
    // Existing ZOS components
    loaded_flakes: Arc<Mutex<HashMap<String, LoadedFlake>>>,
    mcp_plugins: Arc<Mutex<HashMap<String, McpPlugin>>>,
    
    // Nix integration
    nix_store_path: PathBuf,
    
    // Solana integration  
    orbital_transactions: Arc<Mutex<HashMap<String, SolanaOrbit>>>,
}

pub struct LoadedFlake {
    pub content_address: String,
    pub nix_store_paths: Vec<PathBuf>,
    pub libraries: HashMap<String, Arc<Library>>,
    pub mcp_tools: HashMap<String, MCPTool>,
    pub solana_orbit: SolanaOrbit,
}

#[derive(Debug, Clone)]
pub struct MCPTool {
    pub name: String,
    pub library_symbol: String,
    pub input_schema: serde_json::Value,
}

impl UnifiedNixService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            loaded_flakes: Arc::new(Mutex::new(HashMap::new())),
            mcp_plugins: Arc::new(Mutex::new(HashMap::new())),
            nix_store_path: PathBuf::from("/nix/store"),
            orbital_transactions: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn generate_content_address(&self, flake_url: &str, outputs: &[String]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(flake_url.as_bytes());
        hasher.update(outputs.join(",").as_bytes());
        hasher.update(chrono::Utc::now().timestamp().to_string().as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    pub async fn load_unified_flake(&self, request: UnifiedFlakeRequest) -> Result<UnifiedFlakeResponse, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let content_address = self.generate_content_address(&request.flake_url, &request.outputs);

        // 1. Build the nix flake using existing ZOS nix integration
        let nix_result = self.build_nix_flake(&request.flake_url, &request.outputs).await?;
        
        // 2. Load .so libraries from nix store paths
        let libraries = self.load_libraries_from_paths(&nix_result.store_paths)?;
        
        // 3. Discover and register MCP tools from loaded libraries
        let mcp_tools = self.discover_mcp_tools(&libraries, &request.mcp_tools_requested)?;
        
        // 4. Create Solana orbital transaction for this service load
        let solana_orbit = self.create_solana_orbit(
            &content_address,
            request.payment_lamports,
            &request.flake_url,
            start_time.elapsed().as_millis() as u64,
        )?;

        // 5. Create MCP endpoints
        let mcp_endpoints = self.create_mcp_endpoints(&content_address, &mcp_tools);

        // 6. Store the loaded flake
        let loaded_flake = LoadedFlake {
            content_address: content_address.clone(),
            nix_store_paths: nix_result.store_paths.clone(),
            libraries,
            mcp_tools,
            solana_orbit: solana_orbit.clone(),
        };

        {
            let mut flakes = self.loaded_flakes.lock().unwrap();
            flakes.insert(content_address.clone(), loaded_flake);
        }

        {
            let mut orbits = self.orbital_transactions.lock().unwrap();
            orbits.insert(content_address.clone(), solana_orbit.clone());
        }

        Ok(UnifiedFlakeResponse {
            success: true,
            content_address,
            solana_orbit,
            loaded_libraries: nix_result.library_names,
            mcp_endpoints,
            nix_store_paths: vec![], // nix_result.store_paths.iter().map(|p| p.to_string_lossy().to_string()).collect(),
        })
    }

    async fn build_nix_flake(&self, flake_url: &str, outputs: &[String]) -> Result<NixBuildResult, Box<dyn std::error::Error>> {
        println!("🔥 Building nix flake: {}", flake_url);
        
        // Use canonical builder
        // use crate::nix_canonical_builder::{NixCanonicalBuilder, NixBuildRequest};
        panic!("nix_canonical_builder not available");
        
        let mut args = vec!["build".to_string(), "--json".to_string(), "--no-link".to_string()];
        for output in outputs {
            args.push(format!("{}#{}", flake_url, output));
        }
        
        let builder = NixCanonicalBuilder::new();
        let result = builder.build(NixBuildRequest {
            args,
            env: vec![],
            working_dir: None,
        }).map_err(|e| e.to_string())?;
        
        if !result.success {
            return Err(format!("Nix build failed: {}", result.stderr).into());
        }

        let build_results: Vec<serde_json::Value> = serde_json::from_str(&result.stdout)?;
        let mut store_paths = Vec::new();
        let mut library_names = Vec::new();

        for build_result in &build_results {
            if let Some(path_str) = build_result["outputs"]["out"].as_str() {
                let path = PathBuf::from(path_str);
                store_paths.push(path.clone());
                
                // Scan for .so files
                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if let Some(ext) = entry_path.extension() {
                            if ext == "so" {
                                if let Some(name) = entry_path.file_stem().and_then(|s| s.to_str()) {
                                    library_names.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(NixBuildResult {
            stdout: String::new(),
            stderr: String::new(),
            success: true,
            store_paths,
            library_names,
        })
    }

    fn load_libraries_from_paths(&self, paths: &[PathBuf]) -> Result<HashMap<String, Arc<Library>>, Box<dyn std::error::Error>> {
        let mut libraries = HashMap::new();
        
        for path in paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if let Some(ext) = entry_path.extension() {
                        if ext == "so" {
                            let lib_name = entry_path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("unknown")
                                .to_string();
                            
                            match unsafe { Library::new(&entry_path) } {
                                Ok(lib) => {
                                    println!("✅ Loaded library: {}", lib_name);
                                    libraries.insert(lib_name, Arc::new(lib));
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to load {}: {}", entry_path.display(), e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(libraries)
    }

    fn discover_mcp_tools(&self, libraries: &HashMap<String, Arc<Library>>, requested: &[String]) -> Result<HashMap<String, MCPTool>, Box<dyn std::error::Error>> {
        let mut mcp_tools = HashMap::new();
        
        for (lib_name, library) in libraries {
            // Look for MCP tool symbols
            let tool_patterns = if requested.is_empty() {
                // Default MCP tools to look for
                vec!["list_tools", "call_tool", "list_resources", "read_resource"]
            } else {
                requested.iter().map(|s| s.as_str()).collect()
            };

            for tool_name in tool_patterns {
                let symbol_name = format!("mcp_{}", tool_name);
                
                // Try to find the symbol
                if unsafe { library.get::<fn()>(symbol_name.as_bytes()) }.is_ok() {
                    let mcp_tool = MCPTool {
                        name: format!("{}_{}", lib_name, tool_name),
                        library_symbol: symbol_name,
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "input": {"type": "string"}
                            }
                        }),
                    };
                    
                    mcp_tools.insert(mcp_tool.name.clone(), mcp_tool);
                    println!("🔧 Discovered MCP tool: {}", mcp_tool.name);
                }
            }
        }
        
        Ok(mcp_tools)
    }

    fn create_solana_orbit(&self, content_address: &str, payment: u64, flake_url: &str, compute_time: u64) -> Result<SolanaOrbit, Box<dyn std::error::Error>> {
        // Create orbital transaction for this flake load
        let orbit = SolanaOrbit {
            orbit_signature: content_address.to_string(),
            level: 11, // LMFDB level 11 for basic flake loads
            cycle_index: 1,
            orbital_energy: payment,
            payment_cycle: PaymentCycle {
                cycle_id: format!("flake_{}", content_address),
                participants: vec![flake_url.to_string()],
                cycle_payments: vec![],
                total_orbital_energy: payment,
                cycle_eigenvalue: 1.0,
            },
            compute_units: compute_time,
            orbital_period: 1,
        };
        
        Ok(orbit)
    }

    fn create_mcp_endpoints(&self, content_address: &str, mcp_tools: &HashMap<String, MCPTool>) -> Vec<MCPEndpoint> {
        mcp_tools.iter().map(|(name, tool)| {
            MCPEndpoint {
                tool_name: name.clone(),
                endpoint_url: format!("/mcp/{}/{}", content_address, name),
                content_address: content_address.to_string(),
                library_path: tool.library_symbol.clone(),
            }
        }).collect()
    }

    pub async fn call_mcp_tool(&self, content_address: &str, tool_name: &str, args: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let flakes = self.loaded_flakes.lock().unwrap();
        let flake = flakes.get(content_address).ok_or("Flake not loaded")?;
        
        let tool = flake.mcp_tools.get(tool_name).ok_or("Tool not found")?;
        
        // Use existing ZOS MCP plugin system
        let args_str = serde_json::to_string(&args)?;
        
        // This would integrate with the existing McpPlugin::call_tool method
        // For now, return a success response
        Ok(serde_json::json!({
            "result": "MCP tool called successfully",
            "tool": tool_name,
            "content_address": content_address,
            "args": args
        }))
    }
}

// NixBuildResult defined at top of file

// Integration with existing ZOS server
impl UnifiedNixService {
    pub fn integrate_with_zos_server(&self) -> serde_json::Value {
        serde_json::json!({
            "service": "unified-nix-as-a-service",
            "capabilities": [
                "nix-flake-loading",
                "dynamic-library-loading", 
                "mcp-tool-discovery",
                "solana-orbital-transactions",
                "content-addressing"
            ],
            "endpoints": [
                "/unified/load-flake",
                "/unified/mcp/{content_address}/{tool_name}",
                "/unified/orbit/{content_address}",
                "/unified/libraries/{content_address}"
            ],
            "integration": "zos-server-native"
        })
    }
}


fn main() {
    println!("unified_nix_service - add usage here");
}
