# WASM Plugins

WebAssembly plugins for browser-based blockchain interactions.

## libp2p-client

WebSocket-based client for P2P block submission.

```rust
let client = LibP2PClient::new();
client.connect("ws://localhost:9000").await?;
client.submit_block(&block_json).await?;
```

## solana-rpc

WASM interface for Solana RPC operations. See [README](solana-rpc/README.md).
