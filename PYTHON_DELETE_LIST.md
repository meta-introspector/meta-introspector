# Python Files Marked for Deletion

After Rust conversion is verified, delete these files:

- [ ] zos_server.py → CONVERTED to zos_server.rs
- [ ] zos_server_v2.py
- [ ] analyze_error_patterns.py
- [ ] analyze_project_ownership.py
- [ ] classify_nix_failures.py
- [ ] extract_built_packages.py
- [ ] list_successful_packages.py
- [ ] bott8-layout-solver/parse_perf.py
- [ ] bott8-layout-solver/map_perf_to_8d.py
- [ ] setup_perf_probes.py
- [ ] merge_perf_lmfdb.py
- [ ] fetch_top_contracts.py
- [ ] fetch_recent_blocks.py
- [ ] trace_jupiter.py
- [ ] predict_jupiter_branches.py
- [ ] branch_prediction_market.py
- [ ] build_jupiter_cfg.py
- [ ] const_equivalence_nix/compare_orbits.py
- [ ] minizinc/extract_nix_store_frequencies.py

## Verification Steps

1. Build Rust version
2. Test functionality matches Python
3. Delete Python file
4. Update documentation
