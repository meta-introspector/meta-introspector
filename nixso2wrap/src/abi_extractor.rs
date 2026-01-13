use crate::{LibraryInfo, SymbolInfo, SymbolType, FunctionSignature, Parameter};
use anyhow::Result;
use std::path::Path;

pub async fn extract_abi(library_path: &Path) -> Result<LibraryInfo> {
    // This is a more detailed version of the analysis from nix_scanner
    // but focused on a single library with deeper ABI extraction
    
    use goblin::Object;
    use std::fs;
    
    let buffer = fs::read(library_path)?;
    let object = Object::parse(&buffer)?;
    
    let mut symbols = Vec::new();
    let mut dependencies = Vec::new();
    let mut architecture = "unknown".to_string();
    
    match object {
        Object::Elf(elf) => {
            architecture = format!("{:?}", elf.header.e_machine);
            
            // Extract all symbols (not just dynamic)
            for sym in elf.syms.iter() {
                if let Some(name) = elf.strtab.get_at(sym.st_name) {
                    if name.is_empty() {
                        continue;
                    }
                    
                    let symbol_type = match sym.st_type() {
                        goblin::elf::sym::STT_FUNC => SymbolType::Function,
                        goblin::elf::sym::STT_OBJECT => SymbolType::Object,
                        goblin::elf::sym::STT_SECTION => SymbolType::Section,
                        goblin::elf::sym::STT_FILE => SymbolType::File,
                        _ => SymbolType::Unknown,
                    };
                    
                    let demangled_name = demangle_symbol(name);
                    let signature = if matches!(symbol_type, SymbolType::Function) {
                        extract_function_signature(name, &demangled_name)
                    } else {
                        None
                    };
                    
                    symbols.push(SymbolInfo {
                        name: name.to_string(),
                        demangled_name,
                        symbol_type,
                        address: sym.st_value,
                        size: if sym.st_size > 0 { Some(sym.st_size) } else { None },
                        signature,
                    });
                }
            }
            
            // Also extract dynamic symbols
            for sym in elf.dynsyms.iter() {
                if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                    if name.is_empty() || symbols.iter().any(|s| s.name == name) {
                        continue;
                    }
                    
                    let symbol_type = match sym.st_type() {
                        goblin::elf::sym::STT_FUNC => SymbolType::Function,
                        goblin::elf::sym::STT_OBJECT => SymbolType::Object,
                        _ => SymbolType::Unknown,
                    };
                    
                    let demangled_name = demangle_symbol(name);
                    let signature = if matches!(symbol_type, SymbolType::Function) {
                        extract_function_signature(name, &demangled_name)
                    } else {
                        None
                    };
                    
                    symbols.push(SymbolInfo {
                        name: name.to_string(),
                        demangled_name,
                        symbol_type,
                        address: sym.st_value,
                        size: if sym.st_size > 0 { Some(sym.st_size) } else { None },
                        signature,
                    });
                }
            }
            
            // Extract dependencies
            if let Some(dynamic) = elf.dynamic {
                for dyn_entry in dynamic.dyns.iter() {
                    if dyn_entry.d_tag == goblin::elf::dynamic::DT_NEEDED {
                        if let Some(dep_name) = elf.dynstrtab.get_at(dyn_entry.d_val as usize) {
                            dependencies.push(dep_name.to_string());
                        }
                    }
                }
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Only ELF libraries supported for detailed ABI extraction"));
        }
    }
    
    // Sort symbols by address for better organization
    symbols.sort_by_key(|s| s.address);
    
    let metadata = fs::metadata(library_path)?;
    let name = library_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let abi_hash = generate_detailed_abi_hash(&symbols);
    
    Ok(LibraryInfo {
        path: library_path.to_path_buf(),
        name,
        size: metadata.len(),
        symbols,
        dependencies,
        architecture,
        abi_hash,
    })
}

