# Integration: solfunmeme-dioxus → ZOS Server

## What We Have

**solfunmeme-dioxus** already includes:
- ✅ Solana wallet adapter (SolanaWalletAdapter-zos)
- ✅ WASM compilation ready
- ✅ Web APIs (gloo, web-sys)
- ✅ Dioxus UI framework
- ✅ Solana SDK (2.3.0)

## Simple Integration

### 1. Add block_collector module

```rust
// src/block_collector.rs (created above)
pub async fn submit_block(block: Block) -> Result<String, String>
pub fn start_block_collector(wallet_pubkey: String)
```

### 2. Wire into main app

```rust
// src/main.rs
mod block_collector;

use block_collector::start_block_collector;

fn App(cx: Scope) -> Element {
    let wallet = use_wallet(cx);
    
    // Start collector when wallet connected
    use_effect(cx, &wallet.public_key, |pubkey| {
        if let Some(pk) = pubkey {
            start_block_collector(pk.to_string());
        }
        async {}
    });
    
    cx.render(rsx! {
        // ... existing UI ...
    })
}
```

### 3. Build

```bash
cd /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus
dx build --release
```

### 4. Deploy

```bash
# Serve locally
dx serve

# Or deploy to solfunmeme.com
```

## Flow

```
User connects wallet
  ↓
solfunmeme-dioxus starts block_collector
  ↓
Every 10 seconds:
  - Fetch Solana block
  - Submit to ZOS server (localhost:8080/api/submit)
  - Receive payment confirmation
  ↓
Show earnings in UI
```

## UI Component

```rust
fn BlockCollectorStatus(cx: Scope) -> Element {
    let earnings = use_state(cx, || 0.0);
    let blocks_submitted = use_state(cx, || 0);
    
    cx.render(rsx! {
        div { class: "collector-status",
            h3 { "Block Collector" }
            p { "Blocks submitted: {blocks_submitted}" }
            p { "Earnings: {earnings} SOL" }
        }
    })
}
```

## Next Steps

1. [ ] Add `mod block_collector;` to lib.rs
2. [ ] Wire into main app
3. [ ] Test with ZOS server running
4. [ ] Deploy to solfunmeme.com

---

**Result**: Existing solfunmeme-dioxus app becomes a block collector client with minimal changes.
