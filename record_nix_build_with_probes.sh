#!/bin/bash
# Record Nix Rust beta build with perf probes from LMFDB catalog

set -e

OUTPUT_DIR="/mnt/data1/meta-introspector/data/nix_build_telemetry"
TIMESTAMP=$(date +%s)
SESSION="nix_rust_beta_probes_${TIMESTAMP}"

mkdir -p "$OUTPUT_DIR"

echo "🔬 Nix Rust beta build with perf probes"
echo "Session: $SESSION"

# Clean any existing probes
echo "🧹 Cleaning existing probes..."
sudo perf probe --del '*' 2>/dev/null || true

# Add probes for key functions (using actual paths)
echo "🎯 Adding perf probes..."

# Find actual libc path
LIBC=$(find /nix/store -name "libc.so.6" -type f 2>/dev/null | head -1)
if [ -n "$LIBC" ]; then
    echo "  Found libc: $LIBC"
    sudo perf probe -x "$LIBC" malloc 2>&1 | grep -v "Failed" || true
    sudo perf probe -x "$LIBC" free 2>&1 | grep -v "Failed" || true
    sudo perf probe -x "$LIBC" open 2>&1 | grep -v "Failed" || true
fi

# Add probes for rustc if found
RUSTC=$(find /nix/store -name "rustc" -type f -executable 2>/dev/null | head -1)
if [ -n "$RUSTC" ]; then
    echo "  Found rustc: $RUSTC"
    sudo perf probe -x "$RUSTC" main 2>&1 | grep -v "Failed" || true
fi

echo ""
echo "📋 Active probes:"
sudo perf probe -l

echo ""
echo "🚀 Starting perf record..."

cd /mnt/data1/meta-introspector/rust-overlay-test

# Record with probes (use specific probe names)
sudo perf record \
    -e 'probe_libc:malloc' \
    -e 'probe_libc:free' \
    -e 'probe_libc:open' \
    -a \
    -o "$OUTPUT_DIR/${SESSION}.perf.data" \
    -- nix build .#rustNightlyProfiling --rebuild 2>&1 | tee "$OUTPUT_DIR/${SESSION}_build.log"

echo ""
echo "✅ Build complete"

# Generate report
echo "📊 Generating reports..."

sudo perf script -i "$OUTPUT_DIR/${SESSION}.perf.data" \
    > "$OUTPUT_DIR/${SESSION}_script.txt"

sudo perf report -i "$OUTPUT_DIR/${SESSION}.perf.data" --stdio \
    > "$OUTPUT_DIR/${SESSION}_report.txt"

# Count probe hits
echo ""
echo "📈 Probe statistics:"
grep "probe:" "$OUTPUT_DIR/${SESSION}_script.txt" | \
    awk '{print $5}' | sort | uniq -c | sort -rn | head -20

echo ""
echo "💾 Saved to: $OUTPUT_DIR/${SESSION}*"

# Clean up probes
sudo perf probe --del '*' 2>/dev/null || true
