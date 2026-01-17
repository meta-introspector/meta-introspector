# QA Deployment Review

## Current State

### 3 Deployment Systems Found:

#### 1. **tools/deploy.sh** - Universal Deployment Manager
- Location: `/mnt/data1/meta-introspector/tools/deploy.sh`
- Features:
  - Multi-platform (Oracle, HuggingFace, Vercel, Cloudflare, local)
  - Environment support (dev, qa, prod)
  - Registry-based tracking
  - Telemetry integration
- Status: ✅ Most comprehensive

#### 2. **self-deploy.sh** - Self-Deployment Chain
- Location: `/mnt/data1/meta-introspector/self-deploy.sh`
- Features:
  - Dev → QA → Prod → Cloud chain
  - Uses running system to build next version
  - Builds from `~/zos-qa`
- Status: ✅ Working concept

#### 3. **deploy-chain.sh** - Systemd Deployment
- Location: `/mnt/data1/meta-introspector/deploy-chain.sh`
- Features:
  - Systemd service deployment
  - Local QA node setup
  - Oracle integration
- Status: ✅ Systemd-focused

### Minimal Build Server

**File**: `minimal_build_server.rs`
- Axum-based HTTP server
- Bootstrap system for loading libs via Nix
- Build request handling
- Error tracking

## Recommended Unified Approach

### Use `minimal-build-server` as QA Server

```bash
# 1. Build the server
cargo build --release --bin minimal-build-server

# 2. Deploy to QA environment
./tools/deploy.sh deploy local qa-node qa

# 3. Start as systemd service
sudo systemctl start minimal-build-server-qa
```

### Unified Deployment Script

Create: `deploy-qa.sh`

```bash
#!/bin/bash
# Deploy minimal-build-server to QA environment

set -euo pipefail

# Build
echo "📦 Building minimal-build-server..."
cargo build --release --bin minimal-build-server

# Deploy using universal deployer
echo "🚀 Deploying to QA..."
./tools/deploy.sh deploy local qa-node qa

# Configure as systemd service
echo "⚙️  Setting up systemd service..."
sudo tee /etc/systemd/system/minimal-build-server-qa.service <<EOF
[Unit]
Description=Minimal Build Server (QA)
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$PWD
ExecStart=$PWD/target/release/minimal-build-server
Restart=always
Environment="RUST_LOG=info"
Environment="PORT=3001"

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable minimal-build-server-qa
sudo systemctl start minimal-build-server-qa

echo "✅ QA server deployed and running on port 3001"
echo "   Status: sudo systemctl status minimal-build-server-qa"
echo "   Logs: sudo journalctl -u minimal-build-server-qa -f"
```

## Next Steps

1. ✅ Consolidate deployment scripts
2. ✅ Use minimal-build-server as standard QA server
3. ✅ Integrate with tools/deploy.sh for multi-environment support
4. ✅ Add health checks and monitoring
5. ✅ Document API endpoints
