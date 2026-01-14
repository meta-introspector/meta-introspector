# Perf Data Reading Code Audit

## 🎯 Goal
Centralize ALL perf.data reading code into one canonical reader that can:
1. Parse perf.data files
2. Extract symbols and samples
3. Output to JSON/Parquet
4. Feed to MiniZinc via Rust

## 📊 Current Perf Readers (Found 3 files)

### 1. `rust_perf_decoder.rs` ⭐ MAIN READER
**Purpose**: Decode perf.data using linux-perf-data crate
**Features**:
- Uses `PerfFileReader` from linux-perf-data
- Counts symbol samples
- Ranks by frequency
- Outputs JSON

**Key code**:
```rust
use linux_perf_data::{PerfFileReader, PerfFileRecord};

let PerfFileReader { mut perf_file, mut record_iter } =
    PerfFileReader::parse_file(reader)?;

while let Some(record) = record_iter.next_record(&mut perf_file)? {
    // Process record
}
```

### 2. `binary_symbol_study.rs`
**Purpose**: Extract binaries from perf.data, parse with goblin
**Features**:
- Extracts MMAP events
- Parses ELF binaries with goblin
- Builds symbol table
- Address resolution

**Key code**:
```rust
use linux_perf_data::{PerfFileReader, PerfFileRecord};
use goblin::elf::Elf;

// Extract binaries from MMAP events
let binaries = extract_binaries_from_perf(&perf_file_path)?;

// Parse each binary
for binary in &binaries {
    let symbols = parse_binary_symbols(&binary.path, binary.base_addr)?;
}
```

### 3. `perf_canonical_recorder.rs`
**Purpose**: Record perf data (not read)
**Note**: Creates perf.data files, doesn't read them

## 🔧 Solution: Canonical Perf Reader

### Create `perf_canonical_reader.rs`

```rust
// perf_canonical_reader.rs
// THE ONLY PLACE TO READ PERF.DATA FILES
// Centralizes all perf data parsing and analysis

use linux_perf_data::{PerfFileReader, PerfFileRecord};
use goblin::elf::Elf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfAnalysis {
    pub source_file: String,
    pub total_samples: u64,
    pub unique_symbols: usize,
    pub events: Vec<String>,
    pub ranked_symbols: Vec<SymbolRank>,
    pub binaries: Vec<BinaryInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolRank {
    pub symbol: String,
    pub samples: u64,
    pub percentage: f64,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,    // > 100 samples
    Medium,  // > 10 samples
    Low,     // <= 10 samples
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryInfo {
    pub path: String,
    pub base_addr: u64,
    pub symbols: Vec<String>,
}

pub struct PerfCanonicalReader;

impl PerfCanonicalReader {
    /// THE ONLY FUNCTION TO READ PERF.DATA FILES
    pub fn analyze(perf_file_path: &str) -> Result<PerfAnalysis, String> {
        // 1. Parse perf.data
        let file = File::open(perf_file_path)
            .map_err(|e| format!("Failed to open: {}", e))?;
        let reader = BufReader::new(file);
        
        let PerfFileReader { mut perf_file, mut record_iter } =
            PerfFileReader::parse_file(reader)
                .map_err(|e| format!("Failed to parse: {}", e))?;
        
        // 2. Extract events
        let events = Self::extract_events(&perf_file);
        
        // 3. Count symbols
        let (total_samples, symbol_counts) = Self::count_symbols(&mut perf_file, &mut record_iter)?;
        
        // 4. Extract binaries
        let binaries = Self::extract_binaries(&perf_file_path)?;
        
        // 5. Rank symbols
        let ranked_symbols = Self::rank_symbols(symbol_counts, total_samples);
        
        Ok(PerfAnalysis {
            source_file: perf_file_path.to_string(),
            total_samples,
            unique_symbols: ranked_symbols.len(),
            events,
            ranked_symbols,
            binaries,
        })
    }
    
    /// Export to JSON for MiniZinc
    pub fn to_minizinc_json(analysis: &PerfAnalysis) -> String {
        serde_json::to_string_pretty(analysis).unwrap()
    }
    
    /// Export to Parquet
    pub fn to_parquet(analysis: &PerfAnalysis, output_path: &str) -> Result<(), String> {
        // TODO: Write to parquet using arrow
        Ok(())
    }
}
```

## 🔄 Migration Plan

### Phase 1: Create Canonical Reader
- [x] Design `perf_canonical_reader.rs`
- [ ] Implement core parsing
- [ ] Add symbol ranking
- [ ] Add binary extraction
- [ ] Add JSON export
- [ ] Add Parquet export

### Phase 2: Migrate Existing Readers
1. **`rust_perf_decoder.rs`** → Use canonical reader
2. **`binary_symbol_study.rs`** → Use canonical reader

### Phase 3: Integration
- [ ] Connect to `nix_canonical_builder` (capture perf during build)
- [ ] Feed to MiniZinc via Rust
- [ ] Visualize in Bott[8] layout solver

## 🎯 71 Experiments Integration

### Workflow

```rust
// 1. Run 71 nix builds with perf
for i in 1..=71 {
    let result = nix_build_flake(&format!(".#experiment{}", i))?;
    // Perf data automatically captured to data/perf_canonical/
}

// 2. Analyze all 71 perf.data files
let mut all_analyses = Vec::new();
for i in 1..=71 {
    let perf_file = format!("data/perf_canonical/perf_nix_{}_*.perf.data", i);
    let analysis = PerfCanonicalReader::analyze(&perf_file)?;
    all_analyses.push(analysis);
}

// 3. Export to MiniZinc format
let minizinc_data = generate_minizinc_data(&all_analyses);
fs::write("bott8-layout-solver/perf_71_experiments.dzn", minizinc_data)?;

// 4. Solve layout
Command::new("minizinc")
    .args(&["bott8_optimal_layout.mzn", "perf_71_experiments.dzn"])
    .output()?;
```

## 📊 MiniZinc Data Format

```dzn
% perf_71_experiments.dzn
% Generated from 71 perf.data files

num_nodes = 71;

% Node names (experiment IDs)
node_names = [
    "experiment1", "experiment2", ..., "experiment71"
];

% Perf metrics mapped to 8D coordinates
% Real dimension: CPU cycles
node_cycles = [1234567, 2345678, ..., 7654321];

% Complex dimension: Instructions
node_instructions = [9876543, 8765432, ..., 1234567];

% Quaternion dimension: Cache miss rate
node_cache_miss_rate = [0.05, 0.03, ..., 0.08];

% Octonion dimension: Branch miss rate
node_branch_miss_rate = [0.02, 0.01, ..., 0.04];

% Time dimension: Duration
node_duration = [12.5, 15.3, ..., 8.7];

% Information dimension: IPC
node_ipc = [1.5, 2.1, ..., 1.8];

% Social dimension: Context switches
node_context_switches = [100, 150, ..., 80];

% Semantic dimension: Unique symbols
node_unique_symbols = [500, 600, ..., 450];
```

## ✅ Benefits

1. **Single reader** - One place to parse perf.data
2. **Consistent format** - Same JSON/Parquet output
3. **MiniZinc ready** - Direct export to .dzn format
4. **71 experiments** - Batch analysis support
5. **Bott[8] integration** - Feed to layout solver

## 🚀 Next Steps

1. Create `perf_canonical_reader.rs`
2. Migrate `rust_perf_decoder.rs`
3. Migrate `binary_symbol_study.rs`
4. Add MiniZinc export
5. Run 71 experiments
6. Visualize in Bott[8]

---

**Status**: Design complete, ready to implement
**Next**: Create `perf_canonical_reader.rs`
