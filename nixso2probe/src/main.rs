use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, warn, error};

mod probe_injector;
mod parquet_streamer;
mod perf_interface;
mod data_pipeline;
mod compression_engine;

#[derive(Parser)]
#[command(name = "nixso2probe")]
#[command(about = "Inject perf probes and stream structured telemetry to compressed Parquet")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inject probes into running processes
    Inject {
        /// Target process ID or name
        #[arg(short, long)]
        target: String,
        /// Probe configuration file
        #[arg(short, long, default_value = "probes.toml")]
        config: PathBuf,
        /// Output directory for Parquet streams
        #[arg(short, long, default_value = "./probe_data")]
        output: PathBuf,
    },
    /// Scan system for probeable interfaces
    Scan {
        /// Scan depth (1=processes, 2=libraries, 3=functions)
        #[arg(short, long, default_value = "2")]
        depth: u8,
        /// Output scan results
        #[arg(short, long, default_value = "./scan_results.json")]
        output: PathBuf,
    },
    /// Start streaming server
    Stream {
        /// Port to serve streaming data
        #[arg(short, long, default_value = "9090")]
        port: u16,
        /// Data directory
        #[arg(short, long, default_value = "./probe_data")]
        data_dir: PathBuf,
        /// Compression algorithm (zstd, lz4, gzip)
        #[arg(short, long, default_value = "zstd")]
        compression: String,
    },
    /// Generate probe configuration
    Generate {
        /// Target library or binary
        target: PathBuf,
        /// Output configuration file
        #[arg(short, long, default_value = "generated_probes.toml")]
        output: PathBuf,
    },
    /// Real-time monitoring dashboard
    Monitor {
        /// Data source (directory or stream URL)
        source: String,
        /// Dashboard port
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Full pipeline: scan -> inject -> stream
    All {
        /// Target process or system-wide
        #[arg(short, long, default_value = "system")]
        target: String,
        /// Output directory
        #[arg(short, long, default_value = "./nixso2probe_complete")]
        output: PathBuf,
        /// Stream port
        #[arg(short, long, default_value = "9090")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Inject { target, config, output } => {
            info!("🎯 Injecting probes into: {}", target);
            
            let probe_config = probe_injector::load_config(&config).await?;
            let mut injector = probe_injector::ProbeInjector::new(probe_config);
            
            // Set up Parquet streaming
            let streamer = parquet_streamer::ParquetStreamer::new(&output).await?;
            
            // Inject probes and start data collection
            injector.inject_into_target(&target).await?;
            injector.start_collection(streamer).await?;
            
            info!("✅ Probes active, streaming to: {:?}", output);
        }
        
        Commands::Scan { depth, output } => {
            info!("🔍 Scanning system for probeable interfaces (depth: {})", depth);
            
            let scanner = perf_interface::SystemScanner::new();
            let interfaces = scanner.scan_probeable_interfaces(depth).await?;
            
            let json = serde_json::to_string_pretty(&interfaces)?;
            tokio::fs::write(&output, json).await?;
            
            info!("✅ Found {} interfaces, saved to: {:?}", interfaces.len(), output);
        }
        
        Commands::Stream { port, data_dir, compression } => {
            info!("🌊 Starting streaming server on port {}", port);
            info!("📁 Data directory: {:?}", data_dir);
            info!("🗜️ Compression: {}", compression);
            
            let compression_engine = compression_engine::CompressionEngine::new(&compression)?;
            let pipeline = data_pipeline::StreamingPipeline::new(data_dir, compression_engine).await?;
            
            pipeline.start_server(port).await?;
        }
        
        Commands::Generate { target, output } => {
            info!("⚙️ Generating probe configuration for: {:?}", target);
            
            let generator = probe_injector::ConfigGenerator::new();
            let config = generator.analyze_and_generate(&target).await?;
            
            let toml = toml::to_string_pretty(&config)?;
            tokio::fs::write(&output, toml).await?;
            
            info!("✅ Configuration generated: {:?}", output);
        }
        
        Commands::Monitor { source, port } => {
            info!("📊 Starting monitoring dashboard on port {}", port);
            info!("📡 Data source: {}", source);
            
            let monitor = data_pipeline::RealtimeMonitor::new(&source).await?;
            monitor.start_dashboard(port).await?;
        }
        
        Commands::All { target, output, port } => {
            info!("🚀 Running complete nixso2probe pipeline");
            
            // Step 1: Scan system
            info!("Step 1/4: Scanning probeable interfaces...");
            let scanner = perf_interface::SystemScanner::new();
            let interfaces = scanner.scan_probeable_interfaces(3).await?;
            
            tokio::fs::create_dir_all(&output).await?;
            let scan_file = output.join("interfaces.json");
            tokio::fs::write(&scan_file, serde_json::to_string_pretty(&interfaces)?).await?;
            
            // Step 2: Generate optimal probe configuration
            info!("Step 2/4: Generating probe configuration...");
            let generator = probe_injector::ConfigGenerator::new();
            let config = if target == "system" {
                generator.generate_system_wide_config(&interfaces).await?
            } else {
                generator.analyze_and_generate(&PathBuf::from(&target)).await?
            };
            
            let config_file = output.join("probes.toml");
            tokio::fs::write(&config_file, toml::to_string_pretty(&config)?).await?;
            
            // Step 3: Inject probes and start collection
            info!("Step 3/4: Injecting probes...");
            let mut injector = probe_injector::ProbeInjector::new(config);
            let streamer = parquet_streamer::ParquetStreamer::new(&output.join("data")).await?;
            
            if target == "system" {
                injector.inject_system_wide().await?;
            } else {
                injector.inject_into_target(&target).await?;
            }
            
            // Step 4: Start streaming server
            info!("Step 4/4: Starting streaming server...");
            let compression_engine = compression_engine::CompressionEngine::new("zstd")?;
            let pipeline = data_pipeline::StreamingPipeline::new(
                output.join("data"), 
                compression_engine
            ).await?;
            
            // Start collection in background
            tokio::spawn(async move {
                if let Err(e) = injector.start_collection(streamer).await {
                    error!("Collection failed: {}", e);
                }
            });
            
            // Start streaming server
            pipeline.start_server(port).await?;
        }
    }
    
    Ok(())
}
