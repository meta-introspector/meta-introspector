#!/bin/bash
# Simple bootstrap recording (no sudo needed)

LANG=$1
OUTPUT="data/bootstrap_perf/${LANG}_bootstrap.perf.data"
mkdir -p data/bootstrap_perf

echo "🔬 Recording $LANG bootstrap (no sudo)..."
echo "Started: $(date)"

case $LANG in
  bash)
    perf record -o "$OUTPUT" -F 99 -g nix build nixpkgs#bash --rebuild
    ;;
  python)
    perf record -o "$OUTPUT" -F 99 -g nix build nixpkgs#python3 --rebuild
    ;;
  gcc)
    perf record -o "$OUTPUT" -F 99 -g nix build nixpkgs#gcc --rebuild
    ;;
  *)
    echo "Unknown: $LANG"
    exit 1
    ;;
esac

echo "✅ Complete: $(date)"
ls -lh "$OUTPUT"

echo ""
echo "🔄 Converting..."
./target/release/perf2parquet "${LANG}_bootstrap" "$OUTPUT"

echo ""
./query-parquet/target/release/query-parquet "${LANG}_bootstrap_perf.parquet" \
  "SELECT COUNT(*) as samples, COUNT(DISTINCT ip) as ips FROM ${LANG}_bootstrap_perf"
