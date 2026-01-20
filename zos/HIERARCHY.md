# Level Hierarchy - Dependency Layers

Build complete dependency hierarchy from Level 0 upward.

## The Hierarchy

```
Level 0 (Const71)
  ↑ referenced by
Level 1 (references only Level 0)
  ↑ referenced by
Level 2 (references Level 0-1)
  ↑ referenced by
Level 3 (references Level 0-2)
  ↑ ...
Level N
```

## Definitions

### Level 0
- All constants ≤ 71 bytes
- No dependencies
- Genus 0

### Level 1
- Declarations that reference **only** Level 0
- Examples:
  ```rust
  const BUFFER_SIZE: usize = 1024;  // 1024 is Level 0
  type Byte = u8;                    // u8 is Level 0
  ```

### Level 2
- Declarations that reference Level 0 or Level 1
- Examples:
  ```rust
  type Buffer = [u8; BUFFER_SIZE];  // Uses Level 1 BUFFER_SIZE
  const MAX_BUFFERS: usize = BUFFER_SIZE * 2;  // Uses Level 1
  ```

### Level N
- Declarations that reference Level 0 through N-1
- Level = 1 + max(level of dependencies)

## Extraction Algorithm

```rust
// Level 0: Already have (const71.parquet)

// Level 1: Find declarations referencing only Level 0
for decl in all_declarations {
    if decl.references.all(|r| r.level == 0) {
        level1.push(decl);
    }
}

// Level 2: Find declarations referencing Level 0-1
for decl in all_declarations {
    if decl.references.all(|r| r.level <= 1) {
        level2.push(decl);
    }
}

// Continue for Level 3, 4, ...
```

## Schema

```
levelN.parquet:
  - name: string              # Declaration name
  - content: string           # Full declaration
  - level: uint8              # Level number
  - references: list<string>  # What it references
  - source_file: string       # Source location
  - source_line: uint64       # Line number
```

## Statistics

Expected distribution:
- **Level 0**: 10M constants
- **Level 1**: 1M declarations
- **Level 2**: 500K declarations
- **Level 3**: 200K declarations
- **Level 4+**: <100K declarations

Most code is Level 1-3.

## Queries

### Find all Level 1 declarations
```sql
SELECT name, content, source_file
FROM level1
ORDER BY name;
```

### Find what references a Level 0 constant
```sql
SELECT l1.name, l1.content
FROM level1 l1
WHERE list_contains(l1.references, 'const_1024');
```

### Count declarations per level
```sql
SELECT level, COUNT(*) as count
FROM (
  SELECT * FROM level1
  UNION ALL
  SELECT * FROM level2
  UNION ALL
  SELECT * FROM level3
)
GROUP BY level;
```

## Verification

```rust
// Verify Level 1 only references Level 0
for decl in level1 {
    assert!(decl.references.all(|r| r.level == 0));
}

// Verify Level 2 only references Level 0-1
for decl in level2 {
    assert!(decl.references.all(|r| r.level <= 1));
}
```

## Usage

```bash
# Build hierarchy
cargo run --bin build_hierarchy

# Query
duckdb -c "
  SELECT 
    l2.name,
    l2.content,
    list_aggregate(l2.references, 'string_agg', ',') as deps
  FROM 'level2.parquet' l2
  LIMIT 10
"
```

## Integration with ZOS Layers

```
ZOS Layer 0 (SELinux) = Code Level 0-1
ZOS Layer 1 (Services) = Code Level 2-3
ZOS Layer 2 (Build) = Code Level 4-5
...
```

## Example Hierarchy

```rust
// Level 0 (const71)
0x0400  // 1024 as bytes

// Level 1 (references Level 0)
const BUFFER_SIZE: usize = 1024;
type Byte = u8;

// Level 2 (references Level 1)
type Buffer = [Byte; BUFFER_SIZE];
const MAX_BUFFERS: usize = BUFFER_SIZE * 2;

// Level 3 (references Level 2)
struct BufferPool {
    buffers: [Buffer; MAX_BUFFERS],
}
```

## Benefits

1. **Dependency tracking**: Know exactly what depends on what
2. **Build optimization**: Build lower levels first
3. **Change impact**: Know what breaks when Level N changes
4. **Complexity measure**: Higher level = more complex

This creates a **complete dependency graph** of the entire codebase.
