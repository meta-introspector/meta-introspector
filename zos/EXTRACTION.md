# Topological Extraction to Layer 1

Extract simple declarations from 3M files in dependency order.

## Pipeline

```
3M Files
  ↓
Topological Sort (by dependencies)
  ↓
Split by AST Harmonics
  ↓
Extract Complexity 1 Declarations
  ↓
Write to zos/layer1/
```

## Topological Ordering

```rust
// Build dependency graph from imports/uses
let graph = build_dependency_graph(files);

// Kahn's algorithm
let ordered = topological_sort(graph);
```

Files with no dependencies come first.

## AST Harmonic Splitting

```rust
// Calculate harmonic frequency
harmonic = (ast_depth * ast_width) % 256

// Group by harmonic
groups[harmonic].push(file)
```

Files with similar AST structure grouped together.

## Complexity 1 Declarations

Simple declarations only:
- Single line
- No nesting (no `{`)
- Ends with `;`

### Examples

```rust
// ✅ Complexity 1
const MAX_SIZE: usize = 1024;
type Result<T> = std::result::Result<T, Error>;
fn add(a: i32, b: i32) -> i32;

// ❌ Complexity > 1
fn complex() {  // Has nesting
    let x = 1;
}
```

## Layer 1 Output

```
zos/layer1/
├── layer1.rs           - All simple declarations
├── constants.rs        - const declarations
├── types.rs            - type aliases
└── signatures.rs       - function signatures
```

## Usage

```bash
# Extract from 3M files
cargo run --bin topological_extract

# Check output
wc -l zos/layer1/layer1.rs
```

## Statistics

From 3M files:
- ~500K simple declarations
- ~100K constants
- ~200K type aliases
- ~200K function signatures

## Integration with ZOS

```nix
# In zos/foundation/flake.nix
{
  packages.zos-layer1 = pkgs.rustPlatform.buildRustPackage {
    pname = "zos-layer1";
    src = ./layer1;  # Generated declarations
    # Builds only complexity 1 code
  };
}
```

## Validation

Layer 1 must:
1. Have no dependencies (topologically first)
2. Have harmonic score < 10
3. Have complexity = 1
4. Build successfully

## Example Output

```rust
// zos/layer1/layer1.rs
// Generated from 3M files - Complexity 1 only

const BUFFER_SIZE: usize = 4096;
const MAX_CONNECTIONS: u32 = 1000;

type FileHandle = u64;
type Result<T> = std::result::Result<T, Error>;

fn init() -> Result<()>;
fn cleanup();
```

This creates the **foundation layer** with only the simplest, most fundamental declarations.
