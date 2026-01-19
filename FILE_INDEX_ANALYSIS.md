# File Index Analysis

## Existing Find Results

### files.txt (older run)
- **Size**: 3,506,145,308 bytes (3.5 GB)
- **Estimated files**: ~27 million (3.5GB / 128 bytes avg)
- **Date**: Jan 11 19:53
- **Location**: `/mnt/data1/files.txt`

### newfiles.txt (newer run)
- **Size**: 446,117,776 bytes (446 MB)
- **Estimated files**: ~3.5 million (446MB / 128 bytes avg)
- **Date**: Jan 16 15:56
- **Location**: `/mnt/data1/newfiles.txt`

## Key Insight

**27M files → 3.5M files** = Massive deduplication or scope change

This suggests:
- Either the newer run is scoped differently
- Or significant cleanup happened
- Or newfiles.txt is incremental (only new files)

## File Format

Both files contain **relative paths** from find results:

```
./path/to/file1.rs
./another/path/file2.txt
./nix/store/hash-package/bin/program
```

## Usage in Scripts

Let me search for how these are used:

```bash
# Common patterns:
grep -r "files.txt" /mnt/data1/meta-introspector/
grep -r "newfiles.txt" /mnt/data1/meta-introspector/
```

## Absolute vs Relative Paths

### Current State: Relative Paths
```
./meta-introspector/src/main.rs
./nix/store/abc123-rust/bin/rustc
```

### Need: Absolute Paths
```
/mnt/data1/meta-introspector/src/main.rs
/nix/store/abc123-rust/bin/rustc
```

## Conversion Strategy

```bash
# Convert relative to absolute
while IFS= read -r file; do
    # Remove leading ./
    file="${file#./}"
    
    # Determine base path
    if [[ "$file" == nix/store/* ]]; then
        echo "/nix/store/${file#nix/store/}"
    elif [[ "$file" == mnt/* ]]; then
        echo "/${file}"
    else
        echo "/mnt/data1/${file}"
    fi
done < /mnt/data1/files.txt > /mnt/data1/files_absolute.txt
```

## Integration with File Index

```rust
pub struct FileIndexLoader {
    // Load from find results
    pub fn load_from_find_results(path: &Path) -> Result<Vec<PathBuf>> {
        let base_dir = Path::new("/mnt/data1");
        
        BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| {
                let trimmed = line.trim_start_matches("./");
                
                // Convert to absolute
                if trimmed.starts_with("nix/store/") {
                    PathBuf::from("/").join(trimmed)
                } else if trimmed.starts_with("mnt/") {
                    PathBuf::from("/").join(trimmed)
                } else {
                    base_dir.join(trimmed)
                }
            })
            .collect()
    }
}
```

## Scheduled Updates

You mentioned running on a schedule. Recommended:

```bash
#!/bin/bash
# /mnt/data1/meta-introspector/update_file_index.sh

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT="/mnt/data1/files_${TIMESTAMP}.txt"

# Run find
cd /mnt/data1
find . -type f > "$OUTPUT"

# Keep only last 5 runs
ls -t /mnt/data1/files_*.txt | tail -n +6 | xargs rm -f

# Update symlink to latest
ln -sf "$OUTPUT" /mnt/data1/files_latest.txt

echo "✅ Updated: $OUTPUT"
```

## Cron Schedule

```cron
# Run every 6 hours
0 */6 * * * /mnt/data1/meta-introspector/update_file_index.sh

# Or daily at 2 AM
0 2 * * * /mnt/data1/meta-introspector/update_file_index.sh
```

## Analysis Tools

### 1. Diff Between Runs

```bash
# Find new files
comm -13 <(sort /mnt/data1/files.txt) <(sort /mnt/data1/newfiles.txt) > new_files.txt

# Find deleted files
comm -23 <(sort /mnt/data1/files.txt) <(sort /mnt/data1/newfiles.txt) > deleted_files.txt
```

### 2. Statistics

```bash
# Count by extension
cat /mnt/data1/files.txt | \
    sed 's/.*\.//' | \
    sort | uniq -c | sort -rn | head -20

# Count by directory
cat /mnt/data1/files.txt | \
    cut -d/ -f1-3 | \
    sort | uniq -c | sort -rn | head -20
```

### 3. Size Analysis

```bash
# Total size of all files
while IFS= read -r file; do
    [ -f "$file" ] && stat -c%s "$file"
done < /mnt/data1/files_absolute.txt | \
    awk '{sum+=$1} END {print sum/1024/1024/1024 " GB"}'
```

## Integration with SQLite Index

```sql
-- Import find results
CREATE TEMP TABLE find_results (path TEXT);

.mode csv
.import /mnt/data1/files_absolute.txt find_results

-- Merge with existing index
INSERT OR IGNORE INTO files (file_path, file_type)
SELECT path, 'unknown' FROM find_results;

-- Classify based on path patterns
UPDATE files SET file_type = 'git'
WHERE file_path LIKE '%/.git/%' 
   OR EXISTS (
       SELECT 1 FROM git_repos r 
       WHERE files.file_path LIKE r.path || '/%'
   );

UPDATE files SET file_type = 'nix'
WHERE file_path LIKE '/nix/store/%';

UPDATE files SET file_type = 'temp'
WHERE file_path LIKE '%/tmp/%'
   OR file_path LIKE '%/.cache/%'
   OR file_path LIKE '%/target/%';
```

## Recommended Workflow

1. **Keep find results**: Historical record
2. **Convert to absolute**: For database import
3. **Classify by source**: git/nix/temp/wip
4. **Track changes**: Diff between runs
5. **Update incrementally**: Only process new/changed files

## Next Steps

1. Convert existing files.txt and newfiles.txt to absolute paths
2. Import into SQLite index
3. Set up scheduled updates
4. Build diff analysis tools
5. Document all scripts that use these files

## File Provenance

Every file should be traceable to:
- **Git repo** + commit (if in git)
- **Nix store path** (if in /nix/store)
- **Find timestamp** (when discovered)
- **Classification** (git/nix/temp/wip)

This gives us **complete provenance** for all 27M+ files.
