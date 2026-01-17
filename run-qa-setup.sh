#!/bin/bash
# Run this script - it will ask for your password when needed
# This sets up the QA user with SSH/GPG keys and vault

set -euo pipefail

echo "🔧 QA User Setup Script"
echo "======================="
echo "This will create a 'qa' user with SSH/GPG keys and secure vault"
echo ""

# Create QA user
echo "Creating QA user..."
sudo useradd -m -s /bin/bash qa 2>/dev/null || echo "QA user already exists"

# Generate PIN
PIN=$(openssl rand -hex 4)
echo "Generated secure PIN (stored in vault only)"

# Setup SSH key
echo "Generating SSH key..."
sudo -u qa bash -c '
mkdir -p ~/.ssh
chmod 700 ~/.ssh
if [ ! -f ~/.ssh/id_ed25519 ]; then
    ssh-keygen -t ed25519 -C "qa@meta-introspector" -f ~/.ssh/id_ed25519 -N ""
fi
'

# Setup GPG key
echo "Generating GPG key..."
sudo -u qa bash -c '
cat > /tmp/gpg-qa << "EOF"
Key-Type: RSA
Key-Length: 4096
Name-Real: QA User
Name-Email: qa@meta-introspector
Expire-Date: 0
%no-protection
%commit
EOF
gpg --batch --generate-key /tmp/gpg-qa 2>/dev/null || true
rm -f /tmp/gpg-qa
'

# Create vault
echo "Creating secure vault..."
sudo -u qa bash -c "
mkdir -p ~/.vault
chmod 700 ~/.vault
cat > ~/.vault/credentials.json << EOF
{
  \"user\": \"qa\",
  \"pin\": \"$PIN\",
  \"created\": \"$(date -Iseconds)\",
  \"ssh_key\": \"/home/qa/.ssh/id_ed25519\",
  \"ssh_pubkey\": \"/home/qa/.ssh/id_ed25519.pub\"
}
EOF
chmod 600 ~/.vault/credentials.json
"

# Configure Git
echo "Configuring Git..."
sudo -u qa bash -c '
git config --global user.name "QA User"
git config --global user.email "qa@meta-introspector"
'

# Create systemd service
echo "Creating systemd service..."
sudo tee /etc/systemd/system/qa-build-server.service > /dev/null << EOF
[Unit]
Description=QA Build Server
After=network.target

[Service]
Type=simple
User=qa
WorkingDirectory=/home/qa
ExecStart=/mnt/data1/meta-introspector/target/release/minimal-build-server
Restart=always
Environment="PORT=3001"
Environment="ENV=qa"

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable qa-build-server
sudo systemctl start qa-build-server

echo ""
echo "✅ Setup Complete!"
echo "=================="
echo ""
echo "QA User: qa"
echo "PIN: (stored securely in vault)"
echo "SSH Key: /home/qa/.ssh/id_ed25519.pub"
echo "Vault: /home/qa/.vault/credentials.json"
echo "Service: qa-build-server (port 3001)"
echo ""
echo "To view PIN: sudo cat /home/qa/.vault/credentials.json | jq -r .pin"
echo "Test: curl http://localhost:3001/health"
