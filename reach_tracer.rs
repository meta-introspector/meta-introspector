use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ByteReach {
    output_offset: usize,
    input_offsets: Vec<usize>,
    instruction_addrs: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReachabilityMap {
    stage: String,
    input_hash: String,
    output_hash: String,
    input_size: usize,
    output_size: usize,
    byte_reaches: Vec<ByteReach>,
}

struct ReachabilityTracer {
    qemu_path: String,
    plugin_path: String,
    maps: Vec<ReachabilityMap>,
}

impl ReachabilityTracer {
    fn new() -> Self {
        Self {
            qemu_path: "qemu-x86_64".to_string(),
            plugin_path: "./libreachability.so".to_string(),
            maps: Vec::new(),
        }
    }

    fn hash_bytes(data: &[u8]) -> String {
        let hash = Sha256::digest(data);
        format!("{:02x}{:02x}{:02x}{:02x}", hash[0], hash[1], hash[2], hash[3])
    }

    fn parse_reachability(&self, trace_file: &str) -> Vec<ByteReach> {
        let mut reaches = Vec::new();
        
        if let Ok(content) = fs::read_to_string(trace_file) {
            let mut current_offset = None;
            let mut input_offsets = Vec::new();
            let mut insn_addrs = Vec::new();
            
            for line in content.lines() {
                if line.starts_with("Output byte ") {
                    // Save previous
                    if let Some(offset) = current_offset {
                        reaches.push(ByteReach {
                            output_offset: offset,
                            input_offsets: input_offsets.clone(),
                            instruction_addrs: insn_addrs.clone(),
                        });
                    }
                    
                    // Parse new offset
                    current_offset = line.split_whitespace()
                        .nth(2)
                        .and_then(|s| s.trim_end_matches(':').parse().ok());
                    input_offsets.clear();
                    insn_addrs.clear();
                    
                } else if line.contains("Input bytes:") {
                    input_offsets = line.split(':')
                        .nth(1)
                        .map(|s| s.split_whitespace()
                            .filter_map(|n| n.parse().ok())
                            .collect())
                        .unwrap_or_default();
                        
                } else if line.contains("Instructions:") {
                    insn_addrs = line.split(':')
                        .nth(1)
                        .map(|s| s.split_whitespace()
                            .filter_map(|n| {
                                if n.starts_with("0x") {
                                    u64::from_str_radix(&n[2..], 16).ok()
                                } else {
                                    None
                                }
                            })
                            .collect())
                        .unwrap_or_default();
                }
            }
            
            // Save last
            if let Some(offset) = current_offset {
                reaches.push(ByteReach {
                    output_offset: offset,
                    input_offsets,
                    instruction_addrs: insn_addrs,
                });
            }
        }
        
        reaches
    }

    fn trace_stage(&mut self, source: &Path, emit: &str, ext: &str, stage: &str) -> std::io::Result<()> {
        let base = source.file_stem().unwrap().to_str().unwrap();
        let output = format!("{}.{}", base, ext);
        let trace_file = format!("reach_{}.txt", stage);
        
        let input_data = fs::read(source)?;
        
        // Run rustc in QEMU with reachability plugin
        let status = Command::new(&self.qemu_path)
            .arg("-plugin")
            .arg(format!("{},output={}", self.plugin_path, trace_file))
            .arg("/usr/bin/rustc")
            .args(["--emit", emit, "-o", &output, source.to_str().unwrap()])
            .status()?;
        
        if status.success() && Path::new(&output).exists() {
            let output_data = fs::read(&output)?;
            let byte_reaches = self.parse_reachability(&trace_file);
            
            self.maps.push(ReachabilityMap {
                stage: stage.to_string(),
                input_hash: Self::hash_bytes(&input_data),
                output_hash: Self::hash_bytes(&output_data),
                input_size: input_data.len(),
                output_size: output_data.len(),
                byte_reaches,
            });
            
            println!("  ✓ Traced {} output bytes", output_data.len());
        }
        
        Ok(())
    }

    fn trace_compilation(&mut self, source: &Path) -> std::io::Result<()> {
        println!("\n[Reachability] Tracing MIR...");
        self.trace_stage(source, "mir", "mir", "mir")?;
        
        println!("\n[Reachability] Tracing LLVM IR...");
        self.trace_stage(source, "llvm-ir", "ll", "llvm_ir")?;
        
        println!("\n[Reachability] Tracing Assembly...");
        self.trace_stage(source, "asm", "s", "asm")?;
        
        println!("\n[Reachability] Tracing Object...");
        self.trace_stage(source, "obj", "o", "obj")?;
        
        Ok(())
    }

    fn report(&self) {
        println!("\n=== Byte Reachability Report ===\n");
        
        for map in &self.maps {
            println!("Stage: {}", map.stage);
            println!("  Input:  {} bytes (hash: {})", map.input_size, map.input_hash);
            println!("  Output: {} bytes (hash: {})", map.output_size, map.output_hash);
            println!("  Tracked: {} output bytes", map.byte_reaches.len());
            
            if !map.byte_reaches.is_empty() {
                let avg_inputs: f64 = map.byte_reaches.iter()
                    .map(|r| r.input_offsets.len())
                    .sum::<usize>() as f64 / map.byte_reaches.len() as f64;
                let avg_insns: f64 = map.byte_reaches.iter()
                    .map(|r| r.instruction_addrs.len())
                    .sum::<usize>() as f64 / map.byte_reaches.len() as f64;
                    
                println!("  Avg input bytes per output: {:.1}", avg_inputs);
                println!("  Avg instructions per output: {:.1}", avg_insns);
            }
            println!();
        }
    }

    fn report_byte(&self, stage: &str, offset: usize) {
        if let Some(map) = self.maps.iter().find(|m| m.stage == stage) {
            if let Some(reach) = map.byte_reaches.iter().find(|r| r.output_offset == offset) {
                println!("\n=== Output Byte {} in {} ===", offset, stage);
                println!("Input bytes ({}): {:?}", reach.input_offsets.len(), reach.input_offsets);
                println!("Instructions ({}): ", reach.instruction_addrs.len());
                for (i, addr) in reach.instruction_addrs.iter().take(20).enumerate() {
                    println!("  {}: 0x{:x}", i, addr);
                }
                if reach.instruction_addrs.len() > 20 {
                    println!("  ... {} more", reach.instruction_addrs.len() - 20);
                }
            }
        }
    }

    fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.maps)?;
        fs::write(path, json)?;
        println!("Saved reachability maps to {}", path);
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    fs::create_dir_all("reach_output")?;
    
    let sample = "reach_output/sample.rs";
    fs::write(sample, "fn main() { println!(\"Hello, world!\"); }\n")?;
    
    let mut tracer = ReachabilityTracer::new();
    tracer.trace_compilation(Path::new(sample))?;
    tracer.report();
    tracer.save_json("reach_output/reachability.json")?;
    
    // Example: show what contributed to byte 100 in LLVM IR
    tracer.report_byte("llvm_ir", 100);
    
    Ok(())
}
