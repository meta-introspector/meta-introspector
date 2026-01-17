# Byte Reachability Tracing System

## Overview

A QEMU-based system for tracking byte-level data flow through the Rust compiler. For each output byte, it records:
- Which input bytes contributed to it
- Which CPU instructions touched it
- Complete provenance chain

## Components

### 1. QEMU Plugins

**`reachability_plugin.c`** - Core taint tracking engine
- Hooks every memory read/write
- Maintains taint map: memory address → input offset
- Propagates taints through computation
- Records instruction addresses
- Outputs provenance for each output byte

**`rustc_trace_plugin.c`** - Basic metrics
- Instruction count
- Memory read/write totals
- Simpler, faster alternative

### 2. Rust Tracers

**`reach_tracer.rs`** - Full reachability analysis
- Runs rustc under QEMU with reachability plugin
- Parses provenance data
- Generates JSON reports
- Query interface for specific bytes

**`qemu_rustc_tracer.rs`** - Basic QEMU tracing
- Instruction and memory metrics per stage
- Faster than full reachability

**`rustc_tracer.rs`** - Native tracing (no QEMU)
- Direct rustc invocation
- File hashes and sizes
- Timing only

## Build Instructions

```bash
# Build QEMU plugins
./build-reach-plugin.sh      # Reachability plugin
./build-qemu-plugin.sh        # Basic metrics plugin

# Build Rust tracers
cargo build --release --bin reach_tracer
cargo build --release --bin qemu_rustc_tracer
cargo build --release --bin rustc_tracer
```

## Usage Examples

### Full Reachability Trace
```bash
./target/release/reach_tracer
# Output: reach_output/reachability.json
```

### Query Specific Byte
```rust
// In reach_tracer.rs main()
tracer.report_byte("llvm_ir", 100);
```

### Basic Metrics
```bash
./target/release/qemu_rustc_tracer
# Output: qemu_trace_output/qemu_trace.json
```

### Native Trace (Fast)
```bash
./target/release/rustc_tracer
# Output: trace_output/trace_data.json
```

## Data Flow

```
Source File (sample.rs)
    ↓
[QEMU + Plugin]
    ↓
rustc --emit=mir
    ↓
Taint Tracking:
  - Input byte 10 → Memory 0x7fff1234
  - Instruction 0x400500 reads 0x7fff1234
  - Instruction 0x400510 writes 0x7ffe5678
  - Output byte 50 ← Memory 0x7ffe5678
    ↓
Provenance:
  Output[50] ← Input[10] via [0x400500, 0x400510]
```

## Output Format

### Reachability JSON
```json
{
  "stage": "llvm_ir",
  "input_hash": "a1b2c3d4",
  "output_hash": "e5f6g7h8",
  "input_size": 1024,
  "output_size": 4096,
  "byte_reaches": [
    {
      "output_offset": 100,
      "input_offsets": [10, 15, 20],
      "instruction_addrs": [0x7f1234, 0x7f1240]
    }
  ]
}
```

### Text Report
```
=== Byte Reachability Report ===

Stage: llvm_ir
  Input:  1024 bytes (hash: a1b2c3d4)
  Output: 4096 bytes (hash: e5f6g7h8)
  Tracked: 4096 output bytes
  Avg input bytes per output: 3.2
  Avg instructions per output: 12.5

Output byte 100:
  Input bytes: 10 15 20
  Instructions: 0x7f1234 0x7f1240 0x7f1250
```

## Performance

| Tracer | Speed | Detail | Use Case |
|--------|-------|--------|----------|
| rustc_tracer | 1x | Low | Quick overview |
| qemu_rustc_tracer | 10-50x | Medium | Instruction counts |
| reach_tracer | 50-200x | High | Full provenance |

## Use Cases

### 1. Incremental Compilation
Determine which output bytes need recompilation when source changes:
```rust
// If input bytes 10-20 changed
// Find all output bytes that depend on them
let affected = reachability.byte_reaches
    .iter()
    .filter(|r| r.input_offsets.iter().any(|&i| i >= 10 && i <= 20))
    .map(|r| r.output_offset)
    .collect();
```

### 2. Dead Code Detection
Find output bytes with no input provenance:
```rust
let dead_bytes = (0..output_size)
    .filter(|&i| !reachability.byte_reaches.iter().any(|r| r.output_offset == i))
    .collect();
```

### 3. Optimization Analysis
See which source constructs generate most output:
```rust
let hot_inputs: HashMap<usize, usize> = reachability.byte_reaches
    .iter()
    .flat_map(|r| &r.input_offsets)
    .fold(HashMap::new(), |mut map, &offset| {
        *map.entry(offset).or_insert(0) += 1;
        map
    });
```

### 4. Security Auditing
Track sensitive data flow:
```rust
// Mark input bytes 100-200 as sensitive
let sensitive_outputs = reachability.byte_reaches
    .iter()
    .filter(|r| r.input_offsets.iter().any(|&i| i >= 100 && i <= 200))
    .collect();
```

## Integration

### With Cargo
```rust
// build.rs
fn main() {
    if std::env::var("TRACE_COMPILATION").is_ok() {
        trace_with_qemu();
    }
}
```

### With CI/CD
```yaml
# .github/workflows/trace.yml
- name: Trace compilation
  run: |
    ./build-reach-plugin.sh
    cargo build --release --bin reach_tracer
    ./target/release/reach_tracer
    
- name: Upload traces
  uses: actions/upload-artifact@v3
  with:
    name: reachability-traces
    path: reach_output/
```

## Limitations

1. **Memory**: ~100 bytes overhead per tracked output byte
2. **Speed**: 50-200x slower than native compilation
3. **Scale**: Max 10MB output tracking (configurable)
4. **Accuracy**: Taint tracking is conservative (may over-approximate)

## Future Work

- [ ] Value-based taint tracking (not just byte offsets)
- [ ] Source line mapping via debug info
- [ ] Interactive visualization (web UI)
- [ ] Differential analysis (compare two compilations)
- [ ] Machine learning on reachability patterns
- [ ] Integration with rustc internals for exact tracking

## References

- QEMU Plugin API: https://qemu.readthedocs.io/en/latest/devel/tcg-plugins.html
- Taint Analysis: https://en.wikipedia.org/wiki/Taint_checking
- Dynamic Binary Instrumentation: https://en.wikipedia.org/wiki/Dynamic_binary_instrumentation
