#!/usr/bin/env bash
# Mark broken flakes as incomplete experiments

set -e

INCOMPLETE_DIR="/mnt/data1/meta-introspector/incomplete_experiments"
mkdir -p "$INCOMPLETE_DIR"

# Get all projects with undefined variables (except lib)
PROJECTS=(
  "keyword-searcher:searchScript"
  "llm-data-extractor-flake:prompt"
  "archive-flake:archiveDerivation"
  "gemini-prompt-flake:geminiPrompt"
  "prompt-template-flake:promptTemplate"
)

for entry in "${PROJECTS[@]}"; do
  project="${entry%%:*}"
  var="${entry##*:}"
  
  # Find the flake
  flake=$(find /mnt/data1/nix/source/github/meta-introspector -name "flake.nix" -path "*$project*" | head -1)
  
  if [ -z "$flake" ]; then
    echo "⚠️  Not found: $project"
    continue
  fi
  
  project_dir=$(dirname "$flake")
  project_name=$(basename "$project_dir")
  
  echo "Processing: $project_name"
  
  # Create experiment directory
  exp_dir="$INCOMPLETE_DIR/$project_name"
  mkdir -p "$exp_dir"
  
  # Copy flake
  cp "$flake" "$exp_dir/"
  
  # Create README documenting the issue
  cat > "$exp_dir/README.md" << EOFREADME
# $project_name - Incomplete Experiment

## Status: ⚠️ INCOMPLETE

This flake is an incomplete experiment and currently does not build.

## Issue

**Undefined variable:** \`$var\`

## Error

\`\`\`
error: undefined variable '$var'
\`\`\`

## What's Needed

### Option 1: Implement Missing Variable

\`\`\`nix
# Add to flake.nix
let
  $var = pkgs.writeScript "$var" ''
    #!/bin/bash
    # TODO: Implement $var logic
    echo "Not implemented yet"
  '';
in
\`\`\`

### Option 2: Remove Usage

Comment out or remove code that uses \`$var\`.

### Option 3: Mark as TODO

\`\`\`nix
{
  description = "Experimental flake - TODO: implement $var";
  
  # TODO: Define $var
  # $var = ...;
  
  outputs = { ... }: {
    # ...
  };
}
\`\`\`

## Original Location

\`$project_dir\`

## Next Steps

- [ ] Decide if this experiment should be completed
- [ ] Implement missing variable or remove usage
- [ ] Test the fix
- [ ] Move back to main codebase or archive

## Related

- See: UNDEFINED_VARIABLE_ANALYSIS.md
- Category: Incomplete Experiment
- Priority: Low (experimental code)
EOFREADME
  
  # Add marker to original flake
  if ! grep -q "INCOMPLETE EXPERIMENT" "$flake"; then
    sed -i '1i\
# ⚠️ INCOMPLETE EXPERIMENT\
# This flake has undefined variables and does not build.\
# See: incomplete_experiments/'"$project_name"'/README.md\
' "$flake"
  fi
  
  echo "  ✓ Documented in $exp_dir"
done

# Create index
cat > "$INCOMPLETE_DIR/README.md" << 'EOFINDEX'
# Incomplete Experiments

This directory contains flakes that are incomplete experiments and do not currently build.

## Purpose

Rather than failing CI or cluttering error reports, we:
1. Document what's incomplete
2. Explain what's needed to fix
3. Provide clear next steps
4. Keep them separate from working code

## Categories

### Undefined Variables

Projects with missing variable definitions:

EOFINDEX

# Add each project to index
for entry in "${PROJECTS[@]}"; do
  project="${entry%%:*}"
  var="${entry##*:}"
  echo "- **$project**: Missing \`$var\`" >> "$INCOMPLETE_DIR/README.md"
done

cat >> "$INCOMPLETE_DIR/README.md" << 'EOFINDEX'

## How to Complete an Experiment

1. Choose a project from above
2. Read its README.md
3. Implement the missing parts
4. Test: `nix build`
5. If successful, move back to main codebase
6. If abandoned, archive or delete

## Maintenance

- Review quarterly
- Archive abandoned experiments
- Complete or delete old experiments
- Keep this list updated

## Philosophy

**Incomplete experiments are OK!**

We document them rather than:
- ❌ Failing CI on experimental code
- ❌ Hiding errors with placeholder fixes
- ❌ Deleting potentially useful experiments

Instead we:
- ✅ Document what's incomplete
- ✅ Explain what's needed
- ✅ Keep experiments separate
- ✅ Allow easy completion later
EOFINDEX

echo ""
echo "Created incomplete experiments directory: $INCOMPLETE_DIR"
echo "Documented ${#PROJECTS[@]} incomplete experiments"
echo ""
echo "Next steps:"
echo "1. Review: cat $INCOMPLETE_DIR/README.md"
echo "2. Exclude from CI: Add to .gitignore or CI config"
echo "3. Periodic review: Quarterly cleanup"
