# DevOps Consultant Guide - Meta-Introspector

## 🎯 Quick Start for New Users

The **Minimal Build Server** acts as your DevOps consultant - a single entry point to download, build, deploy, and manage the entire meta-introspector ecosystem.

### One-Command Setup

```bash
# Download and start the minimal build server
curl -sSL https://github.com/meta-introspector/meta-introspector/releases/latest/download/minimal-build-server-linux-x86_64 -o minimal-build-server
chmod +x minimal-build-server
./minimal-build-server
```

Server starts on `http://127.0.0.1:3000`

## 📊 Complete Server Inventory

### Current Repository Servers

| Server | Port | Lines | Purpose | Status |
|--------|------|-------|---------|--------|
| **minimal_build_server** | 3000 | 908 | Main DevOps orchestrator | ✅ Production |
| **nix_as_a_service** | 8081 | 404 | Nix builds with MCP | ✅ Production |
| **rust_as_a_service** | 8080 | 287 | Rust compilation API | ✅ Production |
| **telemetry_server** | 8888 | 186 | Build telemetry collection | ✅ Production |
| **universal_client_node** | 3000 | 162 | Universal LLM proxy | 🔧 Development |
| **trading_node** | 8000-8009 | 358 | Meme trading network | 🎮 Demo |
| **unified_nix_service** | 8081 | 367 | Unified Nix+Solana | 🔧 Development |
| **solana_as_a_service** | - | 486 | Solana integration | 🔧 Development |
| **solfunmeme_ca_service** | - | 132 | Content-addressed memes | 🎮 Demo |
| **demo_gemini_nodes** | - | 141 | Gemini protocol nodes | 🎮 Demo |

### External Repository Servers

| Server | Location | Port | Purpose |
|--------|----------|------|---------|
| **p2p_compilation_cluster** | ~/zos-server/ | 8080 | Distributed compilation with libp2p |
| **zos_server** (Python) | Current repo | - | ZOS blockchain server |
| **zos_server_v2** (Python) | Current repo | - | Enhanced ZOS server |

### P2P/Network Services

| Service | Technology | Purpose |
|---------|-----------|---------|
| **meme_swarm** | libp2p | Gossipsub + Kademlia DHT for meme distribution |
| **p2p_network** | libp2p | Distributed findings sharing |

## 🏗️ Minimal Build Server Architecture

### Core Capabilities

The minimal build server (908 lines) provides:

1. **Build Orchestration**
   - Nix builds
   - Cargo builds
   - Binary downloads from releases
   - Error parsing and reporting

2. **DevOps Automation**
   - SSH key setup
   - GPG key configuration
   - Git repository cloning
   - Deployment to custom ports

3. **Development Tools**
   - Grep search across codebase
   - Sed-based file editing
   - Git blame integration
   - Auto-fix compilation errors

4. **P2P Integration**
   - Peer ID generation
   - Consensus loading
   - Contract proposals
   - WASM evaluation

### API Endpoints

#### Build & Deploy
```bash
# Build a target
curl -X POST http://127.0.0.1:3000/compile \
  -H "Content-Type: application/json" \
  -d '{"target": "minimal-build-server"}'

# Deploy to custom port
curl -X POST http://127.0.0.1:3000/deploy \
  -H "Content-Type: application/json" \
  -d '{"target": "minimal-build-server", "port": 3001}'

# Download pre-built binary
curl -X POST http://127.0.0.1:3000/build \
  -H "Content-Type: application/json" \
  -d '{"target": "telemetry-driver", "action": "download"}'
```

#### Repository Management
```bash
# Clone repository
curl -X POST http://127.0.0.1:3000/git \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/user/repo", "path": "/tmp/repo"}'

# Git status
curl http://127.0.0.1:3000/git/status

# Git blame
curl -X POST http://127.0.0.1:3000/git/blame \
  -H "Content-Type: application/json" \
  -d '{"file": "src/main.rs", "line": 42}'
```

#### Development Tools
```bash
# Search codebase
curl -X POST http://127.0.0.1:3000/grep \
  -H "Content-Type: application/json" \
  -d '{"pattern": "TcpListener", "path": "."}'

# Edit file with sed
curl -X POST http://127.0.0.1:3000/sed \
  -H "Content-Type: application/json" \
  -d '{"file": "config.toml", "pattern": "port = 3000", "replacement": "port = 8080"}'

# Auto-fix all errors
curl -X POST http://127.0.0.1:3000/fix-all
```

#### Setup & Configuration
```bash
# Setup SSH keys
curl -X POST http://127.0.0.1:3000/setup/ssh \
  -H "Content-Type: application/json" \
  -d '{"public_key": "ssh-ed25519 AAAA..."}'

# Setup GPG keys
curl -X POST http://127.0.0.1:3000/setup/gpg \
  -H "Content-Type: application/json" \
  -d '{"key": "-----BEGIN PGP PUBLIC KEY BLOCK-----..."}'

# Setup Git config
curl -X POST http://127.0.0.1:3000/setup/git \
  -H "Content-Type: application/json" \
  -d '{"name": "Your Name", "email": "you@example.com"}'
```

