# SOLFUNMEME P2P Block Collection Network

## Architecture

```
┌─────────────────┐
│  WASM Client    │ (Browser/Node.js)
│  - libp2p       │
│  - Block fetch  │
│  - Share blocks │
└────────┬────────┘
         │ libp2p gossipsub
         ▼
┌─────────────────┐
│  Server Node    │
│  - Verify       │
│  - Write Parquet│
│  - Pay clients  │
└─────────────────┘
```

## Client (Pure WASM)

### Responsibilities
1. Connect to Solana RPC
2. Fetch new blocks for SOLFUNMEME contract
3. Share via libp2p to network
4. Get paid for confirmed blocks

### Tech Stack
- **Rust → WASM** (no_std compatible)
- **libp2p-wasm** for P2P
- **solana-client-wasm** for RPC
- **Runs in**: Browser, Node.js, Deno

## Server Node

### Responsibilities
1. Receive blocks from clients
2. Verify block data
3. Write to Parquet
4. Pay clients (SOL/SFM tokens)
5. Detect cheating/stalling

### Payment Model
```
Block confirmed → Pay 0.001 SOL
Block invalid   → Ban client 1 hour
Client stalls   → Cancel, reassign
```

## Project Structure

```
tools/
└── p2p-block-collector/
    ├── client/              # WASM client
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs       # WASM entry
    │       ├── p2p.rs       # libp2p setup
    │       ├── rpc.rs       # Solana RPC
    │       └── block.rs     # Block fetching
    ├── server/              # Server node
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       ├── verify.rs    # Block verification
    │       ├── payment.rs   # Client payments
    │       └── storage.rs   # Parquet writer
    └── shared/              # Shared types
        ├── Cargo.toml
        └── src/
            └── types.rs     # Block, Message types
```

## Minimal Implementation

### Shared Types
```rust
// shared/src/types.rs
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub slot: u64,
    pub hash: String,
    pub transactions: Vec<String>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BlockMessage {
    pub block: Block,
    pub client_id: String,
    pub signature: Vec<u8>,
}
```

### Client (WASM)
```rust
// client/src/lib.rs
use wasm_bindgen::prelude::*;
use libp2p_wasm::*;

#[wasm_bindgen]
pub struct Client {
    peer_id: String,
    rpc_url: String,
}

#[wasm_bindgen]
impl Client {
    pub fn new(rpc_url: String) -> Self {
        Self { peer_id: generate_peer_id(), rpc_url }
    }
    
    pub async fn start(&self) {
        // 1. Connect to libp2p network
        // 2. Subscribe to block requests
        // 3. Fetch blocks from Solana
        // 4. Publish to network
    }
}
```

### Server
```rust
// server/src/main.rs
struct Server {
    clients: HashMap<String, ClientState>,
    storage: ParquetWriter,
}

impl Server {
    async fn handle_block(&mut self, msg: BlockMessage) {
        if self.verify_block(&msg.block).await {
            self.storage.write(&msg.block).await;
            self.pay_client(&msg.client_id, 0.001).await;
        } else {
            self.ban_client(&msg.client_id, Duration::hours(1));
        }
    }
}
```

## Protocol

### 1. Client Registration
```
Client → Server: { "type": "register", "peer_id": "..." }
Server → Client: { "type": "registered", "contract": "BwUT..." }
```

### 2. Block Assignment
```
Server → Client: { "type": "fetch", "slot_range": [100, 200] }
Client → Server: { "type": "block", "data": {...}, "sig": "..." }
```

### 3. Payment
```
Server → Client: { "type": "payment", "amount": 0.001, "tx": "..." }
```

### 4. Cheating Detection
```
- Duplicate blocks → Ban
- Invalid signature → Ban
- Timeout (>30s) → Cancel, reassign
```

## Deployment

### Client (WASM)
```bash
cd tools/p2p-block-collector/client
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --target web
```

### Server
```bash
cd tools/p2p-block-collector/server
cargo build --release
./target/release/p2p-server --port 9000
```

### Nix Flake
```nix
{
  packages = {
    p2p-client-wasm = buildWasm ./client;
    p2p-server = buildRust ./server;
  };
}
```

## Incentive Design

### For Clients
- Earn SOL/SFM for contributing blocks
- Low barrier: runs in browser
- No staking required

### For Network
- Distributed block collection
- Redundancy (multiple clients fetch same blocks)
- Real-time data availability

### Anti-Cheating
- Signature verification
- Cross-validation (3 clients per block)
- Reputation system

## Roadmap

### Week 1: MVP
- [ ] Shared types package
- [ ] Basic WASM client (fetch + share)
- [ ] Basic server (receive + verify)
- [ ] Local testing

### Week 2: P2P
- [ ] libp2p integration
- [ ] Gossipsub for blocks
- [ ] DHT for peer discovery

### Week 3: Payments
- [ ] Solana wallet integration
- [ ] Automatic payments
- [ ] Ban/reputation system

### Week 4: Production
- [ ] Deploy server
- [ ] Web UI for clients
- [ ] Monitoring dashboard

## Next Immediate Steps

1. Create `tools/p2p-block-collector/` structure
2. Set up Cargo workspace
3. Implement shared types
4. Build minimal WASM client
5. Build minimal server
6. Test locally

Ready to start?
