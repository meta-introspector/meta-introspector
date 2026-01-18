#!/usr/bin/env bash
# Fix "self attribute not supported" errors

set -e

echo "Fixing 'self' attribute errors..."
echo ""

# Find all flakes with self.url or self.flake
find /mnt/data1/nix/source/github/meta-introspector -name "flake.nix" -type f | while read flake; do
  if grep -q "^\s*self\s*=\s*{" "$flake"; then
    project=$(basename $(dirname "$flake"))
    echo "Found in: $project"
    echo "  File: $flake"
    
    # Show the problematic section
    echo "  Problem:"
    grep -A 3 "^\s*self\s*=\s*{" "$flake" | sed 's/^/    /'
    
    # Fix: Comment out the self input
    sed -i '/^\s*self\s*=\s*{/,/^\s*};/s/^/# FIXME: /' "$flake"
    
    # Add explanation
    sed -i '/# FIXME:.*self\s*=\s*{/i\
    # ERROR: Cannot set attributes on special "self" input\
    # The "self" input is automatically provided by Nix\
    # If you need to reference another flake, use a different name\
' "$flake"
    
    echo "  ✓ Commented out and documented"
    echo ""
  fi
done

echo "Done! Review the changes and either:"
echo "1. Remove the commented sections"
echo "2. Rename 'self' to something else (e.g., 'projectRoot')"
echo "3. Remove if not needed"
