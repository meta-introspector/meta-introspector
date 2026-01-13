#!/bin/bash

# Build all compiler crates individually to capture full rustc build order

PROJECT_DIR="/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build"
INTERCEPTOR="/mnt/data1/meta-introspector/target/debug/rustc_interceptor"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="/mnt/data1/meta-introspector/all_crates_build_${TIMESTAMP}.log"

echo "🚀 BUILDING ALL RUSTC CRATES"
echo "============================="
echo "Project: $PROJECT_DIR"
echo "Interceptor: $INTERCEPTOR" 
echo "Log: $LOG_FILE"

# Set up environment
export RUSTC="$INTERCEPTOR"
export REAL_RUSTC="rustc"

cd "$PROJECT_DIR"

# Clean previous log
rm -f rustc_build_log.jsonl

echo "📦 Found compiler crates:"
find compiler -name Cargo.toml | head -20

echo ""
echo "🔨 Building each crate..."

# Build each compiler crate
for crate_dir in compiler/*/; do
    if [ -f "$crate_dir/Cargo.toml" ]; then
        crate_name=$(basename "$crate_dir")
        echo "Building $crate_name..."
        cd "$PROJECT_DIR/$crate_dir"
        cargo build --verbose >> "$LOG_FILE" 2>&1 || true
        cd "$PROJECT_DIR"
    fi
done

echo "✅ All crates processed!"
echo "📊 Total rustc invocations: $(wc -l < rustc_build_log.jsonl 2>/dev/null || echo 0)"
echo "📄 Build log: $LOG_FILE"
echo "📄 Rustc log: $PROJECT_DIR/rustc_build_log.jsonl"
