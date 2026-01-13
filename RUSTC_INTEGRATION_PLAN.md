# Rustc Interceptor Integration Plan

## Current System Understanding

### Existing Rustc Interceptor (`rustc_interceptor.rs`)
**Purpose**: Hijacks cargo build process to capture actual compilation order and compress files

**How it works**:
1. **Environment Variable**: Set `RUSTC=./rustc_interceptor.rs` to hijack cargo build
2. **Real-time Interception**: Captures each rustc call with actual file arguments
3. **Build Order Capture**: Files processed in true dependency order as cargo compiles them
4. **Pattern Compression**: Uses token-based compression (97% savings proven)
5. **Metadata Passthrough**: Handles rustc queries (-vV, --version, --print) transparently

**Key Features**:
- **Actual build order**: Gets real cargo dependency sequence
- **File-by-file processing**: Each rustc call processes specific files
- **Compression logging**: Real-time compression statistics
- **No compilation**: Exits after compression (compression-only mode)

### Build Order Pipeline (`build_order_pipeline.rs`)
**Purpose**: In-memory analysis of compressed archives with dependency tracking

**Current limitation**: Uses pattern matching instead of real build order

## Integration Strategy

### Phase 1: Capture Real Build Order
Use rustc interceptor to capture actual build order from a real Rust project:

```bash
# In a real Rust project directory
export RUSTC=/path/to/rustc_interceptor.rs
cargo build --verbose
```

This will generate `rustc_intercept_compression.json` with:
- **Actual file order**: Files in true compilation sequence
- **Real dependencies**: As determined by cargo/rustc
- **Compression data**: Pattern-based compression results

### Phase 2: Apply Build Order to Archive Analysis
Modify build order pipeline to use captured sequence:

1. **Load build order** from `rustc_intercept_compression.json`
2. **Map to archive files** using path matching
3. **Process in captured order** instead of pattern matching
4. **Maintain dependency tracking** with real build sequence

### Phase 3: Combined Analysis
Create unified system that:
- **Captures build order** from real projects using rustc interceptor
- **Applies order** to compressed archive analysis
- **Provides dependency-aware analysis** with actual compilation sequence

## Implementation Plan

### 1. Enhanced Rustc Interceptor
```rust
// Add build order tracking to rustc_interceptor.rs
struct BuildOrderCapture {
    compilation_sequence: Vec<String>,
    file_timestamps: HashMap<String, u64>,
    dependency_graph: HashMap<String, Vec<String>>,
}
```

### 2. Build Order Integration
```rust
// Modify build_order_pipeline.rs to use captured order
fn load_captured_build_order(&mut self, capture_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let capture_data: serde_json::Value = serde_json::from_str(&fs::read_to_string(capture_file)?)?;
    self.build_order = capture_data["compilation_sequence"].as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
}
```

### 3. Archive Mapping
```rust
fn map_build_order_to_archives(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    // Map captured file paths to archive file paths
    // Handle path differences between build environment and archive structure
}
```

## Expected Results

### With Real Build Order
- **All 3,848 files** processed in actual dependency order
- **True compilation sequence** instead of pattern matching
- **Real dependency relationships** captured from cargo
- **Accurate incremental analysis** mirroring actual build process

### Performance Benefits
- **Zero build overhead**: Rustc interceptor adds minimal time
- **In-memory processing**: Archive analysis remains fast
- **Real-world accuracy**: Uses actual Rust project build orders

## Usage Workflow

### Step 1: Capture Build Order
```bash
cd /path/to/rust/project
export RUSTC=/path/to/rustc_interceptor.rs
cargo build --verbose
# Generates rustc_intercept_compression.json with real build order
```

### Step 2: Apply to Archive Analysis
```bash
cargo run --bin integrated_build_pipeline -- --build-order rustc_intercept_compression.json
# Processes 3,848 archive files in captured build order
```

### Step 3: Analyze Results
```bash
# Get dependency-aware analysis with real build order
cat integrated_build_analysis.json
```

This integration will provide the most accurate dependency-aware analysis possible by using the actual Rust compilation process to determine file processing order.
