#!/bin/bash
# Test all binaries and collect errors

echo "🔨 Testing all binaries..."
echo ""

BINS=$(grep -A1 '^\[\[bin\]\]' Cargo.toml | grep '^name = ' | cut -d'"' -f2 | grep -v test_driver | head -20)

for bin in $BINS; do
    echo "Testing $bin..."
    ./target/debug/test_driver "$bin" 2>&1 | grep -E "^(❌|💡)" | head -3
    echo ""
done
