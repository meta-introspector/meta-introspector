# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
set -e

OUTPUT_DIR="/mnt/data1/meta-introspector/data/rustc_build_telemetry"
TIMESTAMP=$(date +%s)
SESSION="rustc_simple_${TIMESTAMP}"
mkdir -p "$OUTPUT_DIR"

cd /mnt/data1/meta-introspector/rustc-from-source
rm -f result

echo "=== Starting perf record (system-wide) ==="
sudo perf record -e cpu-clock -F 99 -g -a -o "$OUTPUT_DIR/${SESSION}.perf.data" &
PERF_PID=$!
echo "Perf PID: $PERF_PID"
sleep 2

echo "=== Starting nix build ==="
nix build . --rebuild -L 2>&1 | tee "$OUTPUT_DIR/${SESSION}_build.log"
BUILD_EXIT=$?

echo "=== Stopping perf ==="
sudo kill -INT $PERF_PID
wait $PERF_PID 2>/dev/null || true

echo "=== Results ==="
ls -lh "$OUTPUT_DIR/${SESSION}.perf.data"
echo "Build exit: $BUILD_EXIT"
