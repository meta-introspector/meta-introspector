# rust-libp2p WASM: Browser to Server

## Architecture

```
Browser (WASM)                    Server (Native)
┌─────────────────┐              ┌──────────────────┐
│ rust-libp2p     │◄────────────►│ rust-libp2p      │
│ compiled to     │   WebRTC/    │ native binary    │
│ WASM            │   WebSocket  │                  │
└─────────────────┘              └──────────────────┘
```

## Client (WASM)

```
tools/wasm-plugins/libp2p-client/
├── Cargo.toml
└── src/
    └── lib.rs
```

### Cargo.toml
```toml
[package]
name = "libp2p-client-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
libp2p = { version = "0.54", features = ["wasm-bindgen", "websocket-websys"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### src/lib.rs
```rust
use libp2p::{
    core::upgrade,
    identity, noise,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Swarm, Transport,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct P2PClient {
    peer_id: String,
}

#[wasm_bindgen]
impl P2PClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public()).to_string();
        
        Self { peer_id }
    }
    
    pub fn get_peer_id(&self) -> String {
        self.peer_id.clone()
    }
    
    pub async fn connect(&self, server_addr: &str) -> Result<JsValue, JsValue> {
        // Create libp2p swarm
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        // WebSocket transport for browser
        let transport = libp2p::websocket_websys::Transport::default()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key).unwrap())
            .multiplex(yamux::Config::default())
            .boxed();
        
        let behaviour = MyBehaviour::new();
        let mut swarm = SwarmBuilder::with_wasm_executor(transport, behaviour, local_peer_id).build();
        
        // Connect to server
        swarm.dial(server_addr.parse().unwrap()).unwrap();
        
        Ok(JsValue::from_str("connected"))
    }
    
    pub async fn submit_block(&self, block_json: &str) -> Result<JsValue, JsValue> {
        // Send block via libp2p stream
        Ok(JsValue::from_str("submitted"))
    }
}
```

## Server (Native)

```rust
// ~/zos-server/src/p2p_listener.rs
use libp2p::{
    core::upgrade,
    identity, noise, tcp, yamux,
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId, Transport,
};

pub async fn start_p2p_listener(state: AppState) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    
    println!("🆔 Local peer id: {}", local_peer_id);
    
    // TCP + WebSocket transport
    let transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::Config::new(&local_key).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();
    
    let behaviour = BlockCollectorBehaviour::new(state);
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id).build();
    
    // Listen on WebSocket for browser clients
    swarm.listen_on("/ip4/0.0.0.0/tcp/9000/ws".parse().unwrap()).unwrap();
    
    println!("🌐 Listening on /ip4/0.0.0.0/tcp/9000/ws");
    
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("📍 Listening on {}", address);
            }
            SwarmEvent::IncomingConnection { .. } => {
                println!("📥 New connection from browser");
            }
            SwarmEvent::Behaviour(event) => {
                handle_behaviour_event(event, &state).await;
            }
            _ => {}
        }
    }
}

async fn handle_behaviour_event(event: BlockCollectorEvent, state: &AppState) {
    match event {
        BlockCollectorEvent::BlockReceived { peer, block } => {
            println!("📦 Block from {}: {:?}", peer, block);
            
            // Call plugin
            let plugin = state.plugin.lock().unwrap();
            if let Ok(result) = plugin.submit_block(&block) {
                println!("✅ Block processed: {}", result);
            }
        }
    }
}
```

## Browser Extension Integration

```javascript
// mod_zos/background.js
import init, { P2PClient } from './wasm/libp2p_client_wasm.js';

await init();

const client = new P2PClient();
console.log('Peer ID:', client.get_peer_id());

// Connect to ZOS server
await client.connect('/ip4/127.0.0.1/tcp/9000/ws');

// Submit blocks
chrome.runtime.onMessage.addListener(async (msg) => {
  if (msg.type === 'submit_block') {
    await client.submit_block(JSON.stringify(msg.data));
  }
});
```

## Build Commands

### WASM Client
```bash
cd tools/wasm-plugins/libp2p-client
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/libp2p_client_wasm.wasm \
  --out-dir pkg --target web
```

### Server
```bash
cd ~/zos-server
cargo build --release
./target/release/zos_server serve
```

## Nix Flake

```nix
{
  packages = {
    libp2p-client-wasm = pkgs.rustPlatform.buildRustPackage {
      pname = "libp2p-client-wasm";
      version = "0.1.0";
      src = ./tools/wasm-plugins/libp2p-client;
      
      nativeBuildInputs = [ pkgs.wasm-bindgen-cli ];
      
      buildPhase = ''
        cargo build --target wasm32-unknown-unknown --release
        wasm-bindgen target/wasm32-unknown-unknown/release/libp2p_client_wasm.wasm \
          --out-dir pkg --target web
      '';
      
      installPhase = ''
        mkdir -p $out
        cp -r pkg/* $out/
      '';
    };
  };
}
```

## Advantages

1. **Same codebase**: rust-libp2p on both sides
2. **Type safety**: Rust types compile to WASM
3. **P2P native**: True peer-to-peer, no HTTP middleman
4. **NAT traversal**: WebRTC hole punching
5. **Decentralized**: No single point of failure

## Connection Flow

```
1. Browser loads WASM
2. Creates libp2p node
3. Dials server: /ip4/127.0.0.1/tcp/9000/ws
4. Establishes encrypted connection (noise)
5. Opens stream: /zos/block/1.0.0
6. Sends block data
7. Server processes via plugin
8. Sends payment confirmation
```

## Next Steps

1. [ ] Create libp2p-client WASM package
2. [ ] Add p2p_listener to zos-server
3. [ ] Build both with Nix
4. [ ] Test browser → server connection
5. [ ] Deploy

---

**Key**: rust-libp2p compiles to WASM, runs in browser, connects to native server via WebSocket/WebRTC.
