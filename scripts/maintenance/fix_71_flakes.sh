#!/usr/bin/env bash
# Fix all const_71_test flakes that use writeText to copy src properly

cd /mnt/data1/meta-introspector/const_71_test

for flake in */flake.nix; do
  dir=$(dirname "$flake")
  
  if grep -q "dontUnpack = true" "$flake"; then
    echo "Fixing: $dir"
    
    # Get the source file extension
    ext=$(grep -oP 'writeText "\K[^"]+' "$flake" | grep -oP '\.\w+$' || echo ".txt")
    filename=$(grep -oP 'writeText "\K[^"]+' "$flake" || echo "source$ext")
    
    # Replace buildPhase to copy file to /build
    sed -i 's|buildPhase = "[^"]*";|buildPhase = "mkdir -p /build \&\& cp $src /build/'"$filename"' \&\& cd /build";|' "$flake"
  fi
done

echo "Done!"
