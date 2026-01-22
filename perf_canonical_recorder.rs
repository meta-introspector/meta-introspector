// perf_canonical_recorder.rs
    // Use: crate::perf::record() - see src/perf/mod.rs
// Collects data in canonical JSON format for downstream analysis
// Can be used as both binary and library

use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// Public API for library usage (types defined below)
// pub use PerfSession;
// pub use SessionType;
// pub use PerfReport;
// pub use SymbolSample;

/// Record a perf session and return the report
pub fn record_session(
    session_type: SessionType,
    command: Vec<String>,
) -> Result<PerfReport, Box<dyn std::error::Error>> {
    let mut session = PerfSession::new(session_type, command, None);
    session.record()?;
    session.generate_report()
}

/// Record a perf session with custom options
pub fn record_session_with_options(
    session_type: SessionType,
    command: Vec<String>,
    probes: Option<Vec<String>>,
    timeout: Option<u64>,
) -> Result<PerfReport, Box<dyn std::error::Error>> {
    let mut session = PerfSession::new(session_type, command, timeout);
    if let Some(probes) = probes {
        session.add_probes(probes);
    }
    session.record()?;
    session.generate_report()
}

#[derive(Debug, Serialize, Deserialize)]
struct PerfSession {
    session_id: String,
    timestamp: u64,
    session_type: SessionType,
    command: Vec<String>,
    duration_secs: Option<u64>,
    output_dir: PathBuf,
    perf_data_path: PathBuf,
    perf_report_path: PathBuf,
    perf_json_path: PathBuf,
    status: SessionStatus,
    custom_probes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SessionType {
    NixBuild,
    RustcBuild,
    CargoTest,
    BinaryExec,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SessionStatus {
    Recording,
    Processing,
    Complete,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
struct PerfReport {
    session_id: String,
    timestamp: u64,
    total_samples: u64,
    top_symbols: Vec<SymbolSample>,
    binaries: Vec<String>,
    libraries: Vec<String>,
    raw_report_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SymbolSample {
    symbol: String,
    samples: u64,
    percentage: f64,
    binary: String,
}

impl PerfSession {
    fn new(session_type: SessionType, command: Vec<String>, duration: Option<u64>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let session_id = format!("perf_{}_{}", 
            match session_type {
                SessionType::NixBuild => "nix",
                SessionType::RustcBuild => "rustc",
                SessionType::CargoTest => "cargo",
                SessionType::BinaryExec => "binary",
                SessionType::Custom => "custom",
            },
            timestamp
        );
        
        let output_dir = PathBuf::from("data/perf_canonical");
        fs::create_dir_all(&output_dir).unwrap();
        
        let perf_data_path = output_dir.join(format!("{}.perf.data", session_id));
        let perf_report_path = output_dir.join(format!("{}_report.txt", session_id));
        let perf_json_path = output_dir.join(format!("{}.json", session_id));
        
        Self {
            session_id,
            timestamp,
            session_type,
            command,
            duration_secs: duration,
            output_dir,
            perf_data_path,
            perf_report_path,
            perf_json_path,
            status: SessionStatus::Recording,
            custom_probes: Vec::new(),
        }
    }
    
    /// Add custom probes (e.g., "probe_*")
    pub fn add_probes(&mut self, probes: Vec<String>) {
        self.custom_probes = probes;
    }
    
    fn record(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    // Use: crate::perf::record() - see src/perf/mod.rs
        println!("   Command: {:?}", self.command);
        println!("   Output: {}", self.perf_data_path.display());
        
    // Use: crate::perf::record() - see src/perf/mod.rs
        let mut perf_cmd = Command::new("perf");
        perf_cmd.arg("record")
            .arg("-F").arg("99")              // 99 Hz sampling
            .arg("-g")                         // Call graph
            .arg("--call-graph").arg("dwarf") // DWARF unwinding
            .arg("-o").arg(&self.perf_data_path);
        
        // Add events
        perf_cmd.arg("-e").arg("cycles,instructions,cache-references,cache-misses,branches,branch-misses");
        
        // Add command to profile
        perf_cmd.arg("--").args(&self.command);
        
        // Execute
        let status = perf_cmd.status()?;
        
        if !status.success() {
            self.status = SessionStatus::Failed;
    // Use: crate::perf::record() - see src/perf/mod.rs
        }
        
        self.status = SessionStatus::Processing;
        Ok(())
    }
    
    fn generate_report(&mut self) -> Result<PerfReport, Box<dyn std::error::Error>> {
        println!("📊 Generating perf report...");
        
        // Generate text report
        let report_output = Command::new("perf")
            .arg("report")
            .arg("-i").arg(&self.perf_data_path)
            .arg("--stdio")
            .arg("-n")
            .arg("--percent-limit").arg("0.01")
            .output()?;
        
        fs::write(&self.perf_report_path, &report_output.stdout)?;
        
        // Parse report
        let report_text = String::from_utf8_lossy(&report_output.stdout);
        let (total_samples, top_symbols) = self.parse_report(&report_text)?;
        
        // Extract binaries and libraries
        let (binaries, libraries) = self.extract_binaries_libraries()?;
        
        let report = PerfReport {
            session_id: self.session_id.clone(),
            timestamp: self.timestamp,
            total_samples,
            top_symbols,
            binaries,
            libraries,
            raw_report_path: self.perf_report_path.to_string_lossy().to_string(),
        };
        
        // Save JSON
        let json = serde_json::to_string_pretty(&report)?;
        fs::write(&self.perf_json_path, json)?;
        
        self.status = SessionStatus::Complete;
        
        println!("✅ Report saved: {}", self.perf_json_path.display());
        
        Ok(report)
    }
    
    fn parse_report(&self, report: &str) -> Result<(u64, Vec<SymbolSample>), Box<dyn std::error::Error>> {
        let mut total_samples = 0u64;
        let mut symbols = Vec::new();
        
        for line in report.lines() {
            // Parse lines like: "  12.34%  1234  binary  [.] symbol_name"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 && parts[0].ends_with('%') {
                let percentage = parts[0].trim_end_matches('%').parse::<f64>().unwrap_or(0.0);
                let samples = parts[1].parse::<u64>().unwrap_or(0);
                let binary = parts[2].to_string();
                let symbol = parts[4..].join(" ");
                
                total_samples += samples;
                
                symbols.push(SymbolSample {
                    symbol,
                    samples,
                    percentage,
                    binary,
                });
                
                if symbols.len() >= 100 {
                    break; // Top 100 symbols
                }
            }
        }
        
        Ok((total_samples, symbols))
    }
    
    fn extract_binaries_libraries(&self) -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
        let script_output = Command::new("perf")
            .arg("script")
            .arg("-i").arg(&self.perf_data_path)
            .output()?;
        
        let script_text = String::from_utf8_lossy(&script_output.stdout);
        
        let mut binaries = std::collections::HashSet::new();
        let mut libraries = std::collections::HashSet::new();
        
        for line in script_text.lines() {
            if line.contains("/nix/store/") || line.contains(".so") {
                if line.contains(".so") {
                    if let Some(path) = line.split_whitespace().find(|s| s.contains(".so")) {
                        libraries.insert(path.to_string());
                    }
                } else if let Some(path) = line.split_whitespace().find(|s| s.starts_with("/nix/store/")) {
                    binaries.insert(path.to_string());
                }
            }
        }
        
        Ok((
            binaries.into_iter().collect(),
            libraries.into_iter().collect(),
        ))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: perf_canonical_recorder <session_type> <command...>");
        eprintln!();
        eprintln!("Session types:");
        eprintln!("  nix       - Nix build");
        eprintln!("  rustc     - Rustc build");
        eprintln!("  cargo     - Cargo test/build");
        eprintln!("  binary    - Binary execution");
        eprintln!("  custom    - Custom command");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  perf_canonical_recorder nix nix build .#hello");
        eprintln!("  perf_canonical_recorder rustc rustc --version");
        eprintln!("  perf_canonical_recorder binary ./my_binary arg1 arg2");
        std::process::exit(1);
    }
    
    let session_type = match args[1].as_str() {
        "nix" => SessionType::NixBuild,
        "rustc" => SessionType::RustcBuild,
        "cargo" => SessionType::CargoTest,
        "binary" => SessionType::BinaryExec,
        "custom" => SessionType::Custom,
        _ => {
            eprintln!("Unknown session type: {}", args[1]);
            std::process::exit(1);
        }
    };
    
    let command = args[2..].to_vec();
    
    let mut session = PerfSession::new(session_type, command, None);
    
    // Record
    if let Err(e) = session.record() {
        eprintln!("❌ Recording failed: {}", e);
        std::process::exit(1);
    }
    
    // Generate report
    match session.generate_report() {
        Ok(report) => {
            println!();
            println!("📊 Session Summary:");
            println!("   Session ID: {}", report.session_id);
            println!("   Total samples: {}", report.total_samples);
            println!("   Top symbols: {}", report.top_symbols.len());
            println!("   Binaries: {}", report.binaries.len());
            println!("   Libraries: {}", report.libraries.len());
            println!();
            println!("   Data: {}", session.perf_data_path.display());
            println!("   Report: {}", session.perf_report_path.display());
            println!("   JSON: {}", session.perf_json_path.display());
        }
        Err(e) => {
            eprintln!("❌ Report generation failed: {}", e);
            std::process::exit(1);
        }
    }
}
