# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
# Record rustc from source build with perf probes

set -e

OUTPUT_DIR="/mnt/data1/meta-introspector/data/rustc_build_telemetry"
TIMESTAMP=$(date +%s)
SESSION="rustc_from_source_${TIMESTAMP}"

mkdir -p "$OUTPUT_DIR"

echo "🔬 Recording rustc from source build with perf probes"
echo "Session: $SESSION"

# Clean existing probes
echo "🧹 Cleaning existing probes..."
sudo perf probe --del '*' 2>/dev/null || true

# Add probes
echo "🎯 Adding perf probes..."

LIBC=$(find /nix/store -name "libc.so.6" -type f 2>/dev/null | head -1)
if [ -n "$LIBC" ]; then
    echo "  Found libc: $LIBC"
    sudo perf probe -x "$LIBC" malloc 2>&1 | grep "Added" || true
    sudo perf probe -x "$LIBC" free 2>&1 | grep "Added" || true
    sudo perf probe -x "$LIBC" open 2>&1 | grep "Added" || true
    sudo perf probe -x "$LIBC" read 2>&1 | grep "Added" || true
    sudo perf probe -x "$LIBC" write 2>&1 | grep "Added" || true
fi

echo ""
echo "📋 Active probes:"
sudo perf probe -l

echo ""
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix

cd /mnt/data1/meta-introspector/rustc-from-source

# Record with probes
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
    -e 'probe_libc:malloc' \
    -e 'probe_libc:free' \
    -e 'probe_libc:open' \
    -e 'probe_libc:read' \
    -e 'probe_libc:write' \
    -F 99 \
    -a \
    -o "$OUTPUT_DIR/${SESSION}.perf.data" \
    -- nix build . -L 2>&1 | tee "$OUTPUT_DIR/${SESSION}_build.log"

echo ""
echo "✅ Build complete"

# Generate reports
echo "📊 Generating reports..."

sudo perf script -i "$OUTPUT_DIR/${SESSION}.perf.data" \
    > "$OUTPUT_DIR/${SESSION}_script.txt"

sudo perf report -i "$OUTPUT_DIR/${SESSION}.perf.data" --stdio \
    > "$OUTPUT_DIR/${SESSION}_report.txt"

# Count probe hits
echo ""
echo "📈 Probe statistics:"
grep "probe_libc:" "$OUTPUT_DIR/${SESSION}_script.txt" | \
    awk '{print $5}' | sort | uniq -c | sort -rn

echo ""
echo "💾 Saved to: $OUTPUT_DIR/${SESSION}*"

# Clean up probes
sudo perf probe --del '*' 2>/dev/null || true
