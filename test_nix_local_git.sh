#!/bin/bash
# Test nix flake update with local git mirror

# Create temporary git config
GIT_CONFIG=$(mktemp)
trap "rm -f $GIT_CONFIG" EXIT

cat > "$GIT_CONFIG" << 'EOF'
[url "file:///mnt/data1/git/github.com/"]
  insteadOf = https://github.com/
  insteadOf = git@github.com:
[url "file:///mnt/data1/git/gitlab.com/"]
  insteadOf = https://gitlab.com/
EOF

echo "🧪 Testing nix flake update with local git..."
echo "Git config: $GIT_CONFIG"
echo ""

cd /mnt/data1/meta-introspector

# Run with local git config
GIT_CONFIG_GLOBAL="$GIT_CONFIG" \
GIT_CONFIG_SYSTEM=/dev/null \
  nix flake update --verbose 2>&1 | grep -E "(downloading|file://|github.com)"
