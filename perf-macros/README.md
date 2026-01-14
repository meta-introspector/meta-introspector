# Perf Proc Macros

Wrap any Rust code in perf recording with zero boilerplate. Capture any value to Parquet.

## 🚀 Quick Start

```rust
use perf_macros::{perf_auto, perf_probe, probe};

#[perf_auto]
fn my_function(x: i32) -> i32 {
    x * 2
}

#[perf_probe]
fn process_data(x: i32, y: String) -> Vec<u8> {
    // All inputs and output captured to parquet
    vec![x as u8]
}

fn main() {
    let result = my_function(42);
    // Perf data automatically sent to data/telemetry/
    
    let data = process_data(42, "hello".to_string());
    // Inputs and output captured to data/probes/*.parquet
    
    probe!(result);
    // Single value captured to parquet
}
```

## 📦 Crates

- **perf-macros** - Proc macros (`#[perf_auto]`, `#[perf_probe]`, `perf!()`, `probe!()`)
- **perf-runtime** - Runtime support (PerfSession, ProbeSession, telemetry, parquet)

## 🔧 Macros

### `#[perf_auto]` - Auto Telemetry

Wraps function in perf recording, sends data to telemetry automatically.

```rust
#[perf_auto]
fn expensive_computation(n: u64) -> u64 {
    (0..n).sum()
}
```

### `#[perf_probe]` - Parquet Capture

Captures all inputs and output to Parquet file.

```rust
#[perf_probe]
fn process_data(x: i32, y: String) -> Vec<u8> {
    vec![x as u8; y.len()]
}
```

**Parquet Schema**:
```
name: string         # Parameter/output name
type_name: string    # Rust type name
value: string        # Debug representation
timestamp: int64     # Unix timestamp
is_output: int64     # 1 if output, 0 if input
```

### `perf!()` - Inline Recording

Wraps code block, returns tuple with result and perf data.

```rust
let (result, perf_data) = perf!({
    expensive_computation()
});

println!("Duration: {:.6}s", perf_data.duration_secs);
println!("IPC: {:.2}", perf_data.ipc());
```

### `probe!()` - Inline Parquet Capture

Captures a single value to Parquet.

```rust
let x = 42;
let y = vec![1, 2, 3];
probe!(x);
probe!(y);
```

## 📊 Data Structures

### PerfData

```rust
pub struct PerfData {
    pub name: String,
    pub timestamp: u64,
    pub duration_secs: f64,
    pub cycles: u64,
    pub instructions: u64,
    pub cache_references: u64,
    pub cache_misses: u64,
    pub branches: u64,
    pub branch_misses: u64,
}

// Helper methods
perf_data.ipc()              // Instructions per cycle
perf_data.cache_miss_rate()  // Cache miss rate
perf_data.branch_miss_rate() // Branch miss rate
```

### ProbeValue

```rust
pub struct ProbeValue {
    pub name: String,
    pub type_name: String,
    pub value_str: String,
    pub timestamp: u64,
}
```

## 📁 Output

### Telemetry (JSON)
```
data/telemetry/perf_auto_<timestamp>.jsonl
```

### Probes (Parquet)
```
data/probes/probe_<function_name>_<timestamp>.parquet
```

## 🔍 Querying Parquet Data

### Using DuckDB

```bash
# Query all probes
duckdb -c "SELECT * FROM 'data/probes/*.parquet'"

# Filter by function
duckdb -c "SELECT * FROM 'data/probes/*.parquet' WHERE name LIKE 'process_data%'"

# Get outputs only
duckdb -c "SELECT * FROM 'data/probes/*.parquet' WHERE is_output = 1"
```

### Using Python

```python
import duckdb

# Query probes
df = duckdb.query("SELECT * FROM 'data/probes/*.parquet'").df()
print(df)
```

## 🔗 Integration

### With Nix Build

```rust
use perf_macros::{perf_auto, perf_probe};

#[perf_auto]
#[perf_probe]
fn nix_build(package: &str) -> Result<String, String> {
    let output = std::process::Command::new("nix")
        .arg("build")
        .arg(format!(".#{}", package))
        .output()
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

### With Bott[8] Layout Solver

```rust
#[perf_probe]
fn compute_8d_layout(nodes: Vec<Node>) -> Layout8D {
    // Inputs and output captured to parquet
    // Feed to Bott[8] solver
    solve_layout(nodes)
}
```

## 🎯 Examples

### Run Examples

```bash
# Basic example
cd perf-macros/example
cargo run

# Probe example
cargo run --bin probe_example
```

### Check Output

```bash
# Telemetry
cat data/telemetry/perf_auto_*.jsonl | jq .

# Probes
duckdb -c "SELECT * FROM 'data/probes/*.parquet'"
```

## 🔮 Use Cases

1. **Performance Profiling** - Track function performance over time
2. **Data Lineage** - Capture inputs/outputs for reproducibility
3. **Debugging** - Record all values for post-mortem analysis
4. **Telemetry** - Centralized performance monitoring
5. **ML Training** - Capture training data automatically
6. **Build Analysis** - Track nix build inputs/outputs

## ✅ Benefits

1. **Zero boilerplate** - Just add `#[perf_probe]`
2. **Type-safe** - Compile-time code generation
3. **Parquet format** - Efficient columnar storage
4. **Queryable** - Use DuckDB, Python, or any Parquet tool
5. **Automatic** - No manual instrumentation
6. **Composable** - Combine with `#[perf_auto]`

## 🔮 Future

- [ ] Actual perf integration (currently mock data)
- [ ] Telemetry server HTTP client
- [ ] Bott[8] layout solver integration
- [ ] Real-time visualization
- [ ] Parquet compression options
- [ ] Custom schema support

---

**Status**: Minimal working version with Parquet capture
**Next**: Integrate actual perf recording + test with nix build

