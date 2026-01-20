# P2P Block Collector

Distributed Solana block collection system using peer-to-peer networking.

## Architecture

- **client**: WASM module for fetching blocks from Solana RPC
- **server**: TCP server for receiving blocks from clients
- **shared**: Common data structures and types

## Usage

### Client (WASM)

```rust
let client = Client::new("https://api.mainnet-beta.solana.com".to_string());
let block = client.fetch_block(12345).await?;
```

### Server

```bash
cargo run --bin server
# Listens on 0.0.0.0:9000
```

## Build

```bash
cargo build --release
```
