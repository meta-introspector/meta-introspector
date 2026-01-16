# 🚀 Minimal Build Server

A self-contained, self-replacing build system that is also a P2P network, smart contract platform, and consensus mechanism.

## One Binary, Everything

```bash
./minimal-build-server              # Server mode
./minimal-build-server build foo    # Client mode
./minimal-build-server install meme # Install recipe
./minimal-build-server swarm        # P2P mode
```

## The Stack

- **Server**: HTTP/TLS built-in (no curl needed)
- **Git**: Built-in git client (no git needed)
- **Build**: Cargo integration
- **WASM**: Eval WASM with traces
- **P2P**: libp2p gossipsub + Kademlia
- **ZK**: Proof of execution
- **Perf**: Linux perf recording
- **Consensus**: Peer agreement on mappings

## The Equivalence

```
nix hash ≡ godel number ≡ emoji ≡ LMFDB orbit ≡ smart contract
```

## Cross-Platform Store

- Linux: `~/.meta-store`
- Windows: `%USERPROFILE%\AppData\Local\meta-store`
- Android: `/data/local/tmp/meta-store`
- iOS: `~/Library/meta-store`
- WASM: `localStorage`

## Proof System

1. **WASM Trace** - Instruction log
2. **ZK Proof** - Cryptographic guarantee
3. **Perf Data** - Kernel-level confirmation

## Consensus

Peers agree on:
- Godel number → Emoji mappings
- Smart contract execution
- Build reproducibility

3+ signatures = consensus reached

## Self-Replacing

```bash
./minimal-build-server reload
# Rebuilds itself and replaces running process
```

## P2P Memes

- Git objects as memes
- WASM blocks as memes
- Recipes as memes
- Spread via gossipsub
- Content-addressed by Godel number

## For n00bs and AI

No dependencies needed:
- No curl
- No git
- No openssl
- Just one binary

Works everywhere:
- Desktop
- Server
- Phone browser
- WASM

## The Vision

Nix store running in iPhone browser localStorage, spreading memes via P2P, with ZK proofs and consensus, where emojis are smart contracts.

🚀 = Deploy
💰 = Transfer
🔨 = Build
