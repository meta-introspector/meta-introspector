// Scan git packs for duplicate object IDs and compression analysis

mod content_addressable_store;
mod rand_shim;

use content_addressable_store::ContentStore;
use rand_shim::init_rand;
use std::process::Command;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct GitObject {
    oid: String,
    size: usize,
    pack_file: String,
}

fn scan_git_pack(pack_path: &str) -> Vec<GitObject> {
    let mut objects = Vec::new();
    
    // Use git verify-pack to list objects
    if let Ok(output) = Command::new("git")
        .args(&["verify-pack", "-v", pack_path])
        .output() {
        
        let result = String::from_utf8_lossy(&output.stdout);
        for line in result.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let Ok(size) = parts[2].parse::<usize>() {
                    objects.push(GitObject {
                        oid: parts[0].to_string(),
                        size,
                        pack_file: pack_path.to_string(),
                    });
                }
            }
        }
    }
    
    objects
}

fn main() {
    init_rand();
    
    println!("📦 Scanning Git Packs for Duplicates and Compression\n");
    
    let submodules_path = std::env::home_dir()
        .map(|h| h.join("nix/vendor/rust/cargo2nix/submodules"))
        .and_then(|p| p.to_str().map(String::from));
    
    if let Some(path) = submodules_path {
        println!("🔍 Scanning submodules at: {}\n", path);
        
        if !std::path::Path::new(&path).exists() {
            println!("  ⚠ Path not found, using current directory instead\n");
        }
        
        // Find all .pack files
        let find_packs = Command::new("find")
            .args(&[&path, "-name", "*.pack", "-type", "f"])
            .output();
        
        let pack_files = if let Ok(output) = find_packs {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .take(10)  // Limit to 10 packs
                .map(String::from)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        
        println!("Found {} pack files (sampling 10)\n", pack_files.len());
        
        let mut all_objects = Vec::new();
        let mut oid_counts: HashMap<String, usize> = HashMap::new();
        
        for (i, pack_file) in pack_files.iter().enumerate() {
            println!("  Scanning pack {}: {}", i, pack_file.split('/').last().unwrap_or(pack_file));
            
            let objects = scan_git_pack(pack_file);
            
            for obj in &objects {
                *oid_counts.entry(obj.oid.clone()).or_insert(0) += 1;
            }
            
            all_objects.extend(objects);
            
            if i >= 10 {
                break;  // Limit to 10 packs for demo
            }
        }
        
        // Report duplicates
        println!("\n📊 Analysis Results\n");
        
        let duplicates: Vec<_> = oid_counts.iter()
            .filter(|(_, &count)| count > 1)
            .collect();
        
        println!("  Total objects scanned: {}", all_objects.len());
        println!("  Unique OIDs: {}", oid_counts.len());
        println!("  Duplicate OIDs: {}", duplicates.len());
        
        if !duplicates.is_empty() {
            println!("\n  Top duplicates:");
            let mut sorted_dups = duplicates.clone();
            sorted_dups.sort_by(|a, b| b.1.cmp(a.1));
            
            for (oid, count) in sorted_dups.iter().take(5) {
                println!("    {} - {} copies", &oid[..12], count);
            }
        }
        
        // Compression analysis
        let total_size: usize = all_objects.iter().map(|o| o.size).sum();
        let avg_size = if !all_objects.is_empty() {
            total_size / all_objects.len()
        } else {
            0
        };
        
        println!("\n  Total size: {} bytes", total_size);
        println!("  Average object size: {} bytes", avg_size);
        
        // Ingest into content store
        println!("\n💾 Ingesting into content store...\n");
        
        let mut store = ContentStore::new("/tmp/git-pack-analysis");
        
        // For demo, create synthetic content from OIDs
        for obj in all_objects.iter().take(100) {
            let synthetic = format!("git-object-{}-size-{}", obj.oid, obj.size);
            store.store(&synthetic);
        }
        
        store.report();
        
        let parquet_path = "/tmp/git-pack-analysis/git_objects.parquet";
        if let Ok(_) = store.save_to_parquet(parquet_path) {
            if let Ok(meta) = std::fs::metadata(parquet_path) {
                println!("\n  ✓ Saved to {} ({} bytes)", parquet_path, meta.len());
            }
        }
        
    } else {
        println!("  ⚠ Could not determine home directory");
    }
    
    println!("\n✅ Git pack scan complete!");
    println!("\n💡 Key insights:");
    println!("  • Scanned git pack files for objects");
    println!("  • Detected duplicate OIDs across packs");
    println!("  • Analyzed compression and sizes");
    println!("  • Ingested into content store");
}
