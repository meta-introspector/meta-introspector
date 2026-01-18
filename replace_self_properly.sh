#!/usr/bin/env bash
# Properly replace invalid 'self' inputs with GitHub URLs

set -e

# Projects and their URLs
declare -A PROJECTS=(
  ["run-zos-tasks"]="github:meta-introspector/time-2025?ref=feature/aimyc-003-cultivation"
  ["orient-test"]="github:meta-introspector/time-2025?ref=feature/aimyc-003-cultivation"
  ["decide"]="github:meta-introspector/time-2025?ref=feature/aimyc-003-cultivation"
  ["observe"]="github:meta-introspector/time-2025?ref=feature/aimyc-003-cultivation"
  ["act"]="github:meta-introspector/time-2025?ref=feature/aimyc-003-cultivation"
  ["orient"]="github:meta-introspector/time-2025?ref=feature/aimyc-003-cultivation"
  ["nix-ngram-indexer"]="github:meta-introspector/time-2025?ref=feature/CRQ-016-nixify-workflow"
  ["nix-llm-context"]="github:meta-introspector/time-2025?ref=feature/CRQ-016-nixify-workflow"
)

for project in "${!PROJECTS[@]}"; do
  url="${PROJECTS[$project]}"
  
  # Find the flake
  flake=$(find /mnt/data1/nix/source/github/meta-introspector -name "flake.nix" -path "*$project*" | head -1)
  
  if [ -z "$flake" ]; then
    echo "⚠️  Not found: $project"
    continue
  fi
  
  echo "Fixing: $project"
  
  # Remove all FIXME lines
  sed -i '/# ERROR: Cannot set attributes/d' "$flake"
  sed -i '/# The "self" input is automatically/d' "$flake"
  sed -i '/# If you need to reference another flake/d' "$flake"
  sed -i '/# FIXME:/d' "$flake"
  
  # Add projectRoot input after flake-utils
  if ! grep -q "projectRoot" "$flake"; then
    sed -i "/flake-utils\.url/a\\    projectRoot = {\n      url = \"$url\";\n      inputs.nixpkgs.follows = \"nixpkgs\";\n    };" "$flake"
    echo "  ✓ Added projectRoot input"
  else
    echo "  ✓ Already has projectRoot"
  fi
done

echo ""
echo "Done! Fixed ${#PROJECTS[@]} projects"
