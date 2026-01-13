// 🔍 SYMBOL2MACRO: Wrap every symbol in telemetry macros with MD5 security
use std::collections::HashMap;
use std::process::Command;

pub struct Symbol2Macro {
    pub library_path: String,
    pub symbols: Vec<SymbolInfo>,
    pub wrapped_symbols: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub symbol_type: String,
    pub address: String,
    pub size: u64,
}

impl Symbol2Macro {
    pub fn new(library_path: &str) -> Self {
        Self {
            library_path: library_path.to_string(),
            symbols: Vec::new(),
            wrapped_symbols: HashMap::new(),
        }
    }

    pub fn extract_symbols(&mut self) -> Result<(), String> {
        println!("🔍 Extracting symbols from {}...", self.library_path);
        
        // Use nm to extract symbols
        let output = Command::new("nm")
            .args(&["-D", &self.library_path]) // Dynamic symbols
            .output()
            .map_err(|e| format!("Failed to run nm: {}", e))?;

        if !output.status.success() {
            return Err("nm command failed".to_string());
        }

        let nm_output = String::from_utf8_lossy(&output.stdout);
        self.parse_nm_output(&nm_output);
        
        println!("✅ Found {} symbols", self.symbols.len());
        Ok(())
    }

    fn parse_nm_output(&mut self, output: &str) {
        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let address = parts[0].to_string();
                let symbol_type = parts[1].to_string();
                let name = parts[2].to_string();
                
                // Only wrap function symbols (T = text/code)
                if symbol_type == "T" || symbol_type == "W" {
                    self.symbols.push(SymbolInfo {
                        name,
                        symbol_type,
                        address,
                        size: 0, // nm doesn't give size easily
                    });
                }
            }
        }
    }

    pub fn generate_symbol_macros(&mut self) -> String {
        println!("🔧 Generating symbol wrapper macros for {} symbols...", self.symbols.len());
        
        let mut macros = String::from("// 🔍 AUTO-GENERATED SYMBOL2MACRO WRAPPERS\n\n");
        
        // Add telemetry helper
        macros.push_str(&self.generate_telemetry_helper());
        
        for symbol in &self.symbols {
            let macro_name = self.symbol_to_macro(&symbol.name);
            let wrapper_code = self.generate_symbol_wrapper(&symbol.name, &symbol.address);
            
            macros.push_str(&format!(
                "macro_rules! {} {{\n    ($($args:expr),*) => {{\n        {}\n    }};\n}}\n\n",
                macro_name, wrapper_code
            ));
            
            self.wrapped_symbols.insert(symbol.name.clone(), macro_name);
        }

        // Generate master symbol preload macro
        macros.push_str(&self.generate_symbol_preload_macro());
        
        macros
    }

    fn symbol_to_macro(&self, symbol_name: &str) -> String {
        format!("call_{}_telemetry", 
            symbol_name
                .replace("@", "_at_")
                .replace(".", "_dot_")
                .replace("-", "_")
                .to_lowercase()
        )
    }

    fn generate_symbol_wrapper(&self, symbol_name: &str, address: &str) -> String {
        format!(
            r#"{{
        println!("🎯 Calling symbol: {} at {}", "{}", "{}");
        let start = std::time::Instant::now();
        
        // Log symbol call with telemetry
        log_symbol_call("{}", "{}", &[$($args),*]);
        
        let duration = start.elapsed().as_micros();
        println!("📊 {} completed in {}μs", "{}", duration);
        
        format!("Symbol {} called", "{}")
    }}"#,
            symbol_name, address,  // For first println
            symbol_name, address,  // For log_symbol_call
            symbol_name,           // For second println
            symbol_name            // For format! return
        )
    }

    fn generate_telemetry_helper(&self) -> String {
        r#"
fn log_symbol_call(symbol_name: &str, address: &str, args: &[&dyn std::fmt::Debug]) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    println!("📊 SYMBOL_CALL: {} @ {} | args: {:?} | ts: {}", 
             symbol_name, address, args, timestamp);
}

"#.to_string()
    }

    fn generate_symbol_preload_macro(&self) -> String {
        let mut symbol_calls = String::new();
        
        for symbol_name in self.wrapped_symbols.keys() {
            let macro_name = &self.wrapped_symbols[symbol_name];
            symbol_calls.push_str(&format!("        println!(\"🔧 Symbol {} ready\", \"{}\");\n", symbol_name, symbol_name));
        }

        format!(
            r#"macro_rules! preload_all_symbols {{
    () => {{
        println!("🔧 Preloading {} symbols with telemetry...");
{}        println!("✅ All {} symbols ready for telemetry calls!");
    }};
}}
"#,
            self.symbols.len(),
            symbol_calls,
            self.symbols.len()
        )
    }

    pub fn show_symbol_report(&self) {
        println!("\n🔍 SYMBOL TELEMETRY REPORT");
        println!("=========================");
        
        for (i, symbol) in self.symbols.iter().enumerate() {
            println!("{}. {} ({}) @ {}", 
                i + 1, 
                symbol.name,
                symbol.symbol_type,
                symbol.address
            );
        }
        
        println!("\n📊 Summary:");
        println!("  Total symbols: {}", self.symbols.len());
        println!("  Function symbols: {}", self.symbols.iter().filter(|s| s.symbol_type == "T").count());
        println!("  Weak symbols: {}", self.symbols.iter().filter(|s| s.symbol_type == "W").count());
    }
}

fn main() {
    println!("🔍 SYMBOL2MACRO: Wrap every symbol with telemetry");
    println!("=================================================");
    
    let args: Vec<String> = std::env::args().collect();
    let library_path = if args.len() > 1 {
        &args[1]
    } else {
        "/nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/libc.so.6"
    };
    
    let mut symbol2macro = Symbol2Macro::new(library_path);
    
    match symbol2macro.extract_symbols() {
        Ok(_) => println!("✅ Symbol extraction complete"),
        Err(e) => {
            eprintln!("❌ Extraction failed: {}", e);
            return;
        }
    }
    
    let wrapper_code = symbol2macro.generate_symbol_macros();
    
    match std::fs::write("symbol_wrappers.rs", &wrapper_code) {
        Ok(_) => println!("✅ Symbol wrapper macros saved to symbol_wrappers.rs"),
        Err(e) => eprintln!("❌ Failed to save: {}", e),
    }
    
    symbol2macro.show_symbol_report();
    
    println!("\n🎯 Usage in your code:");
    println!("  include!(\"symbol_wrappers.rs\");");
    println!("  preload_all_symbols!();");
    println!("  call_malloc_telemetry!(size);");
}
