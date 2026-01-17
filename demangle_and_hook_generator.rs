// 🔥 DEMANGLE AND HOOK GENERATOR
// Extracts symbols from real .so files, demangles them, and generates LD_PRELOAD hooks

use goblin::elf::Elf;
use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct LibrarySymbol {
    name: String,
    demangled: Option<String>,
    is_rust: bool,
    is_function: bool,
}

fn main() {
    println!("🔥 DEMANGLE AND HOOK GENERATOR");
    println!("==============================");
    
    // Load real libraries from strace capture
    let libraries = load_real_libraries();
    println!("📚 Loaded {} real libraries from strace", libraries.len());
    
    let mut all_symbols = Vec::new();
    let mut processed_libs = 0;
    
    for lib_path in &libraries {
        if lib_path.ends_with(".so") || lib_path.ends_with(".so.6") {
            if let Some(symbols) = extract_symbols_from_library(lib_path) {
                println!("🎯 {}: {} symbols", lib_path, symbols.len());
                all_symbols.extend(symbols);
                processed_libs += 1;
            }
        }
    }
    
    println!("✅ Processed {} libraries, found {} total symbols", processed_libs, all_symbols.len());
    
    // Deduplicate symbols
    let mut unique_symbols: Vec<LibrarySymbol> = all_symbols.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    unique_symbols.sort_by(|a, b| a.name.cmp(&b.name));
    
    println!("🔧 {} unique symbols after deduplication", unique_symbols.len());
    
    // Generate LD_PRELOAD hooks
    generate_preload_hooks(&unique_symbols);
    
    println!("🚀 Generated LD_PRELOAD hooks in rust_preload_interceptor/src/generated_hooks.rs");
}

fn load_real_libraries() -> Vec<String> {
    let mut libraries = Vec::new();
    
    // Load from strace capture
    if let Ok(content) = fs::read_to_string("/mnt/data1/meta-introspector/data/build_analysis/real_build_1768332029_libraries.json") {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(libs) = data["libraries"].as_array() {
                for lib in libs {
                    if let Some(path) = lib.as_str() {
                        libraries.push(path.to_string());
                    }
                }
            }
        }
    }
    
    // Also load from ldd dependencies
    if let Ok(content) = fs::read_to_string("/mnt/data1/meta-introspector/data/build_analysis/real_build_1768332029_ldd_deps.json") {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(deps) = data["dependencies"].as_array() {
                for dep in deps {
                    if let Some(path) = dep.as_str() {
                        libraries.push(path.to_string());
                    }
                }
            }
        }
    }
    
    libraries.sort();
    libraries.dedup();
    libraries
}

fn extract_symbols_from_library(lib_path: &str) -> Option<Vec<LibrarySymbol>> {
    if !std::path::Path::new(lib_path).exists() {
        return None;
    }
    
    let buffer = fs::read(lib_path).ok()?;
    let elf = Elf::parse(&buffer).ok()?;
    let mut symbols = Vec::new();
    
    // Extract dynamic symbols (exported functions)
    for sym in &elf.dynsyms {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_bind() == goblin::elf::sym::STB_GLOBAL {
                let is_function = sym.st_type() == goblin::elf::sym::STT_FUNC;
                let is_rust = name.starts_with("_ZN") || name.contains("rust");
                
                let demangled = if is_rust {
                    // Try to demangle Rust symbol
                    rustc_demangle::demangle(name).to_string().into()
                } else {
                    None
                };
                
                symbols.push(LibrarySymbol {
                    name: name.to_string(),
                    demangled,
                    is_rust,
                    is_function,
                });
            }
        }
    }
    
    Some(symbols)
}

fn generate_preload_hooks(symbols: &[LibrarySymbol]) {
    let mut content = String::new();
    
    // Header
    content.push_str("// 🔥 GENERATED LD_PRELOAD HOOKS\n");
    content.push_str("// Auto-generated from real library symbols\n\n");
    content.push_str("use redhook::{hook, real};\n");
    content.push_str("use std::sync::atomic::{AtomicUsize, Ordering};\n");
    content.push_str("use std::os::raw::{c_char, c_int, c_void};\n");
    content.push_str("use libc::{size_t, FILE};\n\n");
    
    // Generate counters for each function
    let function_symbols: Vec<_> = symbols.iter()
        .filter(|s| s.is_function)
        .take(50) // Limit to first 50 functions to avoid overwhelming
        .collect();
    
    for symbol in &function_symbols {
        let counter_name = format!("{}_COUNT", symbol.name.to_uppercase().replace("@", "_"));
        content.push_str(&format!("static {}: AtomicUsize = AtomicUsize::new(0);\n", counter_name));
    }
    content.push('\n');
    
    // Generate hooks for common libc functions first
    generate_common_hooks(&mut content);
    
    // Generate hooks for Rust functions
    for symbol in &function_symbols {
        if symbol.is_rust {
            generate_rust_hook(&mut content, symbol);
        }
    }
    
    // Generate usage summary function
    content.push_str("\n#[no_mangle]\n");
    content.push_str("pub extern \"C\" fn print_hook_usage() {\n");
    content.push_str("    eprintln!(\"📊 HOOK USAGE SUMMARY:\");\n");
    
    for symbol in &function_symbols {
        let counter_name = format!("{}_COUNT", symbol.name.to_uppercase().replace("@", "_"));
        content.push_str(&format!("    let {} = {}.load(Ordering::SeqCst);\n", 
                                 symbol.name.replace("@", "_"), counter_name));
        content.push_str(&format!("    if {} > 0 {{ eprintln!(\"  {}: {{}} calls\", {}); }}\n", 
                                 symbol.name.replace("@", "_"), symbol.name, symbol.name.replace("@", "_")));
    }
    
    content.push_str("}\n");
    
    // Write to file
    fs::write("rust_preload_interceptor/src/generated_hooks.rs", content)
        .expect("Failed to write generated hooks");
}

fn generate_common_hooks(_content: &mut String) {
    // No hardcoded hooks - only use real symbols from goblin
}

fn generate_rust_hook(content: &mut String, symbol: &LibrarySymbol) {
    let safe_name = symbol.name.replace("@", "_").replace(".", "_");
    let counter_name = format!("{}_COUNT", symbol.name.to_uppercase().replace("@", "_"));
    
    // Generate a generic hook for Rust functions
    content.push_str(&"#[no_mangle]\n".to_string());
    content.push_str(&format!("pub extern \"C\" fn {} () {{\n", safe_name));
    content.push_str(&format!("    let count = {}.fetch_add(1, Ordering::SeqCst) + 1;\n", counter_name));
    
    if let Some(demangled) = &symbol.demangled {
        content.push_str(&format!("    eprintln!(\"RUST[{{}}]: {} (demangled: {}) called\", count);\n", 
                                 symbol.name, demangled));
    } else {
        content.push_str(&format!("    eprintln!(\"RUST[{{}}]: {} called\", count);\n", symbol.name));
    }
    
    content.push_str("}\n\n");
}

// Add missing counter declarations
static MALLOC_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static PRINTF_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl PartialEq for LibrarySymbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for LibrarySymbol {}

impl std::hash::Hash for LibrarySymbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
