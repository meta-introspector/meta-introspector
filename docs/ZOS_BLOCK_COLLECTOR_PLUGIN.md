# ZOS Block Collector Plugin

## Architecture

ZOS loads plugins as shared objects (.so files). Each plugin exposes C ABI functions.

```
┌─────────────────────┐
│   ZOS Server        │
│   (libloading)      │
└──────────┬──────────┘
           │ dlopen
           ▼
┌─────────────────────┐
│ libblock_collector  │
│ .so plugin          │
│                     │
│ - register_client   │
│ - submit_block      │
│ - verify_block      │
│ - pay_client        │
└─────────────────────┘
```

## Plugin Structure

```
tools/
└── block-collector-plugin/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

### Cargo.toml
```toml
[package]
name = "block-collector-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Shared library

[dependencies]
p2p-shared = { path = "../p2p-block-collector/shared" }
serde_json = "1"
```

### lib.rs (C ABI)
```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use p2p_shared::*;

#[no_mangle]
pub extern "C" fn register_client(peer_id_ptr: *const c_char) -> *const c_char {
    let peer_id = unsafe { CStr::from_ptr(peer_id_ptr).to_string_lossy() };
    
    // Register client logic
    let response = serde_json::json!({
        "status": "registered",
        "peer_id": peer_id.to_string(),
        "contract": CONTRACT_ADDRESS
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn submit_block(block_json_ptr: *const c_char) -> *const c_char {
    let block_json = unsafe { CStr::from_ptr(block_json_ptr).to_string_lossy() };
    
    // Parse and verify block
    let block: Block = serde_json::from_str(&block_json).unwrap();
    
    // Verify, store, pay
    let response = serde_json::json!({
        "status": "accepted",
        "slot": block.slot,
        "payment": 0.001
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn verify_block(block_json_ptr: *const c_char) -> bool {
    // Verification logic
    true
}

#[no_mangle]
pub extern "C" fn pay_client(client_id_ptr: *const c_char, amount: f64) -> *const c_char {
    let client_id = unsafe { CStr::from_ptr(client_id_ptr).to_string_lossy() };
    
    // Payment via wallet adapter
    let response = serde_json::json!({
        "status": "paid",
        "client": client_id.to_string(),
        "amount": amount,
        "tx": "mock_tx_hash"
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}
```

## ZOS Loader

```rust
// In zos-server
use libloading::Library;

fn load_block_collector_plugin() -> Result<(), Box<dyn std::error::Error>> {
    let plugin = unsafe { 
        Library::new("tools/block-collector-plugin/target/release/libblock_collector_plugin.so")? 
    };
    
    let register = unsafe {
        plugin.get::<unsafe extern "C" fn(*const i8) -> *const i8>(b"register_client")?
    };
    
    let submit = unsafe {
        plugin.get::<unsafe extern "C" fn(*const i8) -> *const i8>(b"submit_block")?
    };
    
    // Use functions
    let peer_id = std::ffi::CString::new("peer_123")?;
    let response = unsafe { register(peer_id.as_ptr()) };
    
    Ok(())
}
```

## Build

```bash
cd tools/block-collector-plugin
cargo build --release

# Output: target/release/libblock_collector_plugin.so
```

## Integration with WASM Client

WASM client calls HTTP endpoint → ZOS loads plugin → Plugin handles request

```
Browser (WASM) → HTTP → ZOS Server → dlopen plugin → Process
```

## Next Steps

1. [ ] Create `tools/block-collector-plugin/`
2. [ ] Implement C ABI functions
3. [ ] Build .so file
4. [ ] Add loader to zos-server
5. [ ] Test with WASM client
