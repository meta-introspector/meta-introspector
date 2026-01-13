use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;
use syn::{parse_file, visit::Visit, Item};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RealBuildOrderProcessor {
    build_order: Vec<String>,
    declarations: HashMap<String, Vec<Declaration>>,
    total_files: u32,
    total_declarations: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Declaration {
    decl_type: String,
    name: String,
    order_index: usize,
    file_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 REAL BUILD ORDER PROCESSOR");
    
    // Check for real rustc intercept data
    let intercept_file = "rustc_intercept_compression.json";
    if !std::path::Path::new(intercept_file).exists() {
        println!("❌ No rustc intercept data found!");
        println!("📋 To capture real build order:");
        println!("   cd /path/to/rust/project");
        println!("   export RUSTC=/mnt/data1/meta-introspector/rustc_interceptor.rs");
        println!("   cargo build --verbose");
        println!("   # This creates rustc_intercept_compression.json with real build order");
        return Ok(());
    }
    
    // Load real build order from rustc interceptor
    let intercept_data = std::fs::read_to_string(intercept_file)?;
    let intercept: serde_json::Value = serde_json::from_str(&intercept_data)?;
    
    let files_array = intercept["files"].as_array().unwrap();
    let build_order: Vec<String> = files_array.iter()
        .map(|entry| entry.as_array().unwrap()[0].as_str().unwrap().to_string())
        .collect();
    
    println!("📋 Real build order captured: {} files", build_order.len());
    
    // Load compressed archives
    let archives = [
        "/nix/store/x7wirg5c34zsgm7b5pvsl1hvq2dvqr9s-rust-src-1.92.0.tar.xz",
        "/nix/store/xp98ag7yvxjk13a3yan8qilb97wsavgy-rust-src-nightly.tar.xz"
    ];

    let mut file_contents: HashMap<String, String> = HashMap::new();
    
    // Load all files into memory
    for archive_path in &archives {
        println!("📦 Loading {}", archive_path);
        let file = File::open(archive_path)?;
        let reader = BufReader::new(file);
        let xz_decoder = xz2::read::XzDecoder::new(reader);
        let mut tar = tar::Archive::new(xz_decoder);

        for entry in tar.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.to_string_lossy().to_string();
            
            if path.ends_with(".rs") {
                let mut content = String::new();
                entry.read_to_string(&mut content)?;
                file_contents.insert(path, content);
            }
        }
    }
    
    println!("💾 Loaded {} Rust files into memory", file_contents.len());
    
    let mut result = RealBuildOrderProcessor {
        build_order: build_order.clone(),
        declarations: HashMap::new(),
        total_files: 0,
        total_declarations: 0,
    };
    
    // Process files in REAL build order
    for (order_index, file_path) in build_order.iter().enumerate() {
        // Find matching file in archive (by filename)
        let filename = std::path::Path::new(file_path).file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        let matching_files: Vec<_> = file_contents.keys()
            .filter(|path| path.ends_with(filename))
            .cloned()
            .collect();
        
        for archive_file in matching_files {
            if let Some(content) = file_contents.get(&archive_file) {
                if let Ok(file) = parse_file(content) {
                    let mut declarations = Vec::new();
                    
                    for item in &file.items {
                        let (decl_type, name) = match item {
                            Item::Fn(f) => ("fn", f.sig.ident.to_string()),
                            Item::Struct(s) => ("struct", s.ident.to_string()),
                            Item::Enum(e) => ("enum", e.ident.to_string()),
                            Item::Trait(t) => ("trait", t.ident.to_string()),
                            Item::Impl(_) => ("impl", format!("impl_{}", declarations.len())),
                            Item::Mod(m) => ("mod", m.ident.to_string()),
                            _ => ("other", format!("item_{}", declarations.len())),
                        };
                        
                        declarations.push(Declaration {
                            decl_type: decl_type.to_string(),
                            name,
                            order_index,
                            file_path: file_path.clone(),
                        });
                    }
                    
                    if !declarations.is_empty() {
                        let decl_count = declarations.len() as u32;
                        result.declarations.insert(archive_file.clone(), declarations);
                        result.total_files += 1;
                        result.total_declarations += decl_count;
                        
                        println!("📄 {} -> {} declarations (build order: {})", 
                                filename, decl_count, order_index);
                    }
                }
            }
        }
    }
    
    // Save results
    let json = serde_json::to_string_pretty(&result)?;
    std::fs::write("real_build_order_declarations.json", json)?;
    
    println!("\n✅ REAL BUILD ORDER PROCESSING COMPLETED:");
    println!("Files processed: {}", result.total_files);
    println!("Total declarations: {}", result.total_declarations);
    println!("Real build order steps: {}", result.build_order.len());
    println!("💾 Saved to: real_build_order_declarations.json");
    
    Ok(())
}
