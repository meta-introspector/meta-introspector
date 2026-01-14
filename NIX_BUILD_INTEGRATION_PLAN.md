# Nix Build Telemetry Integration Plan

## 🎯 Goal
Create a **unified nix build wrapper** that integrates:
1. **Perf recording** (via `perf_canonical_recorder`)
2. **Build telemetry** (existing `nix_build_telemetry.rs`)
3. **LD_PRELOAD wrapping** (existing `unified_nix_builder.rs`)
4. **Structured JSON output** (canonical format)

## 📊 Existing Nix Build Tools

### 1. `unified_nix_builder.rs`
**Purpose**: Centralized nix build with LD_PRELOAD telemetry
**Features**:
- LD_PRELOAD library injection
- Telemetry wrapper initialization
- Build result capture

### 2. `nix_build_telemetry.rs`
**Purpose**: Wrap nix build with operation tracking
**Features**:
- Build timing
- Output capture
- ldd analysis of results

### 3. `unified_nix_so_wrapper.rs`
**Purpose**: Nix build + SO wrapping + telemetry
**Features**:
- Combined nix build + ldd2macro
- Wrapper generation

### 4. `nix_telemetry_integration.rs`
**Purpose**: Nix build + transparent telemetry
**Features**:
- Transparent telemetry macros
- Build.rs integration

## 🔧 New Unified Tool: `nix_canonical_builder.rs`

### Architecture

```
nix_canonical_builder
    ↓
    ├─→ perf_canonical_recorder (perf data)
    ├─→ unified_nix_builder (LD_PRELOAD + telemetry)
    ├─→ ldd analysis (dependencies)
    └─→ Canonical JSON output
```

### Features

1. **Perf Integration**
   - Automatic perf recording during build
   - Canonical perf JSON output

2. **Telemetry Integration**
   - LD_PRELOAD wrapper injection
   - Symbol call tracking
   - Structured telemetry logs

3. **Build Analysis**
   - ldd dependency extraction
   - Binary/library discovery
   - Build timing and metrics

4. **Canonical Output**
   - Single JSON format for all data
   - Compatible with Bott[8] layout solver
   - Ready for downstream analysis

### Output Structure

```json
{
  "session_id": "nix_build_1768405200",
  "timestamp": 1768405200,
  "build": {
    "command": ["nix", "build", ".#hello"],
    "exit_code": 0,
    "duration_secs": 12.34,
    "result_path": "/nix/store/...-hello-2.10",
    "stdout": "...",
    "stderr": "..."
  },
  "perf": {
    "session_id": "perf_nix_1768405200",
    "total_samples": 12345,
    "top_symbols": [...],
    "binaries": [...],
    "libraries": [...]
  },
  "telemetry": {
    "total_calls": 456,
    "wrapped_binaries": 32,
    "wrapped_libraries": 91,
    "log_path": "data/telemetry/nix_build_1768405200.jsonl"
  },
  "dependencies": {
    "ldd_count": 71,
    "libraries": [...],
    "binaries": [...]
  },
  "output_files": {
    "perf_data": "data/perf_canonical/perf_nix_1768405200.perf.data",
    "perf_json": "data/perf_canonical/perf_nix_1768405200.json",
    "telemetry_log": "data/telemetry/nix_build_1768405200.jsonl",
    "build_json": "data/nix_builds/nix_build_1768405200.json"
  }
}
```

## 🔄 Implementation Plan

### Phase 1: Create Core Tool

```rust
// nix_canonical_builder.rs
use std::process::Command;
use serde::{Serialize, Deserialize};

pub struct NixCanonicalBuilder {
    perf_enabled: bool,
    telemetry_enabled: bool,
    session_id: String,
}

impl NixCanonicalBuilder {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            perf_enabled: true,
            telemetry_enabled: true,
            session_id: format!("nix_build_{}", timestamp),
        }
    }
    
    pub fn build(&mut self, args: Vec<String>) -> Result<NixBuildReport, Box<dyn std::error::Error>> {
        // 1. Start perf recording
        let perf_session = if self.perf_enabled {
            Some(self.start_perf_recording(&args)?)
        } else {
            None
        };
        
        // 2. Run nix build with telemetry
        let build_result = self.run_nix_build(&args)?;
        
        // 3. Stop perf and generate report
        let perf_report = if let Some(session) = perf_session {
            Some(self.finish_perf_recording(session)?)
        } else {
            None
        };
        
        // 4. Analyze build result
        let dependencies = self.analyze_dependencies(&build_result.result_path)?;
        
        // 5. Collect telemetry
        let telemetry = self.collect_telemetry()?;
        
        // 6. Generate canonical report
        let report = NixBuildReport {
            session_id: self.session_id.clone(),
            timestamp: self.timestamp,
            build: build_result,
            perf: perf_report,
            telemetry,
            dependencies,
            output_files: self.get_output_files(),
        };
        
        // 7. Save canonical JSON
        self.save_report(&report)?;
        
        Ok(report)
    }
}
```

