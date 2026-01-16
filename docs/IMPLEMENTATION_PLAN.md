# Implementation Plan: Parquet Lattice from Existing Work

## What We Already Have

### ✅ Data Collection
- **71 flakes perf data** - `data/71_flakes_perf/`
- **String usage parquet** - `string_usage.parquet` (1,921 rows)
- **Build telemetry** - `data/build_analysis/` (32 binaries, 92 libraries)
- **LMFDB mappings** - 8,174 symbols mapped
- **Strace captures** - Complete syscall traces

### ✅ Existing Tools (164 .rs files)
- `extract_all_strings.rs` - String extraction from binaries
- `extract_all_string_usage.rs` - String usage to parquet ✓
- `goblin_symbol_extractor.rs` - Symbol extraction
- `nix2parquet.rs` - Nix data to parquet
- `eigenvector_label_mapper.rs` - Eigenvector analysis
- `automorphic_orbit_lmfdb.rs` - Orbit analysis

### ✅ Documentation (98 .md files)
- Complete theory documented
- All concepts defined
- Architecture specified

## What We Need to Build

### Phase 1: Simple Parquet Splice (Week 1)

**Goal**: Parse all Rust source → parquet by AST type

**Tool**: `splice_to_parquet.rs`
```rust
// Input: Rust source files
// Output: data/by_ast_type/{enums,structs,functions}.parquet
```

**Steps**:
1. Use `syn` to parse source
2. Separate by `Item` type (Enum, Struct, Fn, etc.)
3. Write to parquet files by type
4. Add project column for filtering

**Deliverable**: Query all enums with SQL

### Phase 2: Instruction Parquet (Week 2)

**Goal**: Extract all instructions → parquet by type

**Tool**: `instructions_to_parquet.rs`
```rust
// Input: Binary files
// Output: data/by_instruction/{call,load,store,branch}.parquet
```

**Steps**:
1. Use `goblin` + `capstone` (existing code)
2. Disassemble all functions
3. Separate by instruction type
4. Write to parquet by mnemonic

**Deliverable**: Query all call instructions with SQL

### Phase 3: Build Lattice Layers (Week 3)

**Goal**: Create layered parquet structure

**Tool**: `build_lattice.rs`
```rust
// Input: Layer 0 parquet files
// Output: Layers 1-5 via queries
```

**Steps**:
1. Layer 0: Combine existing data (strings, instructions, syscalls)
2. Layer 1: Query to extract enums
3. Layer 2: Query to extract labelers
4. Layer 3: Query to build orbits
5. Layer 4: Query to trace transformations
6. Layer 5: Query to unify model

**Deliverable**: Complete queryable lattice

### Phase 4: Analysis Tools (Week 4)

**Goal**: Query and visualize the lattice

**Tools**:
- `query_lattice.rs` - SQL queries on any layer
- `visualize_lattice.rs` - Generate graphs
- `export_lattice.rs` - Export to JSON/GraphViz

**Deliverable**: Complete analysis pipeline

## Detailed Implementation

### Tool 1: `splice_to_parquet.rs`

```rust
use syn::{File, Item};
use arrow::array::*;
use arrow::datatypes::*;
use parquet::arrow::ArrowWriter;
use rayon::prelude::*;

struct EnumRecord {
    project: String,
    file: String,
    name: String,
    variants: Vec<String>,
    line: u32,
}

fn main() -> Result<()> {
    let project = env::args().nth(1).expect("project name");
    let source_dir = env::args().nth(2).expect("source dir");
    
    // Find all .rs files
    let files: Vec<_> = glob(&format!("{}/**/*.rs", source_dir))?
        .filter_map(Result::ok)
        .collect();
    
    // Process in parallel
    let enums: Vec<_> = files.par_iter()
        .flat_map(|file| extract_enums(file, &project))
        .collect();
    
    // Write to parquet
    write_parquet("data/by_ast_type/enums.parquet", &enums)?;
    
    println!("Extracted {} enums", enums.len());
    Ok(())
}

fn extract_enums(file: &Path, project: &str) -> Vec<EnumRecord> {
    let source = fs::read_to_string(file).ok()?;
    let ast = syn::parse_file(&source).ok()?;
    
    ast.items.into_iter()
        .filter_map(|item| {
            if let Item::Enum(e) = item {
                Some(EnumRecord {
                    project: project.to_string(),
                    file: file.display().to_string(),
                    name: e.ident.to_string(),
                    variants: e.variants.iter()
                        .map(|v| v.ident.to_string())
                        .collect(),
                    line: e.enum_token.span.start().line as u32,
                })
            } else {
                None
            }
        })
        .collect()
}
```

