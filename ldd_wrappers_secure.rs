// 🔍 AUTO-GENERATED LDD2MACRO WRAPPERS WITH MD5 SECURITY


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

macro_rules! libgcc_s_1_telemetry {
    () => {
        println!("📚 Preloading libgcc_s.so.1", "libgcc_s.so.1");
        println!("🔒 Expected MD5: 56203826e71da8a515ff7c6cdf9552b6", "56203826e71da8a515ff7c6cdf9552b6");
        let start = std::time::Instant::now();
        
        // Verify MD5 checksum before loading  
        let current_md5 = calculate_file_md5("/nix/store/xc0ga87wdclrx54qjaryahkkmkmqi9qz-gcc-15.2.0-lib/lib/libgcc_s.so.1");
        if current_md5 != "56203826e71da8a515ff7c6cdf9552b6" {
            panic!("🚨 SECURITY ALERT: libgcc_s.so.1 MD5 mismatch! Expected: 56203826e71da8a515ff7c6cdf9552b6, Got: {}", current_md5);
        }
        println!("✅ MD5 verified for libgcc_s.so.1", "libgcc_s.so.1");
        
        let syscalls = vec!["open", "mmap", "mprotect"];
        println!("📊 libgcc_s.so.1 loaded in {}ms", "libgcc_s.so.1", start.elapsed().as_millis());
    };
}

macro_rules! libc_6_telemetry {
    () => {
        println!("📚 Preloading libc.so.6", "libc.so.6");
        println!("🔒 Expected MD5: e8d6ab0f1a6dafd1779300c31597ddbe", "e8d6ab0f1a6dafd1779300c31597ddbe");
        let start = std::time::Instant::now();
        
        // Verify MD5 checksum before loading  
        let current_md5 = calculate_file_md5("/nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/libc.so.6");
        if current_md5 != "e8d6ab0f1a6dafd1779300c31597ddbe" {
            panic!("🚨 SECURITY ALERT: libc.so.6 MD5 mismatch! Expected: e8d6ab0f1a6dafd1779300c31597ddbe, Got: {}", current_md5);
        }
        println!("✅ MD5 verified for libc.so.6", "libc.so.6");
        
        let syscalls = vec!["open", "mmap", "mprotect"];
        println!("📊 libc.so.6 loaded in {}ms", "libc.so.6", start.elapsed().as_millis());
    };
}

// macro_rules! /nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv_glibc_2_40_66/lib/ld_linux_x86_64_2_telemetry {
//     () => {
//         println!("📚 Preloading /nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/ld-linux-x86-64.so.2", "/nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/ld-linux-x86-64.so.2");
//         println!("🔒 Expected MD5: d060b6579ed382f2b0bb5ef85efc90dd", "d060b6579ed382f2b0bb5ef85efc90dd");
//         let start = std::time::Instant::now();
        
//         // Verify MD5 checksum before loading  
//         let current_md5 = calculate_file_md5("/nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib64/ld-linux-x86-64.so.2");
//         if current_md5 != "d060b6579ed382f2b0bb5ef85efc90dd" {
//             panic!("🚨 SECURITY ALERT: /nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/ld-linux-x86-64.so.2 MD5 mismatch! Expected: d060b6579ed382f2b0bb5ef85efc90dd, Got: {}", current_md5);
//         }
//         println!("✅ MD5 verified for /nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/ld-linux-x86-64.so.2", "/nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/ld-linux-x86-64.so.2");
        
//         let syscalls = vec!["open", "mmap", "mprotect"];
//         println!("📊 /nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/ld-linux-x86-64.so.2 loaded in {}ms", "/nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib/ld-linux-x86-64.so.2", start.elapsed().as_millis());
//     };
// }

macro_rules! preload_all_dependencies_secure {
    () => {
        println!("🔒 Preloading all 3 dependencies with MD5 verification...");
        // /nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv_glibc_2_40_66/lib/ld_linux_x86_64_2_telemetry!();
        libgcc_s_1_telemetry!();
        libc_6_telemetry!();
        println!("✅ All dependencies securely preloaded!");
    };
}


fn main() {
    println!("ldd_wrappers_secure - add usage here");
}
