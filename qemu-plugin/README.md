# QEMU Reachability Plugin (Rust)

Pure Rust implementation of the byte reachability QEMU plugin.

## Advantages over C version

- **Memory safety**: No manual memory management
- **Better data structures**: HashMap instead of GHashTable
- **Easier to extend**: Rust's type system and traits
- **No external dependencies**: Only libc (no glib)

## Building

```bash
cd qemu-plugin
cargo build --release
```

Or use the convenience script:
```bash
./build-rust-plugin.sh
```

## API

The plugin exports two C functions required by QEMU:

```rust
#[no_mangle]
pub extern "C" fn qemu_plugin_version() -> c_int

#[no_mangle]
pub extern "C" fn qemu_plugin_install(
    id: QemuPluginId,
    info: *const QemuInfo,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int
```

## Internal Architecture

```rust
struct PluginState {
    output_file: File,
    byte_provenance: HashMap<usize, ByteProvenance>,  // output_offset -> provenance
    taint_map: HashMap<u64, usize>,                   // memory_addr -> input_offset
    current_insn: u64,
    input_base: u64,
    input_size: u64,
    output_base: u64,
    output_size: u64,
}

struct ByteProvenance {
    input_offsets: Vec<usize>,    // Which input bytes contributed
    insn_addrs: Vec<u64>,         // Which instructions touched this
}
```

## Callbacks

1. **`vcpu_tb_trans`**: Called for each translated block
   - Registers per-instruction callbacks

2. **`vcpu_insn_exec`**: Called before each instruction
   - Updates current instruction address

3. **`vcpu_mem_access`**: Called for each memory access
   - Tracks input reads (adds taint)
   - Tracks output writes (records provenance)

4. **`plugin_exit`**: Called on shutdown
   - Writes final report

## Usage

```bash
# Basic usage
qemu-x86_64 -plugin ./libreachability_rust.so,output=reach.txt \
  /usr/bin/rustc sample.rs

# With memory regions
qemu-x86_64 -plugin ./libreachability_rust.so,\
  input_base=0x7fff0000,\
  input_size=4096,\
  output_base=0x7ffe0000,\
  output_size=8192,\
  output=reach.txt \
  /usr/bin/rustc sample.rs
```

## Output Format

```
=== Byte Reachability Tracer (Rust) ===
Input: 0x7fff0000 - 0x7fff1000 (4096 bytes)
Output: 0x7ffe0000 - 0x7ffe2000 (8192 bytes)

Output byte 0:
  Input bytes: 10 15 20 
  Instructions: 0x400500 0x400510 0x400520 

Output byte 1:
  Input bytes: 11 16 21 
  Instructions: 0x400500 0x400511 0x400521 
...
```

## Performance

Similar to C version (~50-200x slower than native), but:
- Safer (no segfaults from memory errors)
- Easier to debug (Rust error messages)
- More maintainable (type system catches bugs)

## Extending

Add new tracking features:

```rust
// Track value changes
struct ByteProvenance {
    input_offsets: Vec<usize>,
    insn_addrs: Vec<u64>,
    values: Vec<u8>,  // Add this
}

// In vcpu_mem_access
if is_write {
    let value = read_memory_byte(vaddr);  // Need to implement
    prov.values.push(value);
}
```

## Testing

```bash
# Create test program
echo 'fn main() { println!("test"); }' > test.rs

# Run with plugin
qemu-x86_64 -plugin ./libreachability_rust.so,output=test.txt \
  /usr/bin/rustc test.rs

# Check output
cat test.txt
```

## Debugging

Enable Rust backtraces:
```bash
RUST_BACKTRACE=1 qemu-x86_64 -plugin ./libreachability_rust.so ...
```

Add debug prints:
```rust
eprintln!("Tracking output byte {} from input {}", output_offset, input_offset);
```

## Limitations

- Max 10MB output tracking (hardcoded)
- Max 32 input offsets per output byte
- Max 64 instructions per output byte
- No value tracking (only byte offsets)

All limits can be increased by editing the source.
