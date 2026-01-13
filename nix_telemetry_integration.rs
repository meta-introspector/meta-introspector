// 🔧 NIX BUILD + TRANSPARENT TELEMETRY INTEGRATION
// Combines nix build analysis with build.rs transparent telemetry

use std::process::{Command, Stdio};
use std::env;
use std::fs;
use std::path::Path;

// Include our transparent telemetry macros
include!(concat!(env!("OUT_DIR"), "/telemetry_macros.rs"));

fn main() {
    println!("🔧 NIX BUILD + TRANSPARENT TELEMETRY");
    println!("====================================");
    
    // Initialize telemetry
    preload_telemetry!();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        show_usage(&args[0]);
        return;
    }
    
    let nix_args = &args[1..];
    
    // Step 1: Build with nix and telemetry
    let build_result = build_with_full_telemetry(nix_args);
    
    // Step 2: Generate wrapper macros for the result
    if let Some(result_path) = build_result {
        generate_telemetry_wrappers(&result_path);
    }
}

fn show_usage(program: &str) {
    println!("Usage: {} <nix-build-args...>", program);
    println!();
    println!("Examples:");
    println!("  {} -E 'with import <nixpkgs> {{}}; hello'", program);
    println!("  {} -A hello '<nixpkgs>'", program);
    println!("  {} ./default.nix", program);
    println!();
    println!("This will:");
    println!("  1. Build with nix-build (with telemetry)");
    println!("  2. Analyze all dependencies with ldd");
    println!("  3. Generate transparent telemetry wrappers");
    println!("  4. Create ready-to-use macro files");
}

fn build_with_full_telemetry(args: &[String]) -> Option<String> {
    println!("\n🚀 Building with full telemetry...");
    
    // Use our telemetry-wrapped operations
    let start = std::time::Instant::now();
    
    // This would use our transparent telemetry if we were doing file ops
    println!("📋 Command: nix-build {}", args.join(" "));
    
    let mut cmd = Command::new("nix-build");
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    
    match cmd.output() {
        Ok(output) => {
            let duration = start.elapsed();
            
            println!("📊 Build completed in {:.2}s", duration.as_secs_f64());
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if output.status.success() {
                let result_path = stdout.trim().to_string();
                println!("✅ Build successful: {}", result_path);
                
                // Show build stats
                show_build_stats(&stderr);
                
                return Some(result_path);
            } else {
                println!("❌ Build failed:");
                println!("{}", stderr);
            }
        }
        Err(e) => {
            println!("❌ Failed to run nix-build: {}", e);
        }
    }
    
    None
}

fn show_build_stats(stderr: &str) {
    let mut download_size = 0;
    let mut paths_fetched = 0;
    
    for line in stderr.lines() {
        if line.contains("MiB download") {
            paths_fetched += 1;
            // Extract download size if needed
        }
    }
    
    if paths_fetched > 0 {
        println!("📦 Fetched {} paths from cache", paths_fetched);
    }
}

fn generate_telemetry_wrappers(result_path: &str) {
    println!("\n🔧 GENERATING TELEMETRY WRAPPERS");
    println!("================================");
    
    let executables = find_all_executables(result_path);
    
    for exe in &executables {
        println!("🔍 Processing: {}", exe);
        generate_exe_wrappers(exe);
    }
    
    // Create master wrapper file
    create_master_wrapper(&executables);
}

