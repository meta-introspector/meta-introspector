#!/usr/bin/env bash
# Build and run parallel scanner in nix develop environment

set -e

echo "🔨 Building in nix develop environment..."
nix develop -c cargo build --release --bin demo_parallel_scanner

echo ""
echo "🔍 Scanning meta-introspector for duplicates..."
echo ""

# Create file list
find . -name "*.rs" -type f > /tmp/meta-introspector-files.txt
echo "📊 Found $(wc -l < /tmp/meta-introspector-files.txt) Rust files"
echo ""

# Run scanner
nix develop -c cargo run --release --bin demo_parallel_scanner \
  /tmp/meta-introspector-files.txt \
  /tmp/meta-introspector-dups.json \
  24 \
  /mnt/data1/meta-introspector

echo ""
echo "✅ Results saved to: /tmp/meta-introspector-dups.json"
echo ""
echo "📊 Analyzing results..."
if [ -f /tmp/meta-introspector-dups.json ]; then
  echo "  Total records: $(jq length /tmp/meta-introspector-dups.json 2>/dev/null || echo 'N/A')"
  echo "  Sample duplicates:"
  jq -r '.[] | select(.similarity == 1.0) | "  - \(.file1) ↔ \(.file2)"' /tmp/meta-introspector-dups.json 2>/dev/null | head -10 || echo "  (use jq to analyze)"
fi
