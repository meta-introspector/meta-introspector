use crate::{LibraryInfo, NixStoreAnalysis, SymbolInfo, SymbolType};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use tracing::{info, warn, debug};
use walkdir::WalkDir;

pub async fn scan_nix_store(store_path: &Path) -> Result<NixStoreAnalysis> {
    info!("🔍 Starting Nix store scan: {:?}", store_path);
    
    let mut libraries = Vec::new();
    let mut symbol_index: HashMap<String, Vec<String>> = HashMap::new();
    let mut dependency_graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut total_symbols = 0;
    
    // Walk through Nix store looking for .so files
    for entry in WalkDir::new(store_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Check if it's a shared library
        if is_shared_library(path) {
            debug!("Found library: {:?}", path);
            
            match analyze_library(path).await {
                Ok(lib_info) => {
                    // Update symbol index
                    for symbol in &lib_info.symbols {
                        symbol_index
                            .entry(symbol.name.clone())
                            .or_insert_with(Vec::new)
                            .push(lib_info.name.clone());
                    }
                    
                    // Update dependency graph
                    dependency_graph.insert(lib_info.name.clone(), lib_info.dependencies.clone());
                    
                    total_symbols += lib_info.symbols.len();
                    libraries.push(lib_info);
                    
                    if libraries.len() % 100 == 0 {
                        info!("Processed {} libraries...", libraries.len());
                    }
                }
                Err(e) => {
                    warn!("Failed to analyze {:?}: {}", path, e);
                }
            }
        }
    }
    
    info!("✅ Scan complete: {} libraries, {} symbols", libraries.len(), total_symbols);
    
    Ok(NixStoreAnalysis {
        total_libraries: libraries.len(),
        total_symbols,
        libraries,
        symbol_index,
        dependency_graph,
        analysis_timestamp: chrono::Utc::now(),
    })
}

fn is_shared_library(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if extension == "so" {
            return true;
        }
    }
    
    // Check for .so.X.Y.Z pattern
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if file_name.contains(".so.") || file_name.ends_with(".so") {
            return true;
        }
    }
    
    false
}

async fn analyze_library(path: &Path) -> Result<LibraryInfo> {
    use goblin::Object;
    use std::fs;
    
    let buffer = fs::read(path)?;
    let object = Object::parse(&buffer)?;
    
    let mut symbols = Vec::new();
    let mut dependencies = Vec::new();
    let mut architecture = "unknown".to_string();
    
    match object {
        Object::Elf(elf) => {
            architecture = format!("{:?}", elf.header.e_machine);
            
            // Extract dynamic symbols
            for sym in elf.dynsyms.iter() {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    let symbol_type = match sym.st_type() {
                        goblin::elf::sym::STT_FUNC => SymbolType::Function,
                        goblin::elf::sym::STT_OBJECT => SymbolType::Object,
                        goblin::elf::sym::STT_SECTION => SymbolType::Section,
                        goblin::elf::sym::STT_FILE => SymbolType::File,
                        _ => SymbolType::Unknown,
                    };
                    
                    let demangled_name = if name.starts_with("_Z") {
                        cpp_demangle::Symbol::new(name)
                            .ok()
                            .and_then(|s| s.demangle(&cpp_demangle::DemangleOptions::default()).ok())
                    } else if name.starts_with("_R") {
                        rustc_demangle::demangle(name).to_string().into()
                    } else {
                        None
                    };
                    
                    symbols.push(SymbolInfo {
                        name: name.to_string(),
                        demangled_name,
                        symbol_type,
                        address: sym.st_value,
                        size: if sym.st_size > 0 { Some(sym.st_size) } else { None },
                        signature: None, // TODO: Extract from debug info
                    });
                }
            }
            
            // Extract dependencies
            for dyn_entry in &elf.dynamic {
                if let Some(goblin::elf::dynamic::Dyn { d_tag: goblin::elf::dynamic::DT_NEEDED, d_val }) = dyn_entry {
                    if let Some(dep_name) = elf.dynstrtab.get_at(*d_val as usize) {
                        dependencies.push(dep_name.to_string());
                    }
                }
            }
        }
        Object::PE(pe) => {
            architecture = "PE".to_string();
            
            // Extract PE symbols
            for export in &pe.exports {
                if let Some(name) = export.name {
                    symbols.push(SymbolInfo {
                        name: name.to_string(),
                        demangled_name: None,
                        symbol_type: SymbolType::Function,
                        address: export.rva as u64,
                        size: None,
                        signature: None,
                    });
                }
            }
        }
        Object::Mach(mach) => {
            architecture = "Mach-O".to_string();
            // TODO: Extract Mach-O symbols
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported object format"));
        }
    }
    
    let metadata = fs::metadata(path)?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Generate ABI hash
    let abi_hash = generate_abi_hash(&symbols);
    
    Ok(LibraryInfo {
        path: path.to_path_buf(),
        name,
        size: metadata.len(),
        symbols,
        dependencies,
        architecture,
        abi_hash,
    })
}

fn generate_abi_hash(symbols: &[SymbolInfo]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    // Create a stable hash of the ABI
    for symbol in symbols {
        symbol.name.hash(&mut hasher);
        symbol.symbol_type.hash(&mut hasher);
        if let Some(size) = symbol.size {
            size.hash(&mut hasher);
        }
    }
    
    format!("{:x}", hasher.finish())
}

impl std::hash::Hash for SymbolType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
