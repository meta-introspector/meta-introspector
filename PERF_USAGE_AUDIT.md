# Perf Record Usage Audit

## 📊 All Perf Record Usage in Codebase

### 1. `perf_nix_rust_beta.sh`
**Purpose**: Capture perf data from Nix Rust beta build
**Command**:
```bash
sudo perf record -F 99 -g --call-graph dwarf -a -o "${OUTPUT_DIR}/${SESSION}.perf.data" &
```
**Migration**:
```bash
# Before
sudo perf record -F 99 -g --call-graph dwarf -a -o output.perf.data &
PERF_PID=$!
nix build .#rust-beta
kill $PERF_PID

# After
sudo perf_canonical_recorder nix nix build .#rust-beta
```

---

### 2. `perf_nix_rustc_build.sh`
**Purpose**: Capture perf data from Nix Rust build
**Command**:
```bash
sudo perf record -F 99 -g --call-graph dwarf -a -o "${OUTPUT_DIR}/${SESSION}.perf.data" &
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder nix nix build .#rustc
```

---

### 3. `record_nix_build_with_probes.sh`
**Purpose**: Record with custom probes
**Command**:
```bash
sudo perf record \
    -e 'probe_*' \
    -e cycles,instructions,cache-references,cache-misses \
    -F 99 -g --call-graph dwarf -a \
    -o "$OUTPUT_DIR/${SESSION}.perf.data" \
    -- nix-build ...
```
**Migration**: Add probe support to canonical tool
```rust
// In perf_canonical_recorder.rs
fn record_with_probes(&mut self, probes: Vec<String>) -> Result<()> {
    // Add -e 'probe_*' support
}
```

---

### 4. `record_rustc_build.sh`
**Purpose**: Record rustc build
**Command**:
```bash
sudo perf record \
    -e cycles,instructions,cache-references,cache-misses \
    -e branches,branch-misses,page-faults,context-switches \
    -F 99 -g --call-graph dwarf -a \
    -o "$OUTPUT_DIR/${SESSION}.perf.data" \
    -- cargo build --release
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder cargo cargo build --release
```

---

### 5. `record_rustc_simple.sh`
**Purpose**: Simple rustc recording
**Command**:
```bash
sudo perf record -e cpu-clock -F 99 -g -a -o "$OUTPUT_DIR/${SESSION}.perf.data" &
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder rustc rustc --version
```

---

### 6. `perf_rank_symbols.sh`
**Purpose**: Rank symbols by runtime usage
**Command**:
```bash
perf record -F 99 -g --call-graph dwarf -o perf.data -- "$BINARY" &
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder binary "$BINARY"
```

---

### 7. `extract_cost_attribution.sh`
**Purpose**: Cost attribution per language
**Command**:
```bash
perf record -o "$RESULTS_DIR/${lang}_perf.data" "$binary" 2>/dev/null || true
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder binary "$binary"
# Then read from data/perf_canonical/*.json
```

---

### 8. `test_const_equivalence.sh`
**Purpose**: Test const equivalence
**Command**:
```bash
perf record -e cycles,instructions -o "$OUTPUT_DIR/perf_${lang}.data" \
    timeout 5 "$binary" 2>/dev/null || true
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder binary timeout 5 "$binary"
```

---

### 9. `test_const_equivalence_nix.sh`
**Purpose**: Test const equivalence with Nix
**Command**:
```bash
perf record -e cycles,instructions -o "$OUTPUT_DIR/build_${lang}.data" \
    nix build ".#const71-${lang}" 2>&1
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder nix nix build ".#const71-${lang}"
```

---

### 10. `build_and_analyze_const71.sh`
**Purpose**: Build and analyze const 71
**Command**:
```bash
perf record -e cycles,instructions -o "../../$OUTPUT/build_${compiler}.data" \
    "$compiler" const_71.c -o "../../$OUTPUT/binary_${compiler}"
```
**Migration**:
```bash
# After
sudo perf_canonical_recorder binary "$compiler" const_71.c -o output
```

---

### 11. `setup_perf_probes.sh`
**Purpose**: Setup perf probes (documentation only)
**Command**: Example usage shown
```bash
sudo perf record -e 'probe_*' -a -- nix-build ...
```
**Migration**: Update documentation to use canonical tool

---

## 🔧 Migration Strategy

### Phase 1: Add Library Support
Make `perf_canonical_recorder.rs` usable as both binary and library:

