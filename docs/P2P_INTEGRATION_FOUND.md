# P2P Block Collector - Component Integration

## Found Components

### 1. solfunmeme-dioxus
**Location**: `/mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus/`

**Key Dependencies**:
```toml
wallet-adapter = { git = "https://github.com/meta-introspector/SolanaWalletAdapter-zos", branch ="no-template"}
```

**Features**:
- ✅ Solana wallet integration
- ✅ Dioxus WASM frontend
- ✅ Already builds to WASM

### 2. zos_server.rs
**Location**: `/mnt/data1/meta-introspector/zos_server.rs`

**Features**:
- ✅ Actix-web HTTP server
- ✅ Content-addressable storage
- ✅ State management

### 3. zos-server (repo)
**Location**: `/home/mdupont/zos-server`
- Need to check for libp2p integration

## Integration Architecture

```
┌──────────────────────────────────┐
│  solfunmeme-dioxus (WASM Client) │
│  - Wallet Adapter                │
│  - Block Fetcher                 │
│  - P2P Client                    │
└────────────┬─────────────────────┘
             │
             │ WebSocket/libp2p
             ▼
┌──────────────────────────────────┐
│  zos_server.rs (Extended)        │
│  - HTTP API                      │
│  - Block Verification            │
│  - Payment via Wallet Adapter    │
│  - Parquet Storage               │
└──────────────────────────────────┘
```

## Implementation Steps

### Step 1: Extend zos_server.rs
Add P2P block collection endpoints:

```rust
// Add to zos_server.rs
use p2p_shared::{Block, BlockMessage};

async fn register_client(
    data: web::Data<AppState>,
    peer_id: web::Json<String>
) -> HttpResponse {
    // Register client for block collection
}

async fn submit_block(
    data: web::Data<AppState>,
    block: web::Json<BlockMessage>
) -> HttpResponse {
    // Verify block
    // Write to Parquet
    // Pay client via wallet adapter
}
```

### Step 2: Add Wallet Payment to Server
```rust
use wallet_adapter::WalletAdapter;

async fn pay_client(client_pubkey: &str, amount: f64) {
    // Use wallet adapter from solfunmeme-dioxus
    // Send SOL/SFM tokens
}
```

### Step 3: Integrate WASM Client
Reuse solfunmeme-dioxus components:

```rust
// In solfunmeme-dioxus/src/
mod block_collector {
    use wallet_adapter::WalletAdapter;
    
    pub async fn fetch_and_submit_block(slot: u64) {
        // Fetch from Solana RPC
        // Sign with wallet
        // Submit to server
    }
}
```

### Step 4: Build & Deploy
```bash
# Build WASM client
cd /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus
dx build --release

# Build server
cd /mnt/data1/meta-introspector
cargo build --release --bin zos_server

# Run
./target/release/zos_server
```

## Next Actions

1. [ ] Check `/home/mdupont/zos-server` for libp2p code
2. [ ] Add block collection endpoints to `zos_server.rs`
3. [ ] Create block fetcher module in `solfunmeme-dioxus`
4. [ ] Wire wallet adapter for payments
5. [ ] Test locally

Ready to proceed?
