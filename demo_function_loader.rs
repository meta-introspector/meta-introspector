use libloading::{Library, Symbol};
use std::path::PathBuf;

/// Dynamic function loader - loads .so files at runtime
pub struct FunctionLoader {
    libraries: Vec<Library>,
}

impl FunctionLoader {
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }

    /// Load a shared object library
    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(path)? };
        self.libraries.push(lib);
        println!("✅ Loaded: {}", path);
        Ok(())
    }

    /// Call a function from loaded library
    pub unsafe fn call<T>(&self, lib_index: usize, symbol: &str) -> Result<Symbol<T>, Box<dyn std::error::Error>> {
        let lib = &self.libraries[lib_index];
        let func: Symbol<T> = lib.get(symbol.as_bytes())?;
        Ok(func)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Function Loader Demo\n");

    let mut loader = FunctionLoader::new();

    // Load content-address library
    let ca_path = "libs/content-address/target/release/libcontent_address.so";
    if std::path::Path::new(ca_path).exists() {
        loader.load(ca_path)?;

        // Call generate_content_address
        unsafe {
            type GenCA = extern "C" fn(*const u8, usize) -> *mut std::os::raw::c_char;
            let gen_ca: Symbol<GenCA> = loader.call(0, "generate_content_address")?;

            let input = b"hello world";
            let result = gen_ca(input.as_ptr(), input.len());
            let c_str = std::ffi::CStr::from_ptr(result);
            println!("Content Address: {}", c_str.to_str()?);
        }
    }

    // Load MCP library
    let mcp_path = "libs/mcp/target/release/libmcp.so";
    if std::path::Path::new(mcp_path).exists() {
        loader.load(mcp_path)?;

        unsafe {
            type DiscoverTools = extern "C" fn() -> *mut std::os::raw::c_char;
            let discover: Symbol<DiscoverTools> = loader.call(1, "mcp_discover_tools")?;

            let result = discover();
            let c_str = std::ffi::CStr::from_ptr(result);
            println!("\nMCP Tools: {}", c_str.to_str()?);
        }
    }

    // Load git-ops library
    let git_path = "libs/git-ops/target/release/libgit_ops.so";
    if std::path::Path::new(git_path).exists() {
        loader.load(git_path)?;
        println!("\n✅ All libraries loaded successfully!");
    }

    Ok(())
}
