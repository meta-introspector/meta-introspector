# Block Collector Plugin (.so)

✅ **Built successfully!**

## Output
`target/release/libblock_collector_plugin.so`

## C ABI Functions

```c
// Register a client
char* register_client(const char* peer_id);

// Submit a block (verifies, stores, pays)
char* submit_block(const char* block_json);

// Get contract address
char* get_contract();
```

## Usage from ZOS Server

```rust
use libloading::Library;

let plugin = unsafe { 
    Library::new("libblock_collector_plugin.so")? 
};

let submit = unsafe {
    plugin.get::<unsafe extern "C" fn(*const i8) -> *const i8>(b"submit_block")?
};

let block = CString::new(r#"{"slot":12345,"hash":"abc","transactions":[],"timestamp":123,"client_id":"HME..."}"#)?;
let response = unsafe { submit(block.as_ptr()) };
```

## Payment Flow

1. Client submits block
2. Plugin verifies block
3. Plugin creates Solana transaction:
   - From: Server wallet
   - To: Client pubkey
   - Amount: 0.001 SOL
4. Returns tx signature in response

## Response Format

```json
{
  "status": "accepted",
  "slot": 12345,
  "payment": {
    "amount": 0.001,
    "tx_signature": "5tiumuoXRZyw8Gu9VbasNcfdLKkJ9MVYyiqwUaZuMsB1...",
    "from": "SERVER_WALLET_PUBKEY",
    "to": "CLIENT_PUBKEY"
  }
}
```

## Next Steps

1. ✅ WASM plugin built
2. ✅ .so plugin built
3. [ ] Add actual Solana tx creation
4. [ ] Add Parquet storage
5. [ ] Wire into zos-server
