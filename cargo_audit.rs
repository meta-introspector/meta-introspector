use std::collections::{HashSet, HashMap};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("🔍 Cargo Audit: Checking all .rs files are buildable");
    
    // Find all .rs files
    let all_rs_files = find_all_rs_files(".");
    println!("📊 Found {} .rs files", all_rs_files.len());
    
    // Parse Cargo.toml to find declared bins
    let declared_bins = parse_cargo_toml("Cargo.toml");
    println!("📋 Declared {} bins in Cargo.toml", declared_bins.len());
    
    // Find .rs files not in Cargo.toml
    let mut missing = Vec::new();
    for rs_file in &all_rs_files {
        let stem = Path::new(rs_file).file_stem().unwrap().to_str().unwrap();
        if !declared_bins.contains(stem) && !rs_file.contains("/target/") {
            missing.push(rs_file.clone());
        }
    }
    
    println!("\n❌ Missing from Cargo.toml: {}", missing.len());
    for file in &missing {
        println!("  - {}", file);
    }
    
    // Try to build each declared bin
    println!("\n🔨 Testing builds...");
    let mut failed = HashMap::new();
    
    for bin in &declared_bins {
        print!("  Building {}... ", bin);
        let output = Command::new("cargo")
            .args(&["build", "--bin", bin, "--release"])
            .output()
            .expect("Failed to run cargo");
        
        if output.status.success() {
            println!("✅");
        } else {
            println!("❌");
            let stderr = String::from_utf8_lossy(&output.stderr);
            failed.insert(bin.clone(), stderr.to_string());
        }
    }
    
    // Report failures
    if !failed.is_empty() {
        println!("\n⚠️  Failed builds: {}", failed.len());
        fs::write("CARGO_AUDIT_FAILURES.txt", 
            failed.iter()
                .map(|(name, err)| format!("=== {} ===\n{}\n", name, err))
                .collect::<String>()
        ).unwrap();
        println!("📄 Details written to CARGO_AUDIT_FAILURES.txt");
    }
    
    // Generate quarantine report
    if !missing.is_empty() || !failed.is_empty() {
        generate_quarantine_report(&missing, &failed);
    }
    
    println!("\n✅ Audit complete");
}

fn find_all_rs_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && !path.to_str().unwrap().contains("target") {
                files.extend(find_all_rs_files(path.to_str().unwrap()));
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }
    files
}

fn parse_cargo_toml(path: &str) -> HashSet<String> {
    let content = fs::read_to_string(path).unwrap();
    let mut bins = HashSet::new();
    
    for line in content.lines() {
        if line.trim().starts_with("name = ") {
            if let Some(name) = line.split('"').nth(1) {
                bins.insert(name.to_string());
            }
        }
    }
    bins
}

fn generate_quarantine_report(missing: &[String], failed: &HashMap<String, String>) {
    let mut report = String::from("# Cargo Audit Quarantine Report\n\n");
    
    report.push_str("## Missing from Cargo.toml\n\n");
    for file in missing {
        report.push_str(&format!("- [ ] {}\n", file));
    }
    
    report.push_str("\n## Failed Builds\n\n");
    for (name, _) in failed {
        report.push_str(&format!("- [ ] {} - Move to quarantine submodule\n", name));
    }
    
    fs::write("CARGO_AUDIT_QUARANTINE.md", report).unwrap();
    println!("📋 Quarantine report: CARGO_AUDIT_QUARANTINE.md");
}
