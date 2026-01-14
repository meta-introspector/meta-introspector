#!/bin/bash
# Simple perf record of rustc build - no probes, just capture everything

set -e

OUTPUT_DIR="/mnt/data1/meta-introspector/data/rustc_build_telemetry"
TIMESTAMP=$(date +%s)
SESSION="rustc_simple_${TIMESTAMP}"

mkdir -p "$OUTPUT_DIR"

echo "🔬 Recording rustc build with perf (no probes)"
echo "Session: $SESSION"

cd /mnt/data1/meta-introspector/rustc-from-source

# Simple perf record - capture everything
sudo perf record \
    -F 99 \
    -g \
    -a \
    -o "$OUTPUT_DIR/${SESSION}.perf.data" \
    -- nix build . -L 2>&1 | tee "$OUTPUT_DIR/${SESSION}_build.log"

echo ""
echo "✅ Build complete"
echo "📊 Perf data: $OUTPUT_DIR/${SESSION}.perf.data"

# Quick stats
sudo perf report -i "$OUTPUT_DIR/${SESSION}.perf.data" --stdio --percent-limit 1 \
    > "$OUTPUT_DIR/${SESSION}_report.txt"

echo "📈 Top functions:"
head -50 "$OUTPUT_DIR/${SESSION}_report.txt" | grep -E "^\s+[0-9]"

echo ""
echo "💾 Saved to: $OUTPUT_DIR/${SESSION}*"
