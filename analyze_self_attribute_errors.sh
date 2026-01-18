#!/usr/bin/env bash
# Analyze "self attribute not supported" errors

echo "# Flake 'self' Attribute Errors"
echo ""
echo "## The Problem"
echo ""
echo "23 projects have errors like:"
echo '```'
echo "flake 'self' attribute 'url' is not supported"
echo '```'
echo ""
echo "## Root Cause"
echo ""
echo "These flakes are trying to reference themselves incorrectly:"
echo ""
echo '```nix'
echo "# WRONG:"
echo "inputs.self.url = \"...\";"
echo "inputs.self.flake = \"...\";"
echo ""
echo "# The 'self' input is special and automatic"
echo "# You cannot set attributes on it"
echo '```'
echo ""
echo "## Examples"
echo ""

# Find a flake with this error
for proj in nix-ngram-indexer nix-llm-context zos; do
  flake=$(find /mnt/data1/nix/source/github/meta-introspector -name "flake.nix" -path "*$proj*" | head -1)
  if [ -n "$flake" ]; then
    echo "### $proj"
    echo ""
    echo "Location: \`$flake\`"
    echo ""
    echo "Error pattern:"
    echo '```'
    grep -A 2 -B 2 "self\." "$flake" 2>/dev/null | head -10 || echo "Could not extract pattern"
    echo '```'
    echo ""
    break
  fi
done

echo "## Solution"
echo ""
echo "### Option 1: Remove self references"
echo ""
echo '```nix'
echo "# Remove lines like:"
echo "# inputs.self.url = \"...\";"
echo "# inputs.self.flake = \"...\";"
echo '```'
echo ""
echo "### Option 2: Use proper input name"
echo ""
echo '```nix'
echo "# If trying to reference another flake:"
echo "inputs.myFlake.url = \"github:owner/repo\";"
echo ""
echo "# If trying to reference current flake:"
echo "# Just use 'self' directly, no .url needed"
echo '```'
echo ""
echo "### Option 3: Check for typos"
echo ""
echo '```nix'
echo "# Maybe meant to write:"
echo "inputs.nixpkgs.url = \"...\";"
echo "# instead of:"
echo "# inputs.self.url = \"...\";"
echo '```'
echo ""
echo "## Affected Projects (23)"
echo ""

# List all affected projects
jq -r '.projects | to_entries[] | select(.value.reason | contains("self") and contains("not supported")) | .key' /mnt/data1/meta-introspector/nix_build_failures.json | while read proj; do
  echo "- $proj"
done

echo ""
echo "## Recommendation"
echo ""
echo "These are likely copy-paste errors or misunderstandings of flake inputs."
echo ""
echo "**Action:**"
echo "1. Review each flake's inputs section"
echo "2. Remove invalid self.* attributes"
echo "3. Fix any typos (self → nixpkgs?)"
echo "4. Test: \`nix flake check\`"
