use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transformation {
    name: String,
    stage: String,
    input_hash: String,
    output_hash: String,
    input_size: usize,
    output_size: usize,
    duration_ms: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TraceData {
    input_data: Vec<u8>,
    transformations: Vec<Transformation>,
}

impl TraceData {
    fn new() -> Self {
        Self {
            input_data: Vec::new(),
            transformations: Vec::new(),
        }
    }

    fn load_input(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        self.input_data = fs::read(path)?;
        Ok(())
    }

    fn hash_bytes(data: &[u8]) -> String {
        let hash = Sha256::digest(data);
        format!("{:02x}{:02x}{:02x}{:02x}", hash[0], hash[1], hash[2], hash[3])
    }

    fn run_rustc(&self, emit: &str, source: &Path, output: &str, _stage: &str) -> std::io::Result<(bool, f64)> {
        let start = Instant::now();
        let status = Command::new("rustc")
            .args(["--emit", emit, "-o", output, source.to_str().unwrap()])
            .status()?;
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        Ok((status.success(), duration))
    }

    fn trace_stage(&mut self, emit: &str, ext: &str, name: &str, stage: &str, source: &Path, base: &str) -> std::io::Result<()> {
        let output = format!("{}.{}", base, ext);
        let (success, duration) = self.run_rustc(emit, source, &output, stage)?;
        
        println!(" Status: {} ({:.2}ms)", if success { "✓" } else { "✗" }, duration);

        if Path::new(&output).exists() {
            let artifact = fs::read(&output)?;
            self.transformations.push(Transformation {
                name: name.to_string(),
                stage: stage.to_string(),
                input_hash: Self::hash_bytes(&self.input_data),
                output_hash: Self::hash_bytes(&artifact),
                input_size: self.input_data.len(),
                output_size: artifact.len(),
                duration_ms: duration,
            });
        }
        Ok(())
    }

    fn trace_rustc_stages(&mut self, source: impl AsRef<Path>) -> std::io::Result<()> {
        let source = source.as_ref();
        let base = source.file_stem().unwrap().to_str().unwrap();
        self.load_input(source)?;

        println!("\n[Stage 1] MIR generation...");
        self.trace_stage("mir", "mir", "source_to_mir", "mir_generation", source, base)?;

        println!("\n[Stage 2] LLVM IR generation...");
        self.trace_stage("llvm-ir", "ll", "source_to_llvm_ir", "llvm_ir_generation", source, base)?;

        println!("\n[Stage 3] Assembly generation...");
        self.trace_stage("asm", "s", "source_to_asm", "assembly_generation", source, base)?;

        println!("\n[Stage 4] Object generation...");
        self.trace_stage("obj", "o", "source_to_object", "object_generation", source, base)?;

        println!("\n[Stage 5] Full compilation...");
        let (success, duration) = self.run_rustc("link", source, base, "full_binary")?;
        println!(" Status: {} ({:.2}ms)", if success { "✓" } else { "✗" }, duration);

        if Path::new(base).exists() {
            let binary = fs::read(base)?;
            self.transformations.push(Transformation {
                name: "source_to_executable".to_string(),
                stage: "full_binary".to_string(),
                input_hash: Self::hash_bytes(&self.input_data),
                output_hash: Self::hash_bytes(&binary),
                input_size: self.input_data.len(),
                output_size: binary.len(),
                duration_ms: duration,
            });
        }

        Ok(())
    }

    fn report(&self) {
        println!("\n=== Rustc Compilation Trace Report ===\n");
        for t in &self.transformations {
            println!("Stage: {}", t.stage);
            println!("  Transformation: {}", t.name);
            println!("  Input:  {} bytes (hash: {})", t.input_size, t.input_hash);
            println!("  Output: {} bytes (hash: {})", t.output_size, t.output_hash);
            println!("  Duration: {:.2}ms", t.duration_ms);
            let ratio = if t.input_size > 0 { t.output_size as f64 / t.input_size as f64 } else { 0.0 };
            println!("  Size ratio: {:.2}x", ratio);
            println!();
        }
    }

    fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        println!("Saved trace data to {}", path);
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    fs::create_dir_all("trace_output")?;
    
    let sample = "trace_output/sample.rs";
    fs::write(sample, "fn main() { println!(\"Hello, world!\"); }\n")?;

    let mut tracer = TraceData::new();
    tracer.trace_rustc_stages(sample)?;
    tracer.report();
    tracer.save_json("trace_output/trace_data.json")?;

    Ok(())
}
