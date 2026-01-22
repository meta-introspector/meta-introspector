# Enum Lattice: Complexity from Constants

## Vision

Build a complexity lattice starting from the simplest enums, using prime numbers as complexity signals.

## Core Idea

```
Level 0: Binary enum (2 variants)
    ↓
Level 1: Ternary enum (3 variants)
    ↓
Level 2: Prime enums (5, 7, 11, 13, ...)
    ↓
Level N: Combined enums (products, sums)
    ↓
Auto-discover all language features
```

## The Lattice

### Level 0: Binary (Complexity = 0)
```rust
enum Binary {
    Ok,
    NotOk,
}
```
- 2 variants (first prime)
- Simplest possible enum
- Represents boolean choice

### Level 1: Ternary (Complexity = 1)
```rust
enum Ternary {
    Yes,
    No,
    Maybe,
}
```
- 3 variants (second prime)
- Adds uncertainty
- Three-valued logic

### Level 2: Prime 5 (Complexity = 2)
```rust
enum Prime5 {
    Variant0,
    Variant1,
    Variant2,
    Variant3,
    Variant4,
}
```
- 5 variants (third prime)
- Prime number signal
- Natural complexity marker

### Level N: Prime P (Complexity = π(P))
```rust
enum PrimeP {
    Variant0,
    ...,
    VariantP,
}
```
- P variants (Pth prime)
- Complexity = number of primes up to P
- Natural ordering

## Combining Enums

### Product Type
```rust
enum Combined {
    Both(Binary, Ternary),
    First(Binary),
    Second(Ternary),
    Neither,
}
```
- Complexity = C1 + C2 + 1
- Combines two enums
- Creates new level

### Sum Type
```rust
enum Either {
    Left(Binary),
    Right(Ternary),
}
```
- Complexity = max(C1, C2) + 1
- Choice between enums
- Simpler than product

## The Cycle: Compress → Decompress → Compile

```rust
// 1. Generate enum
let enum_code = generate_prime_enum("Prime7", 7);

// 2. Compress
let compressed = compress(&enum_code);

// 3. Decompress
let decompressed = decompress(&compressed);

// 4. Compile
let compiled = compile(&decompressed);

// 5. Verify round-trip
assert_eq!(enum_code, decompressed);
assert!(compiled);
```

## Prime Number Signals

Primes mark natural complexity boundaries:
- 2: Binary choice
- 3: Ternary logic
- 5: Pentagonal symmetry
- 7: Weekly cycle
- 11: Hendecagonal
- 13: Tridecagonal
- ...

Each prime represents a natural complexity level.

## Auto-Discovery Algorithm

```rust
fn discover_patterns(source_code: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    
    // 1. Parse all enums in source
    for enum_item in parse_enums(source_code) {
        let complexity = compute_complexity(&enum_item);
        patterns.push(Pattern {
            name: enum_item.name,
            complexity,
            variants: enum_item.variants.len(),
        });
    }
    
    // 2. Find constants that resonate with examples
    for constant in find_constants(source_code) {
        if resonates_with(&constant, &patterns) {
            label_constant(&constant, &patterns);
        }
    }
    
    // 3. Use labeled constants to find similar code
    for pattern in &patterns {
        let similar = find_similar_code(source_code, pattern);
        patterns.extend(similar);
    }
    
    // 4. Detect duplicates
    let duplicates = find_duplicates(&patterns);
    
    // 5. Auto-expand coverage
    expand_coverage(&mut patterns, &duplicates);
    
    patterns
}
```

## Resonance Detection

```rust
fn resonates_with(constant: &Constant, patterns: &[Pattern]) -> bool {
    // Check if constant value matches pattern complexity
    for pattern in patterns {
        if constant.value == pattern.variants {
            return true;
        }
        
        if constant.value == pattern.complexity {
            return true;
        }
        
        if is_prime(constant.value) && is_prime(pattern.variants) {
            return true;
        }
    }
    
    false
}
```

## Labeling Strategy

```rust
fn label_constant(constant: &Constant, patterns: &[Pattern]) {
    for pattern in patterns {
        if constant.value == pattern.variants {
            constant.add_label(format!("enum_size:{}", pattern.name));
        }
        
        if constant.value == pattern.complexity {
            constant.add_label(format!("complexity:{}", pattern.name));
        }
    }
}
```

## Finding Similar Code

