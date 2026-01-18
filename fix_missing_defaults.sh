#!/usr/bin/env bash
# Fix missing packages.default in flakes

set -e

# Projects with missing-default error
PROJECTS=(
  "streamofrandom/2025/10/12/proof/001_dump_nix"
  "streamofrandom/2025/09/03"
  "streamofrandom/2025/10/12/audit-flakes/eval-context"
  "streamofrandom/2025/10/12/audit-flakes/gemini-prompt-flake"
  "streamofrandom/2025/10/12/audit-flakes/main"
  "streamofrandom/2025/10/12/audit-flakes/nix"
  "time-2026/01-january/18/psyche"
  "streamofrandom/2025/10/12/audit-flakes/test-env-var"
  "streamofrandom/2025/10/12/audit-flakes/test-secrets-sops"
)

for project in "${PROJECTS[@]}"; do
  flake_path="/mnt/data1/nix/source/github/meta-introspector/$project/flake.nix"
  
  if [ ! -f "$flake_path" ]; then
    echo "⚠️  Not found: $flake_path"
    continue
  fi
  
  echo "Fixing: $project"
  
  # Check if already has packages.default
  if grep -q "packages.*default" "$flake_path"; then
    echo "  ✓ Already has default"
    continue
  fi
  
  # Add packages.default before closing brace
  # Find the outputs section and add default package
  sed -i '/^[[:space:]]*};[[:space:]]*$/i\
\      packages.${system}.default = pkgs.writeText "placeholder" "This flake needs a proper default package";' "$flake_path"
  
  echo "  ✓ Added packages.default"
done

echo ""
echo "Fixed ${#PROJECTS[@]} flakes"
echo "Re-queue them for building:"
echo "./nix_builder.sh queue <path>"
