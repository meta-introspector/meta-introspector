#!/usr/bin/env bash
# 🚀 Minimal Bootstrap - Server builds itself

set -euo pipefail

echo "🚀 Minimal Bootstrap"
echo "===================="

# Start minimal server
echo "🌐 Starting minimal build server..."
cargo run --bin minimal-build-server &
SERVER_PID=$!
sleep 2

# Use server to build everything
echo "📦 Building all binaries via server..."

BINARIES=(
    "investor-report-2025"
    "git-sources"
    "github-activity-scanner"
)

for bin in "${BINARIES[@]}"; do
    echo "🔨 Building $bin..."
    curl -s -X POST http://127.0.0.1:3000/build \
        -H "Content-Type: application/json" \
        -d "{\"target\": \"$bin\"}" | jq -r '.errors[]' || true
done

echo "✅ Bootstrap complete!"
echo "🌐 Server PID: $SERVER_PID"
echo "🛑 Kill with: kill $SERVER_PID"
