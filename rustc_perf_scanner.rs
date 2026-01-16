// Fast perf scanner for rustc: compress perf data to fingerprint per test

use std::process::Command;
use std::collections::HashSet;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

mod rand_shim;
use rand_shim::random_u64;

#[derive(Clone)]
pub struct PerfFingerprint {
    pub test_name: String,
    pub ips: HashSet<u64>,
    pub compressed_trace: Vec<u8>,
    pub compression_ratio: f64,
}

pub struct RustcPerfScanner {
    pub fingerprints: Vec<PerfFingerprint>,
}

impl RustcPerfScanner {
    pub fn new() -> Self {
        Self { fingerprints: Vec::new() }
    }
    
    pub fn scan_file(&mut self, path: &str) -> Option<PerfFingerprint> {
        let source = std::fs::read_to_string(path).ok()?;
        
        // Write to temp file
        let temp_path = format!("/tmp/rustc_perf_{}.rs", rand_shim::random_u64());
        std::fs::write(&temp_path, &source).ok()?;
        
        // Compile with perf
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("perf record -e cycles:u -o /tmp/perf.data rustc {} --crate-type lib 2>&1; perf script -i /tmp/perf.data 2>&1 | head -1000", temp_path))
            .output()
            .ok()?;
        
        let perf_output = String::from_utf8_lossy(&output.stdout);
        
        // Extract IPs from perf script
        let mut ips = HashSet::new();
        for line in perf_output.lines() {
            if let Some(ip_str) = line.split_whitespace().nth(2) {
                if let Ok(ip) = u64::from_str_radix(ip_str.trim_start_matches("0x"), 16) {
                    ips.insert(ip);
                }
            }
        }
        
        // Compress trace
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(perf_output.as_bytes()).ok()?;
        let compressed = encoder.finish().ok()?;
        
        let ratio = compressed.len() as f64 / perf_output.len() as f64;
        
        let fingerprint = PerfFingerprint {
            test_name: path.to_string(),
            ips: ips.clone(),
            compressed_trace: compressed,
            compression_ratio: ratio,
        };
        
        self.fingerprints.push(fingerprint.clone());
        
        // Cleanup
        let _ = std::fs::remove_file(&temp_path);
        let _ = std::fs::remove_file("/tmp/perf.data");
        
        Some(fingerprint)
    }
    
    pub fn scan_directory(&mut self, dir: &str, pattern: &str) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(path_str) = entry.path().to_str() {
                    if path_str.ends_with(pattern) {
                        if let Some(fp) = self.scan_file(path_str) {
                            println!("  ✓ {} - {} IPs, ratio {:.3}", 
                                     entry.file_name().to_string_lossy(),
                                     fp.ips.len(),
                                     fp.compression_ratio);
                        }
                    }
                }
            }
        }
    }
    
    pub fn report(&self) {
        println!("\n📊 Rustc Perf Scanner Report");
        println!("  Total fingerprints: {}", self.fingerprints.len());
        
        let total_ips: usize = self.fingerprints.iter()
            .map(|fp| fp.ips.len())
            .sum();
        
        let total_compressed: usize = self.fingerprints.iter()
            .map(|fp| fp.compressed_trace.len())
            .sum();
        
        println!("  Total unique IPs: {}", total_ips);
        println!("  Total compressed: {} bytes", total_compressed);
        println!("  Average IPs per test: {:.1}", 
                 total_ips as f64 / self.fingerprints.len() as f64);
    }
}

mod rand_shim;
