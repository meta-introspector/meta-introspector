#!/usr/bin/env bash
# Replace invalid 'self' inputs with proper GitHub URLs

set -e

echo "Replacing 'self' inputs with absolute GitHub URLs..."
echo ""

# Find all flakes with commented FIXME self inputs
find /mnt/data1/nix/source/github/meta-introspector -name "flake.nix" -type f | while read flake; do
  if grep -q "# FIXME:.*self\s*=" "$flake"; then
    project=$(basename $(dirname "$flake"))
    echo "Processing: $project"
    echo "  File: $flake"
    
    # Extract the URL from the commented line
    url=$(grep "# FIXME:.*url.*github" "$flake" | sed -n 's/.*url = "\([^"]*\)".*/\1/p' | head -1)
    
    if [ -n "$url" ]; then
      echo "  Found URL: $url"
      
      # Remove the FIXME comments and error message
      sed -i '/# ERROR: Cannot set attributes/,/# FIXME: };/d' "$flake"
      
      # Add proper input with a different name
      # Insert before the closing of inputs
      sed -i '/^\s*};.*# end of inputs/i\
    # Reference to project root\
    projectRoot = {\
      url = "'"$url"'";\
      inputs.nixpkgs.follows = "nixpkgs";\
    };' "$flake"
      
      echo "  ✓ Replaced with projectRoot input"
    else
      echo "  ⚠️  Could not extract URL"
    fi
    echo ""
  fi
done

echo "Done! All 'self' inputs replaced with 'projectRoot'"
echo ""
echo "Changes:"
echo "- Removed invalid 'self' input"
echo "- Added 'projectRoot' input with absolute GitHub URL"
echo "- Set nixpkgs.follows to avoid duplication"
echo ""
echo "Test with: nix flake check <path>"
