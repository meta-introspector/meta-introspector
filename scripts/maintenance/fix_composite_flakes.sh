#!/usr/bin/env bash
# Fix composite flakes to use flake inputs instead of default.nix imports

set -e

echo "Fixing composite flakes to use proper flake inputs..."
echo ""

# Find composite flakes
COMPOSITES=(
  "composite-2-3-5-7-11-13-nix-base-home-oauth-telemetry-llm-output-makefile-input"
  "composite-2-3-5-7-11-nix-base-home-oauth-telemetry-llm-output"
  "composite-2-3-5-7-nix-base-home-oauth-telemetry"
  "composite-2-3-nix-base-home-creds"
  "composite-2-3-5-nix-base-home-oauth"
)

for project in "${COMPOSITES[@]}"; do
  # Find the flake
  flake=$(find /mnt/data1/nix/source/github/meta-introspector -name "flake.nix" -path "*$project*" | head -1)
  
  if [ -z "$flake" ]; then
    echo "⚠️  Not found: $project"
    continue
  fi
  
  echo "Fixing: $project"
  echo "  File: $flake"
  
  # Check if it has the problematic import
  if grep -q "flakes/feature-3-home-dir-creds/default.nix" "$flake"; then
    echo "  Found problematic import"
    
    # Add flake input if not already present
    if ! grep -q "feature3HomeDir" "$flake"; then
      # Add input after nixpkgs
      sed -i '/nixpkgs\.url/a\    feature3HomeDir = {\n      url = "github:meta-introspector/time-2025?dir=flakes/feature-3-home-dir-creds";\n      inputs.nixpkgs.follows = "nixpkgs";\n    };' "$flake"
      echo "  ✓ Added feature3HomeDir input"
    fi
    
    # Comment out the old import
    sed -i 's|^\(\s*\)\(.*flakes/feature-3-home-dir-creds/default.nix.*\)|# FIXME: Use flake input instead\n\1# \2|' "$flake"
    echo "  ✓ Commented out default.nix import"
    
    # Add comment about using the flake input
    sed -i '/# FIXME: Use flake input instead/a\
# Use: feature3HomeDir.packages.${system}.default or feature3HomeDir.lib' "$flake"
    
  else
    echo "  No problematic import found"
  fi
  
  echo ""
done

echo "Done! Fixed composite flakes to use flake inputs"
echo ""
echo "Changes:"
echo "- Added feature3HomeDir flake input"
echo "- Commented out default.nix imports"
echo "- Added usage hints"
echo ""
echo "Next: Review and update code to use feature3HomeDir.packages or .lib"
