#!/usr/bin/env bash
# 🚀 Meta-Introspector Bootstrap - Self-building system

set -euo pipefail

echo "🚀 Meta-Introspector Bootstrap"
echo "================================"

# Build all components
echo "📦 Building all binaries..."
nix build .#meta-introspector-binaries
nix build .#zos-server
nix build .#rust-telemetry-driver

# Start zos-server
echo "🌐 Starting ZOS server..."
./result-2/bin/zos-server &
ZOS_PID=$!

# Wait for server to start
sleep 2

# Use server to build itself
echo "🔄 Self-building via ZOS server..."
curl -X POST http://localhost:8080/build \
  -H "Content-Type: application/json" \
  -d '{
    "flake_url": ".",
    "package": "meta-introspector-binaries"
  }'

# Generate report
echo "📊 Generating build report..."
./result/bin/investor-report-2025

echo "✅ Bootstrap complete!"
echo "🌐 ZOS server running on PID: $ZOS_PID"
