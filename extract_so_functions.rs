use goblin::elf::Elf;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_so_file>", args[0]);
        std::process::exit(1);
    }
    
    let so_path = &args[1];
    
    // Read the ELF file
    let buffer = fs::read(so_path)?;
    let elf = Elf::parse(&buffer)?;
    
    println!("// Generated extern \"C\" declarations from {}\n", so_path);
    
    // Iterate through dynamic symbols
    for sym in &elf.dynsyms {
        // Check if this is a function symbol (STT_FUNC = 2)
        // and is defined (not undefined/imported)
        if sym.st_type() == goblin::elf::sym::STT_FUNC && sym.st_shndx != 0 {
            if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                process_symbol(name);
            }
        }
    }
    
    // Also check regular symbol table if present
    for sym in &elf.syms {
        if sym.st_type() == goblin::elf::sym::STT_FUNC && sym.st_shndx != 0 {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                process_symbol(name);
            }
        }
    }
    
    Ok(())
}

fn process_symbol(mangled_name: &str) {
    // Skip empty names
    if mangled_name.is_empty() {
        return;
    }
    
    // Try to demangle Rust symbols first
    if mangled_name.starts_with("_ZN") && mangled_name.contains("rust") {
        let demangled = rustc_demangle::demangle(mangled_name).to_string();
        if demangled != mangled_name {
            let func_name = extract_function_name(&demangled);
            generate_hook_macro(&func_name, mangled_name, &demangled, "rust");
            return;
        }
    }
    
    // Try C++ demangling for _Z symbols
    if mangled_name.starts_with("_Z") {
        if let Ok(symbol) = cpp_demangle::Symbol::new(mangled_name) {
            let demangled = symbol.to_string();
            if demangled != mangled_name {
                let func_name = extract_cpp_function_name(&demangled);
                generate_hook_macro(&func_name, mangled_name, &demangled, "cpp");
                return;
            }
        }
    }
    
    // Common C functions we want to hook
    if is_common_c_function(mangled_name) {
        generate_hook_macro(mangled_name, mangled_name, mangled_name, "c");
    }
}

fn generate_hook_macro(func_name: &str, _original_name: &str, demangled: &str, lang: &str) {
    let safe_name = make_safe_identifier(func_name);
    
    println!("/// {}: {}", lang.to_uppercase(), demangled);
    
    // Generate redhook macro for common functions
    match func_name {
        "malloc" => {
            println!("hook! {{");
            println!("    unsafe fn malloc(size: libc::size_t) -> *mut libc::c_void => my_malloc {{");
            println!("        eprintln!(\"HOOK: malloc({{}})\", size);");
            println!("        real!(malloc)(size)");
            println!("    }}");
            println!("}}");
        },
        "free" => {
            println!("hook! {{");
            println!("    unsafe fn free(ptr: *mut libc::c_void) => my_free {{");
            println!("        eprintln!(\"HOOK: free({{:p}})\", ptr);");
            println!("        real!(free)(ptr)");
            println!("    }}");
            println!("}}");
        },
        "printf" => {
            println!("hook! {{");
            println!("    unsafe fn printf(format: *const libc::c_char) -> libc::c_int => my_printf {{");
            println!("        eprintln!(\"HOOK: printf called\");");
            println!("        real!(printf)(format)");
            println!("    }}");
            println!("}}");
        },
        _ => {
            // For other functions, just generate a counter
            println!("#[no_mangle]");
            println!("pub extern \"C\" fn {}_hook() {{", safe_name);
            println!("    eprintln!(\"HOOK: {} called\");", func_name);
            println!("}}");
        }
    }
    println!();
}

fn is_common_c_function(name: &str) -> bool {
    matches!(name, 
        "malloc" | "free" | "calloc" | "realloc" |
        "printf" | "fprintf" | "sprintf" | "snprintf" |
        "fopen" | "fclose" | "fread" | "fwrite" |
        "open" | "close" | "read" | "write" |
        "pthread_create" | "pthread_join" | "pthread_mutex_lock"
    )
}

fn extract_cpp_function_name(demangled: &str) -> String {
    // Extract function name from C++ demangled signature
    // e.g., "std::vector<int>::push_back(int const&)" -> "push_back"
    
    if let Some(paren_pos) = demangled.find('(') {
        let before_paren = &demangled[..paren_pos];
        if let Some(last_colon) = before_paren.rfind("::") {
            return before_paren[last_colon + 2..].to_string();
        }
        return before_paren.to_string();
    }
    
    // No parentheses, try to get last component
    if let Some(last_colon) = demangled.rfind("::") {
        return demangled[last_colon + 2..].to_string();
    }
    
    demangled.to_string()
}

fn extract_function_name(demangled: &str) -> String {
    // Handle Rust-style demangled names like "module::function"
    // or "crate::module::function"
    
    // First, remove any hash suffix like "::h1234567890abcdef"
    let without_hash = if let Some(pos) = demangled.rfind("::h") {
        // Check if what follows looks like a hash (16 hex chars)
        let after_h = &demangled[pos + 3..];
        if after_h.len() == 16 && after_h.chars().all(|c| c.is_ascii_hexdigit()) {
            &demangled[..pos]
        } else {
            demangled
        }
    } else {
        demangled
    };
    
    // Extract the last component
    if let Some(last_part) = without_hash.split("::").last() {
        // Remove any generic parameters like "<T>"
        if let Some(angle_pos) = last_part.find('<') {
            return last_part[..angle_pos].to_string();
        }
        
        // Remove function signature if present
        if let Some(paren_pos) = last_part.find('(') {
            return last_part[..paren_pos].to_string();
        }
        
        return last_part.to_string();
    }
    
    without_hash.to_string()
}

fn make_safe_identifier(name: &str) -> String {
    // Replace invalid characters with underscores
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}
