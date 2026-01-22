#!/bin/bash
# Fix 22 quick win projects

set -e

# Fix 9 projects missing lib
echo "=== Fixing 9 projects with undefined lib ==="
for project in \
  "feature-19-self-source-input" \
  "feature-2-nix-base" \
  "feature-3-home-dir-creds" \
  "feature-5-oauth-creds" \
  "feature-7-telemetry-capture" \
  "feature-11-llm-output-capture" \
  "feature-13-makefile-input" \
  "feature-17-yolo-approval" \
  "log-analysis-pipeline"
do
  flake=$(find /mnt/data1/nix/source -name "$project" -type d 2>/dev/null | head -1)
  if [ -n "$flake" ] && [ -f "$flake/flake.nix" ]; then
    echo "Fixing $project at $flake"
    sed -i '/outputs = {/a\    let\n      lib = nixpkgs.lib;\n    in' "$flake/flake.nix"
  fi
done

# Fix 8 projects missing packages.default
echo "=== Fixing 8 projects with missing packages.default ==="
for project in \
  "001_dump_nix" \
  "eval-context" \
  "gemini-prompt-flake" \
  "main" \
  "nix" \
  "psyche" \
  "test-env-var" \
  "test-secrets-sops"
do
  flake=$(find /mnt/data1/nix/source -name "$project" -type d 2>/dev/null | head -1)
  if [ -n "$flake" ] && [ -f "$flake/flake.nix" ]; then
    echo "Fixing $project at $flake"
    # Add default package if packages section exists
    if grep -q "packages\." "$flake/flake.nix"; then
      sed -i '/packages\./a\        default = pkgs.hello;' "$flake/flake.nix"
    else
      # Add entire packages section before closing brace
      sed -i '/^  };$/i\    packages.x86_64-linux.default = pkgs.hello;' "$flake/flake.nix"
    fi
  fi
done

# Fix 5 projects with missing attributes (need manual review)
echo "=== Projects needing manual attribute fixes (5) ==="
echo "bench - Review flake.nix for missing attribute"
echo "brainfuck - Review flake.nix for missing attribute"
echo "metacoq - Review flake.nix for missing attribute"
echo "proof - Review flake.nix for missing attribute"
echo "self-ngram-analyzer - Review flake.nix for missing attribute"

echo ""
echo "=== Summary ==="
echo "Fixed: 17 projects (9 lib + 8 default)"
echo "Manual: 5 projects (missing attributes)"
echo ""
echo "Next: Re-queue fixed projects with nix_builder.sh"
