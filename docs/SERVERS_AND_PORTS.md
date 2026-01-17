# Servers and Ports Documentation

## Active Services

### Core Build Services

| Service | Port | Address | Binary | Description |
|---------|------|---------|--------|-------------|
| **Minimal Build Server** | 3000 | 127.0.0.1 | `minimal-build-server` | Main build orchestration server |
| **QA Build Server** | 3001 | 0.0.0.0 | `minimal-build-server` | QA environment build server |
| **Rust-as-a-Service** | 8080 | 0.0.0.0 | `rust_as_a_service` | Rust compilation service |
| **Nix-as-a-Service** | 8081 | 0.0.0.0 | `nix_as_a_service` | Nix build service with MCP |

### Telemetry & Monitoring

| Service | Port | Address | Binary | Description |
|---------|------|---------|--------|-------------|
| **Telemetry Server** | 8888 | 127.0.0.1 | `telemetry_server` | Build telemetry collection |

### Trading Network (Demo)

| Service | Port Range | Address | Binary | Description |
|---------|------------|---------|--------|-------------|
| **Trading Nodes** | 8000-8009 | 0.0.0.0 | `trading_node` | Distributed meme trading network (10 nodes) |

### External Services (Referenced)

| Service | Port | Address | Description |
|---------|------|---------|-------------|
| **Ollama** | 11434 | localhost | Local LLM inference |
| **Solana Devnet** | 8899 | localhost | Solana RPC endpoint |
| **Solana WebSocket** | 8900 | localhost | Solana WebSocket endpoint |

## Docker Configuration

**Default Exposed Port:** 8080

**Container Command:** `minimal-build-server`

**Multi-platform Support:**
- linux/amd64
- linux/arm64

## Service Endpoints

### Minimal Build Server (Port 3000)

```bash
# Compile endpoint
curl -X POST http://127.0.0.1:3000/compile -d '{"target":"foo"}'

# Restart endpoint
curl -X POST http://127.0.0.1:3000/restart

# Git clone endpoint
curl -X POST http://127.0.0.1:3000/git -d '{"url":"..."}'
```

### QA Build Server (Port 3001)

Systemd service configuration:
- **Service Name:** `qa-build-server.service`
- **User:** `qa-user`
- **Working Directory:** `/home/qa-user/builds`
- **Environment:** `PORT=3001`

### Rust-as-a-Service (Port 8080)

HTTP API for Rust compilation requests.

### Nix-as-a-Service (Port 8081)

Unified Nix build service with:
- MCP (Model Context Protocol) integration
- Solana payment support
- Content-addressed builds

### Telemetry Server (Port 8888)

TCP server collecting build telemetry data:
- Syscall traces
- Performance metrics
- Build events

**Client Connection:**
```rust
TcpStream::connect("127.0.0.1:8888")
```

## Port Conflict Resolution

### Auto-kill Script

The `launch-dev-server.sh` script automatically kills processes on port 3000:

```bash
# Check for existing server
lsof -ti:3000 | xargs kill -9 2>/dev/null
```

### Manual Port Check

```bash
# Check what's using a port
lsof -i :3000
netstat -tulpn | grep :3000

# Kill process on port
kill -9 $(lsof -ti:3000)
```

## Service Management

### Systemd Services

**QA Build Server:**
```bash
sudo systemctl start qa-build-server
sudo systemctl status qa-build-server
sudo systemctl stop qa-build-server
```

### Manual Launch

**Minimal Build Server:**
```bash
./result/bin/minimal-build-server
# Listens on 127.0.0.1:3000
```

**Telemetry Server:**
```bash
./result/bin/telemetry_server
# Listens on 127.0.0.1:8888
```

**Trading Network:**
```bash
./launch_trading_network.sh
# Spawns 10 nodes on ports 8000-8009
```

## Network Configuration

### Localhost vs 0.0.0.0

- **127.0.0.1 (localhost):** Local-only access
  - `minimal-build-server` (3000)
  - `telemetry_server` (8888)

- **0.0.0.0 (all interfaces):** External access allowed
  - `qa-build-server` (3001)
  - `rust_as_a_service` (8080)
  - `nix_as_a_service` (8081)
  - `universal_client_node` (3000)
  - `trading_node` (8000-8009)

### Firewall Considerations

For production deployments, restrict access:

```bash
# Allow only specific ports
sudo ufw allow 3001/tcp  # QA server
sudo ufw allow 8080/tcp  # Rust service
sudo ufw allow 8081/tcp  # Nix service
```

## Development vs Production

### Development (Local)
- Services bind to `127.0.0.1`
- No authentication required
- Auto-restart on port conflicts

### Production (QA/Staging)
- Services bind to `0.0.0.0`
- Systemd service management
- User isolation (`qa-user`)
- Port 3001 for QA builds

## Port Assignment Strategy

| Range | Purpose |
|-------|---------|
| 3000-3099 | Build servers |
| 8000-8099 | Application services |
| 8888 | Telemetry/monitoring |
| 11000+ | External integrations |

## Troubleshooting

### Port Already in Use

```bash
# Find process
lsof -i :3000

# Kill and restart
./launch-dev-server.sh  # Auto-kills old process
```

### Service Won't Start

```bash
# Check logs
journalctl -u qa-build-server -f

# Check port availability
netstat -tulpn | grep LISTEN
```

### Connection Refused

1. Verify service is running: `systemctl status <service>`
2. Check firewall: `sudo ufw status`
3. Verify bind address (127.0.0.1 vs 0.0.0.0)
4. Check logs for startup errors

## Related Files

- `launch-dev-server.sh` - Auto-restart dev server on port 3000
- `setup-qa-user.sh` - QA server setup on port 3001
- `deploy-qa.sh` - QA deployment automation
- `launch_trading_network.sh` - Trading network on ports 8000-8009
- `minimal_build_server.rs` - Main build server implementation
- `telemetry_server.rs` - Telemetry collection server
- `rust_as_a_service.rs` - Rust compilation service
- `nix_as_a_service.rs` - Nix build service
- `Dockerfile` - Container configuration (exposes 8080)
