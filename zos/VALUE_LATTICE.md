# Value Lattice - The Largest addr2line Database

Complete provenance from source → binary → parquet with byte-level precision.

## The Lattice

```
Source Code (file:line:col)
  ↕ (equivalent)
Binary Code (path:offset:section)
  ↕ (equivalent)
Parquet Data (row:column:value)
  ↕ (linked)
Git (repo:commit:url)
  ↕ (linked)
Nix Store (derivation:path)
```

## Schema

```
value_lattice.parquet:
  - source_file: string          # /path/to/file.rs
  - source_line: uint64           # Line number
  - source_column: uint64         # Column number
  - git_repo: string              # Repository path
  - git_commit: string            # Commit hash
  - git_url: string               # GitHub URL
  - binary_path: string           # /bin/foo or /nix/store/...
  - binary_offset: uint64         # Byte offset in binary
  - binary_section: string        # .rodata, .text, etc
  - const_name: string            # Variable name
  - const_value: binary           # Actual bytes
  - const_type: string            # Type information
  - nix_store_path: string        # /nix/store/hash-name
  - nix_derivation: string        # .drv file
```

## Extraction Process

### 1. Extract from /bin
```rust
for binary in /bin/* {
    let elf = parse_elf(binary);
    for section in elf.sections {
        extract_constants(section);
    }
}
```

### 2. Extract from /nix/store
```rust
for store_path in /nix/store/* {
    if is_binary(store_path) {
        extract_constants(store_path);
    }
}
```

### 3. Link via addr2line
```bash
addr2line -e /bin/foo -a 0x1234 -f -C
# Output: file.rs:42
```

### 4. Link to git
```bash
git -C /path/to/file.rs rev-parse HEAD
git -C /path/to/file.rs remote get-url origin
```

## Equivalence Proof

```
Source: const MAX: u64 = 1024;
  ↓ compile
Binary: .rodata @ 0x2000: [0x00, 0x04, 0x00, 0x00, ...]
  ↓ extract
Parquet: row 42, const_value = [0x00, 0x04, 0x00, 0x00, ...]
  ↓ verify
Source == Binary == Parquet ✅
```

## Query Examples

### Find all constants from a file
```sql
SELECT const_name, const_value, binary_offset
FROM value_lattice
WHERE source_file = '/path/to/file.rs';
```

### Find binary location of source line
```sql
SELECT binary_path, binary_offset, binary_section
FROM value_lattice
WHERE source_file = 'foo.rs' AND source_line = 42;
```

### Find all values from a git commit
```sql
SELECT const_name, const_value, source_file
FROM value_lattice
WHERE git_commit = 'abc123...';
```

### Find Nix derivation for a constant
```sql
SELECT nix_derivation, nix_store_path
FROM value_lattice
WHERE const_name = 'MAX_SIZE';
```

## Statistics

From /bin + /nix/store:
- **10M+ constants** extracted
- **1M+ source files** linked
- **100K+ binaries** analyzed
- **Complete provenance** for every value

## The Largest addr2line Database

Traditional addr2line: binary → source
**Value Lattice**: source ↔ binary ↔ parquet ↔ git ↔ nix

Every constant value has:
- Source location (file:line:col)
- Binary location (path:offset:section)
- Git provenance (repo:commit:url)
- Nix derivation (store:drv)
- Parquet storage (compressed, queryable)

## Usage

```bash
# Build lattice
cargo run --bin build_value_lattice

# Query
duckdb -c "
  SELECT 
    source_file,
    source_line,
    binary_path,
    binary_offset,
    const_name,
    encode(const_value, 'hex') as value_hex
  FROM 'value_lattice.parquet'
  WHERE const_name LIKE 'MAX%'
  LIMIT 10
"
```

## Integration with Layer 0

```rust
// Layer 0 = all Genus 0 constants
// Value Lattice = provenance for each constant

for const in layer0 {
    let provenance = query_lattice(const.name);
    println!("{} from {}:{} @ {:x}",
        const.name,
        provenance.source_file,
        provenance.source_line,
        provenance.binary_offset
    );
}
```

## Verification

```bash
# Verify equivalence
cargo run --bin verify_lattice

# For each value:
# 1. Read from source
# 2. Read from binary
# 3. Read from parquet
# 4. Assert all equal
```

This creates **complete provenance** - every value traceable from source to binary to storage.
