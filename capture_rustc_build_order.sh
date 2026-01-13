#!/bin/bash
# capture_rustc_build_order.sh - Capture real rustc build order using interceptor
# 
# This script:
# 1. Sets up rustc interceptor to hijack cargo build
# 2. Runs cargo build to capture actual dependency order
# 3. Saves build order data and logs
# 4. Processes the captured order with compressed archives
#
# Usage: ./capture_rustc_build_order.sh [rust_project_path]
# Default: Uses rust-build submodule if no path provided

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUSTC_INTERCEPTOR="$SCRIPT_DIR/target/debug/rustc_interceptor"
DEFAULT_RUST_PROJECT="/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build"
RUST_PROJECT="${1:-$DEFAULT_RUST_PROJECT}"
OUTPUT_DIR="$SCRIPT_DIR/build_order_capture"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "🚀 RUSTC BUILD ORDER CAPTURE"
echo "============================="
echo "Project: $RUST_PROJECT"
echo "Interceptor: $RUSTC_INTERCEPTOR"
echo "Output: $OUTPUT_DIR"
echo "Timestamp: $TIMESTAMP"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Build rustc interceptor if needed
if [ ! -f "$RUSTC_INTERCEPTOR" ]; then
    echo "📦 Building rustc interceptor..."
    cd "$SCRIPT_DIR"
    cargo build --bin rustc_interceptor
fi

# Capture build order
echo "🔄 Capturing build order from: $RUST_PROJECT"
cd "$RUST_PROJECT"

# Set up environment
export RUSTC="$RUSTC_INTERCEPTOR"
export REAL_RUSTC="rustc"

# Run cargo build and capture output
echo "⏱️  Starting cargo build (this may take several minutes)..."
cargo build --verbose > "$OUTPUT_DIR/full_cargo_build_${TIMESTAMP}.log" 2>&1 || true
BUILD_EXIT_CODE=$?

echo "✅ Build completed with exit code: $BUILD_EXIT_CODE"
echo "📄 Full build log saved to: $OUTPUT_DIR/full_cargo_build_${TIMESTAMP}.log"

# Check if intercept data was created
if [ -f "rustc_intercept_compression.json" ]; then
    # Copy intercept data to output directory
    cp "rustc_intercept_compression.json" "$OUTPUT_DIR/rustc_intercept_${TIMESTAMP}.json"
    
    # Also copy to main directory for processing
    cp "rustc_intercept_compression.json" "$SCRIPT_DIR/"
    
    echo "✅ Build order captured successfully!"
    
    # Show summary
    FILES_COUNT=$(jq '.files | length' "$OUTPUT_DIR/rustc_intercept_${TIMESTAMP}.json")
    COMPRESSION_RATIO=$(jq '.compression_ratio' "$OUTPUT_DIR/rustc_intercept_${TIMESTAMP}.json")
    
    echo "📊 CAPTURE SUMMARY:"
    echo "Files in build order: $FILES_COUNT"
    echo "Compression ratio: $COMPRESSION_RATIO"
    
    # Check for rustc_hir
    if jq -r '.files[][0]' "$OUTPUT_DIR/rustc_intercept_${TIMESTAMP}.json" | grep -q "rustc_hir"; then
        echo "✅ rustc_hir files found in build order"
    else
        echo "⚠️  rustc_hir files not found (build may have been incomplete)"
    fi
    
    # Process with compressed archives
    echo "🔄 Processing with compressed archives..."
    cd "$SCRIPT_DIR"
    cargo run --bin build_order_processor > "$OUTPUT_DIR/processing_${TIMESTAMP}.log" 2>&1
    
    if [ -f "real_build_order_declarations.json" ]; then
        mv "real_build_order_declarations.json" "$OUTPUT_DIR/declarations_${TIMESTAMP}.json"
        echo "✅ Archive processing completed!"
    else
        echo "⚠️  Archive processing failed - check logs"
    fi
    
else
    echo "❌ No intercept data found - build may have failed"
    echo "Check logs: $OUTPUT_DIR/cargo_build_${TIMESTAMP}.log"
fi

echo ""
echo "📁 OUTPUT FILES:"
ls -la "$OUTPUT_DIR/"*${TIMESTAMP}*

echo ""
echo "🎯 NEXT STEPS:"
echo "1. Check build order: cat $OUTPUT_DIR/rustc_intercept_${TIMESTAMP}.json"
echo "2. Review declarations: cat $OUTPUT_DIR/declarations_${TIMESTAMP}.json"
echo "3. Check logs if needed: cat $OUTPUT_DIR/cargo_build_${TIMESTAMP}.log"
