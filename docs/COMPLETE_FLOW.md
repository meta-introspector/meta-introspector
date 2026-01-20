# SOLFUNMEME P2P Block Collection Flow

## Complete Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. CLIENTS FETCH                                                │
│                                                                 │
│  Browser A (WASM)          Browser B (WASM)          Browser C  │
│  ┌──────────────┐         ┌──────────────┐         ┌─────────┐ │
│  │ solana_rpc   │         │ solana_rpc   │         │ solana  │ │
│  │ .wasm        │         │ .wasm        │         │ _rpc    │ │
│  └──────┬───────┘         └──────┬───────┘         └────┬────┘ │
│         │                        │                       │      │
│         │ Fetch blocks           │ Fetch blocks          │      │
│         │ slot 100-200           │ slot 201-300          │      │
│         ▼                        ▼                       ▼      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │         Solana RPC (api.mainnet-beta.solana.com)        │  │
│  │         Contract: BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5z... │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ Submit blocks
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. SERVER VERIFIES                                              │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ ZOS Server                                             │    │
│  │  ┌──────────────────┐  ┌──────────────┐  ┌─────────┐ │    │
│  │  │ libblock         │  │ libpayment   │  │ libstor │ │    │
│  │  │ _collector.so    │  │ .so          │  │ age.so  │ │    │
│  │  └────────┬─────────┘  └──────┬───────┘  └────┬────┘ │    │
│  │           │                   │                │      │    │
│  │           │ 1. Verify         │ 2. Pay         │ 3.   │    │
│  │           │    signature      │    0.001 SOL   │ Write│    │
│  │           │    hash           │    to client   │ .par │    │
│  │           │    slot           │                │ quet │    │
│  └───────────┴───────────────────┴────────────────┴──────┘    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ Batch complete
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. SHARE ON HUGGING FACE                                        │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Nightly Job                                              │  │
│  │                                                          │  │
│  │  1. Convert JSON → Parquet                              │  │
│  │     cargo run --bin solana-to-parquet                   │  │
│  │                                                          │  │
│  │  2. Build NAR archive                                   │  │
│  │     nix build .#txns-2026-01                            │  │
│  │                                                          │  │
│  │  3. Push to HuggingFace                                 │  │
│  │     cd hf_dataset                                       │  │
│  │     git add txns/*.parquet                              │  │
│  │     git commit -m "Update $(date)"                      │  │
│  │     git push                                            │  │
│  │                                                          │  │
│  │  4. Upload to Archive.org                               │  │
│  │     ia upload solfunmeme-$(date) *.nar.zst             │  │
│  │                                                          │  │
│  │  5. Pin to IPFS                                         │  │
│  │     ipfs add *.nar.zst                                  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  Result: https://huggingface.co/datasets/introspector/         │
│          solfunmeme                                             │
└─────────────────────────────────────────────────────────────────┘
```

## Minimal Implementation

### Client (WASM Plugin)
```rust
// tools/wasm-plugins/solana-rpc/src/lib.rs
#[wasm_bindgen]
pub async fn fetch_and_submit(start_slot: u64, end_slot: u64) -> Result<JsValue, JsValue> {
    for slot in start_slot..=end_slot {
        // 1. Fetch from Solana
        let block = fetch_block_from_solana(slot).await?;
        
        // 2. Sign with wallet
        let signed = sign_block(block).await?;
        
        // 3. Submit to server
        submit_to_server(signed).await?;
    }
    Ok(JsValue::from_str("done"))
}
```

### Server (.so Plugin)
```rust
// tools/so-plugins/block-collector/src/lib.rs
#[no_mangle]
pub extern "C" fn submit_block(block_json: *const c_char) -> *const c_char {
    let block: Block = parse_json(block_json);
    
    // 1. Verify
    if !verify_signature(&block) { return error("invalid sig"); }
    if !verify_slot(&block) { return error("invalid slot"); }
    
    // 2. Store
    write_to_parquet(&block);
    
    // 3. Pay
    pay_client(&block.client_id, 0.001);
    
    success_response()
}
```

### Nightly Sync Job
```bash
#!/bin/bash
# tools/scripts/sync_to_hf.sh

# Convert to Parquet
cargo run --release --bin solana-to-parquet -- \
  /var/lib/zos/blocks \
  /tmp/solfunmeme

# Build NAR
nix build .#txns-$(date +%Y-%m)

# Push to HF
cd /mnt/data1/meta-introspector/submodules/solfunmeme-introspector/hf_dataset
cp /tmp/solfunmeme_*.parquet txns/$(date +%Y-%m)/
git add .
git commit -m "Blocks collected $(date -I)"
git push

# Archive
ia upload solfunmeme-$(date +%Y-%m) *.nar.zst

# IPFS
ipfs add *.nar.zst
```

## Economics

### For Clients
- Earn 0.001 SOL per verified block
- Low barrier: runs in browser
- No staking required

### For Network
- Distributed collection (no single point of failure)
- Redundancy (multiple clients fetch same blocks)
- Real-time data availability

### For Users
- Free access to data on HuggingFace
- Permanent archive on Archive.org
- P2P distribution via IPFS

## Deployment

### 1. Deploy Server
```bash
cd ~/zos-server
cargo build --release
./target/release/zos-server --port 9000
```

### 2. Deploy Client
```bash
cd /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus
dx build --release
# Deploy to solfunmeme.com
```

### 3. Setup Cron
```cron
# Sync to HF every 6 hours
0 */6 * * * /mnt/data1/meta-introspector/tools/scripts/sync_to_hf.sh
```

## Result

✅ Clients fetch blocks from Solana
✅ Server verifies and stores
✅ Automatic sync to HuggingFace
✅ Permanent archive on Archive.org
✅ P2P distribution via IPFS

**Dataset**: https://huggingface.co/datasets/introspector/solfunmeme
**Archive**: https://archive.org/details/solfunmeme-2026-01
**IPFS**: ipfs://QmXXX...

Ready to implement?
