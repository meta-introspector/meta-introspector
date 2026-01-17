#!/bin/bash
# Setup QA user with SSH/GPG keys and secure vault
set -euo pipefail

QA_USER="qa"
QA_HOME="/home/$QA_USER"
VAULT_DIR="$QA_HOME/.vault"

echo "🔧 Setting up QA user with secure keys"
echo "======================================="

# 1. Create QA user if doesn't exist
if ! id "$QA_USER" &>/dev/null; then
    echo "📝 Creating QA user..."
    sudo useradd -m -s /bin/bash "$QA_USER"
    echo "✅ QA user created"
else
    echo "✅ QA user already exists"
fi

# 2. Generate PIN
echo ""
echo "🔐 Generating secure PIN..."
PIN=$(openssl rand -hex 4)
echo "PIN: $PIN"

# 3. Setup SSH key
echo ""
echo "🔑 Generating SSH key..."
sudo -u "$QA_USER" bash << EOF
mkdir -p "$QA_HOME/.ssh"
chmod 700 "$QA_HOME/.ssh"

if [ ! -f "$QA_HOME/.ssh/id_ed25519" ]; then
    ssh-keygen -t ed25519 -C "qa@meta-introspector" -f "$QA_HOME/.ssh/id_ed25519" -N ""
    echo "✅ SSH key generated"
else
    echo "✅ SSH key already exists"
fi
EOF

# 4. Setup GPG key
echo ""
echo "🔐 Generating GPG key..."
sudo -u "$QA_USER" bash << EOF
cat > /tmp/gpg-batch-qa << 'GPGEOF'
Key-Type: RSA
Key-Length: 4096
Name-Real: QA User
Name-Email: qa@meta-introspector
Expire-Date: 0
%no-protection
%commit
GPGEOF

gpg --batch --generate-key /tmp/gpg-batch-qa 2>/dev/null || echo "GPG key may already exist"
rm -f /tmp/gpg-batch-qa
echo "✅ GPG key generated"
EOF

# 5. Create secure vault
echo ""
echo "🗄️  Creating secure vault..."
sudo -u "$QA_USER" bash << EOF
mkdir -p "$VAULT_DIR"
chmod 700 "$VAULT_DIR"

# Store credentials
cat > "$VAULT_DIR/credentials.json" << VAULTEOF
{
  "user": "$QA_USER",
  "pin": "$PIN",
  "created": "$(date -Iseconds)",
  "ssh_key": "$QA_HOME/.ssh/id_ed25519",
  "ssh_pubkey": "$QA_HOME/.ssh/id_ed25519.pub",
  "gpg_email": "qa@meta-introspector"
}
VAULTEOF

chmod 600 "$VAULT_DIR/credentials.json"
echo "✅ Vault created"
EOF

# 6. Get SSH public key
echo ""
echo "📋 SSH Public Key:"
sudo cat "$QA_HOME/.ssh/id_ed25519.pub"

# 7. Get GPG key ID
echo ""
echo "📋 GPG Key ID:"
sudo -u "$QA_USER" gpg --list-keys --keyid-format LONG qa@meta-introspector 2>/dev/null | grep pub || echo "Run: sudo -u qa gpg --list-keys"

# 8. Configure Git
echo ""
echo "⚙️  Configuring Git..."
sudo -u "$QA_USER" bash << EOF
git config --global user.name "QA User"
git config --global user.email "qa@meta-introspector"
echo "✅ Git configured"
EOF

# 9. Deploy QA service via API
echo ""
echo "🚀 Deploying QA service..."
curl -s -X POST http://localhost:3000/api/deploy \
  -H "Content-Type: application/json" \
  -d '{
    "target": "local",
    "port": 3001,
    "env": "qa",
    "user": "qa"
  }' | jq .

# 10. Create systemd service for QA
echo ""
echo "⚙️  Creating systemd service..."
sudo tee /etc/systemd/system/qa-build-server.service > /dev/null <<EOF
[Unit]
Description=QA Build Server
After=network.target

[Service]
Type=simple
User=$QA_USER
WorkingDirectory=$QA_HOME
ExecStart=/mnt/data1/meta-introspector/target/release/minimal-build-server
Restart=always
RestartSec=10
Environment="PORT=3001"
Environment="RUST_LOG=info"
Environment="ENV=qa"

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable qa-build-server
sudo systemctl start qa-build-server

echo ""
echo "✅ QA User Setup Complete!"
echo "=========================="
echo ""
echo "User: $QA_USER"
echo "Home: $QA_HOME"
echo "PIN: $PIN"
echo "Vault: $VAULT_DIR/credentials.json"
echo "Service: qa-build-server.service"
echo "Port: 3001"
echo ""
echo "Commands:"
echo "  sudo -u qa -i              # Login as QA user"
echo "  sudo systemctl status qa-build-server"
echo "  curl http://localhost:3001/health"
echo ""
echo "Vault access:"
echo "  sudo cat $VAULT_DIR/credentials.json"
EOF

chmod +x setup-qa-user.sh
./setup-qa-user.sh
