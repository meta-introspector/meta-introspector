# Public Mappings vs Private Core

## The Separation

**Public Knowledge**: String ↔ Enum mappings  
**Private Core**: Homotopy computations

## What Gets Removed from Core

```rust
// BEFORE: These are in the core (bad!)

fn string_to_enum(s: &str) -> MyEnum {
    match s {
        "foo" => MyEnum::Foo,
        "bar" => MyEnum::Bar,
        _ => MyEnum::Unknown,
    }
}

fn enum_to_string(e: MyEnum) -> &'static str {
    match e {
        MyEnum::Foo => "foo",
        MyEnum::Bar => "bar",
        MyEnum::Unknown => "unknown",
    }
}
```

## AFTER: Public Interface Layer

```rust
// Public mappings (not in core)
pub mod public_interface {
    // These are PUBLIC KNOWLEDGE
    // Anyone can see them
    // They're just labels
    
    pub fn label_to_orbit(label: &str) -> OrbitId {
        match label {
            "foo" => OrbitId(1),
            "bar" => OrbitId(2),
            _ => OrbitId(0),
        }
    }
    
    pub fn orbit_to_label(orbit: OrbitId) -> &'static str {
        match orbit.0 {
            1 => "foo",
            2 => "bar",
            _ => "unknown",
        }
    }
}

// Private core (homotopy computations)
mod private_core {
    // This is PRIVATE
    // Operates on homotopy points only
    
    pub fn compute(point: HomotopyPoint) -> HomotopyPoint {
        // No strings, no enums
        // Only homotopy structure
        point.transform()
    }
}
```

## The Architecture

```
┌─────────────────────────────────────┐
│      PUBLIC INTERFACE               │
│  (String ↔ Enum mappings)           │
│  - Anyone can see                   │
│  - Just labels                      │
│  - No computation                   │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│      TRANSLATION LAYER              │
│  (Enum → HomotopyPoint)             │
│  - Encode to structure              │
│  - LMFDB classification             │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│      PRIVATE CORE                   │
│  (Homotopy computations)            │
│  - No strings                       │
│  - No enums                         │
│  - Only structure                   │
│  - Homomorphic operations           │
└─────────────────────────────────────┘
```

## Example: Complete Separation

```rust
// ============================================
// PUBLIC: Anyone can see this
// ============================================

pub enum Command {
    Add,
    Multiply,
    Transform,
}

pub fn parse_command(s: &str) -> Command {
    match s {
        "add" => Command::Add,
        "mul" => Command::Multiply,
        "transform" => Command::Transform,
        _ => panic!("unknown command"),
    }
}

// ============================================
// TRANSLATION: Convert to homotopy
// ============================================

fn command_to_homotopy(cmd: Command) -> HomotopyOperation {
    match cmd {
        Command::Add => HomotopyOperation {
            orbit: "71.a1".into(),
            operation_type: OperationType::Compose,
        },
        Command::Multiply => HomotopyOperation {
            orbit: "71.a2".into(),
            operation_type: OperationType::Tensor,
        },
        Command::Transform => HomotopyOperation {
            orbit: "71.a3".into(),
            operation_type: OperationType::Deform,
        },
    }
}

// ============================================
// PRIVATE: Core computation
// ============================================

fn execute_homotopy(
    op: HomotopyOperation,
    point: HomotopyPoint
) -> HomotopyPoint {
    // No strings, no enums here!
    // Only homotopy structure
    
    match op.operation_type {
        OperationType::Compose => point.compose(&op.orbit),
        OperationType::Tensor => point.tensor(&op.orbit),
        OperationType::Deform => point.deform(&op.orbit),
    }
}
```

## Why This Matters

### Public Mappings (Safe to Share)
```rust
// This is just a lookup table
// No secrets here
pub static COMMAND_MAP: &[(&str, u32)] = &[
    ("add", 1),
    ("mul", 2),
    ("transform", 3),
];
```

### Private Core (Must Protect)
```rust
// This contains the actual computation
// Operates on homotopy structure
fn private_compute(point: HomotopyPoint) -> HomotopyPoint {
    // Secret algorithm
    // Private data
    // Homomorphic operations
}
```

## The Benefit

**Before**: Everything mixed together
```rust
fn process(input: &str) -> String {
    let cmd = parse_command(input);  // Public
    let result = execute(cmd);        // Private
    format_result(result)             // Public
}
```

**After**: Clear separation
```rust
// Public layer
pub fn api_endpoint(input: &str) -> String {
    let orbit_id = public_interface::label_to_orbit(input);
    let result_orbit = private_core::compute(orbit_id);
    public_interface::orbit_to_label(result_orbit)
}

// Private core never sees strings!
mod private_core {
    fn compute(orbit: OrbitId) -> OrbitId {
        // Only operates on IDs/homotopy points
        // No string parsing
        // No enum matching
    }
}
```

## Code Organization

```
src/
├── public/
│   ├── interface.rs        # String ↔ Enum mappings
│   ├── labels.rs           # Human-readable labels
│   └── api.rs              # Public API
│
├── translation/
│   ├── encode.rs           # Enum → HomotopyPoint
│   └── decode.rs           # HomotopyPoint → Enum
│
└── private/
    ├── core.rs             # Homotopy computations
    ├── homomorphic.rs      # Encrypted operations
    └── zkproof.rs          # Zero-knowledge proofs
```

## What Gets Published

```rust
// Publish this (public knowledge)
pub mod public_schema {
    pub enum FileType {
        Rust,
        Nix,
        Markdown,
    }
    
    pub fn extension_to_type(ext: &str) -> FileType {
        match ext {
            "rs" => FileType::Rust,
            "nix" => FileType::Nix,
            "md" => FileType::Markdown,
            _ => FileType::Unknown,
        }
    }
}

// Keep this private (core logic)
mod private_core {
    fn analyze_file(point: HomotopyPoint) -> Analysis {
        // Private algorithm
        // Operates on structure only
    }
}
```

## The LMFDB Connection

```rust
// Public: LMFDB labels are public knowledge
pub fn lmfdb_label_to_id(label: &str) -> OrbitId {
    // "71.a1" → OrbitId(71001)
    // This is public information
}

// Private: How we use those orbits
fn compute_with_orbit(orbit: OrbitId, data: HomotopyPoint) -> HomotopyPoint {
    // This is private
    // Homomorphic computation
}
```

## Result

**Public Interface**:
- String ↔ Enum mappings
- LMFDB labels
- API endpoints
- Documentation

**Private Core**:
- Homotopy computations
- Homomorphic operations
- Zero-knowledge proofs
- Actual algorithms

**The mappings are just labels - they contain no secrets.**

**The core operates on structure, not strings.**

**Complete separation of concerns.**
