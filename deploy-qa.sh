#!/bin/bash
# Deploy minimal-build-server to QA environment
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

echo "🚀 QA Deployment: minimal-build-server"
echo "======================================"

# Build
echo "📦 Building minimal-build-server..."
cargo build --release --bin minimal-build-server

# Test binary
if [ ! -x target/release/minimal-build-server ]; then
    echo "❌ Build failed"
    exit 1
fi
echo "✅ Build successful"

# Deploy using universal deployer
echo ""
echo "🚀 Deploying to QA environment..."
./tools/deploy.sh deploy local qa-node qa

# Create systemd service
echo ""
echo "⚙️  Setting up systemd service..."
sudo tee /etc/systemd/system/minimal-build-server-qa.service > /dev/null <<EOF
[Unit]
Description=Minimal Build Server (QA)
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$ROOT
ExecStart=$ROOT/target/release/minimal-build-server
Restart=always
RestartSec=10
Environment="RUST_LOG=info"
Environment="PORT=3001"
Environment="ENV=qa"

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable minimal-build-server-qa
sudo systemctl restart minimal-build-server-qa

# Wait for startup
sleep 2

# Check status
echo ""
echo "📊 Service Status:"
sudo systemctl status minimal-build-server-qa --no-pager || true

# Test endpoint
echo ""
echo "🧪 Testing endpoint..."
if curl -s http://localhost:3001/health > /dev/null 2>&1; then
    echo "✅ Server is responding"
else
    echo "⚠️  Server not responding yet (may still be starting)"
fi

echo ""
echo "✅ QA Deployment Complete!"
echo ""
echo "Commands:"
echo "  Status:  sudo systemctl status minimal-build-server-qa"
echo "  Logs:    sudo journalctl -u minimal-build-server-qa -f"
echo "  Stop:    sudo systemctl stop minimal-build-server-qa"
echo "  Restart: sudo systemctl restart minimal-build-server-qa"
echo ""
echo "Endpoints:"
echo "  Health:  http://localhost:3001/health"
echo "  Build:   http://localhost:3001/build"
