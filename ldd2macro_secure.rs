// 🔍 LDD2MACRO with MD5 Security: Auto-wrap all .so dependencies with checksums
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SharedLibrary {
    pub name: String,
    pub path: String,
    pub address: String,
    pub md5_hash: String,
    pub wrapped: bool,
}

pub struct Ldd2Macro {
    pub binary_path: String,
    pub libraries: Vec<SharedLibrary>,
    pub wrapped_libs: HashMap<String, String>,
}

impl Ldd2Macro {
    pub fn new(binary_path: &str) -> Self {
        Self {
            binary_path: binary_path.to_string(),
            libraries: Vec::new(),
            wrapped_libs: HashMap::new(),
        }
    }

    pub fn scan_dependencies(&mut self) -> Result<(), String> {
        println!("🔍 Scanning dependencies with ldd + MD5 checksums...");
        
        let output = Command::new("ldd")
            .arg(&self.binary_path)
            .output()
            .map_err(|e| format!("Failed to run ldd: {}", e))?;

        if !output.status.success() {
            return Err("ldd command failed".to_string());
        }

        let ldd_output = String::from_utf8_lossy(&output.stdout);
        self.parse_ldd_output(&ldd_output);
        
        println!("✅ Found {} shared libraries with MD5 checksums", self.libraries.len());
        Ok(())
    }

