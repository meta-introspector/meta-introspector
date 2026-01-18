# Undefined Variable Analysis - Think Before Fixing

## The Problem

32 projects have undefined variables. Before blindly adding them, we should understand:

1. **Why are they undefined?**
2. **What should they actually be?**
3. **Are these experimental/incomplete flakes?**
4. **Should we fix them or document them as broken?**

## Breakdown by Variable

### `lib` (9 projects)

**What it should be:** `nixpkgs.lib` - Nix standard library

**Why undefined:**
- Missing `let lib = nixpkgs.lib;` or `inherit (nixpkgs) lib;`
- Common in flakes that use lib functions without importing

**Should we fix?** ✅ YES
- Standard pattern
- Clear fix
- High success probability

**Fix:**
```nix
outputs = { self, nixpkgs, ... }:
  let
    lib = nixpkgs.lib;
    # or
    inherit (nixpkgs) lib;
  in
  {
    # rest of outputs
  }
```

### `searchScript` (3 projects)

**What it should be:** A custom script for searching

**Why undefined:**
- These are likely incomplete experiments
- Script was planned but never implemented

**Should we fix?** ⚠️ MAYBE
- Need to look at each project
- Might be intentionally incomplete
- Could add placeholder or mark as TODO

**Options:**
1. Add placeholder: `searchScript = pkgs.writeScript "search" "echo TODO";`
2. Comment out usage: `# searchScript = ...;`
3. Document as incomplete

### `archiveDerivation` (3 projects)

**What it should be:** A derivation for archiving

**Why undefined:**
- Custom function that was never defined
- Likely experimental code

**Should we fix?** ⚠️ MAYBE
- Need context from each project
- Might be part of larger unfinished feature

### `geminiPrompt` (3 projects)

**What it should be:** Prompt template for Gemini API

**Why undefined:**
- Part of LLM integration experiments
- Prompt was planned but not created

**Should we fix?** ⚠️ MAYBE
- Could add placeholder prompt
- Or mark as requiring manual input

### `promptTemplate` (2 projects)

**What it should be:** Generic prompt template

**Why undefined:**
- Similar to geminiPrompt
- Template not yet created

**Should we fix?** ⚠️ MAYBE
- Add placeholder or document

## Decision Framework

### Criteria for Auto-Fix

✅ **Should auto-fix if:**
1. Standard Nix pattern (like `lib`)
2. Clear what it should be
3. Won't break functionality
4. High probability of success

❌ **Should NOT auto-fix if:**
1. Custom/experimental code
2. Unclear what value should be
3. Might hide real issues
4. Part of incomplete feature

### Proposed Approach

#### Phase 1: Safe Fixes (9 projects)
Fix `lib` - standard pattern, clear fix:
```bash
# Projects with undefined lib:
- feature-19-self-source-input
- log-analysis-pipeline
- feature-5-oauth-creds
- feature-7-telemetry
- feature-11-llm-output
- feature-13-makefile-input
- feature-2-nix-base
- feature-3-home-dir-creds
- composite-2-3-nix-base-home
```

#### Phase 2: Document Others (23 projects)
For custom variables, create documentation:
```nix
# TODO: Define searchScript
# searchScript should be a script that searches for patterns
# Example:
# searchScript = pkgs.writeScript "search" ''
#   #!/bin/bash
#   grep -r "$1" .
# '';
```

#### Phase 3: Mark as Experimental
Add to flake.nix:
```nix
{
  description = "Experimental flake - incomplete";
  
  # KNOWN ISSUES:
  # - searchScript undefined (needs implementation)
  # - archiveDerivation undefined (needs implementation)
  
  outputs = { ... }: {
    # ...
  };
}
```

## Implementation Plan

### Step 1: Fix `lib` (Safe)
```bash
#!/usr/bin/env bash
# fix_lib_undefined.sh

PROJECTS=(
  "feature-19-self-source-input"
  "log-analysis-pipeline"
  "feature-5-oauth-creds"
  # ... 6 more
)

for proj in "${PROJECTS[@]}"; do
  flake="/path/to/$proj/flake.nix"
  
  # Add lib import after outputs line
  sed -i '/outputs = /a\    let\n      lib = nixpkgs.lib;\n    in' "$flake"
done
```

### Step 2: Document Others
```bash
#!/usr/bin/env bash
# document_undefined.sh

# For each project with custom undefined vars
# Add TODO comments explaining what's needed
```

### Step 3: Create Tracking Issue
```markdown
# Undefined Variables Tracking

## Fixed (9)
- [x] lib in feature-19-self-source-input
- [x] lib in log-analysis-pipeline
- ...

## Needs Implementation (23)
- [ ] searchScript in keyword-searcher (needs search logic)
- [ ] archiveDerivation in ... (needs archive logic)
- [ ] geminiPrompt in ... (needs prompt template)
```

## Questions to Answer

1. **Are these experimental flakes meant to work?**
   - Check git history
   - Look for related issues/PRs
   - Ask: "Was this ever working?"

2. **Should we keep them in the build queue?**
   - If experimental: mark as such, don't fail CI
   - If meant to work: fix properly
   - If abandoned: archive or delete

3. **What's the maintenance strategy?**
   - Document incomplete features
   - Create issues for TODOs
   - Regular cleanup of abandoned experiments

## Recommendation

**Don't blindly fix everything.**

Instead:
1. ✅ Fix `lib` (9 projects) - safe, standard
2. 📝 Document others (23 projects) - explain what's needed
3. 🏷️ Tag experimental flakes - don't fail CI
4. 🧹 Consider archiving abandoned experiments

This way we:
- Fix real issues
- Document incomplete work
- Don't hide problems
- Maintain code quality