fn demangle_symbol(name: &str) -> Option<String> {
    // Try C++ demangling first
    if name.starts_with("_Z") {
        if let Ok(symbol) = cpp_demangle::Symbol::new(name) {
            if let Ok(demangled) = symbol.demangle(&cpp_demangle::DemangleOptions::default()) {
                return Some(demangled);
            }
        }
    }
    
    // Try Rust demangling
    if name.starts_with("_R") || name.contains("rust") {
        let demangled = rustc_demangle::demangle(name).to_string();
        if demangled != name {
            return Some(demangled);
        }
    }
    
    None
}

fn extract_function_signature(mangled_name: &str, demangled_name: &Option<String>) -> Option<FunctionSignature> {
    // This is a simplified signature extraction
    // In a real implementation, you'd parse DWARF debug info or use more sophisticated tools
    
    if let Some(demangled) = demangled_name {
        return parse_demangled_signature(demangled);
    }
    
    // Fallback: try to infer from mangled name patterns
    parse_mangled_signature(mangled_name)
}

fn parse_demangled_signature(demangled: &str) -> Option<FunctionSignature> {
    // Simple regex-based parsing of demangled C++ signatures
    use regex::Regex;
    
    lazy_static::lazy_static! {
        static ref CPP_FUNC_REGEX: Regex = Regex::new(
            r"^(.+?)\s+(.+?)\((.*?)\)(?:\s+const)?$"
        ).unwrap();
        
        static ref RUST_FUNC_REGEX: Regex = Regex::new(
            r"^(.+?)::(.+?)(?:\((.*?)\))?$"
        ).unwrap();
    }
    
    // Try C++ pattern
    if let Some(captures) = CPP_FUNC_REGEX.captures(demangled) {
        let return_type = captures.get(1)?.as_str().trim().to_string();
        let _function_name = captures.get(2)?.as_str().trim();
        let params_str = captures.get(3)?.as_str().trim();
        
        let parameters = parse_parameters(params_str);
        
        return Some(FunctionSignature {
            return_type,
            parameters,
            calling_convention: Some("cdecl".to_string()),
        });
    }
    
    // Try Rust pattern (simplified)
    if let Some(captures) = RUST_FUNC_REGEX.captures(demangled) {
        let _namespace = captures.get(1)?.as_str();
        let _function_name = captures.get(2)?.as_str();
        let params_str = captures.get(3).map(|m| m.as_str()).unwrap_or("");
        
        let parameters = parse_parameters(params_str);
        
        return Some(FunctionSignature {
            return_type: "()".to_string(), // Rust functions often don't show return type in simple demangle
            parameters,
            calling_convention: Some("rust".to_string()),
        });
    }
    
    None
}

fn parse_mangled_signature(_mangled: &str) -> Option<FunctionSignature> {
    // TODO: Implement mangled name parsing for different ABIs
    None
}

fn parse_parameters(params_str: &str) -> Vec<Parameter> {
    if params_str.is_empty() || params_str == "void" {
        return Vec::new();
    }
    
    // Simple comma-split parsing (doesn't handle nested templates properly)
    params_str
        .split(',')
        .enumerate()
        .map(|(i, param)| {
            let param = param.trim();
            let is_pointer = param.contains('*') || param.contains('&');
            let is_const = param.starts_with("const ");
            
            let param_type = param
                .replace("const ", "")
                .replace('*', "")
                .replace('&', "")
                .trim()
                .to_string();
            
            Parameter {
                name: Some(format!("arg{}", i)),
                param_type,
                is_pointer,
                is_const,
            }
        })
        .collect()
}

fn generate_detailed_abi_hash(symbols: &[SymbolInfo]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    // Create a more detailed hash including signatures
    for symbol in symbols {
        symbol.name.hash(&mut hasher);
        symbol.symbol_type.hash(&mut hasher);
        
        if let Some(signature) = &symbol.signature {
            signature.return_type.hash(&mut hasher);
            for param in &signature.parameters {
                param.param_type.hash(&mut hasher);
                param.is_pointer.hash(&mut hasher);
                param.is_const.hash(&mut hasher);
            }
        }
        
        if let Some(size) = symbol.size {
            size.hash(&mut hasher);
        }
    }
    
    format!("{:016x}", hasher.finish())
}
