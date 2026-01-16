# Git Object Scanner: Compression Archaeology

## Vision

Understand git compression at the byte level by observing pack/unpack operations.

## Key Insights

1. **Each repo has natural compression patterns** - Discover them
2. **Compressed bytes have semantics** - Assign meaning
3. **Perf reveals compression behavior** - Record pack/unpack
4. **Custom scanners per repo** - Optimized decoders
5. **Keyword index from compressed data** - Direct access

## Architecture

```
Git Objects (.git/objects/pack/*.pack)
    ↓
Perf record git pack/unpack
    ↓
Analyze compression patterns
    ↓
Build custom scanner
    ↓
Extract keywords directly from compressed form
    ↓
Index without full decompression
```

## Implementation Plan

### Phase 1: Git Object Analysis

```rust
// Read git pack files
fn read_pack_file(path: &str) -> Vec<u8>;

// Parse pack header
struct PackHeader {
    signature: [u8; 4],  // "PACK"
    version: u32,
    num_objects: u32,
}

// Parse pack objects
enum GitObject {
    Commit(Vec<u8>),
    Tree(Vec<u8>),
    Blob(Vec<u8>),
    Tag(Vec<u8>),
}
```

### Phase 2: Compression Pattern Discovery

```rust
// Record git operations with perf
fn perf_record_git_pack() -> PerfData;
fn perf_record_git_unpack() -> PerfData;

// Analyze patterns
struct CompressionPattern {
    byte_sequences: Vec<Vec<u8>>,
    frequencies: HashMap<Vec<u8>, usize>,
    semantics: HashMap<Vec<u8>, String>,
}

fn discover_patterns(pack: &[u8]) -> CompressionPattern;
```

### Phase 3: Semantic Assignment

```rust
// Assign meaning to compressed sequences
fn assign_semantics(pattern: &[u8]) -> Option<Semantic>;

enum Semantic {
    Keyword(String),      // "fn", "struct", "impl"
    Identifier(String),   // Function/type names
    Delimiter,            // Braces, parens
    Whitespace,
    Comment,
}
```

### Phase 4: Custom Scanner

```rust
// Build repo-specific scanner
struct RepoScanner {
    patterns: CompressionPattern,
    decoder: Box<dyn Fn(&[u8]) -> Option<Token>>,
}

impl RepoScanner {
    fn scan_compressed(&self, data: &[u8]) -> Vec<Token>;
    fn find_keyword(&self, keyword: &str, data: &[u8]) -> Vec<usize>;
}
```

### Phase 5: Keyword Index

```rust
// Index without full decompression
struct CompressedIndex {
    keywords: HashMap<String, Vec<Location>>,
    patterns: CompressionPattern,
}

struct Location {
    pack_file: String,
    offset: usize,
    compressed_form: Vec<u8>,
}

impl CompressedIndex {
    fn search(&self, keyword: &str) -> Vec<Location>;
    fn extract(&self, loc: &Location) -> String;
}
```

## Git Pack Format

```
Pack file structure:
- Header (12 bytes)
  - "PACK" signature (4 bytes)
  - Version (4 bytes)
  - Number of objects (4 bytes)
- Objects (variable)
  - Type + size (variable length)
  - Compressed data (zlib)
- Checksum (20 bytes SHA-1)
```

## Compression Patterns to Discover

1. **Repeated keywords** - "fn", "struct", "impl"
2. **Common identifiers** - Function names
3. **Structural patterns** - Indentation, braces
4. **Delta compression** - References to other objects
5. **String literals** - Common strings

## Perf Recording Strategy

```bash
# Record git pack operation
perf record -e cycles,instructions,cache-misses \
    -g --call-graph dwarf \
    git pack-objects --all --stdout > /dev/null

# Record git unpack operation  
perf record -e cycles,instructions,cache-misses \
    -g --call-graph dwarf \
    git unpack-objects < pack_file

# Analyze
perf report --stdio
perf script > pack_trace.txt
```

## Pattern Discovery Algorithm

