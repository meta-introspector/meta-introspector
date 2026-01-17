// Nix/Cargo Callback Interceptor - Capture build metadata for symbol dissolution
// Intercepts: rustc, cargo, nix-build callbacks with full context

use std::fs::OpenOptions;
use std::io::Write;

/// Intercept rustc callback - captures source, crate info, compilation flags
#[macro_export]
macro_rules! intercept_rustc {
    () => {{
        let rustc_data = json!({
            "callback": "rustc",
            "crate_name": std::env::var("CARGO_PKG_NAME").ok(),
            "crate_version": std::env::var("CARGO_PKG_VERSION").ok(),
            "source_dir": std::env::var("CARGO_MANIFEST_DIR").ok(),
            "target_dir": std::env::var("CARGO_TARGET_DIR").ok(),
            "profile": std::env::var("PROFILE").ok(),
            "opt_level": std::env::var("OPT_LEVEL").ok(),
            "debug": std::env::var("DEBUG").ok(),
            "features": std::env::var("CARGO_FEATURE_").ok(),
            "rustc": std::env::var("RUSTC").ok(),
            "rustc_wrapper": std::env::var("RUSTC_WRAPPER").ok(),
        });
        
        log_callback("rustc", &rustc_data);
        rustc_data
    }};
}

/// Intercept cargo callback - captures dependencies, build graph
#[macro_export]
macro_rules! intercept_cargo {
    () => {{
        let cargo_data = json!({
            "callback": "cargo",
            "cargo_home": std::env::var("CARGO_HOME").ok(),
            "cargo": std::env::var("CARGO").ok(),
            "cargo_pkg_name": std::env::var("CARGO_PKG_NAME").ok(),
            "cargo_pkg_authors": std::env::var("CARGO_PKG_AUTHORS").ok(),
            "cargo_pkg_description": std::env::var("CARGO_PKG_DESCRIPTION").ok(),
            "cargo_pkg_repository": std::env::var("CARGO_PKG_REPOSITORY").ok(),
            "cargo_pkg_license": std::env::var("CARGO_PKG_LICENSE").ok(),
            "dep_count": std::env::vars()
                .filter(|(k, _)| k.starts_with("DEP_"))
                .count(),
        });
        
        log_callback("cargo", &cargo_data);
        cargo_data
    }};
}

/// Intercept nix callback - captures derivation, store paths, build inputs
#[macro_export]
macro_rules! intercept_nix {
    () => {{
        let nix_data = json!({
            "callback": "nix",
            "nix_build_top": std::env::var("NIX_BUILD_TOP").ok(),
            "out": std::env::var("out").ok(),
            "src": std::env::var("src").ok(),
            "name": std::env::var("name").ok(),
            "system": std::env::var("system").ok(),
            "builder": std::env::var("builder").ok(),
            "native_build_inputs": std::env::var("nativeBuildInputs").ok(),
            "build_inputs": std::env::var("buildInputs").ok(),
            "propagated_build_inputs": std::env::var("propagatedBuildInputs").ok(),
            "phases": std::env::var("phases").ok(),
            "current_phase": std::env::var("curPhase").ok(),
        });
        
        log_callback("nix", &nix_data);
        nix_data
    }};
}

/// Intercept linker callback - captures linked libraries, symbols
#[macro_export]
macro_rules! intercept_linker {
    ($binary_path:expr) => {{
        use goblin::elf::Elf;
        use std::fs;
        
        let linker_data = fs::read($binary_path).ok().and_then(|data| {
            Elf::parse(&data).ok().map(|elf| {
                json!({
                    "callback": "linker",
                    "binary": $binary_path,
                    "dynsym_count": elf.dynsyms.len(),
                    "libraries": elf.libraries.iter().collect::<Vec<_>>(),
                    "soname": elf.soname,
                    "is_64": elf.is_64,
                    "entry": elf.entry,
                    "sections": elf.section_headers.len(),
                })
            })
        });
        
        if let Some(data) = &linker_data {
            log_callback("linker", data);
        }
        
        linker_data
    }};
}

/// Master interceptor - captures ALL build context
#[macro_export]
macro_rules! intercept_all {
    () => {{
        json!({
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "rustc": intercept_rustc!(),
            "cargo": intercept_cargo!(),
            "nix": intercept_nix!(),
        })
    }};
}

fn log_callback(_callback_type: &str, data: &serde_json::Value) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/build_callbacks.jsonl")
    {
        writeln!(f, "{}", data).ok();
    }
}

/// Use intercepted data to enrich symbol dissolution
#[macro_export]
macro_rules! dissolve_with_context {
    ($symbol:expr) => {{
        let build_context = intercept_all!();
        
        json!({
            "symbol": $symbol,
            "build_context": build_context,
            "abi": dissolve_abi!($symbol, build_context["rustc"]["target_dir"]),
            "bytes": dissolve_bytes!($symbol, build_context["rustc"]["target_dir"]),
            "source": {
                "crate": build_context["cargo"]["cargo_pkg_name"],
                "version": build_context["cargo"]["cargo_pkg_version"],
                "repository": build_context["cargo"]["cargo_pkg_repository"],
            },
            "nix_context": {
                "derivation": build_context["nix"]["name"],
                "store_path": build_context["nix"]["out"],
                "build_inputs": build_context["nix"]["build_inputs"],
            },
        })
    }};
}

fn main() {
    println!("nix_cargo_interceptor - add usage here");
}
