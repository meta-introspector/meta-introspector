#!/bin/bash

# Capture x.py bootstrap calls to cargo/rustc

PROJECT_DIR="/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build"
INTERCEPTOR="/mnt/data1/meta-introspector/target/debug/rustc_interceptor"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="/mnt/data1/meta-introspector/xpy_bootstrap_${TIMESTAMP}.log"

echo "🚀 X.PY BOOTSTRAP CAPTURE"
echo "========================="
echo "Project: $PROJECT_DIR"
echo "Interceptor: $INTERCEPTOR"
echo "Log: $LOG_FILE"

cd "$PROJECT_DIR"

# Set up environment to intercept rustc calls
export RUSTC="$INTERCEPTOR"
export REAL_RUSTC="rustc"

# Clean previous logs
rm -f rustc_build_log.jsonl

echo "🔨 Running x.py build --stage 1 library/std..."
./x.py build --stage 1 library/std > "$LOG_FILE" 2>&1 || true

echo "✅ Bootstrap completed!"
echo "📊 Rustc invocations: $(wc -l < rustc_build_log.jsonl 2>/dev/null || echo 0)"
echo "📄 Bootstrap log: $LOG_FILE"
echo "📄 Rustc log: rustc_build_log.jsonl"
