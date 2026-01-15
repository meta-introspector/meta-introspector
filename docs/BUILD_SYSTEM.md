# Build System Documentation

## Canonical Build Log Capture

All build attempts are captured as structured data in `data/build_logs/`.

### Quick Start

```bash
# Capture build logs for current repo
./scripts/capture_build_log.sh

# View latest build summary
cat data/build_logs/$(ls -t data/build_logs/ | head -1)/metadata.json
```

### Build Log Structure

```
data/build_logs/
└── YYYYMMDD_HHMMSS/
    ├── full_build.log       # Complete build output
    ├── error_summary.txt    # Sorted error counts
    ├── warning_summary.txt  # Sorted warning counts
    ├── success_count.txt    # Number of successful builds
    └── metadata.json        # Build session metadata
```

### Metadata Format

```json
{
  "repo": "zos-server",
  "session_id": "20260115_135800",
  "timestamp": "2026-01-15T13:58:00-05:00",
  "total_binaries": 75,
  "errors": 45,
  "warnings": 12,
  "successes": 30
}
```

## Multi-Repo Build Capture

### zos-server (75 binaries)

```bash
cd ~/zos-server
./scripts/capture_build_log.sh
```

### meta-introspector (184 binaries)

```bash
cd /mnt/data1/meta-introspector
./scripts/capture_build_log.sh
```

## Build Error Analysis

### View Error Summary

```bash
# Most common errors
cat data/build_logs/LATEST/error_summary.txt | head -10
```

### Error Categories

- **E0601**: Missing main function (test files)
- **E0432**: Unresolved imports (missing dependencies)
- **E0282**: Type annotations needed
- **E0277**: Trait bounds not satisfied
- **E0433**: Unresolved crate (missing Cargo.toml entry)

## Fixing Build Errors

### 1. Comment Out Broken Binaries

```bash
# Remove binary from Cargo.toml
sed -i '/name = "broken_binary"/,+2d' Cargo.toml
```

### 2. Add Missing Dependencies

```bash
cargo add missing_crate
```

### 3. Preserve Intent with Macros

For files that can't build yet, preserve intent:

```rust
#[cfg(feature = "rustc_internals")]
use rustc_hir::*;

// TODO: Requires rustc internals
#[allow(dead_code)]
fn analyze_hir() {
    todo!("Move to zombie-driver repo")
}
```

## Build Data as Canonical Records

All build logs are:
- **Timestamped**: Unique session IDs
- **Structured**: JSON metadata + text logs
- **Versioned**: Tracked in git (via .gitignore exceptions)
- **Analyzable**: Ready for Markov/perf analysis

## Next Steps

1. Capture baseline build logs for both repos
2. Triage errors by category
3. Fix or comment out broken binaries
4. Re-capture to measure progress
5. Analyze build differences with perf
