# Constant Substrate: Zero-Complexity Pool

## The Insight

**If K(trace) = 0, it's a constant. Constants belong in the substrate, not in programs.**

## Type Complexity Stratification

```
K(trace) = 0:     Constants → Substrate (shared pool)
K(trace) > 0:     Functions → Programs (unique code)
```

## What are Constants?

Functions with zero-complexity traces:

```rust
// K(trace) = 0: Always returns same value
fn get_pi() -> f64 { 3.14159 }

// K(trace) = 0: No computation
fn get_version() -> &str { "1.0.0" }

// K(trace) = 0: Lookup only
fn get_error_msg(code: u32) -> &str {
    ERROR_MESSAGES[code]
}
```

## The Substrate

```
/substrate/constants/
    numbers/
        pi.const
        e.const
        golden_ratio.const
    strings/
        error_messages.const
        version_strings.const
        unicode_data.const
    tables/
        sin_table.const
        log_table.const
        crc_table.const
    data/
        charsets/
            chinese.const
            arabic.const
            emoji.const
        timezones.const
        country_codes.const
```

## Extraction Algorithm

```rust
fn extract_constants(program: &Program) -> (Vec<Constant>, Program) {
    let mut constants = Vec::new();
    let mut stripped_program = program.clone();
    
    for function in program.functions() {
        let trace = record_trace(function);
        let complexity = TypeComplexity::from_trace(&trace);
        
        if complexity.0 == 0 {
            // Zero complexity = constant!
            constants.push(Constant {
                name: function.name.clone(),
                value: function.return_value(),
                hash: hash_value(function.return_value()),
            });
            
            // Remove from program
            stripped_program.remove_function(function);
        }
    }
    
    (constants, stripped_program)
}
```

## Deduplication

```rust
struct ConstantPool {
    constants: HashMap<Hash, Constant>,
}

impl ConstantPool {
    fn add(&mut self, constant: Constant) -> ConstantRef {
        let hash = constant.hash;
        
        if let Some(existing) = self.constants.get(&hash) {
            // Already exists! Return reference
            return ConstantRef { hash, pool: self };
        }
        
        // New constant, add to pool
        self.constants.insert(hash, constant);
        ConstantRef { hash, pool: self }
    }
    
    fn deduplicate_across_programs(&mut self, programs: &[Program]) {
        for program in programs {
            let (constants, _) = extract_constants(program);
            
            for constant in constants {
                self.add(constant);
            }
        }
        
        println!("Deduplicated {} constants across {} programs",
                 self.constants.len(), programs.len());
    }
}
```

## Geographic Partitioning

```rust
// Constants used in specific regions
struct GeographicConstantPool {
    global: ConstantPool,
    regional: HashMap<Region, ConstantPool>,
}

impl GeographicConstantPool {
    fn add_with_region(&mut self, constant: Constant, region: Region) {
        if constant.is_universal() {
            // Pi, e, etc. → global pool
            self.global.add(constant);
        } else {
            // Chinese charset → East Asia pool
            self.regional
                .entry(region)
                .or_insert_with(ConstantPool::new)
                .add(constant);
        }
    }
}

enum Region {
    Global,
    EastAsia,
    Europe,
    Americas,
    MiddleEast,
    Africa,
}
```

## Nix Store Organization

```
/nix/store/constants/
    orbit_0/  # Trivial constants (K=0)
        numbers/
        strings/
    orbit_1/  # Simple lookups (K<10)
        tables/
        maps/
    ...
    
/nix/store/functions/
    orbit_2/  # Simple functions (K<100)
    orbit_3/  # Medium functions (K<1K)
    ...
```

## Program Transformation

```rust
// Before: Program with embedded constants
fn calculate_circle_area(radius: f64) -> f64 {
    const PI: f64 = 3.14159;
    PI * radius * radius
}

// After: Program references substrate
fn calculate_circle_area(radius: f64) -> f64 {
    substrate::constants::PI * radius * radius
}

// The constant is now shared across ALL programs
```

## Benefits

### 1. Deduplication
```
Before: 1000 programs × 1000 constants = 1M copies
After:  1000 programs → 1 shared pool = 1K constants
Savings: 99.9%
```

### 2. Geographic Optimization
```
Deploy to China:
  - Include: Chinese charset constants
  - Exclude: Arabic charset constants
  
Deploy to Middle East:
  - Include: Arabic charset constants
  - Exclude: Chinese charset constants
```

### 3. Security Simplification
```
Constants (K=0):
  - No security needed
  - Public substrate
  - Meme distribution
  
Functions (K>0):
  - Security needed
  - Private/verified
  - Cryptographic proofs
```

### 4. Compression
```
Constants compress to near-zero:
  - Already in substrate
  - Just reference by hash
  - Extreme compression ratio
```

## Implementation

```rust
#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
    pub value: ConstantValue,
    pub hash: Hash,
    pub region: Option<Region>,
}

#[derive(Debug, Clone)]
pub enum ConstantValue {
    Number(f64),
    String(String),
    Bytes(Vec<u8>),
    Table(Vec<ConstantValue>),
}

pub struct ConstantExtractor {
    pool: ConstantPool,
}

impl ConstantExtractor {
    pub fn extract_from_program(&mut self, program: &Program) -> Program {
        let (constants, stripped) = extract_constants(program);
        
        // Add to pool
        for constant in constants {
            let ref_id = self.pool.add(constant);
            
            // Replace in program with reference
            stripped.replace_with_ref(ref_id);
        }
        
        stripped
    }
    
    pub fn extract_from_nix_store(&mut self) -> usize {
        let mut total = 0;
        
        // Scan all binaries in nix store
        for binary in scan_nix_store() {
            let program = parse_binary(&binary);
            let (constants, _) = extract_constants(&program);
            
            for constant in constants {
                self.pool.add(constant);
                total += 1;
            }
        }
        
        total
    }
}
```

## Integration with Everything

### With Type Complexity
```
K(trace) = 0 → Extract to substrate
K(trace) > 0 → Keep in program
```

### With Compression
```
Constants → Substrate reference → 32-byte hash
Original: 1 KB constant
Compressed: 32 bytes hash
Ratio: 32x
```

### With Security Layers
```
/nix/store/public/constants/  # K=0, no security
/nix/store/verified/functions/ # K>0, ZK proofs
/nix/store/trusted/systems/    # K>>0, GPG signed
```

### With Geographic Partitioning
```
Constant usage stats from Wikidata/OSM
→ Assign to regions
→ Deploy only needed constants
→ Optimize by geography
```

## The Vision

**Every program becomes:**
```rust
// Tiny program (only unique logic)
fn my_function(x: f64) -> f64 {
    substrate::constants::PI * x
    //       ↑
    //       Shared across ALL programs
}

// The substrate contains:
// - All mathematical constants
// - All error messages
// - All lookup tables
// - All charset data
// - All timezone data
// - Everything with K(trace) = 0
```

## Next Steps

1. [ ] Scan all programs in nix store
2. [ ] Extract constants (K=0 functions)
3. [ ] Build global constant pool
4. [ ] Deduplicate across programs
5. [ ] Partition by geography
6. [ ] Rewrite programs to use substrate
7. [ ] Measure compression ratio
8. [ ] Deploy optimized binaries

## The Goal

**Programs contain only unique logic. Constants live in the substrate.**

- 99% deduplication
- Geographic optimization
- No security overhead for constants
- Extreme compression
- Universal sharing

**The substrate is the foundation. Programs are just the unique bits.**