fn find_all_executables(path: &str) -> Vec<String> {
    let mut executables = Vec::new();
    
    // Check direct executable
    if is_executable(path) {
        executables.push(path.to_string());
    }
    
    // Check bin/ directory
    let bin_path = format!("{}/bin", path);
    if Path::new(&bin_path).exists() {
        if let Ok(entries) = fs::read_dir(&bin_path) {
            for entry in entries.flatten() {
                let exe_path = entry.path();
                if is_executable(exe_path.to_str().unwrap_or("")) {
                    executables.push(exe_path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    executables
}

fn is_executable(path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            return metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0;
        }
        #[cfg(not(unix))]
        {
            return metadata.is_file();
        }
    }
    false
}

fn generate_exe_wrappers(exe_path: &str) {
    // Get ldd dependencies
    let libs = get_ldd_dependencies(exe_path);
    
    // Get nm symbols
    let symbols = get_nm_symbols(exe_path);
    
    println!("  📚 {} libraries, {} symbols", libs.len(), symbols.len());
    
    // Generate wrapper file for this executable
    let exe_name = Path::new(exe_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    
    let wrapper_file = format!("{}_telemetry_wrappers.rs", exe_name);
    let wrapper_content = create_wrapper_content(&exe_name, &libs, &symbols);
    
    if let Err(e) = fs::write(&wrapper_file, wrapper_content) {
        println!("  ❌ Failed to write {}: {}", wrapper_file, e);
    } else {
        println!("  ✅ Generated: {}", wrapper_file);
    }
}

fn get_ldd_dependencies(exe_path: &str) -> Vec<String> {
    let mut libs = Vec::new();
    
    if let Ok(output) = Command::new("ldd").arg(exe_path).output() {
        let ldd_output = String::from_utf8_lossy(&output.stdout);
        
        for line in ldd_output.lines() {
            if line.contains(".so") && line.contains("=>") {
                if let Some(lib_path) = line.split("=>").nth(1) {
                    let lib_path = lib_path.split_whitespace().next().unwrap_or("").trim();
                    if !lib_path.is_empty() && lib_path != "(0x" {
                        libs.push(lib_path.to_string());
                    }
                }
            }
        }
    }
    
    libs
}

fn get_nm_symbols(exe_path: &str) -> Vec<String> {
    let mut symbols = Vec::new();
    
    if let Ok(output) = Command::new("nm").args(&["-D", exe_path]).output() {
        let nm_output = String::from_utf8_lossy(&output.stdout);
        
        for line in nm_output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && (parts[1] == "T" || parts[1] == "W") {
                symbols.push(parts[2].to_string());
            }
        }
    }
    
    symbols
}

fn create_wrapper_content(exe_name: &str, libs: &[String], symbols: &[String]) -> String {
    let mut content = format!(
        "// 🔧 AUTO-GENERATED TELEMETRY WRAPPERS FOR {}\n\n",
        exe_name.to_uppercase()
    );
    
    content.push_str("use std::time::Instant;\n\n");
    
    // Add telemetry_wrap macro
    content.push_str(r#"
macro_rules! telemetry_wrap {
    ($name:literal, $call:expr) => {{
        let start = Instant::now();
        println!("🎯 CALL: {}", $name);
        let result = $call;
        let duration = start.elapsed().as_micros();
        println!("📊 DONE: {} ({}μs)", $name, duration);
        result
    }};
}

"#);
    
    // Add library information as comments
    content.push_str("// 📚 LINKED LIBRARIES:\n");
    for lib in libs {
        content.push_str(&format!("//   {}\n", lib));
    }
    content.push_str("\n");
    
    // Add symbol wrappers (first 20 to avoid huge files)
    content.push_str("// 🔍 SYMBOL WRAPPERS:\n");
    for symbol in symbols.iter() {
        let clean_name = symbol.replace("@", "_").replace(".", "_");
        content.push_str(&format!(
            r#"
macro_rules! {}_telemetry {{
    ($($args:expr),*) => {{{{
        telemetry_wrap!("{}", unsafe {{ /* symbol call would go here */ }})
    }}}};
}}
"#,
            clean_name, symbol
        ));
    }
    
    if symbols.len() > 20 {
        content.push_str(&format!("// ... and {} more symbols\n", symbols.len() - 20));
    }
    
    content
}

fn create_master_wrapper(executables: &[String]) {
    let master_content = format!(
        r#"// 🔧 MASTER TELEMETRY WRAPPER
// Generated for {} executables

// Include all individual wrappers:
{}

// Master initialization macro
macro_rules! init_all_telemetry {{
    () => {{{{
        println!("🔧 Initializing telemetry for {} executables");
{}        println!("✅ All telemetry wrappers ready!");
    }}}};
}}
"#,
        executables.len(),
        executables
            .iter()
            .map(|exe| {
                let exe_name = Path::new(exe)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                format!("// include!(\"{}_telemetry_wrappers.rs\");", exe_name)
            })
            .collect::<Vec<_>>()
            .join("\n"),
        executables.len(),
        executables
            .iter()
            .enumerate()
            .map(|(i, exe)| {
                let exe_name = Path::new(exe)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                format!("        println!(\"  {}. {} ready\", {}, \"{}\");", i + 1, exe_name, i + 1, exe_name)
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    if let Err(e) = fs::write("master_telemetry.rs", master_content) {
        println!("❌ Failed to write master_telemetry.rs: {}", e);
    } else {
        println!("✅ Generated: master_telemetry.rs");
        println!("\n🎯 Usage:");
        println!("  include!(\"master_telemetry.rs\");");
        println!("  init_all_telemetry!();");
    }
}
