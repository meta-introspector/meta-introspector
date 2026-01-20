# ZOS P2P Block Collector - Plugin Architecture

## Architecture

```
┌─────────────────────────────────┐
│  Browser/WASM Client            │
│  ┌───────────────────────────┐  │
│  │ solfunmeme-dioxus         │  │
│  │ loads WASM plugins:       │  │
│  │  - solana_rpc.wasm        │  │
│  │  - block_fetcher.wasm     │  │
│  │  - wallet_adapter.wasm    │  │
│  └───────────────────────────┘  │
└────────────┬────────────────────┘
             │ HTTP/WebSocket
             ▼
┌─────────────────────────────────┐
│  ZOS Server                     │
│  ┌───────────────────────────┐  │
│  │ libloading                │  │
│  │ loads .so plugins:        │  │
│  │  - libblock_collector.so  │  │
│  │  - libpayment.so          │  │
│  │  - libstorage.so          │  │
│  └───────────────────────────┘  │
└─────────────────────────────────┘
```

## Client-Side WASM Plugins

### 1. Solana RPC Plugin (solana_rpc.wasm)
```rust
// tools/wasm-plugins/solana-rpc/src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn fetch_block(slot: u64) -> Result<JsValue, JsValue> {
    // Fetch from Solana RPC
    // Return block data
}

#[wasm_bindgen]
pub async fn get_signatures(address: &str) -> Result<JsValue, JsValue> {
    // Get signatures for address
}
```

### 2. Block Fetcher Plugin (block_fetcher.wasm)
```rust
// tools/wasm-plugins/block-fetcher/src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn collect_blocks(start_slot: u64, end_slot: u64) -> Result<JsValue, JsValue> {
    // Collect blocks in range
    // Use solana_rpc plugin
}
```

### 3. Wallet Adapter Plugin (Already exists!)
```rust
// From solfunmeme-dioxus
wallet-adapter = { git = "https://github.com/meta-introspector/SolanaWalletAdapter-zos" }
```

## Server-Side .so Plugins

### 1. Block Collector Plugin (libblock_collector.so)
```rust
// tools/so-plugins/block-collector/src/lib.rs
#[no_mangle]
pub extern "C" fn register_client(peer_id: *const c_char) -> *const c_char {
    // Register client
}

#[no_mangle]
pub extern "C" fn submit_block(block_json: *const c_char) -> *const c_char {
    // Verify and accept block
}
```

### 2. Payment Plugin (libpayment.so)
```rust
// tools/so-plugins/payment/src/lib.rs
#[no_mangle]
pub extern "C" fn pay_client(client_id: *const c_char, amount: f64) -> *const c_char {
    // Send SOL/SFM tokens
}
```

### 3. Storage Plugin (libstorage.so)
```rust
// tools/so-plugins/storage/src/lib.rs
#[no_mangle]
pub extern "C" fn write_parquet(block_json: *const c_char) -> bool {
    // Write to Parquet file
}
```

## Project Structure

```
tools/
├── wasm-plugins/           # Client-side WASM plugins
│   ├── solana-rpc/
│   │   ├── Cargo.toml     # crate-type = ["cdylib"]
│   │   └── src/lib.rs
│   ├── block-fetcher/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── shared/
│       └── types.rs
│
└── so-plugins/             # Server-side .so plugins
    ├── block-collector/
    │   ├── Cargo.toml     # crate-type = ["cdylib"]
    │   └── src/lib.rs
    ├── payment/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── storage/
        ├── Cargo.toml
        └── src/lib.rs
```

## Client Plugin Loading (Dioxus)

```rust
// In solfunmeme-dioxus
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/plugins/solana_rpc.js")]
extern "C" {
    async fn fetch_block(slot: u64) -> JsValue;
}

fn App(cx: Scope) -> Element {
    let block = use_future(cx, (), |_| async move {
        fetch_block(12345).await
    });
    
    // Render UI
}
```

## Server Plugin Loading (ZOS)

```rust
// In zos-server
use libloading::Library;

struct PluginManager {
    block_collector: Library,
    payment: Library,
    storage: Library,
}

impl PluginManager {
    fn load() -> Self {
        Self {
            block_collector: unsafe { 
                Library::new("tools/so-plugins/block-collector/target/release/libblock_collector.so").unwrap()
            },
            payment: unsafe {
                Library::new("tools/so-plugins/payment/target/release/libpayment.so").unwrap()
            },
            storage: unsafe {
                Library::new("tools/so-plugins/storage/target/release/libstorage.so").unwrap()
            },
        }
    }
}
```

## Build Commands

### WASM Plugins
```bash
cd tools/wasm-plugins/solana-rpc
wasm-pack build --target web

cd ../block-fetcher
wasm-pack build --target web
```

### .so Plugins
```bash
cd tools/so-plugins/block-collector
cargo build --release

cd ../payment
cargo build --release

cd ../storage
cargo build --release
```

## Benefits

1. **Modularity**: Each feature is a plugin
2. **Hot Reload**: Replace plugins without restarting
3. **Security**: Plugins run in isolated contexts
4. **Reusability**: Same plugin system for client and server
5. **Distribution**: Users can add custom plugins

## Next Steps

1. [ ] Create `tools/wasm-plugins/solana-rpc/`
2. [ ] Create `tools/so-plugins/block-collector/`
3. [ ] Wire plugins into solfunmeme-dioxus
4. [ ] Wire plugins into zos-server
5. [ ] Test end-to-end

Ready to start with minimal solana-rpc WASM plugin?
