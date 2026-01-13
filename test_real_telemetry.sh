#!/bin/bash
# 🔥 TEST REAL BUILD TELEMETRY: Build preload lib and test updated telemetry

echo "🔥 TESTING REAL BUILD TELEMETRY"
echo "==============================="

# 1. Build the LD_PRELOAD interceptor
echo "🔧 Building LD_PRELOAD interceptor..."
cd /mnt/data1/meta-introspector/rust_preload_interceptor
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Failed to build LD_PRELOAD interceptor"
    exit 1
fi

echo "✅ LD_PRELOAD interceptor built successfully"

# 2. Test the updated telemetry system
echo ""
echo "📊 Testing updated telemetry with real build data..."
cd /mnt/data1/meta-introspector

PROJECT_NAME=real_build_test \
LD_PRELOAD=/mnt/data1/meta-introspector/rust_preload_interceptor/target/release/librust_preload_interceptor.so \
/nix/store/3hgackxpbkjachs6qncykjbl0n9a2yla-rustc-1.94.0-nightly-2026-01-12-x86_64-unknown-linux-gnu/bin/rustc --version

echo ""
echo "📋 Checking telemetry output..."
LATEST_LOG=$(ls -t /mnt/data1/meta-introspector/data/telemetry/real_build_test_*.jsonl 2>/dev/null | head -1)

if [ -f "$LATEST_LOG" ]; then
    echo "✅ Telemetry captured: $LATEST_LOG"
    echo "📊 Content:"
    cat "$LATEST_LOG"
else
    echo "❌ No telemetry log found"
fi

echo ""
echo "🎯 TEST COMPLETE!"
