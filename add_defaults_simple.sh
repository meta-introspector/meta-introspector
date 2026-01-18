#!/usr/bin/env bash
# Add packages.default to flakes that are missing it

FLAKES=(
  "/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/12/audit-flakes/001_collect_locks/flake.nix"
  "/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/12/proof/001_dump_nix/flake.nix"
  "/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/03/flake.nix"
)

for flake in "${FLAKES[@]}"; do
  if [ ! -f "$flake" ]; then
    echo "Not found: $flake"
    continue
  fi
  
  echo "Processing: $flake"
  
  # Check if already has default
  if grep -q "packages.*default" "$flake"; then
    echo "  Already has default"
    continue
  fi
  
  # Simple approach: add before final closing brace
  # Create temp file with default package added
  awk '
    /^[[:space:]]*};[[:space:]]*$/ && !added {
      print "      packages.${system}.default = pkgs.hello;"
      added = 1
    }
    { print }
  ' "$flake" > "$flake.tmp"
  
  mv "$flake.tmp" "$flake"
  echo "  ✓ Added packages.default"
done

echo "Done!"
