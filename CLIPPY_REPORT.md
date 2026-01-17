# Clippy Demo2Code Report

## Configuration Fixed

Updated `clippy.toml` with valid standard clippy options:
- `disallowed-names`: Bans demo, mock, fake, stub, placeholder, test_data
- `cognitive-complexity-threshold`: 15
- `too-many-arguments-threshold`: 5

Custom lints commented out (require plugin).

## Current Violations Found

### High Priority (unwrap_used)
```
build.rs:6:19: warning: used `unwrap()` on a `Result` value
build.rs:9:5: warning: used `unwrap()` on a `Result` value
libnix.rs:23:22: warning: used `unwrap()` on a `Result` value
libnix.rs:28:15: warning: used `unwrap()` on an `Option` value
```

**Action**: Replace with `?` operator or proper error handling

### Medium Priority (unused code)
```
telemetry_lib/src/lib.rs:3:9: warning: unused import: `std::time::Instant`
telemetry_lib/src/lib.rs:78:8: warning: function `record_call` is never used
libnix.rs:4:27: warning: unused import: `Symbol`
perf_runtime.rs:6:4: warning: function `main` is never used
```

**Action**: Remove unused code or mark with `#[allow(dead_code)]` if intentional

### Low Priority (manifest)
```
warning: file `/mnt/data1/meta-introspector/demo_proof_table.rs` found to be present in multiple build targets
warning: unused manifest key: bin.225.*
```

**Action**: Clean up Cargo.toml

## Recommended Command

```bash
# Check all violations
cargo clippy --all-targets -- -W clippy::unwrap_used -D clippy::todo

# Auto-fix what's possible
cargo clippy --fix --allow-dirty --allow-staged

# Check specific file
cargo clippy --bin libnix -- -W clippy::unwrap_used
```

## Next Steps

1. Fix unwrap() calls (high priority)
2. Remove unused code
3. Clean Cargo.toml
4. Run `cargo fix` for auto-fixable issues
5. Enable in CI with `-D warnings` to block merges
