use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ByteReach {
    output_offset: usize,
    input_offsets: Vec<usize>,
    instruction_addrs: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
struct ReachProfile {
    input_count: usize,
    insn_count: usize,
    insn_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InputCluster {
    profile: ReachProfile,
    input_bytes: Vec<usize>,
    output_bytes: Vec<usize>,
    example_insns: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Source2Test {
    clusters: Vec<InputCluster>,
    input_coverage: HashMap<usize, usize>,  // input_byte -> cluster_id
}

impl Source2Test {
    fn new() -> Self {
        Self {
            clusters: Vec::new(),
            input_coverage: HashMap::new(),
        }
    }

    fn profile_reach(&self, reach: &ByteReach) -> ReachProfile {
        let insn_hash = if reach.instruction_addrs.is_empty() {
            "none".to_string()
        } else {
            let mut hasher = Sha256::new();
            for addr in &reach.instruction_addrs {
                hasher.update(addr.to_le_bytes());
            }
            format!("{:x}", hasher.finalize())[..8].to_string()
        };

        ReachProfile {
            input_count: reach.input_offsets.len(),
            insn_count: reach.instruction_addrs.len(),
            insn_hash,
        }
    }

    fn cluster_by_profile(&mut self, reaches: &[ByteReach]) {
        let mut profile_map: HashMap<ReachProfile, Vec<usize>> = HashMap::new();

        for (idx, reach) in reaches.iter().enumerate() {
            let profile = self.profile_reach(reach);
            profile_map.entry(profile).or_default().push(idx);
        }

        for (profile, indices) in profile_map {
            let mut input_set = HashSet::new();
            let mut output_bytes = Vec::new();
            let mut example_insns = Vec::new();

            for &idx in &indices {
                let reach = &reaches[idx];
                output_bytes.push(reach.output_offset);
                
                for &input_off in &reach.input_offsets {
                    input_set.insert(input_off);
                }

                if example_insns.is_empty() {
                    example_insns = reach.instruction_addrs.clone();
                }
            }

            let input_bytes: Vec<usize> = input_set.into_iter().collect();
            let cluster_id = self.clusters.len();

            for &input_byte in &input_bytes {
                self.input_coverage.insert(input_byte, cluster_id);
            }

            self.clusters.push(InputCluster {
                profile,
                input_bytes,
                output_bytes,
                example_insns,
            });
        }
    }

    fn split_input_by_clusters(&self, input_data: &[u8], base_name: &str) -> std::io::Result<()> {
        fs::create_dir_all("source2test_splits")?;

        for (cluster_id, cluster) in self.clusters.iter().enumerate() {
            let mut split_data = Vec::new();
            
            for &byte_offset in &cluster.input_bytes {
                if byte_offset < input_data.len() {
                    split_data.push(input_data[byte_offset]);
                }
            }

            let filename = format!("source2test_splits/{}_{:03}_profile_{}.bin",
                base_name, cluster_id, cluster.profile.insn_hash);
            fs::write(&filename, &split_data)?;
            
            println!("  Cluster {}: {} input bytes → {} output bytes ({})",
                cluster_id, cluster.input_bytes.len(), 
                cluster.output_bytes.len(), filename);
        }

        Ok(())
    }

    fn generate_test_cases(&self, _base_name: &str) -> std::io::Result<()> {
        fs::create_dir_all("source2test_tests")?;

        for (cluster_id, cluster) in self.clusters.iter().enumerate() {
            let test_code = format!(
r#"// Auto-generated test for cluster {}
// Profile: {} inputs, {} instructions, hash {}

#[test]
fn test_cluster_{}() {{
    // Input bytes: {:?}
    // Output bytes: {:?}
    // Instructions: {:?}
    
    // TODO: Add assertions
}}
"#,
                cluster_id,
                cluster.profile.input_count,
                cluster.profile.insn_count,
                cluster.profile.insn_hash,
                cluster_id,
                &cluster.input_bytes[..cluster.input_bytes.len().min(10)],
                &cluster.output_bytes[..cluster.output_bytes.len().min(10)],
                &cluster.example_insns[..cluster.example_insns.len().min(5)]
            );

            let filename = format!("source2test_tests/test_cluster_{:03}.rs", cluster_id);
            fs::write(&filename, test_code)?;
        }

        Ok(())
    }

    fn report(&self) {
        println!("\n=== Source2Test Clustering Report ===\n");
        println!("Total clusters: {}", self.clusters.len());
        println!("Input bytes covered: {}\n", self.input_coverage.len());

        for (id, cluster) in self.clusters.iter().enumerate() {
            println!("Cluster {}:", id);
            println!("  Profile: {} inputs, {} insns, hash {}",
                cluster.profile.input_count,
                cluster.profile.insn_count,
                cluster.profile.insn_hash);
            println!("  Input bytes: {}", cluster.input_bytes.len());
            println!("  Output bytes: {}", cluster.output_bytes.len());
            println!();
        }
    }

    fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        println!("Saved clustering to {}", path);
        Ok(())
    }
}

fn parse_reachability(trace_file: &str) -> Vec<ByteReach> {
    let mut reaches = Vec::new();
    
    if let Ok(content) = fs::read_to_string(trace_file) {
        let mut current_offset = None;
        let mut input_offsets = Vec::new();
        let mut insn_addrs = Vec::new();
        
        for line in content.lines() {
            if line.starts_with("Output byte ") {
                if let Some(offset) = current_offset {
                    reaches.push(ByteReach {
                        output_offset: offset,
                        input_offsets: input_offsets.clone(),
                        instruction_addrs: insn_addrs.clone(),
                    });
                }
                
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

fn main() -> std::io::Result<()> {
    let sample = "source2test_sample.rs";
    fs::write(sample, r#"
fn add(a: i32, b: i32) -> i32 { a + b }
fn mul(a: i32, b: i32) -> i32 { a * b }
fn main() {
    println!("{}", add(1, 2));
    println!("{}", mul(3, 4));
}
"#)?;

    println!("Running reachability trace...");
    // Assume reach_tracer has already run and produced reach_llvm_ir.txt
    
    let trace_file = "reach_llvm_ir.txt";
    if !Path::new(trace_file).exists() {
        println!("Note: Run reach_tracer first to generate {}", trace_file);
        println!("Creating mock data for demonstration...");
        
        // Mock data
        let reaches = vec![
            ByteReach {
                output_offset: 0,
                input_offsets: vec![0, 1, 2],
                instruction_addrs: vec![0x400500, 0x400510],
            },
            ByteReach {
                output_offset: 1,
                input_offsets: vec![0, 1, 2],
                instruction_addrs: vec![0x400500, 0x400510],
            },
            ByteReach {
                output_offset: 100,
                input_offsets: vec![50, 51, 52],
                instruction_addrs: vec![0x400600, 0x400610, 0x400620],
            },
        ];
        
        let mut s2t = Source2Test::new();
        s2t.cluster_by_profile(&reaches);
        s2t.report();
        
        let input_data = fs::read(sample)?;
        s2t.split_input_by_clusters(&input_data, "sample")?;
        s2t.generate_test_cases("sample")?;
        s2t.save_json("source2test_clusters.json")?;
        
    } else {
        let reaches = parse_reachability(trace_file);
        println!("Parsed {} output bytes\n", reaches.len());
        
        let mut s2t = Source2Test::new();
        s2t.cluster_by_profile(&reaches);
        s2t.report();
        
        let input_data = fs::read(sample)?;
        s2t.split_input_by_clusters(&input_data, "sample")?;
        s2t.generate_test_cases("sample")?;
        s2t.save_json("source2test_clusters.json")?;
    }

    println!("\n✅ Source2Test complete!");
    println!("  • source2test_splits/ - Input byte splits by cluster");
    println!("  • source2test_tests/ - Generated test cases");
    println!("  • source2test_clusters.json - Full clustering data");

    Ok(())
}
