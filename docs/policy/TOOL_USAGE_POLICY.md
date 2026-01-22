# Tool Usage Policy: Slow I/O and Massive Data

## Problem
- `find` on /mnt/data1 (7.3TB, 80% full) is extremely slow
- `glob` on large directories causes timeouts
- Scanning /nix/store repeatedly is inefficient

## Policy

### ❌ NEVER Use
```bash
# DON'T: Scan entire filesystem
find /mnt/data1 -name "*.rs"
find /nix/store -name "*-with-logs"

# DON'T: Recursive glob on large dirs
glob("**/*.nix")
```

### ✅ ALWAYS Use Instead
```bash
# DO: Use git ls-files (only tracked files)
git ls-files | grep "\.rs$"

# DO: Use pre-indexed lists
cat nix_build_packages.json | jq -r '.[] | .key'

# DO: Direct paths from known locations
ls /nix/store/*-with-logs 2>/dev/null

# DO: Cached results
cat project_index.txt
```

## Approved Patterns

### 1. Git-Tracked Files Only
```bash
# Fast: Only scans git index
git ls-files "*.nix"
git ls-files "*.rs" | head -100
```

### 2. Pre-Indexed Data
```bash
# Use existing JSON/CSV indexes
cat nix_build_packages.json
cat FILE_GIT_MAPPING.csv
```

### 3. Direct Store Queries
```bash
# Fast: Direct glob in /nix/store (single level)
ls /nix/store/*-with-logs
nix-store -q --references /nix/store/abc-project
```

### 4. Cached Indexes
```bash
# Create index once, reuse many times
ls /nix/store > /tmp/store_index.txt
grep "with-logs" /tmp/store_index.txt
```

## Build Script Fix

### ❌ Old (Slow)
```bash
flake_dir=$(find /mnt/data1 -name "$project" -type d)
```

### ✅ New (Fast)
```bash
# Use pre-built index
flake_dir=$(grep "$project" project_paths.txt)

# Or use known structure
flake_dir="/mnt/data1/time-2026/01-january/18/solflake/smart_contracts/solana/$project"
```

## Implementation

### Create Project Index (Once)
```bash
# Run once, cache results
git ls-files | grep "flake.nix" | xargs dirname > flake_paths.txt
```

### Use Index (Many Times)
```bash
# Fast lookups
grep "Jupiter" flake_paths.txt
```

## Monitoring

### Allowed
- `ps aux` (process list)
- `top` (system stats)
- `df -h` (disk usage)
- `free -h` (memory)
- `uptime` (load average)

### Restricted
- `find /mnt/data1` (too slow)
- `du -sh /mnt/data1` (too slow)
- `ls -R /nix/store` (too slow)

## Enforcement

All scripts must:
1. Use git ls-files for file discovery
2. Use pre-indexed data for lookups
3. Cache results for repeated queries
4. Never scan entire filesystem

## Exceptions

Only use `find` when:
1. Limited to small directory (< 1000 files)
2. Max depth specified (`-maxdepth 2`)
3. No alternative exists
4. User explicitly approves
