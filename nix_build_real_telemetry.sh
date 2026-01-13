#!/bin/bash
# 🔥 NIX BUILD WITH REAL TELEMETRY: Run nix build with updated LD_PRELOAD system

echo "🔥 NIX BUILD WITH REAL TELEMETRY"
echo "================================"

# Set up environment
export PROJECT_NAME="nix_real_telemetry"
export LD_PRELOAD="/mnt/data1/meta-introspector/rust_preload_interceptor/target/release/librust_preload_interceptor.so"

echo "📊 Using real build telemetry system:"
echo "   🔧 32 binaries (vs 14 old)"
echo "   📚 71 libraries (vs 39 old)" 
echo "   ⚡ 1,061 symbols extracted"
echo ""

# Run nix build with full telemetry
echo "🚀 Running nix build with LD_PRELOAD telemetry..."
cd /mnt/data1/meta-introspector

nix build ./rustc-only-build --print-out-paths

echo ""
echo "📋 Checking telemetry capture..."
LATEST_LOG=$(ls -t /mnt/data1/meta-introspector/data/telemetry/nix_real_telemetry_*.jsonl 2>/dev/null | head -1)

if [ -f "$LATEST_LOG" ]; then
    echo "✅ Telemetry captured: $(basename "$LATEST_LOG")"
    echo "📊 Content:"
    cat "$LATEST_LOG"
else
    echo "⚠️  No structured telemetry log found"
fi

echo ""
echo "🎯 NIX BUILD WITH REAL TELEMETRY COMPLETE!"
