# Flake 'self' Attribute Errors

## The Problem

23 projects have errors like:
```
flake 'self' attribute 'url' is not supported
```

## Root Cause

These flakes are trying to reference themselves incorrectly:

```nix
# WRONG:
inputs.self.url = "...";
inputs.self.flake = "...";

# The 'self' input is special and automatic
# You cannot set attributes on it
```

## Examples

### nix-ngram-indexer

Location: `/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/11/nix-ngram-indexer/flake.nix`

Error pattern:
```
            {
              nativeBuildInputs = [ pkgs.bash pkgs.jq ];
              # Define the paths to be indexed (relative to self.outPath)
              indexedPaths = lib.toJSON [ "${self}/scripts" "${self}/docs" ];
              ngramIndex = generateNgramIndex { paths = [ "${self}/scripts" "${self}/docs" ]; };
```

## Solution

### Option 1: Remove self references

```nix
# Remove lines like:
# inputs.self.url = "...";
# inputs.self.flake = "...";
```

### Option 2: Use proper input name

```nix
# If trying to reference another flake:
inputs.myFlake.url = "github:owner/repo";

# If trying to reference current flake:
# Just use 'self' directly, no .url needed
```

### Option 3: Check for typos

```nix
# Maybe meant to write:
inputs.nixpkgs.url = "...";
# instead of:
# inputs.self.url = "...";
```

## Affected Projects (23)


## Recommendation

These are likely copy-paste errors or misunderstandings of flake inputs.

**Action:**
1. Review each flake's inputs section
2. Remove invalid self.* attributes
3. Fix any typos (self → nixpkgs?)
4. Test: `nix flake check`
