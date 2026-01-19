# Absolute Path Usage Audit

## Current Usage Patterns

### 1. Temporary File Lists (Relative Paths)
```bash
# scan_self.sh
find . -name "*.rs" -type f > /tmp/meta-introspector-files.txt

# local_cicd_runner.sh  
find . -name "*.rs" > /tmp/rs_files.txt
```

**Issue**: Relative paths break when scripts change directory

### 2. Cluster Files (Relative Paths)
```bash
# cluster_files.sh
sort -t/ -k9 "$INPUT_FILE" > "${OUTPUT_DIR}/sorted_files.txt"
split -l "$CHUNK_SIZE" -d -a 3 sorted_files.txt chunk_
```

**Issue**: Assumes current directory context

### 3. Nix Store (Absolute Paths) ✅
```bash
# test_real_telemetry.sh
/nix/store/3hgackxpbkjachs6qncykjbl0n9a2yla-rustc-1.94.0-nightly-2026-01-12-x86_64-unknown-linux-gnu/bin/rustc --version
```

**Good**: Absolute paths work everywhere

## Recommendation: Standardize on Absolute Paths

### Benefits
1. **No ambiguity**: Always know exact file location
2. **Works from any directory**: Scripts can run anywhere
3. **Better for databases**: SQLite/Postgres need absolute paths
4. **Easier debugging**: Clear what file is being processed

### Migration Strategy

```bash
#!/bin/bash
# Convert relative to absolute paths

convert_to_absolute() {
    local input_file="$1"
    local output_file="$2"
    local base_dir="${3:-/mnt/data1}"
    
    while IFS= read -r line; do
        # Skip empty lines
        [ -z "$line" ] && continue
        
        # Remove leading ./
        line="${line#./}"
        
        # Convert to absolute
        if [[ "$line" == /* ]]; then
            # Already absolute
            echo "$line"
        elif [[ "$line" == nix/store/* ]]; then
            # Nix store path
            echo "/${line}"
        else
            # Relative to base_dir
            echo "${base_dir}/${line}"
        fi
    done < "$input_file" > "$output_file"
}

# Usage
convert_to_absolute /mnt/data1/files.txt /mnt/data1/files_absolute.txt
convert_to_absolute /mnt/data1/newfiles.txt /mnt/data1/newfiles_absolute.txt
```

## Updated Script Patterns

### Before (Relative)
```bash
find . -name "*.rs" > /tmp/rs_files.txt
concept_map_builder /tmp/rs_files.txt
```

### After (Absolute)
```bash
find /mnt/data1/meta-introspector -name "*.rs" > /tmp/rs_files_absolute.txt
concept_map_builder /tmp/rs_files_absolute.txt
```

## Database Integration

```sql
-- With absolute paths, we can:

-- 1. Check if file exists
SELECT file_path FROM files 
WHERE file_path = '/mnt/data1/meta-introspector/src/main.rs';

-- 2. Join with git repos
SELECT f.file_path, r.name, r.commit
FROM files f
JOIN git_repos r ON f.file_path LIKE r.path || '/%';

-- 3. Find files by directory
SELECT file_path FROM files
WHERE file_path LIKE '/mnt/data1/meta-introspector/src/%';
```

## Action Items

1. **Convert existing files**:
   ```bash
   ./convert_to_absolute.sh /mnt/data1/files.txt /mnt/data1/files_absolute.txt
   ./convert_to_absolute.sh /mnt/data1/newfiles.txt /mnt/data1/newfiles_absolute.txt
   ```

2. **Update all scripts** to use absolute paths:
   - scan_self.sh
   - local_cicd_runner.sh
   - cluster_files.sh
   - Any script using find

3. **Update scheduled jobs** to generate absolute paths:
   ```bash
   # Instead of: find . -type f > files.txt
   # Use: find /mnt/data1 -type f > files_absolute.txt
   ```

4. **Document convention**:
   - All file lists use absolute paths
   - Stored in /mnt/data1/meta-introspector/indexes/
   - Named with timestamp: files_YYYYMMDD_HHMMSS.txt

## File Naming Convention

```
/mnt/data1/meta-introspector/indexes/
├── files_20260111_195300.txt          # Old find run (relative)
├── files_20260111_195300_absolute.txt # Converted to absolute
├── files_20260116_155600.txt          # New find run (relative)
├── files_20260116_155600_absolute.txt # Converted to absolute
├── files_latest.txt -> files_20260116_155600_absolute.txt
└── file_index.db                      # SQLite database
```

## Verification

```bash
# Check all paths are absolute
grep -v '^/' /mnt/data1/files_absolute.txt && echo "❌ Found relative paths!" || echo "✅ All absolute"

# Count files by type
grep -c '^/nix/store/' /mnt/data1/files_absolute.txt  # Nix files
grep -c '^/mnt/data1/' /mnt/data1/files_absolute.txt  # Data files
```

## Summary

**Current State**: Mix of relative and absolute paths  
**Target State**: All absolute paths  
**Benefit**: Consistency, reliability, database integration  
**Effort**: Convert existing files + update ~5 scripts
