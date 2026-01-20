# SOLFUNMEME Plugin Architecture - Stripped Down

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ solfunmeme-dioxus (minimal core)                            │
│                                                             │
│  - UI framework (Dioxus)                                    │
│  - Plugin loader (client-side WASM)                         │
│  - Plugin loader (server-side .so)                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Plugins (dynamically loaded)                                │
│                                                             │
│  Client (.wasm):                    Server (.so):           │
│  - wallet_plugin.wasm               - libwallet.so          │
│  - solana_rpc_plugin.wasm           - libsolana.so          │
│  - block_collector_plugin.wasm      - libblock_collector.so│
│  - social_data_plugin.wasm          - libsocial_data.so    │
└─────────────────────────────────────────────────────────────┘
```

## Core Structure

```
solfunmeme-dioxus/
├── src/
│   ├── main.rs              # Minimal entry point
│   ├── plugin_loader.rs     # Plugin system
│   └── ui.rs                # Basic UI shell
├── plugins/
│   ├── client/              # WASM plugins
│   │   ├── wallet/
│   │   ├── solana-rpc/
│   │   ├── block-collector/
│   │   └── social-data/
│   └── server/              # .so plugins (for ZOS)
│       ├── wallet/
│       ├── solana/
│       ├── block-collector/
│       └── social-data/
└── Cargo.toml               # Minimal deps
```

## Minimal Core Cargo.toml

```toml
[package]
name = "solfunmeme-dioxus"
version = "2.0.0"
edition = "2021"

[dependencies]
dioxus = { version = "0.7", features = ["router"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# WASM only
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window"] }

# No workspace members - all plugins are separate
```

## Plugin Loader

```rust
// src/plugin_loader.rs
use wasm_bindgen::prelude::*;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

pub trait Plugin {
    fn name(&self) -> &str;
    fn init(&mut self);
    fn call(&self, method: &str, args: &str) -> String;
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }
    
    pub async fn load_plugin(&mut self, url: &str) {
        // Load WASM plugin dynamically
        #[cfg(target_arch = "wasm32")]
        {
            let module = wasm_bindgen_futures::JsFuture::from(
                js_sys::WebAssembly::instantiate_streaming(
                    &web_sys::window().unwrap().fetch_with_str(url),
                    &js_sys::Object::new()
                )
            ).await.unwrap();
            
            // Store plugin
        }
    }
    
    pub fn call_plugin(&self, plugin_name: &str, method: &str, args: &str) -> String {
        for plugin in &self.plugins {
            if plugin.name() == plugin_name {
                return plugin.call(method, args);
            }
        }
        "Plugin not found".to_string()
    }
}
```

## Minimal UI

```rust
// src/ui.rs
use dioxus::prelude::*;

pub fn App() -> Element {
    let plugin_manager = use_signal(|| PluginManager::new());
    
    rsx! {
        div { class: "app",
            h1 { "SOLFUNMEME" }
            
            // Plugins render themselves
            PluginContainer { manager: plugin_manager }
        }
    }
}

fn PluginContainer(manager: Signal<PluginManager>) -> Element {
    rsx! {
        div { class: "plugins",
            // Each plugin provides its own UI component
        }
    }
}
```

## Plugin Example (Wallet)

```
plugins/client/wallet/
├── Cargo.toml
└── src/
    └── lib.rs

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

```rust
// plugins/client/wallet/src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WalletPlugin {
    connected: bool,
}

#[wasm_bindgen]
impl WalletPlugin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { connected: false }
    }
    
    pub fn name(&self) -> String {
        "wallet".to_string()
    }
    
    pub fn call(&self, method: &str, args: &str) -> String {
        match method {
            "connect" => self.connect(),
            "get_balance" => self.get_balance(),
            _ => "Unknown method".to_string()
        }
    }
    
    fn connect(&self) -> String {
        // Connect wallet
        "connected".to_string()
    }
    
    fn get_balance(&self) -> String {
        // Get balance
        "1.5".to_string()
    }
}
```

## Migration Plan

### Phase 1: Extract Core (Week 1)
- [ ] Create new minimal Cargo.toml
- [ ] Move main.rs to minimal entry point
- [ ] Create plugin_loader.rs
- [ ] Remove all crates/* from workspace

### Phase 2: Convert to Plugins (Week 2)
- [ ] wallet → plugins/client/wallet
- [ ] solana-rpc → plugins/client/solana-rpc
- [ ] block-collector → plugins/client/block-collector
- [ ] Each builds independently

### Phase 3: Server Plugins (Week 3)
- [ ] Create .so versions for ZOS server
- [ ] Same API, different runtime
- [ ] Deploy to ~/zos-server/plugins/

### Phase 4: Dynamic Loading (Week 4)
- [ ] Load plugins at runtime
- [ ] Hot reload support
- [ ] Plugin marketplace

## Benefits

1. **Minimal Core**: ~100 lines vs 10,000+
2. **Independent Plugins**: Each builds separately
3. **No Vendor Hell**: Plugins manage their own deps
4. **Hot Reload**: Update plugins without rebuilding core
5. **Reusable**: Same plugins work in browser and server

## Next Steps

1. [ ] Create `solfunmeme-dioxus-v2/` directory
2. [ ] Implement minimal core
3. [ ] Extract wallet as first plugin
4. [ ] Test plugin loading
5. [ ] Migrate remaining features

Ready to start?
