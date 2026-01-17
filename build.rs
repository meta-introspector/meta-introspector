use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("telemetry_macros.rs");
    
    fs::write(&dest_path, "// Telemetry macros stub\n").unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
}
