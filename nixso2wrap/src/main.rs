use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, warn, error};
use walkdir::WalkDir;

mod abi_extractor;
mod mcp_generator;
mod nix_scanner;
mod wrapper_generator;

#[derive(Parser)]
#[command(name = "nixso2wrap")]
#[command(about = "Generate MCP services and ABI interfaces for all Nix store shared libraries")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan Nix store for all .so files
    Scan {
        /// Nix store path (default: /nix/store)
        #[arg(short, long, default_value = "/nix/store")]
        store_path: PathBuf,
        /// Output directory for results
        #[arg(short, long, default_value = "./nixso_analysis")]
        output: PathBuf,
    },
    /// Extract ABI from specific library
    Extract {
        /// Path to shared library
        library: PathBuf,
        /// Output JSON file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Generate MCP service for library
    Mcp {
        /// ABI JSON file
        abi_file: PathBuf,
        /// Output MCP service directory
        #[arg(short, long, default_value = "./mcp_services")]
        output: PathBuf,
    },
    /// Generate dynamic wrapper
    Wrap {
        /// Library to wrap
        library: PathBuf,
        /// Output wrapper library
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Start MCP server for all wrapped libraries
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value = "8080")]
        port: u16,
        /// Directory containing MCP services
        #[arg(short, long, default_value = "./mcp_services")]
        services_dir: PathBuf,
    },
    /// Full pipeline: scan -> extract -> generate -> serve
    All {
        /// Nix store path
        #[arg(short, long, default_value = "/nix/store")]
        store_path: PathBuf,
        /// Output directory
        #[arg(short, long, default_value = "./nixso_complete")]
        output: PathBuf,
        /// Server port
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub symbols: Vec<SymbolInfo>,
    pub dependencies: Vec<String>,
    pub architecture: String,
    pub abi_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub name: String,
    pub demangled_name: Option<String>,
    pub symbol_type: SymbolType,
    pub address: u64,
    pub size: Option<u64>,
    pub signature: Option<FunctionSignature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SymbolType {
    Function,
    Object,
    Section,
    File,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub return_type: String,
    pub parameters: Vec<Parameter>,
    pub calling_convention: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Option<String>,
    pub param_type: String,
    pub is_pointer: bool,
    pub is_const: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NixStoreAnalysis {
    pub total_libraries: usize,
    pub total_symbols: usize,
    pub libraries: Vec<LibraryInfo>,
    pub symbol_index: HashMap<String, Vec<String>>, // symbol -> libraries
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scan { store_path, output } => {
            info!("🔍 Scanning Nix store: {:?}", store_path);
            let analysis = nix_scanner::scan_nix_store(&store_path).await?;
            
            std::fs::create_dir_all(&output)?;
            let output_file = output.join("nix_store_analysis.json");
            
            let json = serde_json::to_string_pretty(&analysis)?;
            std::fs::write(&output_file, json)?;
            
            info!("✅ Analysis complete: {} libraries, {} symbols", 
                analysis.total_libraries, analysis.total_symbols);
            info!("📄 Results saved to: {:?}", output_file);
        }
        
        Commands::Extract { library, output } => {
            info!("🔬 Extracting ABI from: {:?}", library);
            let lib_info = abi_extractor::extract_abi(&library).await?;
            
            let output_file = output.unwrap_or_else(|| {
                let mut path = library.clone();
                path.set_extension("abi.json");
                path
            });
            
            let json = serde_json::to_string_pretty(&lib_info)?;
            std::fs::write(&output_file, json)?;
            
            info!("✅ ABI extracted: {} symbols", lib_info.symbols.len());
            info!("📄 Saved to: {:?}", output_file);
        }
        
        Commands::Mcp { abi_file, output } => {
            info!("🚀 Generating MCP service from: {:?}", abi_file);
            let abi_json = std::fs::read_to_string(&abi_file)?;
            let lib_info: LibraryInfo = serde_json::from_str(&abi_json)?;
            
            std::fs::create_dir_all(&output)?;
            mcp_generator::generate_mcp_service(&lib_info, &output).await?;
            
            info!("✅ MCP service generated in: {:?}", output);
        }
        
        Commands::Wrap { library, output } => {
            info!("🔗 Generating wrapper for: {:?}", library);
            let wrapper_path = wrapper_generator::generate_wrapper(&library, output).await?;
            
            info!("✅ Wrapper generated: {:?}", wrapper_path);
        }
        
        Commands::Serve { port, services_dir } => {
            info!("🌐 Starting MCP server on port {}", port);
            info!("📁 Serving from: {:?}", services_dir);
            
            mcp_generator::start_mcp_server(port, &services_dir).await?;
        }
        
        Commands::All { store_path, output, port } => {
            info!("🚀 Running complete nixso2wrap pipeline");
            
            // Step 1: Scan
            info!("Step 1/4: Scanning Nix store...");
            let analysis = nix_scanner::scan_nix_store(&store_path).await?;
            
            std::fs::create_dir_all(&output)?;
            let analysis_file = output.join("analysis.json");
            std::fs::write(&analysis_file, serde_json::to_string_pretty(&analysis)?)?;
            
            // Step 2: Extract ABIs for top libraries
            info!("Step 2/4: Extracting ABIs...");
            let abi_dir = output.join("abis");
            std::fs::create_dir_all(&abi_dir)?;
            
            for (i, lib) in analysis.libraries.iter().take(100).enumerate() {
                if i % 10 == 0 {
                    info!("Processing library {}/{}: {}", i + 1, 100, lib.name);
                }
                
                let abi_file = abi_dir.join(format!("{}.abi.json", lib.name));
                if let Ok(lib_info) = abi_extractor::extract_abi(&lib.path).await {
                    let _ = std::fs::write(&abi_file, serde_json::to_string_pretty(&lib_info)?);
                }
            }
            
            // Step 3: Generate MCP services
            info!("Step 3/4: Generating MCP services...");
            let mcp_dir = output.join("mcp_services");
            std::fs::create_dir_all(&mcp_dir)?;
            
            for entry in std::fs::read_dir(&abi_dir)? {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    let abi_json = std::fs::read_to_string(entry.path())?;
                    if let Ok(lib_info) = serde_json::from_str::<LibraryInfo>(&abi_json) {
                        let _ = mcp_generator::generate_mcp_service(&lib_info, &mcp_dir).await;
                    }
                }
            }
            
            // Step 4: Start server
            info!("Step 4/4: Starting MCP server...");
            mcp_generator::start_mcp_server(port, &mcp_dir).await?;
        }
    }
    
    Ok(())
}