```rust
// lib.rs
pub mod perf_recorder {
    pub use crate::{PerfSession, SessionType, PerfReport};
    
    pub fn record_session(
        session_type: SessionType,
        command: Vec<String>,
    ) -> Result<PerfReport, Box<dyn std::error::Error>> {
        let mut session = PerfSession::new(session_type, command, None);
        session.record()?;
        session.generate_report()
    }
}
```

### Phase 2: Update Shell Scripts
Create wrapper function for shell scripts:

```bash
# perf_canonical_wrapper.sh
perf_record_canonical() {
    local session_type="$1"
    shift
    sudo perf_canonical_recorder "$session_type" "$@"
}

# Usage in scripts
perf_record_canonical nix nix build .#hello
```

### Phase 3: Migrate One by One
Priority order:
1. ✅ `perf_nix_rust_beta.sh` - Most used
2. ✅ `perf_nix_rustc_build.sh` - Most used
3. ✅ `record_rustc_build.sh` - Important
4. ✅ `perf_rank_symbols.sh` - Frequently used
5. ⚠️ `record_nix_build_with_probes.sh` - Needs probe support
6. ✅ Others - Straightforward

### Phase 4: Add Missing Features
Features needed for full migration:

1. **Probe support** (`record_nix_build_with_probes.sh`)
   ```rust
   fn add_probes(&mut self, probes: Vec<String>) {
       for probe in probes {
           self.perf_cmd.arg("-e").arg(probe);
       }
   }
   ```

2. **Background recording** (several scripts)
   ```rust
   fn record_background(&mut self) -> Result<Child> {
       // Start perf in background, return PID
   }
   ```

3. **Timeout support** (`test_const_equivalence.sh`)
   ```rust
   fn with_timeout(&mut self, secs: u64) {
       self.timeout = Some(secs);
   }
   ```

## 📝 Updated Scripts

### Template for Migration

**Before**:
```bash
#!/usr/bin/env bash
OUTPUT_DIR="data/perf_rankings"
SESSION="my_session_$(date +%s)"

sudo perf record -F 99 -g --call-graph dwarf -a \
    -o "${OUTPUT_DIR}/${SESSION}.perf.data" \
    -- my-command args

sudo perf report -i "${OUTPUT_DIR}/${SESSION}.perf.data" \
    --stdio > "${OUTPUT_DIR}/${SESSION}_report.txt"

# Manual parsing...
```

**After**:
```bash
#!/usr/bin/env bash
# All perf recording now goes through canonical tool
sudo perf_canonical_recorder custom my-command args

# Output automatically in data/perf_canonical/*.json
# Ready for downstream analysis
```

## 🎯 Benefits of Migration

1. **Standardization** - All perf data in same format
2. **No duplication** - One tool, one format
3. **Easy integration** - JSON output for all tools
4. **Versioning** - Schema evolution tracked
5. **Debugging** - Consistent output structure
6. **Automation** - No manual parsing needed

## 📊 Migration Checklist

- [ ] Add library support to `perf_canonical_recorder.rs`
- [ ] Add probe support (`-e 'probe_*'`)
- [ ] Add background recording mode
- [ ] Add timeout support
- [ ] Create `perf_canonical_wrapper.sh` helper
- [ ] Migrate `perf_nix_rust_beta.sh`
- [ ] Migrate `perf_nix_rustc_build.sh`
- [ ] Migrate `record_rustc_build.sh`
- [ ] Migrate `perf_rank_symbols.sh`
- [ ] Migrate `extract_cost_attribution.sh`
- [ ] Migrate `test_const_equivalence.sh`
- [ ] Migrate `test_const_equivalence_nix.sh`
- [ ] Migrate `build_and_analyze_const71.sh`
- [ ] Migrate `record_rustc_simple.sh`
- [ ] Update `record_nix_build_with_probes.sh` (needs probes)
- [ ] Update `setup_perf_probes.sh` documentation
- [ ] Test all migrations
- [ ] Update downstream tools to read canonical JSON

## 🚀 Quick Start

```bash
# 1. Build canonical tool
cargo build --release --bin perf_canonical_recorder

# 2. Test with simple command
sudo ./target/release/perf_canonical_recorder rustc rustc --version

# 3. Check output
ls -lh data/perf_canonical/
cat data/perf_canonical/perf_rustc_*.json | jq .

# 4. Migrate first script
# Edit perf_nix_rust_beta.sh to use canonical tool

# 5. Test migration
./perf_nix_rust_beta.sh

# 6. Verify output format
cat data/perf_canonical/perf_nix_*.json | jq .
```

---

**Status**: Audit complete, 11 scripts identified, migration plan ready
**Next**: Add library support and migrate first script
