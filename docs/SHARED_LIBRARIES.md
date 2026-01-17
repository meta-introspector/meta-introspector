# Shared Function Libraries

## Architecture

Each unique server function is now a **standardized shared object** (.so) that can be loaded as either:
- **Static crate**: Linked at compile time
- **Dynamic library**: Loaded at runtime via `libloading`

## Available Libraries

| Library | Size | Functions | Purpose |
|---------|------|-----------|---------|
| **libcontent_address.so** | 414KB | `generate_content_address()` | Content addressing with SHA256 |
| **libmcp.so** | 419KB | `mcp_discover_tools()`, `MCPRegistry` | MCP tool discovery and calling |
| **libgit_ops.so** | 458KB | `git_clone()`, `git_status_rust()` | Git operations |

## Usage

### Static Linking (Compile Time)

```rust
// Cargo.toml
[dependencies]
content-address = { path = "libs/content-address" }

// main.rs
use content_address::generate_content_address_rust;

fn main() {
    let addr = generate_content_address_rust(b"hello", &[]);
    println!("Address: {}", addr);
}
```

### Dynamic Loading (Runtime)

```rust
use libloading::{Library, Symbol};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new("libs/content-address/target/release/libcontent_address.so")? };
    
    unsafe {
        type GenCA = extern "C" fn(*const u8, usize) -> *mut std::os::raw::c_char;
        let gen_ca: Symbol<GenCA> = lib.get(b"generate_content_address")?;
        
        let input = b"hello world";
        let result = gen_ca(input.as_ptr(), input.len());
        let c_str = std::ffi::CStr::from_ptr(result);
        println!("Address: {}", c_str.to_str()?);
    }
    
    Ok(())
}
```

## Building Libraries

```bash
# Build all libraries
cd libs/content-address && cargo build --release
cd libs/mcp && cargo build --release
cd libs/git-ops && cargo build --release

# Or build from root
cargo build --release -p content-address
cargo build --release -p mcp
cargo build --release -p git-ops
```

## Demo

```bash
# Run function loader demo
cargo build --bin demo_function_loader --release
./target/release/demo_function_loader
```

Output:
```
🔧 Function Loader Demo

✅ Loaded: libs/content-address/target/release/libcontent_address.so
Content Address: ca:sha256:b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
✅ Loaded: libs/mcp/target/release/libmcp.so

MCP Tools: [{"name":"nix_build","description":"Build Nix flake","library":"libnix.so","symbol":"nix_build"}]
✅ Loaded: libs/git-ops/target/release/libgit_ops.so

✅ All libraries loaded successfully!
```

## Library API

### content-address

```rust
// Rust API
pub fn generate_content_address_rust(input: &[u8], metadata: &[String]) -> String

// C API
#[no_mangle]
pub extern "C" fn generate_content_address(input: *const u8, input_len: usize) -> *mut c_char
```

### mcp

```rust
// Rust API
pub trait MCPProvider {
    fn discover_tools(&self) -> Result<Vec<MCPTool>, Error>;
    fn call_tool(&self, name: &str, args: Value) -> Result<Value, Error>;
}

pub struct MCPRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, tool: MCPTool);
    pub fn get(&self, name: &str) -> Option<&MCPTool>;
}

// C API
#[no_mangle]
pub extern "C" fn mcp_discover_tools() -> *mut c_char
```

### git-ops

```rust
// Rust API
pub fn git_clone_rust(url: &str, path: Option<String>) -> Result<(), Error>
pub fn git_status_rust(path: &str) -> Result<GitStatusResponse, Error>

// C API
#[no_mangle]
pub extern "C" fn git_clone(url: *const c_char, path: *const c_char) -> i32
```

## Integration with Servers

### minimal-build-server

```rust
use content_address::generate_content_address_rust;
use git_ops::{git_clone_rust, git_status_rust};

async fn build(req: BuildRequest) -> Json<BuildResponse> {
    // Use shared libraries
    let addr = generate_content_address_rust(req.target.as_bytes(), &[]);
    git_clone_rust(&req.url, Some("/tmp/build".to_string()))?;
    // ...
}
```

### unified-nix-service

```rust
use content_address::generate_content_address_rust;
use mcp::{MCPRegistry, MCPProvider};

impl UnifiedNixService {
    pub fn generate_content_address(&self, flake_url: &str, outputs: &[String]) -> String {
        generate_content_address_rust(flake_url.as_bytes(), outputs)
    }
}
```

## Benefits

1. **No Duplication**: Single implementation per function
2. **Hot Reload**: Update .so without recompiling servers
3. **Language Agnostic**: C ABI works with any language
4. **Versioning**: Each .so can be versioned independently
5. **Testing**: Test libraries in isolation
6. **Distribution**: Ship .so files separately

## Roadmap

### Phase 1 (Complete)
- [x] content-address library
- [x] mcp library
- [x] git-ops library
- [x] Function loader demo

### Phase 2 (Next)
- [ ] file-ops library (grep, sed)
- [ ] error-parser library
- [ ] p2p-contracts library
- [ ] trading-engine library

### Phase 3
- [ ] Migrate all servers to use libraries
- [ ] Remove duplicated code
- [ ] Add versioning
- [ ] Create registry

## File Structure

```
libs/
├── content-address/
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   └── target/release/
│       └── libcontent_address.so
├── mcp/
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   └── target/release/
│       └── libmcp.so
└── git-ops/
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    └── target/release/
        └── libgit_ops.so
```

---

**Status:** Phase 1 Complete  
**Last Updated:** 2026-01-17
