#!/usr/bin/env bash
# Fix flake = false to flake = true for proper flake inputs

set -e

echo "Fixing 'flake = false' to use proper flake inputs..."
echo ""

# Find all flakes with "flake = false"
find /mnt/data1/nix/source/github/meta-introspector -name "flake.nix" -type f | while read flake; do
  if grep -q "flake = false" "$flake"; then
    project=$(basename $(dirname "$flake"))
    echo "Found in: $project"
    
    # Change flake = false to flake = true
    sed -i 's/flake = false;.*$/# Use as a flake, not a path/' "$flake"
    
    # Remove the flake = false line entirely and add comment
    sed -i '/# Use as a flake, not a path/d' "$flake"
    sed -i 's/^\(\s*\)flake = false;/\1# flake = true (default, removed)/' "$flake"
    
    echo "  ✓ Changed to use as flake"
  fi
done

echo ""
echo "Done! All 'flake = false' changed to use proper flakes"
echo ""
echo "This means:"
echo "- Inputs are now treated as flakes"
echo "- Access via: feature2.packages.\${system}.default"
echo "- Or: feature2.lib.\${system}"
echo "- Not: import feature2 { ... }"
