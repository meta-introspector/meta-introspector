#!/usr/bin/env bash
set -euo pipefail

echo "🔬 Recording cargo2nix generation with perf"
echo "============================================"

# Generate Cargo.nix with perf recording
perf record -g -o cargo2nix.perf.data -- \
  nix run github:cargo2nix/cargo2nix -- -f Cargo.nix 2>&1 | tee cargo2nix.log

# Build a derivation that stores the perf data
nix build --impure --expr '
  with import <nixpkgs> {};
  stdenv.mkDerivation {
    name = "cargo2nix-perf-data";
    src = ./.;
    installPhase = ''
      mkdir -p $out/perf
      cp cargo2nix.perf.data $out/perf/build.perf.data || true
      cp cargo2nix.log $out/perf/build.log || true
    '';
  }
' -o result-cargo2nix-perf

echo ""
echo "✅ Perf data stored in: $(readlink result-cargo2nix-perf)"
echo ""
echo "Analyze with:"
echo "  nix build .#analyze-orbits $(readlink result-cargo2nix-perf)"
