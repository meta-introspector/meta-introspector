# Const71 - All Constants Up To Size 71

Complete collection of all constant values ≤ 71 bytes from all binaries.

## Why 71?

71 is a prime number and represents a natural boundary:
- Most primitive constants are < 71 bytes
- Strings, arrays, structs fit within 71
- Larger values are typically composite

## Collection

```
/bin/* + /nix/store/*
  ↓ extract .rodata
All byte sequences [1..71]
  ↓ deduplicate
Unique constants
  ↓ save
const71.parquet
```

## Schema

```
const71.parquet:
  - value: binary           # The constant bytes
  - size: uint8             # Length (1-71)
  - binary_path: string     # Where found
  - binary_offset: uint64   # Byte offset
  - source_file: string     # Source location
  - source_line: uint64     # Line number
```

## Statistics

Expected from /bin + /nix/store:
- **Size 1**: ~256 values (all bytes)
- **Size 2**: ~10K values
- **Size 4**: ~100K values (u32 constants)
- **Size 8**: ~500K values (u64, f64)
- **Size 16**: ~1M values (u128, arrays)
- **Size 32**: ~2M values (strings, hashes)
- **Size 64**: ~5M values (keys, buffers)
- **Size 71**: ~10M total unique constants

## Distribution

```
Size  Count      Examples
1     256        0x00, 0x01, 0xFF
2     10K        0x0000, 0x0001, 0xFFFF
4     100K       0, 1, 1024, MAX_U32
8     500K       0, 1, timestamps, pointers
16    1M         UUIDs, MD5 hashes
32    2M         SHA256 hashes, keys
64    5M         Buffers, large arrays
71    10M        All constants
```

## Queries

### Find all constants of size 8
```sql
SELECT encode(value, 'hex'), binary_path
FROM const71
WHERE size = 8
LIMIT 10;
```

### Find specific value
```sql
SELECT binary_path, binary_offset, source_file
FROM const71
WHERE value = '\x00\x00\x00\x00\x00\x00\x04\x00'  -- 1024 as u64
```

### Count by size
```sql
SELECT size, COUNT(*) as count
FROM const71
GROUP BY size
ORDER BY size;
```

### Find most common constants
```sql
SELECT value, COUNT(*) as occurrences
FROM const71
GROUP BY value
ORDER BY occurrences DESC
LIMIT 100;
```

## Integration with Layer 0

```rust
// Layer 0 = Genus 0 declarations
// Const71 = All constant values

for const in layer0 {
    let values = query_const71(const.name);
    println!("{} has {} occurrences", const.name, values.len());
}
```

## Usage

```bash
# Collect all constants
cargo run --bin collect_const71

# Query
duckdb -c "
  SELECT 
    size,
    COUNT(*) as count,
    COUNT(DISTINCT value) as unique_values
  FROM 'const71.parquet'
  GROUP BY size
  ORDER BY size
"
```

## Compression

Raw binaries: ~100GB
Extracted constants: ~10GB
Parquet (deduplicated): ~500MB

## Applications

### 1. Constant Deduplication
Find duplicate constants across binaries for optimization.

### 2. Value Analysis
Statistical analysis of constant distributions.

### 3. Security
Find hardcoded keys, passwords, secrets.

### 4. Optimization
Pre-compute common constants in Layer 0.

## Example Output

```
Size 1: 256 constants
  0x00 (NULL) - 1M occurrences
  0x01 (true) - 500K occurrences
  0xFF (MAX) - 200K occurrences

Size 4: 100K constants
  0x00000000 (0) - 2M occurrences
  0x00000001 (1) - 1M occurrences
  0x00000400 (1024) - 100K occurrences

Size 8: 500K constants
  0x0000000000000000 (0) - 5M occurrences
  0x0000000000000001 (1) - 2M occurrences
```

This creates a **complete catalog** of all constant values in the system.
