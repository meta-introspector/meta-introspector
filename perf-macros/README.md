# Perf Proc Macros

Wrap any Rust code in perf recording with zero boilerplate.

## 🚀 Quick Start

```rust
use perf_macros::perf_auto;

#[perf_auto]
fn my_function(x: i32) -> i32 {
    x * 2
}

fn main() {
    let result = my_function(42);
    // Perf data automatically sent to data/telemetry/
}
```

## 📦 Crates

- **perf-macros** - Proc macros (`#[perf_auto]`, `perf!()`)
- **perf-runtime** - Runtime support (PerfSession, PerfData, telemetry)

## 🔧 Macros

### `#[perf_auto]` - Auto Telemetry

Wraps function in perf recording, sends data to telemetry automatically.

```rust
#[perf_auto]
fn expensive_computation(n: u64) -> u64 {
    (0..n).sum()
}
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

### `#[perf_record]` - Return Perf Data

Wraps function, modifies return type to include perf data.

```rust
#[perf_record]
fn my_function(x: i32) -> i32 {
    x * 2
}

// Returns: (i32, PerfData)
let (result, perf_data) = my_function(42);
```

## 📊 Perf Data

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

## 📁 Output

All perf data automatically saved to:
```
data/telemetry/perf_auto_<timestamp>.jsonl
```

## 🔗 Integration

### With Nix Build

```rust
use perf_macros::perf_auto;

#[perf_auto]
fn nix_build(package: &str) -> Result<String, String> {
    let output = std::process::Command::new("nix")
        .arg("build")
        .arg(format!(".#{}", package))
        .output()
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

### With Telemetry Server

Perf data automatically sent to `http://localhost:8080/telemetry/perf` if server is running.

## 🎯 Example

```bash
cd perf-macros/example
cargo run
```

Check `data/telemetry/` for output.

## 🔮 Future

- [ ] Actual perf integration (currently mock data)
- [ ] Telemetry server HTTP client
- [ ] Bott[8] layout solver integration
- [ ] Real-time perf visualization

---

**Status**: Minimal working version
**Next**: Integrate actual perf recording
