#!/bin/bash
# apply_logger_to_all.sh - Wrap all 500 flakes with universal-build-logger

set -e

echo "🔧 Applying universal-build-logger to all flakes"
echo

LOGGER_PATH="/mnt/data1/meta-introspector/universal-build-logger"
SOURCE_PATH="/mnt/data1/nix/source"
WRAPPED=0
SKIPPED=0

# Find all flakes
flakes=$(find "$SOURCE_PATH" -name "flake.nix" -type f)
total=$(echo "$flakes" | wc -l)

echo "Found $total flakes"
echo

for flake in $flakes; do
  dir=$(dirname "$flake")
  project=$(basename "$dir")
  
  # Skip if already wrapped
  if grep -q "build-logger" "$flake" 2>/dev/null; then
    echo "⏭️  Skip: $project (already wrapped)"
    ((SKIPPED++))
    continue
  fi
  
  echo "🔨 Wrapping: $project"
  
  # Backup original
  cp "$flake" "$flake.backup"
  
  # Check if flake has packages.*.default
  if grep -q "packages\." "$flake"; then
    # Wrap existing package
    sed -i "s|packages\.\([^.]*\)\.default = \(.*\);|packages.\1.default-unwrapped = \2;\n      packages.\1.default = (import $LOGGER_PATH/flake.nix).lib.wrap { inherit pkgs self; project = packages.\1.default-unwrapped; projectName = \"$project\"; };|" "$flake"
    ((WRAPPED++))
  else
    echo "  ⚠️  No packages.default found, skipping"
    ((SKIPPED++))
  fi
done

echo
echo "✅ Complete!"
echo "  Wrapped: $WRAPPED"
echo "  Skipped: $SKIPPED"
echo "  Total: $total"
echo
echo "Next: Queue all wrapped flakes for rebuild"