```rust
fn find_similar_code(source: &str, pattern: &Pattern) -> Vec<Pattern> {
    let mut similar = Vec::new();
    
    // Look for enums with same number of variants
    for enum_item in parse_enums(source) {
        if enum_item.variants.len() == pattern.variants {
            similar.push(Pattern {
                name: enum_item.name,
                complexity: pattern.complexity,
                variants: pattern.variants,
            });
        }
    }
    
    // Look for match statements with same number of arms
    for match_expr in parse_matches(source) {
        if match_expr.arms.len() == pattern.variants {
            similar.push(Pattern {
                name: format!("match_{}", match_expr.scrutinee),
                complexity: pattern.complexity,
                variants: pattern.variants,
            });
        }
    }
    
    similar
}
```

## Duplicate Detection

```rust
fn find_duplicates(patterns: &[Pattern]) -> Vec<(Pattern, Pattern)> {
    let mut duplicates = Vec::new();
    
    for i in 0..patterns.len() {
        for j in i+1..patterns.len() {
            if patterns[i].variants == patterns[j].variants {
                duplicates.push((patterns[i].clone(), patterns[j].clone()));
            }
        }
    }
    
    duplicates
}
```

## Coverage Expansion

```rust
fn expand_coverage(patterns: &mut Vec<Pattern>, duplicates: &[(Pattern, Pattern)]) {
    // For each duplicate pair, generate intermediate complexities
    for (p1, p2) in duplicates {
        let min_complexity = p1.complexity.min(p2.complexity);
        let max_complexity = p1.complexity.max(p2.complexity);
        
        // Fill in missing complexity levels
        for c in min_complexity..max_complexity {
            if !patterns.iter().any(|p| p.complexity == c) {
                // Generate enum at this complexity level
                let prime = nth_prime(c);
                patterns.push(Pattern {
                    name: format!("Prime{}", prime),
                    complexity: c,
                    variants: prime,
                });
            }
        }
    }
}
```

## Integration with Universal Labeler

```rust
// Combine enum lattice with semantic labeler
fn label_enum_in_compressed_form(
    compressed: &[u8],
    labeler: &UniversalLabeler,
    lattice: &ComplexityLattice
) -> Vec<String> {
    let mut labels = Vec::new();
    
    // Get semantic labels from decompression
    let semantic_labels = labeler.label_bytes(compressed);
    
    // Get complexity labels from lattice
    for label in semantic_labels {
        if label.contains("enum") {
            // Find matching pattern in lattice
            for level in &lattice.levels {
                for enum_name in &level.enums {
                    if label.contains(enum_name) {
                        labels.push(format!("complexity:{}", level.complexity));
                        labels.push(format!("prime:{}", level.enums.len()));
                    }
                }
            }
        }
    }
    
    labels
}
```

## The Complete Pipeline

```
1. Generate enum at complexity N
    ↓
2. Compress with zlib
    ↓
3. Record perf during decompression
    ↓
4. Record perf during compilation
    ↓
5. Correlate IPs → semantic labels
    ↓
6. Discover patterns in existing code
    ↓
7. Label constants that resonate
    ↓
8. Find similar code
    ↓
9. Detect duplicates
    ↓
10. Auto-expand coverage
    ↓
11. Build complete lattice
```

## Example: Full Cycle

```rust
// 1. Build lattice
let mut lattice = ComplexityLattice::new();
lattice.build(10);

// 2. Generate enum at level 3 (prime 7)
let enum_code = generate_prime_enum("Prime7", 7);

// 3. Compress
let compressed = compress(&enum_code);

// 4. Train labeler
let mut labeler = UniversalLabeler::new();
labeler.train_on_enum(&compressed, &enum_code);

// 5. Discover patterns in codebase
let patterns = lattice.discover_patterns(source_code);

// 6. Label everything
for pattern in patterns {
    let labels = label_enum_in_compressed_form(
        &pattern.compressed,
        &labeler,
        &lattice
    );
    println!("{}: {:?}", pattern.name, labels);
}
```

## Next Steps

1. [ ] Generate enums at all prime complexities
2. [ ] Compress and decompress each
3. [ ] Record perf for both operations
4. [ ] Train universal labeler
5. [ ] Scan codebase for enums
6. [ ] Find resonating constants
7. [ ] Label similar code
8. [ ] Detect duplicates
9. [ ] Auto-expand coverage
10. [ ] Build complete lattice

## The Goal

**Every enum in the codebase gets a complexity score and semantic label derived from its structure and how the compiler processes it.**

The lattice provides natural ordering, and auto-discovery ensures complete coverage.