### Phase 2: Integration Points

1. **Perf Integration**
   ```rust
   fn start_perf_recording(&self, args: &[String]) -> Result<PerfSession> {
       use perf_canonical_recorder::record_session;
       // Start perf in background
   }
   ```

2. **Telemetry Integration**
   ```rust
   fn run_nix_build(&self, args: &[String]) -> Result<BuildResult> {
       use unified_nix_builder::NixBuilder;
       // Run with LD_PRELOAD
   }
   ```

3. **Dependency Analysis**
   ```rust
   fn analyze_dependencies(&self, result_path: &str) -> Result<Dependencies> {
       // Run ldd on result
       // Extract binaries and libraries
   }
   ```

### Phase 3: Canonical Output

```rust
#[derive(Serialize, Deserialize)]
pub struct NixBuildReport {
    session_id: String,
    timestamp: u64,
    build: BuildResult,
    perf: Option<PerfReport>,
    telemetry: TelemetryReport,
    dependencies: Dependencies,
    output_files: OutputFiles,
}
```

## 🚀 Usage Examples

### Basic Build
```bash
nix_canonical_builder build .#hello
```

### Build with Options
```bash
nix_canonical_builder build .#rustc --perf --telemetry
```

### Build without Perf
```bash
nix_canonical_builder build .#hello --no-perf
```

### Analyze Existing Build
```bash
nix_canonical_builder analyze /nix/store/...-hello-2.10
```

## 📊 Output Files

All output in canonical locations:

```
data/
├── nix_builds/
│   └── nix_build_1768405200.json          # Main canonical output
├── perf_canonical/
│   ├── perf_nix_1768405200.perf.data      # Raw perf data
│   └── perf_nix_1768405200.json           # Perf report
└── telemetry/
    └── nix_build_1768405200.jsonl         # Telemetry log
```

## 🔗 Integration with Existing Systems

### 1. Bott[8] Layout Solver
```bash
# Build with canonical tool
nix_canonical_builder build .#hello

# Feed to layout solver
python3 bott8-layout-solver/nix_build_to_8d.py \
  data/nix_builds/nix_build_*.json
```

### 2. 71 Discovery Analysis
```bash
# Build 71 packages
for pkg in $(cat packages_71.txt); do
  nix_canonical_builder build ".#$pkg"
done

# Analyze all 71
python3 analyze_71_nix_builds.py data/nix_builds/
```

### 3. LMFDB Classification
```bash
# Build and classify
nix_canonical_builder build .#rustc
python3 classify_nix_build_lmfdb.py \
  data/nix_builds/nix_build_*.json \
  --levels 11,23,47,71
```

## ✅ Benefits

1. **Single Entry Point** - One tool for all nix builds
2. **Comprehensive Data** - Perf + telemetry + dependencies
3. **Canonical Format** - Standardized JSON output
4. **Easy Integration** - Works with all downstream tools
5. **Reproducible** - Session ID tracks everything
6. **Extensible** - Easy to add new features

## 📝 Migration Checklist

- [ ] Create `nix_canonical_builder.rs`
- [ ] Integrate `perf_canonical_recorder` as library
- [ ] Integrate `unified_nix_builder` for LD_PRELOAD
- [ ] Add ldd dependency analysis
- [ ] Add telemetry collection
- [ ] Create canonical JSON schema
- [ ] Add CLI argument parsing
- [ ] Test with simple build (hello)
- [ ] Test with complex build (rustc)
- [ ] Update shell scripts to use new tool
- [ ] Create integration tools (nix_build_to_8d.py, etc.)
- [ ] Document usage and examples

## 🎯 Next Steps

1. **Create skeleton** of `nix_canonical_builder.rs`
2. **Add perf integration** using `perf_canonical_recorder` as lib
3. **Add telemetry integration** using `unified_nix_builder`
4. **Test with simple build** (nix build .#hello)
5. **Generate first canonical JSON** output
6. **Feed to Bott[8] solver** for visualization

---

**Status**: Plan complete, ready to implement
**Next**: Create `nix_canonical_builder.rs` skeleton
