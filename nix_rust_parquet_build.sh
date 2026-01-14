#!/usr/bin/env bash
# Nix Rust Build with Parquet Telemetry Streaming

set -e

SESSION_ID="nix_rust_beta_$(date +%s)"
OUTPUT_DIR="/mnt/data1/meta-introspector/data/parquet_telemetry"
PARQUET_FILE="$OUTPUT_DIR/${SESSION_ID}.parquet"

mkdir -p "$OUTPUT_DIR"

echo "🚀 Nix Rust Beta Build with Parquet Telemetry"
echo "Session: $SESSION_ID"
echo "Output: $PARQUET_FILE"
echo ""

# Build our telemetry library first
echo "✅ Using telemetry library: /mnt/data1/nix-telemetry/target/release/libnix_telemetry.so"
echo ""

TELEMETRY_LIB="/mnt/data1/nix-telemetry/target/release/libnix_telemetry.so"

if [ ! -f "$TELEMETRY_LIB" ]; then
    echo "❌ Telemetry library not found: $TELEMETRY_LIB"
    exit 1
fi

echo "✅ Telemetry library ready: $TELEMETRY_LIB"
echo ""

# Set up environment for telemetry capture
export LD_PRELOAD="$TELEMETRY_LIB"
export TELEMETRY_SESSION_ID="$SESSION_ID"
export TELEMETRY_PARQUET="$PARQUET_FILE"
export RUST_BACKTRACE=1

echo "🎯 Starting Nix build with LD_PRELOAD telemetry..."
echo "   LD_PRELOAD=$TELEMETRY_LIB"
echo ""

# Run Nix build with telemetry
cd /mnt/data1/meta-introspector/rust-overlay-test

# Use timeout to limit build time for proof-of-concept
timeout 300 nix build .#rustNightlyProfiling --print-build-logs 2>&1 | tee "$OUTPUT_DIR/${SESSION_ID}_build.log" || {
    echo ""
    echo "⏱️  Build timeout (5 min) - captured telemetry so far"
}

echo ""
echo "✅ Telemetry capture complete"
echo "📊 Results:"
ls -lh "$OUTPUT_DIR/${SESSION_ID}"* 2>/dev/null || echo "  (files in $OUTPUT_DIR)"
echo ""
echo "🎯 Next: Analyze Parquet data to see captured .so calls"
