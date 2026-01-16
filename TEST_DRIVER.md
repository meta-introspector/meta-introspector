# Test Driver

Direct build testing tool with telemetry support.

## Features

- **Direct testing** - Bypasses server, tests build functions directly
- **Error parsing** - Extracts errors with file/line context
- **Fix suggestions** - Auto-generates actionable fixes
- **Nix support** - Build with `nix develop`
- **Perf recording** - Capture performance data during builds
- **Error storage** - Collects errors by type and binary

## Usage

```bash
# Basic build test
./target/debug/test_driver <binary_name>

# Build with nix develop
./target/debug/test_driver <binary_name> --nix

# Record perf data
./target/debug/test_driver <binary_name> --perf

# Both nix and perf
./target/debug/test_driver <binary_name> --nix --perf
```

## Example Output

```
🔨 Testing build: personal_data_sovereignty
📦 Using nix develop
📊 Recording perf data

⏱️  Build took: 2.34s

📊 Found 5 errors

❌ EE0599 in personal_data_sovereignty.rs:78
   no method named `clone` found for struct `PersonalProfile`

       77 |         
   →   78 |         self.profiles.insert(user_id.to_string(), profile.clone());
       79 |         profile
       80 |     }

   💡 Add #[derive(Clone)] to the struct/enum definition

📊 Perf data saved to: /tmp/build_personal_data_sovereignty_12345.perf

📈 Top functions:
   45.2%  rustc
   12.3%  LLVM
    8.1%  cargo
```

## Scripts

### test_build.sh
Test single binary with error summary:
```bash
./test_build.sh <binary_name>
```

### test_all.sh
Test all binaries (first 20):
```bash
./test_all.sh
```

## Error Suggestions

The driver auto-generates fix suggestions for common errors:

| Error | Suggestion |
|-------|-----------|
| E0599 (missing clone) | Add `#[derive(Clone)]` |
| E0433 (sha256) | Use `sha2` crate |
| E0433 (gix) | Move to `libgit.so` |
| E0433 (reqwest) | Move to `libhttp.so` |
| E0277 (Handler) | Check axum signature |
| E0601 (main) | Add `main()` or remove `[[bin]]` |

## Architecture

```
test_driver.rs
    ↓
error_store.rs (collects errors)
    ↓
suggest_fix() (generates suggestions)
```

## Integration with Server

The test driver uses the same error parsing and storage as the minimal-build-server, but runs standalone for faster iteration.

## Perf Data

Perf files are saved to `/tmp/build_<name>_<pid>.perf`

View with:
```bash
perf report -i /tmp/build_<name>_<pid>.perf
```

## Next Steps

- [ ] Integrate nix_canonical_builder for full telemetry
- [ ] Save perf data to parquet
- [ ] Auto-apply simple fixes
- [ ] Generate compiler dumps (MIR, LLVM IR)
