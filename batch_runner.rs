use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct JobConfig {
    name: String,
    binary: String,
    args: Vec<String>,
    timeout_seconds: Option<u64>,
    output_file: Option<String>,
    depends_on: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchConfig {
    jobs: Vec<JobConfig>,
    max_parallel: u8,
    global_timeout_minutes: u32,
}

#[derive(Debug)]
struct JobResult {
    name: String,
    success: bool,
    duration_seconds: f64,
    output_size_bytes: u64,
    error_message: Option<String>,
}

struct BatchRunner {
    config: BatchConfig,
    results: Vec<JobResult>,
}

impl BatchRunner {
    fn from_config_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: BatchConfig = serde_json::from_str(&content)?;
        
        Ok(Self {
            config,
            results: Vec::new(),
        })
    }
    
    fn run_all_jobs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 BATCH RUNNER: {} jobs", self.config.jobs.len());
        
        let start_time = Instant::now();
        
        for job in &self.config.jobs {
            println!("\n▶️  Running: {}", job.name);
            let result = self.run_job(job);
            
            if result.success {
                println!("✅ {} completed in {:.2}s", job.name, result.duration_seconds);
            } else {
                println!("❌ {} failed: {:?}", job.name, result.error_message);
            }
            
            self.results.push(result);
        }
        
        let total_time = start_time.elapsed();
        self.print_summary(total_time);
        
        Ok(())
    }
    
    fn run_job(&self, job: &JobConfig) -> JobResult {
        let start_time = Instant::now();
        
        let mut cmd = Command::new("cargo");
        cmd.arg("run").arg("--bin").arg(&job.binary);
        
        for arg in &job.args {
            cmd.arg(arg);
        }
        
        // Handle output redirection
        let output = if let Some(output_file) = &job.output_file {
            match cmd.output() {
                Ok(output) => {
                    let _ = fs::write(output_file, &output.stdout);
                    output
                }
                Err(e) => {
                    return JobResult {
                        name: job.name.clone(),
                        success: false,
                        duration_seconds: start_time.elapsed().as_secs_f64(),
                        output_size_bytes: 0,
                        error_message: Some(e.to_string()),
                    };
                }
            }
        } else {
            match cmd.output() {
                Ok(output) => output,
                Err(e) => {
                    return JobResult {
                        name: job.name.clone(),
                        success: false,
                        duration_seconds: start_time.elapsed().as_secs_f64(),
                        output_size_bytes: 0,
                        error_message: Some(e.to_string()),
                    };
                }
            }
        };
        
        let output_size = if let Some(output_file) = &job.output_file {
            fs::metadata(output_file).map(|m| m.len()).unwrap_or(0)
        } else {
            output.stdout.len() as u64
        };
        
        JobResult {
            name: job.name.clone(),
            success: output.status.success(),
            duration_seconds: start_time.elapsed().as_secs_f64(),
            output_size_bytes: output_size,
            error_message: if output.status.success() {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            },
        }
    }
    
    fn print_summary(&self, total_time: std::time::Duration) {
        println!("\n📊 BATCH SUMMARY:");
        println!("Total time: {:.2} seconds", total_time.as_secs_f64());
        
        let successful = self.results.iter().filter(|r| r.success).count();
        let failed = self.results.len() - successful;
        
        println!("Jobs: {} successful, {} failed", successful, failed);
        
        let total_output_mb: f64 = self.results.iter()
            .map(|r| r.output_size_bytes as f64)
            .sum::<f64>() / 1_000_000.0;
        
        println!("Total output: {:.2} MB", total_output_mb);
        
        println!("\n📋 JOB DETAILS:");
        for result in &self.results {
            let status = if result.success { "✅" } else { "❌" };
            println!("{} {} ({:.2}s, {:.2}MB)", 
                status, 
                result.name, 
                result.duration_seconds,
                result.output_size_bytes as f64 / 1_000_000.0
            );
        }
    }
}

fn create_compression_batch() -> Result<(), Box<dyn std::error::Error>> {
    let batch = BatchConfig {
        jobs: vec![
            JobConfig {
                name: "Grammar Compression Test".to_string(),
                binary: "grammar_rust_compressor".to_string(),
                args: vec![],
                timeout_seconds: Some(60),
                output_file: Some("grammar_test.log".to_string()),
                depends_on: vec![],
            },
            JobConfig {
                name: "Prove Compression".to_string(),
                binary: "prove_compression".to_string(),
                args: vec![],
                timeout_seconds: Some(120),
                output_file: Some("compression_proof.log".to_string()),
                depends_on: vec![],
            },
            JobConfig {
                name: "Crossbeam Analysis".to_string(),
                binary: "crossbeam_rustc_analyzer_complete".to_string(),
                args: vec!["/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/rustc_ast".to_string()],
                timeout_seconds: Some(300),
                output_file: Some("crossbeam_analysis.log".to_string()),
                depends_on: vec![],
            },
        ],
        max_parallel: 3,
        global_timeout_minutes: 10,
    };
    
    let config_json = serde_json::to_string_pretty(&batch)?;
    fs::write("compression_batch.json", config_json)?;
    
    println!("📝 Created compression_batch.json");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: batch_runner <config.json> | --create-compression-batch");
        return Ok(());
    }
    
    if args[1] == "--create-compression-batch" {
        create_compression_batch()?;
        return Ok(());
    }
    
    let config_path = Path::new(&args[1]);
    let mut runner = BatchRunner::from_config_file(config_path)?;
    runner.run_all_jobs()?;
    
    Ok(())
}