#### System Management
```bash
# List available binaries
curl http://127.0.0.1:3000/binaries

# Get error summary
curl http://127.0.0.1:3000/errors

# Restart server
curl -X POST http://127.0.0.1:3000/restart

# Upgrade server
curl -X POST http://127.0.0.1:3000/upgrade

# Help/API documentation
curl http://127.0.0.1:3000/help
```

#### P2P & Blockchain
```bash
# Get peer info
curl http://127.0.0.1:3000/peer/info

# Propose contract
curl -X POST http://127.0.0.1:3000/contract/propose \
  -H "Content-Type: application/json" \
  -d '{"terms": "...", "parties": [...]}'

# Sign contract
curl -X POST http://127.0.0.1:3000/contract/sign \
  -H "Content-Type: application/json" \
  -d '{"contract_id": "abc123", "signature": "..."}'

# Execute emoji command
curl -X POST http://127.0.0.1:3000/emoji/exec \
  -H "Content-Type: application/json" \
  -d '{"emoji": "🔥", "args": ["build"]}'

# Evaluate WASM
curl -X POST http://127.0.0.1:3000/wasm/eval \
  --data-binary @module.wasm
```

## 🌐 Service Ecosystem

### Port Allocation Strategy

| Range | Purpose | Examples |
|-------|---------|----------|
| 3000-3099 | Build servers | minimal-build-server (3000), QA (3001) |
| 8000-8099 | Application services | trading-node (8000-8009), rust-service (8080), nix-service (8081) |
| 8888 | Monitoring | telemetry-server |
| 11000+ | External integrations | Ollama (11434) |

### Service Dependencies

```
minimal-build-server (3000)
├── nix-as-a-service (8081)
│   └── Nix store
├── rust-as-a-service (8080)
│   └── rustc + cargo
├── telemetry-server (8888)
│   └── Parquet storage
└── p2p-compilation-cluster (8080)
    └── libp2p network
```

## 🚀 Deployment Scenarios

### Scenario 1: Local Development

```bash
# Start minimal build server
./minimal-build-server

# Build and test locally
curl -X POST http://127.0.0.1:3000/compile -d '{"target":"demo_hello"}'
```

**Ports Used:** 3000 (localhost only)

### Scenario 2: QA Environment

```bash
# Deploy QA server on port 3001
curl -X POST http://127.0.0.1:3000/deploy \
  -d '{"target":"minimal-build-server", "port":3001}'

# Or use setup script
./setup-qa-user.sh
sudo systemctl start qa-build-server
```

**Ports Used:** 3001 (0.0.0.0 - external access)

### Scenario 3: Full Stack

```bash
# Start all services
./minimal-build-server &                    # Port 3000
./result/bin/telemetry_server &             # Port 8888
./result/bin/rust_as_a_service &            # Port 8080
./result/bin/nix_as_a_service &             # Port 8081

# Or use Nix
nix run .#minimal-build-server
```

### Scenario 4: Distributed Trading Network

```bash
# Launch 10 trading nodes
./launch_trading_network.sh

# Nodes run on ports 8000-8009
# Each node trades memes via shared memory bus
```

### Scenario 5: P2P Compilation Cluster

```bash
# Start P2P cluster (from zos-server repo)
cd ~/zos-server
cargo run --bin p2p_compilation_cluster

# Nodes discover each other via libp2p
# Distributed compilation across network
```

## 📦 Binary Distribution

### Available Binaries (220 total)

The minimal build server can download and deploy any of the 220 binaries:

```bash
# List all available binaries
curl http://127.0.0.1:3000/binaries

# Download specific binary
curl -X POST http://127.0.0.1:3000/build \
  -d '{"target":"telemetry-driver", "action":"download"}'
```

### Pre-built Releases

Download from GitHub Releases:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)  
- Windows (x86_64)

### Docker Images

```bash
# Pull latest
docker pull ghcr.io/meta-introspector/meta-introspector:latest

# Run minimal build server
docker run -p 3000:8080 ghcr.io/meta-introspector/meta-introspector:latest
```

**Note:** Docker exposes port 8080 by default, maps to internal 3000.

## 🔧 Configuration

### Environment Variables

```bash
# Minimal Build Server
PORT=3000                    # Server port
RUST_LOG=info               # Log level

# Telemetry Server
TELEMETRY_SESSION_ID=...    # Session identifier
TELEMETRY_PARQUET=...       # Parquet output file

# Rust/Nix Services
RUSTC_WRAPPER=...           # Custom rustc wrapper
NIX_PATH=...                # Nix channel path
```

### Systemd Service Template

```ini
[Unit]
Description=Meta-Introspector Build Server
After=network.target

[Service]
Type=simple
User=qa-user
WorkingDirectory=/home/qa-user/builds
Environment="PORT=3001"
ExecStart=/usr/local/bin/minimal-build-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## 🔍 Monitoring & Telemetry

### Telemetry Server (Port 8888)

Collects build telemetry:
- Syscall traces
- Performance metrics
- Build events
- Symbol frequencies

```bash
# Start telemetry server
./result/bin/telemetry_server

