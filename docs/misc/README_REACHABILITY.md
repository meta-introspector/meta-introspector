# Byte Reachability Tracer for Rustc

Tracks which input bytes and instructions contribute to each output byte during Rust compilation.

## Architecture

### QEMU Plugin (`reachability_plugin.c`)
- Instruments every memory access and instruction
- Maintains taint tracking: input bytes → memory → output bytes
- Records instruction addresses that touch each output byte
- Outputs detailed provenance for each output byte

### Rust Wrapper (`reach_tracer.rs`)
- Runs rustc under QEMU with the reachability plugin
- Parses provenance data
- Generates JSON reports
- Provides query interface

## Building

```bash
# Build the QEMU plugin
./build-reach-plugin.sh

# Build the Rust tracer
cargo build --release --bin reach_tracer
```

## Usage

```bash
# Trace a Rust file
./target/release/reach_tracer

# Query specific byte
# Edit reach_tracer.rs main() to call:
tracer.report_byte("llvm_ir", 100);
```

## Output Format

### JSON Structure
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
      "instruction_addrs": [0x7f1234, 0x7f1240, ...]
    }
  ]
}
```

### Text Report
```
Output byte 100:
  Input bytes: 10 15 20 25 30
  Instructions: 0x7f1234 0x7f1240 0x7f1250 ...
```

## Use Cases

1. **Compiler Optimization Analysis**
   - See which source bytes affect which output bytes
   - Identify dead code (output bytes with no input provenance)

2. **Incremental Compilation**
   - Determine minimal recompilation needed for source changes
   - Build dependency graphs at byte level

3. **Security Analysis**
   - Track information flow through compilation
   - Identify potential side channels

4. **Debugging**
   - Understand why specific output was generated
   - Trace back from binary to source

## Limitations

- Memory overhead: ~100 bytes per tracked output byte
- Max 10MB output tracking (configurable in plugin)
- QEMU slowdown: ~10-100x slower than native
- Requires QEMU user-mode with plugin support

## Advanced Usage

### Custom Memory Regions
```bash
qemu-x86_64 -plugin ./libreachability.so,\
  input_base=0x7fff0000,\
  input_size=4096,\
  output_base=0x7ffe0000,\
  output_size=8192,\
  output=custom.txt \
  /usr/bin/rustc sample.rs
```

### Integration with Build Systems
```rust
// In build.rs
use std::process::Command;

fn trace_compilation() {
    Command::new("qemu-x86_64")
        .args(["-plugin", "./libreachability.so"])
        .arg("/usr/bin/rustc")
        .args(["--emit=llvm-ir", "src/lib.rs"])
        .status()
        .expect("Failed to trace");
}
```

## Performance Tips

1. **Limit tracking**: Only track specific memory regions
2. **Sample mode**: Track every Nth byte instead of all
3. **Parallel tracing**: Run multiple QEMU instances for different stages
4. **Post-processing**: Parse trace files offline

## Future Enhancements

- [ ] Dynamic taint analysis with value tracking
- [ ] Instruction disassembly in reports
- [ ] Source line mapping via DWARF
- [ ] Interactive visualization
- [ ] Differential reachability (compare two compilations)
- [ ] Machine learning on reachability patterns
