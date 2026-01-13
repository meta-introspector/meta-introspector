#!/bin/bash

# 🔥 NIX REBUILD TELEMETRY CAPTURE
# Captures structured telemetry from nix rebuild process

set -e

TELEMETRY_DIR="/mnt/data1/meta-introspector/data/telemetry"
PROJECT_NAME="nix_rebuild"
TIMESTAMP=$(date +%s)
LOG_FILE="$TELEMETRY_DIR/nix_rebuild_${TIMESTAMP}.log"

echo "🔥 NIX REBUILD TELEMETRY CAPTURE"
echo "================================="
echo "📊 Project: $PROJECT_NAME"
echo "📁 Telemetry dir: $TELEMETRY_DIR"
echo "📄 Log file: $LOG_FILE"
echo ""

# Create telemetry directory
mkdir -p "$TELEMETRY_DIR"

# Export project name for structured logging
export PROJECT_NAME="$PROJECT_NAME"

echo "🚀 Starting nix rebuild with telemetry..."
cd /mnt/data1/meta-introspector

# Run with both LD_PRELOAD and macro telemetry
cargo run --bin custom_rust_nightly_build 2>&1 | tee "$LOG_FILE"

echo ""
echo "✅ NIX REBUILD COMPLETE"
echo "📊 Check telemetry files:"
ls -la "$TELEMETRY_DIR"/*.jsonl 2>/dev/null || echo "No JSONL files found"
echo "📄 Full log: $LOG_FILE"
