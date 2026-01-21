#!/usr/bin/env bash
set -euo pipefail

echo "🔬 Recording cargo2nix generation with perf"
echo "============================================"

# Generate Cargo.nix with perf recording
perf record -g -o cargo2nix.perf.data -- \
  nix run github:cargo2nix/cargo2nix -- -f Cargo.nix 2>&1 | tee cargo2nix.log

echo ""
echo "✅ Perf data recorded: cargo2nix.perf.data"
echo "   Size: $(stat -c%s cargo2nix.perf.data 2>/dev/null || stat -f%z cargo2nix.perf.data) bytes"
echo ""
echo "Store reference in git:"
echo "  hf-build-telemetry-upload/perf-refs/cargo2nix-$(date +%s).json"
