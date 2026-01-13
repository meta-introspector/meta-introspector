// 🔥 MASTER ALL CALLS WRAPPER
// Session: allcalls_1768325184

// Include all binary wrappers:
// include!("ld_all_calls_wrapper.rs");
// include!("as_all_calls_wrapper.rs");
// include!("rustc_all_calls_wrapper.rs");
// include!("gcc_all_calls_wrapper.rs");
// include!("sh_all_calls_wrapper.rs");
// include!("cc_all_calls_wrapper.rs");
// include!("gcc_all_calls_wrapper.rs");
// include!("cc_all_calls_wrapper.rs");
// include!("ld_all_calls_wrapper.rs");
// include!("readlink_all_calls_wrapper.rs");
// include!("ld_all_calls_wrapper.rs");
// include!("cc_all_calls_wrapper.rs");
// include!("collect2_all_calls_wrapper.rs");
// include!("rustc_all_calls_wrapper.rs");

macro_rules! init_all_call_wrappers {
    () => {{
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::time::{SystemTime, UNIX_EPOCH};
        
        std::fs::create_dir_all("/mnt/data1/meta-introspector/data/telemetry").ok();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let project = std::env::var("PROJECT_NAME").unwrap_or_else(|_| "rust_nightly".to_string());
        let log_file = format!("/mnt/data1/meta-introspector/data/telemetry/{}_{}.jsonl", project, timestamp);
        
        println!("🔥 INITIALIZING ALL CALL WRAPPERS -> {}", log_file);
        
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file) {
            writeln!(file, r#"{{\"type\":\"init\",\"message\":\"All call wrappers initialized\",\"timestamp\":{},\"project\":\"{}\",\"binaries\":{},\"libraries\":{},\"symbols\":{}}}\"#, 
                    timestamp, project, 14, 39, 38).ok();
        }
        
        println!("  1. ld ({} libs, {} syms)", 0, 0);
        println!("  2. as ({} libs, {} syms)", 2, 6);
        println!("  3. rustc ({} libs, {} syms)", 0, 0);
        println!("  4. gcc ({} libs, {} syms)", 2, 6);
        println!("  5. sh ({} libs, {} syms)", 2, 0);
        println!("  6. cc ({} libs, {} syms)", 0, 0);
        println!("  7. gcc ({} libs, {} syms)", 3, 6);
        println!("  8. cc ({} libs, {} syms)", 0, 0);
        println!("  9. ld ({} libs, {} syms)", 6, 6);
        println!("  10. readlink ({} libs, {} syms)", 5, 2);
        println!("  11. ld ({} libs, {} syms)", 2, 6);
        println!("  12. cc ({} libs, {} syms)", 0, 0);
        println!("  13. collect2 ({} libs, {} syms)", 3, 6);
        println!("  14. rustc ({} libs, {} syms)", 14, 0);
        println!("✅ All call wrappers initialized!");
    }};
}