# Clients auto-connect to 127.0.0.1:8888
```

### Error Tracking

```bash
# Get current errors
curl http://127.0.0.1:3000/errors

# Response includes:
# - Error type (missing import, type mismatch, etc.)
# - File and line number
# - Suggested fixes
```

## 🎓 Learning Path for New Users

### Step 1: Download & Start (5 minutes)

```bash
# Download minimal build server
curl -sSL https://github.com/meta-introspector/meta-introspector/releases/latest/download/minimal-build-server-linux-x86_64 -o minimal-build-server
chmod +x minimal-build-server
./minimal-build-server
```

### Step 2: Explore API (10 minutes)

```bash
# Get help
curl http://127.0.0.1:3000/help

# List available binaries
curl http://127.0.0.1:3000/binaries

# Check system status
curl http://127.0.0.1:3000/peer/info
```

### Step 3: Build Something (15 minutes)

```bash
# Clone a repo
curl -X POST http://127.0.0.1:3000/git \
  -d '{"url":"https://github.com/rust-lang/rustlings"}'

# Build it
curl -X POST http://127.0.0.1:3000/compile \
  -d '{"target":"rustlings"}'

# Check for errors
curl http://127.0.0.1:3000/errors
```

### Step 4: Deploy (20 minutes)

```bash
# Deploy to QA environment
curl -X POST http://127.0.0.1:3000/deploy \
  -d '{"target":"minimal-build-server", "port":3001}'

# Verify deployment
curl http://127.0.0.1:3001/help
```

### Step 5: Advanced Features (30+ minutes)

- Setup SSH/GPG keys
- Configure Git
- Use grep/sed for code editing
- Auto-fix compilation errors
- Integrate with P2P network
- Deploy trading nodes

## 🔐 Security Considerations

### Local Development
- Server binds to `127.0.0.1` (localhost only)
- No authentication required
- Safe for local experimentation

### Production Deployment
- Bind to `0.0.0.0` for external access
- Use systemd for service management
- Run as dedicated user (e.g., `qa-user`)
- Configure firewall rules
- Consider adding authentication layer

### Recommended Firewall Rules

```bash
# Allow only specific ports
sudo ufw allow 3001/tcp  # QA server
sudo ufw allow 8080/tcp  # Rust service
sudo ufw allow 8081/tcp  # Nix service
sudo ufw deny 8888/tcp   # Telemetry (internal only)
```

## 🐛 Troubleshooting

### Server Won't Start

```bash
# Check if port is in use
lsof -i :3000

# Kill existing process
kill -9 $(lsof -ti:3000)

# Or use auto-kill script
./launch-dev-server.sh
```

### Build Failures

```bash
# Get detailed errors
curl http://127.0.0.1:3000/errors

# Try auto-fix
curl -X POST http://127.0.0.1:3000/fix-all

# Check logs
journalctl -u qa-build-server -f
```

### Connection Issues

1. Verify server is running: `ps aux | grep minimal-build-server`
2. Check bind address: `netstat -tulpn | grep 3000`
3. Test locally: `curl http://127.0.0.1:3000/help`
4. Check firewall: `sudo ufw status`

## 📚 Related Documentation

- [SERVERS_AND_PORTS.md](SERVERS_AND_PORTS.md) - Complete port reference
- [QUICKSTART.md](../QUICKSTART.md) - 5-minute quick start
- [BUILD_FIXING_GUIDE.md](BUILD_FIXING_GUIDE.md) - Fix compilation errors
- [NIX_BUILD_READY.md](../NIX_BUILD_READY.md) - Nix build guide

## 🤝 Contributing

The minimal build server is designed to be extended. Add new endpoints by:

1. Define request/response structs
2. Implement async handler function
3. Add route in `main()` function
4. Update help endpoint documentation

Example:
```rust
#[derive(Deserialize)]
struct MyRequest {
    param: String,
}

async fn my_handler(Json(req): Json<MyRequest>) -> Json<serde_json::Value> {
    json!({"result": req.param})
}

// In main():
.route("/my-endpoint", post(my_handler))
```

## 📊 Metrics & Analytics

### Build Statistics

The server tracks:
- Total builds
- Success/failure rates
- Build times
- Error frequencies
- Most-used binaries

Access via telemetry server on port 8888.

### Performance Monitoring

```bash
# Monitor with perf
sudo perf record -p $(pgrep minimal-build-server)
sudo perf report

# Or use telemetry driver
./result/bin/telemetry-driver --monitor minimal-build-server
```

## 🎯 Roadmap

### Planned Features

- [ ] Web UI dashboard
- [ ] Authentication/authorization
- [ ] Build caching
- [ ] Distributed builds via P2P
- [ ] Kubernetes deployment
- [ ] Metrics dashboard
- [ ] Webhook integrations
- [ ] CI/CD pipeline templates

### Integration Goals

- [ ] GitHub Actions integration
- [ ] GitLab CI integration
- [ ] Jenkins plugin
- [ ] Terraform provider
- [ ] Ansible playbook
- [ ] Docker Compose templates

---

**Version:** 1.0.0  
**Last Updated:** 2026-01-17  
**Maintainer:** Meta-Introspector Team
