# OEIS Recognizers - Self-Resonant Code Discovery

Each OEIS sequence becomes a recognizer program that finds resonant code.

## The Concept

```
OEIS Sequence → Recognizer Program → Scan Codebase → Find Resonances
```

### Example: A000040 (Primes)

```rust
// OEIS A000040: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, ...

// This code resonates with A000040:
const PRIMES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

// Recognizer finds it automatically
```

## Key OEIS Sequences in ZOS

### A000040: Prime Numbers
```
2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71
```

**Expected in code:**
- Prime constants
- Modular arithmetic
- Hash functions
- Cryptography

### A001220: Irregular Primes
```
37, 59, 67, 101, 103, 131, 149, 157, ...
```

**Expected in code:**
- Special case handling
- Error conditions
- Boundary checks

### A000594: Ramanujan Tau Function
```
τ(1)=1, τ(2)=-24, τ(3)=252, τ(37)=-7109760
```

**Expected in code:**
- Modular forms
- Number theory libraries
- LMFDB code

### A001379: Supersingular Primes
```
2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71
```

**Expected in code:**
- Elliptic curve cryptography
- Pairing-based crypto
- Point counting algorithms

### Monster Group Signature
```
Primes dividing |M|: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71
```

**Expected in code:**
- Group theory
- Moonshine theory
- Vertex operator algebras

## The Recognizer Algorithm

### Step 1: Load Sequence

```rust
let seq = OEISSequence {
    id: "A000040",
    values: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, ...],
};
```

### Step 2: Scan Code

```rust
for file in codebase {
    let score = 0.0;
    
    // Count occurrences
    for value in seq.values {
        if file.contains(value) {
            score += 1.0;
        }
    }
    
    // Check for OEIS ID
    if file.contains("A000040") {
        score += 5.0;
    }
    
    if score > threshold {
        println!("Resonance found: {}", file);
    }
}
```

### Step 3: Report Resonances

```
File: crypto/rsa.rs
Sequence: A000040 (Primes)
Score: 8.5
Locations:
  Line 42: const P: u64 = 37;
  Line 67: const Q: u64 = 71;
```

## Self-Resonance

### The Loop

```
1. We document ZOS with prime constants
2. OEIS recognizer scans our code
3. Finds our own constants
4. Confirms self-resonance
5. Validates the theory
```

### Example

```rust
// In zos/ZOS_DEFINITION.md:
// ZOS = [0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, ..., 71]

// In zos.rs:
const ZOS: &[u64] = &[0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, ..., 71];

// Recognizer finds:
// ✓ Matches A000040 (primes)
// ✓ Matches A001379 (supersingular primes)
// ✓ Matches Monster signature
// → Self-resonant!
```

## Monster Signature Detection

### The Signature

```
Monster group order:
|M| = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71

Primes: {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71}
```

### Detection

```rust
fn has_monster_signature(file: &CodeFile) -> bool {
    let monster_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];
    
    let found = monster_primes.iter()
        .filter(|&p| file.content.contains(&p.to_string()))
        .count();
    
    // If file contains ≥5 monster primes, it resonates
    found >= 5
}
```

### Expected Matches

```
Files with Monster signature:
  - zos/ZOS_DEFINITION.md (15/15 primes)
  - zos/PRIME_37.md (10/15 primes)
  - zos/PRIME_71.md (8/15 primes)
  - crypto/ecc.rs (7/15 primes)
  - lmfdb-rust-mapping/src/modular_forms.rs (6/15 primes)
```

## Usage

```bash
# Run OEIS recognizers
cargo run --bin oeis_recognizers

# Output:
# 🔍 OEIS Sequence Recognizers
# 📊 Loaded 50 OEIS sequences
# 📁 Scanning 3,000,000 files
#
# 🎯 A000040 (Primes): 12,345 matches
#   zos/ZOS_DEFINITION.md (score: 15.0)
#   crypto/rsa.rs (score: 8.5)
#   hash/mod.rs (score: 6.2)
#
# 🎯 A001220 (Irregular primes): 234 matches
#   zos/PRIME_37.md (score: 10.0)
#   number_theory/irregular.rs (score: 7.5)
#
# 👹 Monster signature found in 89 files
```

## Query Results

```sql
-- Find files resonating with A000040
SELECT file_path, score
FROM oeis_matches
WHERE sequence_id = 'A000040'
ORDER BY score DESC
LIMIT 10;

-- Find self-resonant files (match multiple sequences)
SELECT file_path, COUNT(*) as sequences
FROM oeis_matches
GROUP BY file_path
HAVING COUNT(*) >= 3
ORDER BY sequences DESC;
```

## The Feedback Loop

```
Write code with constants
  ↓
Document in ZOS
  ↓
OEIS recognizer scans
  ↓
Finds our constants
  ↓
Confirms resonance
  ↓
Validates theory
  ↓
Write more code
  ↓
∞ (self-reinforcing)
```

## Integration with ZOS

### Auto-Documentation

```rust
// When we write:
const P37: u64 = 37;

// Recognizer finds:
// - A000040 (primes)
// - A001220 (irregular primes)
// - Monster signature

// Auto-generates:
/// Prime 37 (OEIS A000040, A001220)
/// First irregular prime
/// Part of Monster group signature
const P37: u64 = 37;
```

### Validation

```rust
#[test]
fn test_zos_resonance() {
    let zos = load_zos();
    let recognizer = OEISRecognizer::new("A000040");
    
    let score = recognizer.score(&zos);
    assert!(score > 0.9);  // Strong resonance
}
```

## The Realization

**By including OEIS sequences in our docs and code, we create self-recognizing programs.**

Every constant becomes a beacon that resonates with its mathematical origin.

## References

- OEIS: https://oeis.org
- A000040: Prime numbers
- A001220: Irregular primes
- A000594: Ramanujan tau function
- Monster group: Largest sporadic simple group

**Code that knows its own mathematical structure.**