### Tool 2: `instructions_to_parquet.rs`

```rust
use goblin::elf::Elf;
use capstone::prelude::*;
use parquet::arrow::ArrowWriter;

struct InstructionRecord {
    project: String,
    binary: String,
    function: String,
    address: u64,
    mnemonic: String,
    operands: String,
}

fn main() -> Result<()> {
    let project = env::args().nth(1).expect("project");
    let binary_path = env::args().nth(2).expect("binary");
    
    let binary = fs::read(&binary_path)?;
    let elf = Elf::parse(&binary)?;
    
    // Disassemble
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()?;
    
    let instructions = disassemble_all(&elf, &cs, &project, &binary_path)?;
    
    // Write by type
    for (mnemonic, insns) in group_by_mnemonic(instructions) {
        let path = format!("data/by_instruction/{}.parquet", mnemonic);
        write_parquet(&path, &insns)?;
    }
    
    Ok(())
}
```

### Tool 3: `build_lattice.rs`

```rust
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();
    
    // Register Layer 0 tables
    ctx.register_parquet("strings", "data/by_ast_type/strings.parquet", Default::default()).await?;
    ctx.register_parquet("instructions", "data/by_instruction/all.parquet", Default::default()).await?;
    
    // Layer 1: Extract enums
    let enums = ctx.sql("
        SELECT DISTINCT string_value as enum_name
        FROM strings
        WHERE string_value LIKE '%Kind%'
    ").await?;
    
    write_parquet("data/lattice/layer1_enums.parquet", enums).await?;
    
    // Layer 2: Extract labelers
    let labelers = ctx.sql("
        SELECT enum_name, function_name as labeler
        FROM layer1_enums
        JOIN strings ON strings.string_value = layer1_enums.enum_name
        WHERE function_name LIKE '%Display%'
    ").await?;
    
    write_parquet("data/lattice/layer2_labelers.parquet", labelers).await?;
    
    // Continue for layers 3-5...
    
    Ok(())
}
```

## Timeline

### Week 1: Splice to Parquet
- Day 1-2: Implement `splice_to_parquet.rs`
- Day 3-4: Process rustc source
- Day 5: Test queries

### Week 2: Instructions
- Day 1-2: Implement `instructions_to_parquet.rs`
- Day 3-4: Process binaries
- Day 5: Test queries

### Week 3: Lattice
- Day 1-2: Implement `build_lattice.rs`
- Day 3-4: Build all layers
- Day 5: Verify structure

### Week 4: Analysis
- Day 1-2: Query tools
- Day 3-4: Visualization
- Day 5: Documentation

## Success Metrics

- [ ] All rustc enums in parquet (queryable)
- [ ] All instructions by type in parquet
- [ ] 5-layer lattice built
- [ ] SQL queries work on all layers
- [ ] Visualization generates graphs
- [ ] Documentation updated

## Next Steps

1. **Start with splice_to_parquet.rs** - Simplest, highest value
2. **Test on small project** - Verify approach
3. **Scale to rustc** - Full source base
4. **Add instructions** - Binary analysis
5. **Build lattice** - Layer by layer

## The Key Insight

**We already have most of the pieces. Just need to:**
1. Parse source → parquet (new)
2. Organize existing data into layers (new)
3. Add SQL query interface (new)

Everything else exists!
