#!/bin/bash
# Check if Nix store paths are reproducible

echo "=== NIX REPRODUCIBILITY CHECK ==="
echo

# Pick a successful build
store_path="/nix/store/n6haia96vypk03jcs68pazx1y6f58y9q-Drift_Protocol"

if [ -d "$store_path" ]; then
  echo "✅ Found: $store_path"
  echo
  
  # Can we get the derivation?
  echo "1. Derivation (.drv):"
  nix-store -qd "$store_path" 2>/dev/null || echo "  ❌ No derivation found"
  echo
  
  # Can we get the build inputs?
  echo "2. Build inputs:"
  nix-store -q --references "$store_path" 2>/dev/null | head -5 || echo "  ❌ No references"
  echo
  
  # Can we get the source?
  echo "3. Source derivation:"
  nix-store -q --tree "$store_path" 2>/dev/null | grep -E "(src|source)" | head -3 || echo "  ❌ No source found"
  echo
  
  # Can we rebuild it?
  echo "4. Rebuild command:"
  drv=$(nix-store -qd "$store_path" 2>/dev/null)
  if [ -n "$drv" ]; then
    echo "  nix-store --realise $drv"
    echo "  ✅ Reproducible from derivation"
  else
    echo "  ❌ Cannot rebuild - no derivation"
  fi
else
  echo "❌ Store path not found"
fi

echo
echo "=== THE PROBLEM ==="
echo "Nix store paths exist, but:"
echo "1. No explicit source link (which git commit?)"
echo "2. No flake.lock preserved (which input versions?)"
echo "3. No build metadata (when? by whom? why?)"
echo
echo "=== THE SOLUTION ==="
echo "Wrap every build with metadata:"
echo "  - Git commit hash"
echo "  - flake.lock snapshot"
echo "  - Build timestamp"
echo "  - Rebuild command"
echo "  - All inputs pinned"