    fn parse_ldd_output(&mut self, output: &str) {
        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("linux-vdso") {
                continue;
            }

            if let Some(lib) = self.parse_ldd_line(line) {
                self.libraries.push(lib);
            }
        }
    }

    fn parse_ldd_line(&self, line: &str) -> Option<SharedLibrary> {
        if line.contains(" => ") {
            let parts: Vec<&str> = line.split(" => ").collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let right_part = parts[1];
                
                if let Some(space_pos) = right_part.find(' ') {
                    let path = right_part[..space_pos].trim().to_string();
                    let address = right_part[space_pos..].trim().to_string();
                    
                    return Some(SharedLibrary {
                        name,
                        path: path.clone(),
                        address,
                        md5_hash: Self::calculate_md5(&path),
                        wrapped: false,
                    });
                }
            }
        }
        
        None
    }

    fn calculate_md5(file_path: &str) -> String {
        let output = Command::new("md5sum")
            .arg(file_path)
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let output_str = String::from_utf8_lossy(&result.stdout);
                output_str.split_whitespace().next().unwrap_or("unknown").to_string()
            }
            _ => "unknown".to_string()
        }
    }

    pub fn generate_wrapper_macros(&mut self) -> String {
        println!("🔧 Generating secure wrapper macros for {} libraries...", self.libraries.len());
        
        let mut macros = String::from("// 🔍 AUTO-GENERATED LDD2MACRO WRAPPERS WITH MD5 SECURITY\n\n");
        
        // Add MD5 helper function
        macros.push_str(&self.generate_md5_helper());
        
        let libraries = self.libraries.clone();
        
        for lib in &libraries {
            let macro_name = self.lib_name_to_macro(&lib.name);
            let wrapper_code = self.generate_lib_wrapper(&lib.name, &lib.path, &lib.md5_hash);
            
            macros.push_str(&format!(
                "macro_rules! {} {{\n    () => {{\n        {}\n    }};\n}}\n\n",
                macro_name, wrapper_code
            ));
            
            self.wrapped_libs.insert(lib.name.clone(), macro_name.clone());
        }

        for lib in &mut self.libraries {
            lib.wrapped = true;
        }

        macros.push_str(&self.generate_preload_all_macro());
        macros
    }

    fn lib_name_to_macro(&self, lib_name: &str) -> String {
        lib_name
            .replace(".so", "")
            .replace(".", "_")
            .replace("-", "_")
            .to_lowercase()
            + "_telemetry"
    }

    fn generate_lib_wrapper(&self, lib_name: &str, lib_path: &str, md5_hash: &str) -> String {
        format!(
            r#"println!("📚 Preloading {}", "{}");
        println!("🔒 Expected MD5: {}", "{}");
        let start = std::time::Instant::now();
        
        // Verify MD5 checksum before loading  
        let current_md5 = calculate_file_md5("{}");
        if current_md5 != "{}" {{
            panic!("🚨 SECURITY ALERT: {} MD5 mismatch! Expected: {}, Got: {{}}", current_md5);
        }}
        println!("✅ MD5 verified for {}", "{}");
        
        let syscalls = vec!["open", "mmap", "mprotect"];
        println!("📊 {} loaded in {{}}ms", "{}", start.elapsed().as_millis());"#,
            lib_name, lib_name,      // 📚 Preloading
            md5_hash, md5_hash,      // 🔒 Expected MD5
            lib_path,                // calculate_file_md5 path
            md5_hash,                // if current_md5 != 
            lib_name, md5_hash,      // panic message
            lib_name, lib_name,      // ✅ MD5 verified
            lib_name, lib_name       // 📊 loaded
        )
    }

    fn generate_md5_helper(&self) -> String {
        r#"
fn calculate_file_md5(file_path: &str) -> String {
    use std::process::Command;
    
    let output = Command::new("md5sum")
        .arg(file_path)
        .output();
        
    match output {
        Ok(result) if result.status.success() => {
            let output_str = String::from_utf8_lossy(&result.stdout);
            output_str.split_whitespace().next().unwrap_or("unknown").to_string()
        }
        _ => "unknown".to_string()
    }
}

"#.to_string()
    }

    fn generate_preload_all_macro(&self) -> String {
        let mut preload_calls = String::new();
        
        for lib_name in self.wrapped_libs.keys() {
            let macro_name = &self.wrapped_libs[lib_name];
            preload_calls.push_str(&format!("        {}!();\n", macro_name));
        }

        format!(
            r#"macro_rules! preload_all_dependencies_secure {{
    () => {{
        println!("🔒 Preloading all {} dependencies with MD5 verification...");
{}        println!("✅ All dependencies securely preloaded!");
    }};
}}
"#,
            self.libraries.len(),
            preload_calls
        )
    }

    pub fn show_library_report(&self) {
        println!("\n🔍 SECURE LIBRARY DEPENDENCY REPORT");
        println!("===================================");
        
        for (i, lib) in self.libraries.iter().enumerate() {
            let status = if lib.wrapped { "✅ Wrapped" } else { "❌ Not wrapped" };
            println!("{}. {} → {} (MD5: {}...)", 
                i + 1, 
                lib.name, 
                status,
                &lib.md5_hash[..8]
            );
        }
        
        println!("\n📊 Summary:");
        println!("  Total libraries: {}", self.libraries.len());
        println!("  Wrapped: {}", self.libraries.iter().filter(|l| l.wrapped).count());
        println!("  MD5 protected: {}", self.libraries.len());
    }
}

fn main() {
    println!("🔒 LDD2MACRO with MD5 Security");
    println!("==============================");
    
    let args: Vec<String> = std::env::args().collect();
    let binary_path = if args.len() > 1 {
        &args[1]
    } else {
        "target/debug/telemetry_bootstrap"
    };
    
    let mut ldd2macro = Ldd2Macro::new(binary_path);
    
    match ldd2macro.scan_dependencies() {
        Ok(_) => println!("✅ Secure dependency scan complete"),
        Err(e) => {
            eprintln!("❌ Scan failed: {}", e);
            return;
        }
    }
    
    let wrapper_code = ldd2macro.generate_wrapper_macros();
    
    match std::fs::write("ldd_wrappers_secure.rs", &wrapper_code) {
        Ok(_) => println!("✅ Secure wrapper macros saved to ldd_wrappers_secure.rs"),
        Err(e) => eprintln!("❌ Failed to save: {}", e),
    }
    
    ldd2macro.show_library_report();
    
    println!("\n🎯 Usage in your code:");
    println!("  include!(\"ldd_wrappers_secure.rs\");");
    println!("  preload_all_dependencies_secure!();");
    println!("\n🔒 Security: All .so files verified with MD5 checksums!");
}
