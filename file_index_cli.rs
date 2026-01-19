//! # File Index CLI
//! 
//! Command-line interface for querying the file index service.
//! Shell scripts use this instead of `find` and `grep`.
//!
//! ## Usage
//! 
//! ```bash
//! # Find Rust files
//! file-index query ext rs
//! 
//! # Find Cargo.toml files
//! file-index query name Cargo.toml
//! 
//! # Find files matching pattern
//! file-index query pattern "src/main"
//! 
//! # Get top priority files
//! file-index priority --limit 100
//! 
//! # Get statistics
//! file-index stats
//! ```

use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// File index CLI
#[derive(Parser)]
#[command(name = "file-index")]
#[command(about = "Query the file index service", long_about = None)]
struct Cli {
    /// Server URL
    #[arg(long, default_value = "http://127.0.0.1:3030")]
    server: String,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query files
    Query {
        /// Query type: ext, name, pattern
        #[arg(value_name = "TYPE")]
        query_type: String,
        
        /// Query value
        #[arg(value_name = "VALUE")]
        value: String,
        
        /// Output format: paths, json, count
        #[arg(short, long, default_value = "paths")]
        format: String,
    },
    
    /// Get top priority files
    Priority {
        /// Number of results
        #[arg(short, long, default_value = "100")]
        limit: usize,
        
        /// Output format
        #[arg(short, long, default_value = "paths")]
        format: String,
    },
    
    /// Get predicted queries
    Predict,
    
    /// Get index statistics
    Stats,
    
    /// Refresh index
    Refresh,
    
    /// Health check
    Health,
}

#[derive(Deserialize)]
struct Response<T> {
    success: bool,
    data: T,
    count: usize,
}

#[derive(Deserialize, Serialize)]
struct FileEntry {
    path: PathBuf,
    size: u64,
    modified: u64,
    extension: Option<String>,
    is_dir: bool,
    access_count: u64,
    last_accessed: u64,
    priority_score: f64,
}

#[derive(Deserialize, Serialize)]
struct IndexStats {
    total_files: usize,
    total_size: u64,
    total_queries: u64,
    unique_queries: usize,
    cache_hit_rate: f64,
}

fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    
    match cli.command {
        Commands::Query { query_type, value, format } => {
            let url = match query_type.as_str() {
                "ext" => format!("{}/query/ext/{}", cli.server, value),
                "name" => format!("{}/query/name/{}", cli.server, value),
                "pattern" => format!("{}/query/pattern?q={}", cli.server, value),
                _ => {
                    eprintln!("Invalid query type. Use: ext, name, or pattern");
                    std::process::exit(1);
                }
            };
            
            match client.get(&url).send() {
                Ok(response) => {
                    if let Ok(data) = response.json::<Response<Vec<FileEntry>>>() {
                        output_files(&data.data, &format);
                    } else {
                        eprintln!("Failed to parse response");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Request failed: {}", e);
                    eprintln!("Is the server running? Try: cargo run --bin file-index-server");
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Priority { limit, format } => {
            let url = format!("{}/priority?limit={}", cli.server, limit);
            
            match client.get(&url).send() {
                Ok(response) => {
                    if let Ok(data) = response.json::<Response<Vec<FileEntry>>>() {
                        output_files(&data.data, &format);
                    } else {
                        eprintln!("Failed to parse response");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Request failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Predict => {
            let url = format!("{}/predict", cli.server);
            
            match client.get(&url).send() {
                Ok(response) => {
                    if let Ok(data) = response.json::<Response<Vec<String>>>() {
                        println!("🔮 Predicted queries:");
                        for (i, query) in data.data.iter().enumerate() {
                            println!("  {}. {}", i + 1, query);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Request failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Stats => {
            let url = format!("{}/stats", cli.server);
            
            match client.get(&url).send() {
                Ok(response) => {
                    if let Ok(stats) = response.json::<IndexStats>() {
                        println!("📊 Index Statistics:");
                        println!("  Total files:    {}", stats.total_files);
                        println!("  Total size:     {} bytes", stats.total_size);
                        println!("  Total queries:  {}", stats.total_queries);
                        println!("  Unique queries: {}", stats.unique_queries);
                        println!("  Cache hit rate: {:.2}%", stats.cache_hit_rate * 100.0);
                    }
                }
                Err(e) => {
                    eprintln!("Request failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Refresh => {
            let url = format!("{}/refresh", cli.server);
            
            match client.post(&url).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("✅ Index refreshed successfully");
                    } else {
                        eprintln!("❌ Failed to refresh index");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Request failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Health => {
            let url = format!("{}/health", cli.server);
            
            match client.get(&url).send() {
                Ok(response) => {
                    if let Ok(health) = response.json::<serde_json::Value>() {
                        println!("{}", serde_json::to_string_pretty(&health).unwrap());
                    }
                }
                Err(e) => {
                    eprintln!("Request failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

/// Output files in specified format
fn output_files(files: &[FileEntry], format: &str) {
    match format {
        "paths" => {
            for file in files {
                println!("{}", file.path.display());
            }
        }
        "json" => {
            println!("{}", serde_json::to_string_pretty(files).unwrap());
        }
        "count" => {
            println!("{}", files.len());
        }
        "detailed" => {
            for file in files {
                println!("{} ({} bytes, score: {:.2})", 
                    file.path.display(), 
                    file.size,
                    file.priority_score
                );
            }
        }
        _ => {
            eprintln!("Invalid format. Use: paths, json, count, or detailed");
            std::process::exit(1);
        }
    }
}
