# QEMU Reachability Plugin with Parquet Output

Rust QEMU plugin that maps input bytes → instructions → output bytes and stores results in Parquet format.

## Features

- **Byte-level taint tracking**: Tracks which input bytes reach which output bytes
- **Instruction decoding**: Uses goblin to decode x86_64 instructions
- **Parquet output**: Stores reachability data in columnar format
- **Text report**: Human-readable summary

## Output Schema

### Parquet Columns
```
input_offset: UInt64    - Offset of input byte
insn_addr: UInt64       - Address of instruction
output_offset: UInt64   - Offset of output byte
```

### Text Report
```
Output byte 0:
  Input bytes: 10 15 20
  Instructions: 0x400500 0x400510
```

## Building

```bash
cd qemu-plugin
cargo build --release
```

Output: `target/release/libqemu_reachability_plugin.so`

## Usage

```bash
qemu-x86_64 \
  -plugin ./target/release/libqemu_reachability_plugin.so,\
output=reach.txt,\
parquet=reach.parquet \
  /usr/bin/rustc sample.rs
```

## Reading Parquet Output

### Python
```python
import pyarrow.parquet as pq

table = pq.read_table('reach.parquet')
df = table.to_pandas()

# Find what reaches output byte 100
df[df['output_offset'] == 100]

# Find all outputs from input byte 50
df[df['input_offset'] == 50]

# Count instructions per output
df.groupby('output_offset')['insn_addr'].count()
```

### Rust
```rust
use parquet::file::reader::{FileReader, SerializedFileReader};

let file = File::open("reach.parquet")?;
let reader = SerializedFileReader::new(file)?;

for row in reader.get_row_iter(None)? {
    println!("{:?}", row);
}
```

## Analysis Examples

### Input Coverage
```python
# Which input bytes are used?
used_inputs = df['input_offset'].unique()
print(f"Used {len(used_inputs)} input bytes")
```

### Output Provenance
```python
# How many inputs contribute to each output?
provenance = df.groupby('output_offset')['input_offset'].nunique()
print(f"Average inputs per output: {provenance.mean()}")
```

### Instruction Hotspots
```python
# Which instructions are most active?
hot_insns = df['insn_addr'].value_counts().head(10)
print("Top 10 instructions:")
print(hot_insns)
```

## Integration

### With Complexity Analysis
```bash
# 1. Trace with QEMU
qemu-x86_64 -plugin ./libqemu_reachability_plugin.so,parquet=reach.parquet rustc input.rs

# 2. Analyze parquet
python analyze_reach.py reach.parquet > clusters.json

# 3. Classify complexity
homotopy_classifier < clusters.json
```

### With CI/CD
```yaml
- name: Trace compilation
  run: |
    qemu-x86_64 -plugin ./libqemu_reachability_plugin.so,parquet=reach.parquet rustc src/lib.rs
    
- name: Upload trace
  uses: actions/upload-artifact@v3
  with:
    name: reachability-trace
    path: reach.parquet
```

## Performance

- **Overhead**: ~10-100x slower than native (due to taint tracking)
- **Memory**: ~100 bytes per tracked output byte
- **Parquet size**: ~24 bytes per record (3 × UInt64)

## Limitations

- Max 10MB output tracking (configurable)
- x86_64 only (for instruction decoding)
- Requires QEMU user-mode with plugin support

## Future Enhancements

- [ ] Full instruction disassembly with goblin
- [ ] Value tracking (not just byte offsets)
- [ ] Compression for large traces
- [ ] Streaming parquet writes
- [ ] Multi-architecture support
