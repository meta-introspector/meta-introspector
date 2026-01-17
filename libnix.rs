// libnix - Nix wrapper that loads system libraries on demand
// Usage: nix!("ssl", "git", "curl")

use libloading::Library;
use std::collections::HashMap;
use std::sync::Mutex;

// Public modules for shims/wrappers
pub mod rand_shim;
pub mod perf_runtime;

static LOADED_LIBS: Mutex<Option<HashMap<String, Library>>> = Mutex::new(None);

#[macro_export]
macro_rules! nix {
    ($($lib:expr),*) => {{
        let libs = vec![$($lib),*];
        $crate::load_via_nix(&libs)
    }};
}

pub fn load_via_nix(libs: &[&str]) -> Result<(), String> {
    let mut loaded = LOADED_LIBS.lock().unwrap();
    if loaded.is_none() {
        *loaded = Some(HashMap::new());
    }
    
    let map = loaded.as_mut().unwrap();
    
    for lib in libs {
        if map.contains_key(*lib) {
            continue;
        }
        
        // Use nix to find the library
        let nix_path = find_lib_via_nix(lib)?;
        
        // Load it
        let library = unsafe {
            Library::new(&nix_path)
                .map_err(|e| format!("Failed to load {}: {}", lib, e))?
        };
        
        map.insert(lib.to_string(), library);
        println!("✅ Loaded {} from {}", lib, nix_path);
    }
    
    Ok(())
}

fn find_lib_via_nix(lib: &str) -> Result<String, String> {
    // Map common names to nix packages
    let nix_pkg = match lib {
        "ssl" | "openssl" => "openssl",
        "git" | "git2" => "libgit2",
        "curl" => "curl",
        _ => lib,
    };
    
    // Use nix to get the library path
    let output = std::process::Command::new("nix")
        .args(["eval", "--raw", &format!("nixpkgs#{}.out", nix_pkg)])
        .output()
        .map_err(|e| format!("Failed to run nix: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("Nix eval failed for {}", nix_pkg));
    }
    
    let store_path = String::from_utf8_lossy(&output.stdout).to_string();
    
    // Find the .so file
    let lib_name = format!("lib{}.so", lib);
    let lib_path = format!("{}/lib/{}", store_path, lib_name);
    
    if std::path::Path::new(&lib_path).exists() {
        Ok(lib_path)
    } else {
        Err(format!("Library not found at {}", lib_path))
    }
}

#[no_mangle]
pub extern "C" fn libnix_load(libs: *const *const i8, count: usize) -> i32 {
    let lib_names: Vec<&str> = unsafe {
        std::slice::from_raw_parts(libs, count)
            .iter()
            .map(|&p| std::ffi::CStr::from_ptr(p).to_str().unwrap())
            .collect()
    };
    
    match load_via_nix(&lib_names) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_ssl() {
        let result = nix!("ssl");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_load_multiple() {
        let result = nix!("ssl", "git", "curl");
        assert!(result.is_ok());
    }
}
