#!/usr/bin/env bash
# Mark path error projects as incomplete

INCOMPLETE_DIR="/mnt/data1/meta-introspector/incomplete_experiments"

# Composite flakes with path errors
COMPOSITES=(
  "composite-2-3-5-7-11-13-nix-base-home-oauth-telemetry-llm-output-makefile-input"
  "composite-2-3-5-7-11-nix-base-home-oauth-telemetry-llm-output"
  "composite-2-3-5-7-nix-base-home-oauth-telemetry"
  "composite-2-3-nix-base-home-creds"
  "composite-2-3-5-nix-base-home-oauth"
)

for project in "${COMPOSITES[@]}"; do
  echo "Marking as incomplete: $project"
  
  exp_dir="$INCOMPLETE_DIR/$project"
  mkdir -p "$exp_dir"
  
  cat > "$exp_dir/README.md" << EOFREADME
# $project - Incomplete Experiment

## Status: ⚠️ INCOMPLETE - Path Error

This composite flake references paths that don't exist in the Git repository.

## Issue

**Missing path:** \`flakes/feature-3-home-dir-creds/default.nix\`

## Error

\`\`\`
Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git repository
\`\`\`

## Root Cause

The path exists but has \`flake.nix\`, not \`default.nix\`:
- Exists: \`flakes/feature-3-home-dir-creds/flake.nix\` ✅
- Expected: \`flakes/feature-3-home-dir-creds/default.nix\` ❌

## Fix Options

### Option 1: Create default.nix
\`\`\`nix
# flakes/feature-3-home-dir-creds/default.nix
{ pkgs, ... }:
{
  # Export what's needed from flake.nix
}
\`\`\`

### Option 2: Use flake input instead
\`\`\`nix
inputs.feature3 = {
  url = "github:meta-introspector/time-2025?dir=flakes/feature-3-home-dir-creds";
};
\`\`\`

### Option 3: Fix import path
\`\`\`nix
# Change from:
imports = [ ./flakes/feature-3-home-dir-creds/default.nix ];
# To:
imports = [ ./flakes/feature-3-home-dir-creds/flake.nix ];
\`\`\`

## Category

Experimental composite flake - combines multiple features

## Priority

Low - experimental code, not actively used

## Related

- See: analyze_path_errors.md
- Category: Path Error
EOFREADME
  
  echo "  ✓ Documented"
done

echo ""
echo "Marked ${#COMPOSITES[@]} composite flakes as incomplete"
echo "Reason: Path errors (missing default.nix)"
