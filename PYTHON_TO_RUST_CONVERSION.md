# Python to Rust Conversion Plan

**Total**: 19 Python files to convert

## Priority 1: Servers (Critical)
- [ ] zos_server.py → zos_server.rs
- [ ] zos_server_v2.py → zos_server_v2.rs

## Priority 2: Analysis Tools
- [ ] analyze_error_patterns.py → analyze_error_patterns.rs
- [ ] analyze_project_ownership.py → analyze_project_ownership.rs
- [ ] classify_nix_failures.py → classify_nix_failures.rs
- [ ] extract_built_packages.py → extract_built_packages.rs
- [ ] list_successful_packages.py → list_successful_packages.rs

## Priority 3: Performance Tools
- [ ] bott8-layout-solver/parse_perf.py → parse_perf.rs
- [ ] bott8-layout-solver/map_perf_to_8d.py → map_perf_to_8d.rs
- [ ] setup_perf_probes.py → setup_perf_probes.rs
- [ ] merge_perf_lmfdb.py → merge_perf_lmfdb.rs

## Priority 4: Blockchain Tools
- [ ] fetch_top_contracts.py → fetch_top_contracts.rs
- [ ] fetch_recent_blocks.py → fetch_recent_blocks.rs
- [ ] trace_jupiter.py → trace_jupiter.rs
- [ ] predict_jupiter_branches.py → predict_jupiter_branches.rs
- [ ] branch_prediction_market.py → branch_prediction_market.rs
- [ ] build_jupiter_cfg.py → build_jupiter_cfg.rs

## Priority 5: Misc
- [ ] const_equivalence_nix/compare_orbits.py → compare_orbits.rs
- [ ] minizinc/extract_nix_store_frequencies.py → extract_nix_store_frequencies.rs

## Conversion Strategy

1. Read Python file
2. Identify dependencies
3. Convert to Rust with equivalent crates
4. Build as cdylib (.so)
5. Test functionality
6. Delete Python file

## Package as .so

All converted tools will be built as:
```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

Load via ZOS server dynamic loading.
