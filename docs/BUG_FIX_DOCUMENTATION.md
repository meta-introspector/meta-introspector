# Git Repository URL Bug Fix

## Problem
The `complete_indexer.rs` was creating fake `github.com/unknown/` URLs for repositories that had valid GitHub remotes, causing corrupted master index with 30K+ fake entries.

## Root Cause
The `infer_canonical_url()` function only looked for the **first** URL containing "github.com", but many repos have **multiple remotes**:
- `origin` → local path (e.g., `/home/mdupont/nix/vendor/rust/...`)
- `origin-gh` → GitHub URL (e.g., `https://github.com/meta-introspector/rust`)

When it found the local path first, it skipped the GitHub URL and defaulted to "unknown".

## Fix Applied
Modified `infer_canonical_url()` in `/home/mdupont/zombie_driver2/complete_indexer.rs`:

1. **Collect ALL remote URLs** from git config
2. **Prefer GitHub URLs** over local paths
3. **Fall back to HTTP URLs** if no GitHub found
4. **Only use "unknown"** as absolute last resort

## Files Fixed
- ✅ `/home/mdupont/zombie_driver2/complete_indexer.rs` - Main indexer
- ⚠️ `/home/mdupont/zombie_driver2/canonical_structure_builder.rs` - Still needs same fix

## Next Steps
1. Apply same fix to `canonical_structure_builder.rs`
2. Run the fixed indexer to regenerate clean master index
3. Verify no more fake "unknown" URLs are created

## Test Case
**Before:** `zombie_driver2` → `https://github.com/unknown/zombie_driver2`
**After:** `zombie_driver2` → `https://github.com/meta-introspector/rust` (from origin-gh remote)
