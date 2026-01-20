# LibP2P Browser-to-Server with Devtools

## Architecture

```
Browser (WASM)
  ↓ libp2p-devtools
  ↓ WebRTC/WebTransport
rust-libp2p Server
  ↓ telemetry API
ZOS Server
```

## Components

### 1. Browser Client (WASM)
- `@libp2p/devtools` - Debug inspector UI
- `libp2p` - P2P networking
- WebRTC/WebTransport for browser support
- Send logs/metrics to server

### 2. Rust Server
- `rust-libp2p` - P2P server
- WebRTC/WebTransport listener
- Forward telemetry to ZOS API

### 3. Debug UI
- Embedded libp2p-inspector
- Real-time log viewer
- Network topology view
- Works on Android (no console needed)

## Implementation

### Browser WASM Plugin
```typescript
import { createLibp2p } from 'libp2p'
import { webRTC } from '@libp2p/webrtc'
import { noise } from '@chainsafe/libp2p-noise'
import { devtools } from '@libp2p/devtools'

const node = await createLibp2p({
  transports: [webRTC()],
  connectionEncryption: [noise()],
  services: {
    devtools: devtools()
  }
})

// Send telemetry
async function sendLog(level, message, context) {
  await fetch('/api/telemetry/log', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ level, message, context, timestamp: new Date().toISOString() })
  })
}
```

### Rust Server
```rust
use libp2p::{
    core::upgrade,
    noise, webrtc, yamux,
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId, Transport,
};

// WebRTC listener for browser clients
let transport = webrtc::tokio::Transport::new(
    keypair.clone(),
    webrtc::tokio::Certificate::generate(&mut rand::thread_rng())?,
);

let swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();
swarm.listen_on("/ip4/0.0.0.0/udp/9090/webrtc-direct".parse()?)?;
```

## References
- https://github.com/ipshipyard/js-libp2p-inspector
- https://github.com/libp2p/rust-libp2p
- https://blog.libp2p.io/rust-libp2p-browser-to-server/
