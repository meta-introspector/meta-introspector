# Nix Build Error Pattern Analysis

## Overview

Analyzed 144 failed builds, focusing on the 129 "other" category errors.

## Top Error Patterns

### 1. Undefined Variable (32 projects, 40%)

**Most common undefined variables:**
- `lib` (9 projects) - Missing nixpkgs.lib import
- `searchScript` (3 projects) - Missing script definition
- `archiveDerivation` (3 projects) - Missing derivation
- `geminiPrompt` (3 projects) - Missing prompt template
- `promptTemplate` (2 projects) - Missing template

**Fix:**
```nix
# Add to flake.nix
let
  lib = nixpkgs.lib;
  # or
  inherit (nixpkgs) lib;
in
```

### 2. Flake Attribute Not Supported (23 projects, 29%)

**Common issues:**
- `flake 'self' attribute 'url' is not supported` (multiple projects)
- `flake 'self' attribute 'flake' is not supported`

**Cause:** Invalid flake input syntax

**Fix:**
```nix
# Wrong:
inputs.self.url = "...";

# Right:
inputs.myInput.url = "...";
```

### 3. Path Not In Git (12 projects, 15%)

**Common missing paths:**
- `flakes/feature-3-home-dir-creds/default.nix`
- `10/12/audit-flakes/002_extract_data/flake.nix`
- `10/15/zos/ooda/tasks/act/flake.nix`

**Cause:** Git submodules not initialized or paths moved

**Fix:**
```bash
git submodule update --init --recursive
# or update path references
```

### 4. Cannot Find Flake (6 projects, 8%)

**Missing flakes:**
- `flake:impureLlmResult`
- `flake:hackathon-status-raw`
- `flake:consumer`

**Cause:** Using flake registry references that don't exist

**Fix:**
```nix
# Wrong:
inputs.myFlake.url = "flake:impureLlmResult";

# Right:
inputs.myFlake.url = "github:owner/repo";
```

### 5. Duplicate Attribute (2 projects, 3%)

**Examples:**
- `aarch64-linux` already defined
- `buildInputs` already defined

**Fix:** Remove duplicate definitions

### 6. Other Issues (4 projects, 5%)

- File not found (2)
- Build failed (1)
- Coercion error (1)

## Recommendations

### Quick Wins (32 projects)

**Fix undefined `lib` (9 projects):**
```bash
# Add to each flake.nix
sed -i '/outputs = /a\    let\n      lib = nixpkgs.lib;\n    in' flake.nix
```

### Medium Effort (23 projects)

**Fix flake attribute errors:**
- Review and fix `self` references
- Use proper input syntax

### Requires Investigation (12 projects)

**Fix path errors:**
- Initialize git submodules
- Update path references
- Check if files were moved/deleted

## Priority Order

1. **Undefined variables** (32) - Easy fix, high impact
2. **Flake attributes** (23) - Medium fix, high impact
3. **Path errors** (12) - Requires investigation
4. **Missing flakes** (6) - Need to find correct URLs
5. **Other** (6) - Case-by-case

## Expected Impact

- Fix undefined variables: +8% success rate
- Fix flake attributes: +6% success rate
- Fix path errors: +3% success rate
- **Total potential**: +17% (63.6% → 80.6%)
