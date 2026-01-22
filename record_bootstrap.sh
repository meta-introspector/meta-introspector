# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
# Record complete toolchain bootstrap

LANG=$1
OUTPUT_DIR="data/bootstrap_perf"
mkdir -p "$OUTPUT_DIR"

echo "🔬 Recording $LANG bootstrap..."
echo "Started: $(date)"

# Get nix path
NIX=$(which nix)

# Start system-wide # Use: nix run github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix#perf-build -- .#target
sudo # Use: nix run github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix#perf-build -- .#target
PERF_PID=$!

echo "📊 Perf recording started (PID: $PERF_PID)"
sleep 2

# Force rebuild from source
case $LANG in
  bash)
    echo "Building bash from source (~2 minutes)..."
    $NIX build nixpkgs#bash --rebuild 2>&1 | tail -20
    ;;
  gcc)
    echo "Building gcc from source (~5 minutes)..."
    $NIX build nixpkgs#gcc --rebuild 2>&1 | tail -20
    ;;
  python)
    echo "Building python from source (~3 minutes)..."
    $NIX build nixpkgs#python3 --rebuild 2>&1 | tail -20
    ;;
  *)
    echo "Unknown language: $LANG"
    sudo kill $PERF_PID
    exit 1
    ;;
esac

# Stop recording
echo "Stopping perf..."
sudo kill -INT $PERF_PID
wait $PERF_PID 2>/dev/null

echo "✅ Recording complete: $(date)"
sudo ls -lh "$OUTPUT_DIR/${LANG}_bootstrap.perf.data"

# Fix permissions
sudo chown $USER:$USER "$OUTPUT_DIR/${LANG}_bootstrap.perf.data"

echo ""
echo "🔄 Converting to parquet..."
./target/release/perf2parquet "${LANG}_bootstrap" "$OUTPUT_DIR/${LANG}_bootstrap.perf.data"

echo ""
echo "📊 Quick stats:"
./query-parquet/target/release/query-parquet "${LANG}_bootstrap_perf.parquet" \
  "SELECT COUNT(*) as samples, COUNT(DISTINCT ip) as unique_ips FROM ${LANG}_bootstrap_perf" 2>&1 | grep -A 3 "^|"
