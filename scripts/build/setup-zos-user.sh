#!/bin/bash
# Configure zos user for nix operations with git cache

set -e

echo "🔧 Setting up zos user for nix operations..."

# Create zos user if doesn't exist
if ! id -u zos &>/dev/null; then
    echo "Creating zos user..."
    sudo useradd -r -m -d /home/zos -s /bin/bash zos
    echo "✅ zos user created"
else
    echo "✅ zos user exists"
fi

# Create zos nix config directory
sudo mkdir -p /home/zos/.config/nix
sudo chown -R zos:zos /home/zos/.config

# Configure zos to use git cache
echo "Configuring zos nix.conf..."
sudo tee /home/zos/.config/nix/nix.conf > /dev/null << 'EOF'
# ZOS Nix Configuration
# System user for reproducible builds with local git cache

experimental-features = nix-command flakes
connect-timeout = 5
stalled-download-timeout = 10
url-literals = false

# ===== Local Git Mirror Configuration =====
EOF

# Add all git cache hosts
count=0
for host in /mnt/data1/git/*/; do
  hostname=$(basename "$host")
  
  # Skip non-host directories
  [[ "$hostname" == "links" ]] && continue
  [[ "$hostname" == "data" ]] && continue
  [[ "$hostname" == "file" ]] && continue
  [[ "$hostname" == "home" ]] && continue
  [[ "$hostname" == "git" ]] && continue
  [[ "$hostname" == "ssh" ]] && continue
  [[ "$hostname" =~ ^git\+ssh$ ]] && continue
  [[ "$hostname" =~ ^ssh\+ ]] && continue
  [[ "$hostname" =~ @ ]] && continue
  
  # Add rewrites
  echo "git-config = url.file:///mnt/data1/git/$hostname/.insteadOf=https://$hostname/" | sudo tee -a /home/zos/.config/nix/nix.conf > /dev/null
  echo "git-config = url.file:///mnt/data1/git/$hostname/.insteadOf=git://$hostname/" | sudo tee -a /home/zos/.config/nix/nix.conf > /dev/null
  echo "git-config = url.file:///mnt/data1/git/$hostname/.insteadOf=git@$hostname:" | sudo tee -a /home/zos/.config/nix/nix.conf > /dev/null
  
  ((count++))
done

echo "" | sudo tee -a /home/zos/.config/nix/nix.conf > /dev/null
echo "# ===== End Local Git Mirror =====" | sudo tee -a /home/zos/.config/nix/nix.conf > /dev/null

sudo chown zos:zos /home/zos/.config/nix/nix.conf

echo "✅ Configured $count hosts for zos user"

# Give zos read access to git cache
echo "Setting permissions on git cache..."
sudo chmod -R a+rX /mnt/data1/git
echo "✅ Git cache readable by zos"

# Give zos write access to nix store (if needed)
if [ -d /nix/store ]; then
    echo "✅ Nix store accessible"
fi

# Create wrapper script for running nix commands as zos
cat > /tmp/nix-as-zos.sh << 'EOF'
#!/bin/bash
# Run nix commands as zos user with clean environment

if [ $# -eq 0 ]; then
    echo "Usage: nix-as-zos <nix-command> [args...]"
    echo ""
    echo "Examples:"
    echo "  nix-as-zos flake update"
    echo "  nix-as-zos build .#default"
    echo "  nix-as-zos develop"
    exit 1
fi

# Run as zos with clean environment
sudo -u zos -i nix "$@"
EOF

sudo mv /tmp/nix-as-zos.sh /usr/local/bin/nix-as-zos
sudo chmod +x /usr/local/bin/nix-as-zos

echo ""
echo "✅ Setup complete!"
echo ""
echo "Usage:"
echo "  nix-as-zos flake update    # Uses git cache, no rate limits"
echo "  nix-as-zos build .#default # Clean environment"
echo ""
echo "User separation:"
echo "  mdupont: Normal git (github.com)"
echo "  zos:     Nix operations (local cache)"
