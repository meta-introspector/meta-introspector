// build.rs - Scan ALL /nix/store with LMFDB harmonic filtering

use goblin::elf::Elf;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("🔬 Scanning /nix/store with LMFDB harmonic filtering");
    
    let filter = std::env::var("LMFDB_HARMONIC_FILTER").unwrap_or_else(|_| "all".to_string());
    let filter_percent = std::env::var("LMFDB_FILTER_PERCENT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    
    println!("🎵 Filter: {} (top {}%)", filter, filter_percent);
    
    let libs = find_all_nix_libs();
    println!("✅ Found {} libraries", libs.len());
    
    let mut all_functions = Vec::new();
    
    for (i, lib_path) in libs.iter().enumerate().take(50) {
        if let Ok(functions) = inspect_library(&lib_path) {
            if i % 10 == 0 {
                println!("📦 {} - {} functions", 
                    std::path::Path::new(&lib_path).file_name().unwrap().to_str().unwrap(),
                    functions.len());
            }
            
            for (name, pattern, args) in functions {
                let conductor = calculate_conductor(&name, &pattern, &args);
                all_functions.push((name, pattern, args, conductor));
            }
        }
    }
    
    println!("✅ Scanned {} functions", all_functions.len());
    
    // Deduplicate by name
    let mut seen = std::collections::HashSet::new();
    all_functions.retain(|(name, _, _, _)| seen.insert(name.clone()));
    
    // Sort by conductor and take top percent
    all_functions.sort_by(|a, b| b.3.cmp(&a.3));
    let count = (all_functions.len() * filter_percent / 100).max(1);
    all_functions.truncate(count);
    
    println!("🔬 Filtered to {} functions", all_functions.len());
    
    // Generate wrappers using universal wrapper
    let wrapper_code = std::fs::read_to_string("universal_wrapper.rs")
        .expect("universal_wrapper.rs not found");
    
    let mut wrappers = String::from("// LMFDB filtered wrappers - all call __universal_wrapper\n\n");
    wrappers.push_str(&wrapper_code);
    wrappers.push_str("\n\n// Generated wrappers\n\n");
    
    for (name, pattern, _args, conductor) in all_functions {
        let hex: String = pattern.iter().take(8).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
        let pattern_bytes: String = pattern.iter().take(16).map(|b| format!("0x{:02x}", b)).collect::<Vec<_>>().join(", ");
        
        wrappers.push_str(&format!(
            r#"
// {} - conductor:{} - {}
#[no_mangle]
pub unsafe extern "C" fn {}() -> u64 {{
    let real_fn = libc::dlsym(libc::RTLD_NEXT, b"{}\\0".as_ptr() as *const _);
    __universal_wrapper(
        real_fn,
        b"{}\\0".as_ptr(),
        [{}].as_ptr(),
        {},
    )
}}
"#,
            name, conductor, hex,
            name,
            name,
            name,
            pattern_bytes,
            conductor
        ));
    }
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::write(std::path::Path::new(&out_dir).join("instrumented_wrappers.rs"), wrappers).unwrap();
}

fn find_all_nix_libs() -> Vec<String> {
    std::process::Command::new("find")
        .args(&["/nix/store", "-maxdepth", "3", "-name", "*.so*"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines().map(|s| s.to_string()).collect())
        .unwrap_or_default()
}

fn inspect_library(path: &str) -> Result<Vec<(String, Vec<u8>, Vec<String>)>, Box<dyn std::error::Error>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    let mut functions = Vec::new();
    
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 && sym.st_size > 0 {
                let offset = sym.st_value as usize;
                let size = sym.st_size as usize;
                if offset < buffer.len() && offset + size <= buffer.len() {
                    let pattern = buffer[offset..offset + size.min(16)].to_vec();
                    functions.push((name.to_string(), pattern, vec![]));
                }
            }
        }
    }
    Ok(functions)
}

fn calculate_conductor(name: &str, pattern: &[u8], _args: &[String]) -> u32 {
    let base = 3000;
    let name_score = name.len() as u32 * 10;
    let pattern_score = pattern.iter().filter(|&&b| b != 0).count() as u32 * 100;
    let category = if name.contains("malloc") || name.contains("alloc") { 2000 }
        else if name.contains("read") || name.contains("write") { 800 }
        else if name.contains("crypt") || name.contains("hash") { 2000 }
        else { 0 };
    base + name_score + pattern_score + category
}
