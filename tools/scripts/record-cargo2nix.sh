#!/usr/bin/env bash
set -euo pipefail

echo "🔬 Recording cargo2nix generation with perf"
echo "============================================"

OUT="zos-results/cargo2nix-perf"
mkdir -p "$OUT"

# Record cargo2nix generating Cargo.nix
perf record -g -o "$OUT/cargo2nix.perf.data" -- \
  nix run github:cargo2nix/cargo2nix -- -f 2>&1 | tee "$OUT/cargo2nix.log"

echo ""
echo "✅ Recorded to: $OUT/cargo2nix.perf.data"
echo ""
echo "Analyze with:"
echo "  perf report -i $OUT/cargo2nix.perf.data"
echo "  ./target/release/extract_orbits $OUT/cargo2nix.perf.data"
