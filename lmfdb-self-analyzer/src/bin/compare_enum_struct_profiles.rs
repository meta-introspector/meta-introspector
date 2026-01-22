// Compare rustc self-profile between enum and struct compilation
// Auto-label features based on profile differences

use std::process::Command;
use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Rustc Profile Comparison: Enum vs Struct\n");
    
    // Create test files
    let enum_code = r#"
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}
fn main() {}
"#;
    
    let struct_code = r#"
struct MyStruct {
    field1: i32,
    field2: String,
}
fn main() {}
"#;
    
    fs::write("/tmp/test_enum.rs", enum_code)?;
    fs::write("/tmp/test_struct.rs", struct_code)?;
    
    // Compile with self-profile
    println!("📊 Compiling enum with self-profile...");
    Command::new("rustc")
        .args(&[
            "/tmp/test_enum.rs",
            "-Zself-profile=/tmp/enum_profile",
            "-Zself-profile-events=default",
        ])
        .output()?;
    
    println!("📊 Compiling struct with self-profile...");
    Command::new("rustc")
        .args(&[
            "/tmp/test_struct.rs",
            "-Zself-profile=/tmp/struct_profile",
            "-Zself-profile-events=default",
        ])
        .output()?;
    
    // Compile with perf
    // Use: crate::perf::record() - see src/perf/mod.rs
    Command::new("perf")
        .args(&[
            "record",
            "-o", "/tmp/enum_perf.data",
            "-g",
            "rustc", "/tmp/test_enum.rs",
        ])
        .output()?;
    
    // Use: crate::perf::record() - see src/perf/mod.rs
    Command::new("perf")
        .args(&[
            "record",
            "-o", "/tmp/struct_perf.data",
            "-g",
            "rustc", "/tmp/test_struct.rs",
        ])
        .output()?;
    
    println!("\n✅ Profiles generated:");
    println!("   Enum self-profile: /tmp/enum_profile.mm_profdata");
    println!("   Struct self-profile: /tmp/struct_profile.mm_profdata");
    println!("   Enum perf: /tmp/enum_perf.data");
    println!("   Struct perf: /tmp/struct_perf.data");
    
    println!("\n🏷️  Auto-labeling features from profile differences...");
    
    // Extract differences
    let enum_funcs = extract_perf_functions("/tmp/enum_perf.data")?;
    let struct_funcs = extract_perf_functions("/tmp/struct_perf.data")?;
    
    println!("\n📌 Enum-specific functions:");
    for func in enum_funcs.keys() {
        if !struct_funcs.contains_key(func) {
            println!("   - {}", func);
        }
    }
    
    println!("\n📌 Struct-specific functions:");
    for func in struct_funcs.keys() {
        if !enum_funcs.contains_key(func) {
            println!("   - {}", func);
        }
    }
    
    Ok(())
}

fn extract_perf_functions(perf_file: &str) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    let output = Command::new("perf")
        .args(&["script", "-i", perf_file])
        .output()?;
    
    let mut funcs = HashMap::new();
    let script = String::from_utf8_lossy(&output.stdout);
    
    for line in script.lines() {
        if line.contains("rustc") && line.contains("(") {
            if let Some(func) = line.split_whitespace().last() {
                *funcs.entry(func.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    Ok(funcs)
}
