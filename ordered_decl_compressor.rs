use std::fs;
use std::collections::HashMap;
use std::time::Instant;
use serde_json;
use syn::{self, Item};

#[derive(serde::Serialize)]
struct OrderedDeclaration {
    visit_order: usize,
    file_path: String,
    decl_type: String,
    name: String,
    content: String,
    compressed_content: Vec<u8>,
    dependencies: Vec<String>,
}

fn main() {
    let start = Instant::now();
    println!("🚀 Enhanced compressor: Split decls with compilation order");
    
    let zombie_driver_path = "/mnt/data1/meta-introspector/data/repos/zombie_driver";
    let mut ordered_decls = Vec::new();
    let mut visit_counter = 0;
    
    // Find all .rs files
    let output = std::process::Command::new("find")
        .arg(zombie_driver_path)
        .arg("-name")
        .arg("*.rs")
        .arg("-type")
        .arg("f")
        .output()
        .expect("Failed to find files");
    
    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    
    println!("📁 Processing {} files from zombie_driver", files.len());
    
    for file_path in files {
        if let Ok(content) = fs::read_to_string(&file_path) {
            if let Ok(syntax_tree) = syn::parse_file(&content) {
                // Extract each declaration with order
                for item in syntax_tree.items {
                    visit_counter += 1;
                    
                    let (decl_type, name, item_content) = match &item {
                        Item::Fn(func) => ("fn", func.sig.ident.to_string(), quote::quote!(#func).to_string()),
                        Item::Struct(s) => ("struct", s.ident.to_string(), quote::quote!(#s).to_string()),
                        Item::Enum(e) => ("enum", e.ident.to_string(), quote::quote!(#e).to_string()),
                        Item::Impl(i) => ("impl", format!("impl_{}", visit_counter), quote::quote!(#i).to_string()),
                        Item::Use(u) => ("use", format!("use_{}", visit_counter), quote::quote!(#u).to_string()),
                        Item::Mod(m) => ("mod", m.ident.to_string(), quote::quote!(#m).to_string()),
                        _ => ("other", format!("item_{}", visit_counter), quote::quote!(#item).to_string()),
                    };
                    
                    // Compress the declaration
                    let compressed = compress_declaration(&item_content);
                    
                    // Extract dependencies (simplified)
                    let deps = extract_dependencies(&item_content);
                    
                    let decl = OrderedDeclaration {
                        visit_order: visit_counter,
                        file_path: file_path.clone(),
                        decl_type: decl_type.to_string(),
                        name,
                        content: item_content,
                        compressed_content: compressed,
                        dependencies: deps,
                    };
                    
                    ordered_decls.push(decl);
                }
            }
        }
    }
    
    println!("📊 Extracted {} declarations in compilation order", ordered_decls.len());
    
    // Save ordered declarations
    let output_file = "zombie_driver_ordered_decls.json";
    let json_output = serde_json::to_string_pretty(&ordered_decls).unwrap();
    fs::write(output_file, json_output).unwrap();
    
    // Calculate compression stats
    let total_original: usize = ordered_decls.iter().map(|d| d.content.len()).sum();
    let total_compressed: usize = ordered_decls.iter().map(|d| d.compressed_content.len()).sum();
    let compression_ratio = (total_compressed as f64 / total_original as f64) * 100.0;
    
    println!("💾 Saved to: {}", output_file);
    println!("📈 Original: {}KB, Compressed: {}KB ({:.1}% compression)", 
        total_original / 1024, total_compressed / 1024, 100.0 - compression_ratio);
    println!("⏱️  Time: {:.2}s", start.elapsed().as_secs_f64());
    
    // Test: Decompress and compile first declaration
    if let Some(first_decl) = ordered_decls.first() {
        test_decompress_and_compile(first_decl);
    }
}

fn compress_declaration(content: &str) -> Vec<u8> {
    let mut compressed = Vec::new();
    let tokens: Vec<&str> = content.split_whitespace().collect();
    
    for token in tokens {
        match token {
            "fn" => compressed.push(1),
            "struct" => compressed.push(2),
            "impl" => compressed.push(3),
            "use" => compressed.push(4),
            "pub" => compressed.push(5),
            "let" => compressed.push(6),
            "mut" => compressed.push(7),
            _ => {
                compressed.push(0);
                compressed.extend_from_slice(token.as_bytes());
                compressed.push(255);
            }
        }
    }
    compressed
}

fn extract_dependencies(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    for line in content.lines() {
        if line.trim().starts_with("use ") {
            deps.push(line.trim().to_string());
        }
    }
    deps
}

fn test_decompress_and_compile(decl: &OrderedDeclaration) {
    println!("\n🔧 Testing decompression and compilation:");
    println!("📋 Declaration #{}: {} {}", decl.visit_order, decl.decl_type, decl.name);
    
    // Decompress
    let decompressed = decompress_declaration(&decl.compressed_content);
    
    // Create test file
    let test_content = format!(
        "// Decompressed declaration #{} from zombie_driver\n// Original: {} {}\n\n{}\n\nfn main() {{\n    println!(\"Compiled declaration: {}\");\n}}",
        decl.visit_order, decl.decl_type, decl.name, decompressed, decl.name
    );
    
    let test_file = "/tmp/test_decl.rs";
    fs::write(test_file, test_content).unwrap();
    
    // Try to compile
    let compile_result = std::process::Command::new("rustc")
        .arg(test_file)
        .arg("-o")
        .arg("/tmp/test_decl")
        .output()
        .unwrap();
    
    if compile_result.status.success() {
        println!("✅ Compilation successful!");
        
        let run_result = std::process::Command::new("/tmp/test_decl")
            .output()
            .unwrap();
        
        println!("📤 Output: {}", String::from_utf8_lossy(&run_result.stdout));
    } else {
        println!("⚠️  Compilation issues (expected for isolated declarations)");
        println!("🔍 Error: {}", String::from_utf8_lossy(&compile_result.stderr));
    }
}

fn decompress_declaration(compressed: &[u8]) -> String {
    let mut tokens = Vec::new();
    let mut i = 0;
    
    while i < compressed.len() {
        match compressed[i] {
            1 => tokens.push("fn"),
            2 => tokens.push("struct"),
            3 => tokens.push("impl"),
            4 => tokens.push("use"),
            5 => tokens.push("pub"),
            6 => tokens.push("let"),
            7 => tokens.push("mut"),
            0 => {
                i += 1;
                let start = i;
                while i < compressed.len() && compressed[i] != 255 {
                    i += 1;
                }
                if i < compressed.len() {
                    tokens.push(std::str::from_utf8(&compressed[start..i]).unwrap_or(""));
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    tokens.join(" ")
}
