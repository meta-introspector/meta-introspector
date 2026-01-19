#!/bin/bash
# Setup GitHub caching proxy (explicit configuration, not iptables)

PROXY_PORT=8080
CACHE_DIR="$HOME/.github-cache"

mkdir -p "$CACHE_DIR"

echo "🔧 Configuring git to use caching proxy..."

# Configure git to use proxy (explicit, not intercepting)
git config --global http.proxy "http://127.0.0.1:$PROXY_PORT"
git config --global https.proxy "http://127.0.0.1:$PROXY_PORT"

# Or per-repo:
# git config http.proxy "http://127.0.0.1:$PROXY_PORT"

echo "✅ Git configured to use proxy on port $PROXY_PORT"
echo ""
echo "To disable:"
echo "  git config --global --unset http.proxy"
echo "  git config --global --unset https.proxy"