```rust
fn discover_compression_patterns(pack: &[u8]) -> CompressionPattern {
    let mut patterns = CompressionPattern::new();
    
    // 1. Find repeated byte sequences
    for window_size in 2..32 {
        for window in pack.windows(window_size) {
            *patterns.frequencies.entry(window.to_vec()).or_insert(0) += 1;
        }
    }
    
    // 2. Filter high-frequency patterns
    let common: Vec<_> = patterns.frequencies.iter()
        .filter(|(_, &count)| count > 100)
        .collect();
    
    // 3. Decompress samples and assign semantics
    for (bytes, _) in common {
        if let Some(decompressed) = try_decompress(bytes) {
            if let Some(semantic) = classify(&decompressed) {
                patterns.semantics.insert(bytes.clone(), semantic);
            }
        }
    }
    
    patterns
}
```

## Semantic Classification

```rust
fn classify(text: &str) -> Option<String> {
    // Rust keywords
    if matches!(text, "fn" | "struct" | "impl" | "trait" | "mod") {
        return Some(format!("rust_keyword:{}", text));
    }
    
    // Function signatures
    if text.starts_with("fn ") && text.contains('(') {
        return Some("function_signature".to_string());
    }
    
    // Type definitions
    if text.starts_with("struct ") || text.starts_with("enum ") {
        return Some("type_definition".to_string());
    }
    
    None
}
```

## Custom Scanner Generation

```rust
fn generate_scanner(patterns: &CompressionPattern) -> RepoScanner {
    // Build lookup table for fast pattern matching
    let mut lookup = HashMap::new();
    
    for (bytes, semantic) in &patterns.semantics {
        lookup.insert(bytes.clone(), semantic.clone());
    }
    
    // Create decoder function
    let decoder = move |data: &[u8]| -> Option<Token> {
        // Try to match known patterns
        for len in (2..32).rev() {
            if data.len() >= len {
                if let Some(semantic) = lookup.get(&data[..len]) {
                    return Some(Token {
                        semantic: semantic.clone(),
                        bytes: data[..len].to_vec(),
                    });
                }
            }
        }
        None
    };
    
    RepoScanner {
        patterns: patterns.clone(),
        decoder: Box::new(decoder),
    }
}
```

## Keyword Search Without Decompression

```rust
fn search_compressed(
    keyword: &str,
    pack: &[u8],
    patterns: &CompressionPattern
) -> Vec<usize> {
    let mut results = Vec::new();
    
    // Find compressed form of keyword
    let compressed_forms: Vec<_> = patterns.semantics.iter()
        .filter(|(_, sem)| sem.contains(keyword))
        .map(|(bytes, _)| bytes)
        .collect();
    
    // Search for compressed forms directly
    for form in compressed_forms {
        let mut pos = 0;
        while let Some(offset) = find_bytes(&pack[pos..], form) {
            results.push(pos + offset);
            pos += offset + form.len();
        }
    }
    
    results
}
```

## Integration with Test Driver

```rust
// Add to test_driver.rs
fn scan_git_objects(repo_path: &str) -> CompressedIndex {
    let pack_dir = format!("{}/.git/objects/pack", repo_path);
    let mut index = CompressedIndex::new();
    
    for pack_file in find_pack_files(&pack_dir) {
        let data = read_pack_file(&pack_file);
        let patterns = discover_compression_patterns(&data);
        
        // Index keywords
        for keyword in RUST_KEYWORDS {
            let locations = search_compressed(keyword, &data, &patterns);
            index.add_keyword(keyword, locations);
        }
    }
    
    index
}
```

## Next Steps

1. [ ] Parse git pack file format
2. [ ] Record perf data for pack/unpack
3. [ ] Discover byte patterns
4. [ ] Assign semantics to patterns
5. [ ] Build custom scanner
6. [ ] Create keyword index
7. [ ] Search without decompression
8. [ ] Integrate with test driver

## The Goal

**Search git repos at the compressed byte level without full decompression.**

Each repo gets a custom scanner optimized for its compression patterns.
