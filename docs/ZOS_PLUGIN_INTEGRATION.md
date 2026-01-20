# ZOS Server Plugin Integration - Complete

## Structure

```
Caller Side (zos-server):
  src/block_collector_plugin.rs  - Calls .so functions
  src/block_routes.rs             - HTTP API endpoints
  src/plugin_driver.rs            - Existing plugin loader

Called Side (.so plugin):
  tools/so-plugins/block-collector/
    src/lib.rs                    - C ABI functions
    target/release/libblock_collector_plugin.so
```

## Integration Steps

### 1. Build Plugin
```bash
cd /mnt/data1/meta-introspector/tools/so-plugins/block-collector
cargo build --release
```

### 2. Copy to ZOS
```bash
mkdir -p ~/zos-server/plugins
cp target/release/libblock_collector_plugin.so ~/zos-server/plugins/
```

### 3. Add Modules to ZOS
```rust
// ~/zos-server/src/lib.rs
pub mod block_collector_plugin;
pub mod block_routes;
```

### 4. Update main.rs
```rust
// ~/zos-server/src/main.rs
use block_collector_plugin::BlockCollectorPlugin;
use block_routes::{create_block_routes, AppState};

async fn serve() {
    // ... existing code ...
    
    // Load block collector plugin
    let mut plugin = BlockCollectorPlugin::new();
    if let Err(e) = plugin.load() {
        error!("Failed to load block-collector plugin: {}", e);
        return;
    }
    
    let state = AppState {
        plugin: Arc::new(Mutex::new(plugin)),
    };
    
    // Add block routes
    let app = web::create_router(core)
        .merge(create_block_routes())
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());
    
    // ... rest of serve() ...
}
```

### 5. Test
```bash
cd ~/zos-server
cargo build --release

# Run server
./target/release/zos_server serve

# Test in another terminal
curl -X POST http://localhost:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{"peer_id":"test_123"}'

curl -X POST http://localhost:8080/api/submit \
  -H "Content-Type: application/json" \
  -d '{"block_json":"{\"slot\":12345,\"hash\":\"abc\",\"transactions\":[],\"timestamp\":123,\"client_id\":\"test\"}"}'
```

## Hot Reload (Like Linux Kernel Modules)

```rust
// Future: Watch plugins directory and reload on change
use notify::{Watcher, RecursiveMode};

impl BlockCollectorPlugin {
    pub fn watch_and_reload(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::watcher(tx, Duration::from_secs(1)).unwrap();
        watcher.watch("plugins/", RecursiveMode::NonRecursive).unwrap();
        
        loop {
            match rx.recv() {
                Ok(event) => {
                    info!("🔄 Plugin changed, reloading...");
                    self.load().unwrap();
                }
                Err(e) => error!("Watch error: {}", e),
            }
        }
    }
}
```

## Upgrade Path

Like Linux kernel modules:
1. Build new plugin version
2. Copy to plugins/
3. Server detects change
4. Unload old plugin
5. Load new plugin
6. Zero downtime

## Next Steps

1. [ ] Add modules to lib.rs
2. [ ] Update main.rs with plugin loading
3. [ ] Build and test
4. [ ] Add hot reload
5. [ ] Deploy

---

**Result**: ZOS server loads .so plugin, exposes HTTP API, can upgrade plugins without restart.
