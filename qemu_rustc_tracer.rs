use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct QemuTrace {
    stage: String,
    instructions: u64,
    mem_read: u64,
    mem_write: u64,
    duration_ms: f64,
}

struct QemuRustcTracer {
    qemu_path: String,
    plugin_path: String,
    traces: Vec<QemuTrace>,
}

impl QemuRustcTracer {
    fn new() -> Self {
        Self {
            qemu_path: "qemu-x86_64".to_string(),
            plugin_path: "./librustc_trace.so".to_string(),
            traces: Vec::new(),
        }
    }

    fn run_rustc_in_qemu(&mut self, args: &[&str], stage: &str) -> std::io::Result<()> {
        let trace_file = format!("trace_{}.txt", stage);
        
        let start = std::time::Instant::now();
        
        let status = Command::new(&self.qemu_path)
            .arg("-plugin")
            .arg(format!("{},output={}", self.plugin_path, trace_file))
            .arg("/usr/bin/rustc")
            .args(args)
            .status()?;
        
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        
        if status.success() && Path::new(&trace_file).exists() {
            let content = fs::read_to_string(&trace_file)?;
            let trace = self.parse_trace(&content, stage, duration);
            self.traces.push(trace);
        }
        
        Ok(())
    }

    fn parse_trace(&self, content: &str, stage: &str, duration: f64) -> QemuTrace {
        let mut instructions = 0;
        let mut mem_read = 0;
        let mut mem_write = 0;
        
        for line in content.lines() {
            if line.contains("Instructions executed:") {
                instructions = line.split(':').nth(1)
                    .and_then(|s| s.trim().parse().ok())
                    .unwrap_or(0);
            } else if line.contains("Memory read:") {
                mem_read = line.split(':').nth(1)
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if line.contains("Memory written:") {
                mem_write = line.split(':').nth(1)
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            }
        }
        
        QemuTrace {
            stage: stage.to_string(),
            instructions,
            mem_read,
            mem_write,
            duration_ms: duration,
        }
    }

    fn trace_compilation(&mut self, source: &str) -> std::io::Result<()> {
        let base = Path::new(source).file_stem().unwrap().to_str().unwrap();
        
        println!("[QEMU] Tracing MIR generation...");
        self.run_rustc_in_qemu(&["--emit=mir", "-o", &format!("{}.mir", base), source], "mir")?;
        
        println!("[QEMU] Tracing LLVM IR generation...");
        self.run_rustc_in_qemu(&["--emit=llvm-ir", "-o", &format!("{}.ll", base), source], "llvm_ir")?;
        
        println!("[QEMU] Tracing assembly generation...");
        self.run_rustc_in_qemu(&["--emit=asm", "-o", &format!("{}.s", base), source], "asm")?;
        
        println!("[QEMU] Tracing object generation...");
        self.run_rustc_in_qemu(&["--emit=obj", "-o", &format!("{}.o", base), source], "obj")?;
        
        println!("[QEMU] Tracing full compilation...");
        self.run_rustc_in_qemu(&[source, "-o", base], "binary")?;
        
        Ok(())
    }

    fn report(&self) {
        println!("\n=== QEMU Rustc Trace Report ===\n");
        for trace in &self.traces {
            println!("Stage: {}", trace.stage);
            println!("  Instructions: {}", trace.instructions);
            println!("  Memory Read: {} bytes", trace.mem_read);
            println!("  Memory Write: {} bytes", trace.mem_write);
            println!("  Duration: {:.2}ms", trace.duration_ms);
            println!();
        }
    }

    fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.traces)?;
        fs::write(path, json)?;
        println!("Saved QEMU trace to {}", path);
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    fs::create_dir_all("qemu_trace_output")?;
    
    let sample = "qemu_trace_output/sample.rs";
    fs::write(sample, "fn main() { println!(\"Hello, world!\"); }\n")?;
    
    let mut tracer = QemuRustcTracer::new();
    tracer.trace_compilation(sample)?;
    tracer.report();
    tracer.save_json("qemu_trace_output/qemu_trace.json")?;
    
    Ok(())
}
