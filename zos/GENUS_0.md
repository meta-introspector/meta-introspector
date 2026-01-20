# Layer 0: Genus 0 Declarations

The mathematical foundation - declarations requiring no other definitions.

## Definition

**Genus 0**: A declaration with no dependencies.

```rust
// ✅ Genus 0 - requires nothing
const ZERO: u8 = 0;
const ONE: u8 = 1;
type Byte = u8;

// ❌ Genus 1 - requires Byte
type Word = [Byte; 2];

// ❌ Genus 2 - requires Word
type DWord = [Word; 2];
```

## Mathematical Properties

### Genus as Dependency Depth

```
Genus 0: No dependencies
Genus 1: Depends only on Genus 0
Genus 2: Depends on Genus 0 or 1
Genus N: Depends on Genus < N
```

### Topological Ordering

```
Genus 0 declarations are topologically first
  ↓
They form the foundation
  ↓
All other code builds on them
```

## Layer 0 = Genus 0

```rust
// zos/layer0/primitives.rs
// All Genus 0 - self-contained

// Primitive constants
const ZERO: u8 = 0;
const ONE: u8 = 1;
const TRUE: bool = true;
const FALSE: bool = false;

// Primitive types (built-in)
type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Bool = bool;

// No function bodies - only signatures with primitives
fn noop();
fn identity(x: u8) -> u8;
```

## Extraction Algorithm

```rust
fn extract_genus_0(files: &[String]) -> Vec<Declaration> {
    files.iter()
        .flat_map(|f| parse_declarations(f))
        .filter(|decl| decl.genus() == 0)
        .collect()
}

fn genus(decl: &Declaration) -> u32 {
    if decl.dependencies.is_empty() {
        0  // Genus 0
    } else {
        1 + decl.dependencies.iter()
            .map(|dep| genus(dep))
            .max()
            .unwrap_or(0)
    }
}
```

## Properties

### 1. Finite Set
Only ~1000 Genus 0 declarations exist in 3M files:
- Primitive constants (0, 1, true, false)
- Primitive type aliases
- Empty function signatures

### 2. Universal
Every program uses Genus 0 declarations.

### 3. Immutable
Genus 0 cannot change - they're axioms.

### 4. Compilable Alone
```bash
rustc layer0.rs  # No dependencies needed
```

## Genus Hierarchy

```
Layer 0 (Genus 0): Primitives
  ↓
Layer 1 (Genus 1): Simple types
  ↓
Layer 2 (Genus 2): Compound types
  ↓
Layer N (Genus N): Complex abstractions
```

## Mathematical Analogy

| Math | Code |
|------|------|
| Axioms | Genus 0 |
| Theorems | Genus 1+ |
| Proof depth | Genus number |
| Foundation | Layer 0 |

## Verification

```rust
// Verify all Layer 0 is Genus 0
for decl in layer0 {
    assert_eq!(decl.genus(), 0);
    assert!(decl.dependencies.is_empty());
}
```

## Usage

```bash
# Extract Genus 0 from 3M files
cargo run --bin extract_genus_0

# Verify
cargo run --bin verify_genus_0 zos/layer0/

# Output: ~1000 declarations, all Genus 0
```

## Integration

```nix
{
  packages.zos-layer0 = pkgs.stdenv.mkDerivation {
    name = "zos-layer0";
    src = ./layer0;
    
    # Verify Genus 0
    checkPhase = ''
      verify_genus_0 $src
    '';
    
    # No dependencies
    buildInputs = [];
  };
}
```

## The Foundation

Layer 0 is **axiomatic** - it requires nothing, defines everything.

All programs are built on these ~1000 Genus 0 declarations.
