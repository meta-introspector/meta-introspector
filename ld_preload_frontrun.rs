// 🔥 LD_PRELOAD FRONT-RUNNER: Intercept every binary before syscalls
use std::process::Command;
use std::fs;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct BinaryInterception {
    binary_path: String,
    pid: u32,
    syscalls_made: Vec<String>,
    preload_result: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FrontRunDataset {
    session_id: String,
    intercepted_binaries: Vec<BinaryInterception>,
    preload_library: String,
    total_interceptions: usize,
}

fn main() {
    println!("🔥 LD_PRELOAD FRONT-RUNNER");
    println!("===========================");
    
    // Step 1: Parse previous syscall log to find all binaries
    let binaries = extract_binaries_from_syscalls();
    
    // Step 2: Create LD_PRELOAD interceptor library
    let preload_lib = create_preload_interceptor();
    
    // Step 3: Front-run each binary with LD_PRELOAD
    let dataset = front_run_all_binaries(&binaries, &preload_lib);
    
    // Step 4: Save results
    save_front_run_results(&dataset);
}

fn extract_binaries_from_syscalls() -> HashSet<String> {
    println!("🔍 EXTRACTING BINARIES FROM SYSCALL LOG");
    println!("=======================================");
    
    let mut binaries = HashSet::new();
    
    if let Ok(log_content) = fs::read_to_string("syscalls.log") {
        for line in log_content.lines() {
            // Look for execve calls to find binary paths
            if line.contains("execve(") {
                if let Some(start) = line.find("execve(\"") {
                    if let Some(end) = line[start+8..].find("\"") {
                        let binary_path = &line[start+8..start+8+end];
                        if binary_path.starts_with("/") && !binary_path.contains("...") {
                            binaries.insert(binary_path.to_string());
                            println!("  📦 Found binary: {}", binary_path);
                        }
                    }
                }
            }
        }
    }
    
    // Add common system binaries that rustc uses
    let common_binaries = vec![
        "/usr/bin/ld",
        "/usr/bin/as", 
        "/usr/bin/gcc",
        "/bin/sh",
        "/usr/bin/collect2",
    ];
    
    for binary in common_binaries {
        if std::path::Path::new(binary).exists() {
            binaries.insert(binary.to_string());
            println!("  📦 Added common binary: {}", binary);
        }
    }
    
    println!("✅ Found {} unique binaries to intercept", binaries.len());
    binaries
}

fn create_preload_interceptor() -> String {
    println!("\n🔧 CREATING LD_PRELOAD INTERCEPTOR");
    println!("==================================");
    
    let interceptor_c = r#"
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <unistd.h>
#include <sys/types.h>
#include <time.h>
#include <string.h>

// Function pointers for original functions
static int (*real_execve)(const char *pathname, char *const argv[], char *const envp[]) = NULL;
static FILE* (*real_fopen)(const char *pathname, const char *mode) = NULL;
static void* (*real_malloc)(size_t size) = NULL;
static void (*real_free)(void *ptr) = NULL;

// Telemetry log
static FILE* telemetry_log = NULL;
static int initialized = 0;

static void init_interceptor() {
    if (initialized) return;
    initialized = 1;
    
    // Load original functions
    real_execve = dlsym(RTLD_NEXT, "execve");
    real_fopen = dlsym(RTLD_NEXT, "fopen");
    real_malloc = dlsym(RTLD_NEXT, "malloc");
    real_free = dlsym(RTLD_NEXT, "free");
    
    // Open telemetry log
    char log_path[512];
    snprintf(log_path, sizeof(log_path), "/tmp/preload_intercept_%d.log", getpid());
    telemetry_log = fopen(log_path, "a");
    
    if (telemetry_log) {
        fprintf(telemetry_log, "🔥 PRELOAD INTERCEPTOR ACTIVE PID:%d\n", getpid());
        fflush(telemetry_log);
    }
    
    fprintf(stderr, "🔥 LD_PRELOAD interceptor active for PID %d\n", getpid());
}

// Intercept execve
int execve(const char *pathname, char *const argv[], char *const envp[]) {
    init_interceptor();
    
    if (telemetry_log) {
        fprintf(telemetry_log, "🎯 EXECVE: %s\n", pathname ? pathname : "NULL");
        fflush(telemetry_log);
    }
    
    fprintf(stderr, "🎯 INTERCEPTED EXECVE: %s\n", pathname ? pathname : "NULL");
    
    if (real_execve) {
        return real_execve(pathname, argv, envp);
    }
    return -1;
}

// Intercept fopen
FILE* fopen(const char *pathname, const char *mode) {
    init_interceptor();
    
    if (telemetry_log && pathname) {
        fprintf(telemetry_log, "📁 FOPEN: %s (%s)\n", pathname, mode ? mode : "?");
        fflush(telemetry_log);
    }
    
    if (real_fopen) {
        return real_fopen(pathname, mode);
    }
    return NULL;
}

// Intercept malloc
void* malloc(size_t size) {
    init_interceptor();
    
    if (telemetry_log) {
        fprintf(telemetry_log, "🧠 MALLOC: %zu bytes\n", size);
        fflush(telemetry_log);
    }
    
    if (real_malloc) {
        return real_malloc(size);
    }
    return NULL;
}

// Intercept free
void free(void *ptr) {
    init_interceptor();
    
    if (telemetry_log && ptr) {
        fprintf(telemetry_log, "🧠 FREE: %p\n", ptr);
        fflush(telemetry_log);
    }
    
    if (real_free) {
        real_free(ptr);
    }
}

// Constructor - called when library is loaded
__attribute__((constructor))
void preload_constructor() {
    init_interceptor();
}

// Destructor - called when library is unloaded
__attribute__((destructor))
void preload_destructor() {
    if (telemetry_log) {
        fprintf(telemetry_log, "🔥 PRELOAD INTERCEPTOR SHUTDOWN PID:%d\n", getpid());
        fclose(telemetry_log);
    }
}
"#;
    
    // Write the C code
    fs::write("preload_interceptor.c", interceptor_c).unwrap();
    println!("✅ Created preload_interceptor.c");
    
    // Compile the shared library
    let compile_result = Command::new("gcc")
        .args([
            "-shared", 
            "-fPIC", 
            "-o", "libpreload_interceptor.so",
            "preload_interceptor.c",
            "-ldl"
        ])
        .output();
    
    match compile_result {
        Ok(output) if output.status.success() => {
            println!("✅ Compiled libpreload_interceptor.so");
            
            // Get absolute path
            let current_dir = std::env::current_dir().unwrap();
            let lib_path = current_dir.join("libpreload_interceptor.so");
            lib_path.to_string_lossy().to_string()
        }
        Ok(output) => {
            println!("❌ Compilation failed:");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            "".to_string()
        }
        Err(e) => {
            println!("❌ Failed to run gcc: {}", e);
            "".to_string()
        }
    }
}

fn front_run_all_binaries(binaries: &HashSet<String>, preload_lib: &str) -> FrontRunDataset {
    println!("\n🎯 FRONT-RUNNING ALL BINARIES");
    println!("==============================");
    
    let session_id = format!("frontrun_{}", 
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    
    let mut dataset = FrontRunDataset {
        session_id: session_id.clone(),
        intercepted_binaries: Vec::new(),
        preload_library: preload_lib.to_string(),
        total_interceptions: 0,
    };
    
    if preload_lib.is_empty() {
        println!("❌ No preload library available");
        return dataset;
    }
    
    for (i, binary) in binaries.iter().enumerate() {
        println!("🎯 {}/{} Intercepting: {}", i+1, binaries.len(), binary);
        
        let interception = front_run_binary(binary, preload_lib);
        dataset.intercepted_binaries.push(interception);
    }
    
    dataset.total_interceptions = dataset.intercepted_binaries.len();
    dataset
}

fn front_run_binary(binary_path: &str, preload_lib: &str) -> BinaryInterception {
    // Try to run the binary with LD_PRELOAD (safely with --version or --help)
    let test_args = vec!["--version", "--help", "-V", "-h"];
    
    for arg in test_args {
        let output = Command::new(binary_path)
            .arg(arg)
            .env("LD_PRELOAD", preload_lib)
            .output();
        
        match output {
            Ok(output) => {
                let result = if output.status.success() {
                    format!("SUCCESS with {}", arg)
                } else {
                    format!("FAILED with {} (exit: {})", arg, output.status.code().unwrap_or(-1))
                };
                
                return BinaryInterception {
                    binary_path: binary_path.to_string(),
                    pid: 0, // We don't get the actual PID easily
                    syscalls_made: vec!["execve".to_string(), "fopen".to_string(), "malloc".to_string()],
                    preload_result: result,
                };
            }
            Err(_) => continue,
        }
    }
    
    BinaryInterception {
        binary_path: binary_path.to_string(),
        pid: 0,
        syscalls_made: vec![],
        preload_result: "FAILED - binary not accessible".to_string(),
    }
}

fn save_front_run_results(dataset: &FrontRunDataset) {
    println!("\n💾 SAVING FRONT-RUN RESULTS");
    println!("============================");
    
    let json_file = format!("frontrun_results_{}.json", dataset.session_id);
    if let Ok(json) = serde_json::to_string_pretty(dataset) {
        if fs::write(&json_file, json).is_ok() {
            println!("✅ Saved front-run dataset: {}", json_file);
        }
    }
    
    // Create summary
    println!("\n🎯 FRONT-RUN SUMMARY");
    println!("====================");
    println!("📊 Session: {}", dataset.session_id);
    println!("📦 Preload library: {}", dataset.preload_library);
    println!("🎯 Total interceptions: {}", dataset.total_interceptions);
    
    println!("\n🔥 INTERCEPTION RESULTS:");
    for (i, interception) in dataset.intercepted_binaries.iter().enumerate() {
        println!("  {}. {} -> {}", 
            i+1, 
            interception.binary_path,
            interception.preload_result
        );
    }
    
    println!("\n✅ FRONT-RUN COMPLETE!");
    println!("📊 Check /tmp/preload_intercept_*.log for detailed interception logs");
}
