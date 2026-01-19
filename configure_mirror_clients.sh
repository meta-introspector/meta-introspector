#!/bin/bash
# Configure Nix and Cargo to use local GitHub mirror

MIRROR_URL="http://127.0.0.1:9418"  # Git protocol
MIRROR_PATH="/mnt/data1/github-mirror"

echo "🔧 Configuring build tools to use GitHub mirror..."

# 1. Configure git globally
git config --global url."$MIRROR_URL/".insteadOf "https://github.com/"
git config --global url."$MIRROR_URL/".insteadOf "git@github.com:"

# 2. Configure Cargo
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml <<EOF

[source.crates-io]
replace-with = "mirror"

[source.mirror]
registry = "https://github.com/rust-lang/crates.io-index"

[net]
git-fetch-with-cli = true

# Redirect all GitHub URLs to local mirror
[net.git-fetch-with-cli]
enable = true
EOF

# 3. Configure Nix
mkdir -p ~/.config/nix
cat >> ~/.config/nix/nix.conf <<EOF

# Use local GitHub mirror
substituters = http://127.0.0.1:5000
trusted-public-keys = 

# Git settings
tarball-ttl = 0
EOF

echo "✅ Configuration complete"
echo ""
echo "To revert:"
echo "  git config --global --unset url.\"$MIRROR_URL/\".insteadOf"
