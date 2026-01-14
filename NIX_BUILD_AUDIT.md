# Nix Build Call Audit

## 🎯 Goal
Find ALL Rust code that calls `nix build` and centralize to ONE audited, perf-instrumented location.

## 📊 Current Nix Build Callers (Found 40 files)

### Primary Nix Builders (4 files)

1. **`unified_nix_builder.rs`** ⭐ MAIN CANDIDATE
   - Already centralized
   - Has telemetry support
   - LD_PRELOAD integration
   - 15 matches

2. **`nix_build_telemetry.rs`**
   - Telemetry wrapper
   - 11 matches
   - Should use unified builder

3. **`unified_nix_so_wrapper.rs`**
   - Nix + SO wrapping
   - 14 matches
   - Should use unified builder

4. **`nix_telemetry_integration.rs`**
   - Transparent telemetry
   - 8 matches
   - Should use unified builder

### Service/API Layers (2 files)

5. **`unified_nix_service.rs`**
   - Async service wrapper
   - 6 matches
   - Should use unified builder

6. **`nix_as_a_service.rs`**
   - Service API
   - 4 matches
   - Should use unified builder

### Specialized Tools (5 files)

7. **`custom_rust_nightly_build.rs`**
   - Uses unified_nix_builder ✅
   - 5 matches

8. **`nix_cargo_interceptor.rs`**
   - Callback interceptor
   - 5 matches

9. **`perf_canonical_recorder.rs`**
   - Perf recording
   - 5 matches
   - Already has SessionType::NixBuild ✅

10. **`nixso2probe/src/nix_rust_layers.rs`**
    - Layer generation
    - 7 matches

11. **`rustc_syscall_proof.rs`**
    - Direct nix-build call
    - 1 match

### Test/Example Files (29 files)
- Various test files with hardcoded paths
- Should use unified builder

## 🔧 Solution: Canonical Nix Builder

### Create `nix_canonical_builder.rs`

```rust
// nix_canonical_builder.rs
// THE ONLY PLACE TO CALL NIX BUILD
// All nix builds go through here with full instrumentation

use perf_macros::{perf_auto, perf_probe};
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixBuildRequest {
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixBuildResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub store_paths: Vec<String>,
    pub duration_secs: f64,
}

pub struct NixCanonicalBuilder {
    telemetry_enabled: bool,
    perf_enabled: bool,
}

impl NixCanonicalBuilder {
    pub fn new() -> Self {
        Self {
            telemetry_enabled: true,
            perf_enabled: true,
        }
    }
    
    /// THE ONLY FUNCTION THAT CALLS NIX BUILD
    #[perf_auto]
    #[perf_probe]
    pub fn build(&self, request: NixBuildRequest) -> Result<NixBuildResult, String> {
        // This is the ONLY place where Command::new("nix") happens
        let mut cmd = Command::new("nix");
        cmd.args(&request.args);
        
        // Add environment variables
        for (key, value) in &request.env {
            cmd.env(key, value);
        }
        
        // Set working directory
        if let Some(dir) = &request.working_dir {
            cmd.current_dir(dir);
        }
        
        // Execute
        let start = std::time::Instant::now();
        let output = cmd.output().map_err(|e| format!("Failed to execute nix: {}", e))?;
        let duration = start.elapsed();
        
        // Parse output
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Extract store paths
        let store_paths = self.extract_store_paths(&stdout);
        
        Ok(NixBuildResult {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout,
            stderr,
            store_paths,
            duration_secs: duration.as_secs_f64(),
        })
    }
    
    fn extract_store_paths(&self, output: &str) -> Vec<String> {
        output.lines()
            .filter(|line| line.starts_with("/nix/store/"))
            .map(|s| s.to_string())
            .collect()
    }
}

// Convenience functions
pub fn nix_build(args: &[&str]) -> Result<NixBuildResult, String> {
    let builder = NixCanonicalBuilder::new();
    builder.build(NixBuildRequest {
        args: args.iter().map(|s| s.to_string()).collect(),
        env: vec![],
        working_dir: None,
    })
}

pub fn nix_build_flake(flake: &str) -> Result<NixBuildResult, String> {
    nix_build(&["build", flake])
}
```

## 🔄 Migration Plan

### Phase 1: Create Canonical Builder
- [x] Design `nix_canonical_builder.rs`
- [ ] Add `#[perf_auto]` and `#[perf_probe]`
- [ ] Add telemetry integration
- [ ] Add LD_PRELOAD support
- [ ] Test with simple build

### Phase 2: Migrate Primary Builders
Priority order:

1. **`unified_nix_builder.rs`** → Use canonical builder internally
2. **`nix_build_telemetry.rs`** → Use canonical builder
3. **`unified_nix_so_wrapper.rs`** → Use canonical builder
4. **`nix_telemetry_integration.rs`** → Use canonical builder

### Phase 3: Migrate Services
5. **`unified_nix_service.rs`** → Use canonical builder
6. **`nix_as_a_service.rs`** → Use canonical builder

### Phase 4: Migrate Specialized Tools
7. **`nix_cargo_interceptor.rs`** → Use canonical builder
8. **`nixso2probe/src/nix_rust_layers.rs`** → Use canonical builder
9. **`rustc_syscall_proof.rs`** → Use canonical builder
10. **`recursive_rustc_wrapper.rs`** → Use canonical builder

### Phase 5: Update Tests
- Update all test files to use canonical builder
- Remove hardcoded nix-build calls

## ✅ Benefits

1. **Single Audit Point** - Only one place to review nix calls
2. **Full Instrumentation** - Every nix build has perf + telemetry
3. **Parquet Capture** - All inputs/outputs captured
4. **Consistent API** - Same interface everywhere
5. **Easy Updates** - Change once, affects all callers
6. **Security** - Single point for security checks

## 🎯 Enforcement

### Lint Rule
```rust
// Add to clippy.toml or custom lint
#[deny(direct_nix_call)]
// Deny: Command::new("nix")
// Deny: Command::new("nix-build")
// Allow: Only in nix_canonical_builder.rs
```

### Code Review Checklist
- [ ] No `Command::new("nix")` outside canonical builder
- [ ] No `Command::new("nix-build")` outside canonical builder
- [ ] All nix calls use `nix_canonical_builder::nix_build()`

## 📊 Current State

**Total files calling nix**: 40
**Primary builders**: 4
**Services**: 2
**Specialized tools**: 5
**Tests**: 29

**After migration**: 1 (nix_canonical_builder.rs)

## 🚀 Next Steps

1. Create `nix_canonical_builder.rs`
2. Add to Cargo.toml as library
3. Test with simple build
4. Migrate `unified_nix_builder.rs` first
5. Update all callers one by one
6. Add lint rule to prevent new direct calls
7. Document usage

---

**Status**: Audit complete, 40 files identified
**Next**: Create `nix_canonical_builder.rs` with full instrumentation
