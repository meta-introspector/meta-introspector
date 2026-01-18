# Nix Build Error Matrix

**Generated**: $(date)

## Error Type Summary

| Error Type | Count | Description |
|------------|-------|-------------|
| flake-url | 21 | Flake URL error |
| assertion | 7 | Assertion failure |
| cannot-build | 40 | Cannot build derivation |
| missing-default | 73 | Missing packages.default |
| other | 99 | Unknown |

## Projects by Error Type

### flake-url: Flake URL error

- 14_20260118
- act_20260118
- crq-document-check_20260118
- data-lattice-builder_20260118
- decide_20260118
- dwim_20260118
- lmfdb2nix_20260118
- loop2_20260118
- meta-indexer_20260118
- meta-orchestrator_20260118
- nar-binstore-builder_20260118
- nix-duplication-detector_20260118
- nix-llm-context_20260118
- nix-ngram-indexer_20260118
- observe_20260118
- oeis-indexer_20260118
- orient_20260118
- orient-test_20260118
- run-zos-tasks_20260118
- test-bug-repro-nix-2gram-indexer_2026011
- workflow-tasks_20260118

**Total**: 21 projects

### assertion: Assertion failure

- 003_generate_virtual_packages_20260118
- 004_fold_to_matrix_20260118
- 005_final_report_20260118
- audit-flakes_20260118
- examples_20260118
- nar_20260118
- test-package-bag-of-words_20260118

**Total**: 7 projects

### cannot-build: Cannot build derivation

- 000_rnix_dump_20260118
- 002_binstore_locator_20260118
- 002c_collected_locks_derivation_20260118
- 002d_processed_lock_files_20260118
- 003_generate_bow_histogram_20260118
- 003_sanitize_extracted_data_20260118
- 004_nix_to_solana_translator_20260118
- 13_20260118
- 1inch_Router_20260118
- 2025-01-27-gemini-hello-world_20260118
- Aave_V3_Pool_20260118
- agda_20260118
- Balancer_Vault_20260118
- bench_20260118
- cargo-watch_20260118
- colosseum_20260118
- Compound_cETH_20260118
- coq_20260118
- Curve_3pool_20260118
- datalog_20260118
- flake_auditor_20260118
- gemini-integration_20260118
- grep-nar-flake_20260118
- idris2_20260118
- isabelle_20260118
- lean4_20260118
- Lido_stETH_20260118
- log_analyzer_20260118
- MakerDAO_DAI_20260118
- meta-introspector_20260118
- meta-introspector_20260118
- minizinc_20260118
- proof_20260118
- rustc-from-source_20260118
- rustc-from-source_20260118
- smt2_20260118
- SushiSwap_Router_20260118
- Uniswap_V2_Router_20260118
- Uniswap_V3_Router_20260118
- z3_20260118

**Total**: 40 projects

### expect-test: Test failure


**Total**: 0 projects

### missing-default: Missing packages.default

- 001_collect_locks_20260118
- 001_dump_nix_20260118
- 03_20260118
- 10_20260118
- 10_20260118
- 1-build-system_20260118
- 2025_20260118
- aggregator_20260118
- bag-of-words-generator_20260118
- base_20260118
- colosseum-mirror_20260118
- context_20260118
- context_20260118
- data-sources_20260118
- eval-context_20260118
- fetch-github-data_20260118
- filter_20260118
- flake_auditor_20260118
- full-graph_20260118
- gemini-prompt-flake_20260118
- get-nix-file-list_20260118
- github-api-wrapper_20260118
- github-data_20260118
- github-grep-flake_20260118
- github-to-foaf_20260118
- jobs_20260118
- layer1_20260118
- lean4_20260118
- llm-api-wrapper_20260118
- main_20260118
- mcts-nix_20260118
- micro_20260118
- minizinc-nix_20260118
- nar-bridge-flake_20260118
- nix_20260118
- nix-log-consumer-flake_20260118
- nix-to-poem-vial_20260118
- reader_20260118
- read-md-vial_20260118
- read-rs-vial_20260118
- repro-issue_20260118
- rust-libc-interceptor_20260118
- rust-libc-interceptor_20260118
- seed-data_20260118
- seed-graph_20260118
- solana_20260118
- solana-nix_20260118
- solfunmeme-profile_20260118
- sorter-suggester_20260118
- source_20260118
- source-config_20260118
- spore-cultivation-lab-flake_20260118
- ssh_20260118
- test-env-var_20260118
- test_nix_project_20260118
- test-secrets-sops_20260118
- test-urls_20260118
- time_20260118
- typenum-1.18.0_20260118
- vial-placeholder_20260118
- with-rust_20260118
- wrap-gemini-secrets_20260118
- zos-bootstrap_20260118
- zos-bootstrap_20260118
- zos-production_20260118
- zos-production_20260118
- zos-production_20260118
- zos-qa_20260118
- zos-qa_20260118
- zos-qa_20260118
- zos-spore-flake_20260118
- zos-spore-vial_20260118
- zos-spore-vial-flake_20260118

**Total**: 73 projects

### jq-error: jq file error


**Total**: 0 projects

### no-such-file: File not found


**Total**: 0 projects

## Error Matrix (Top 50 Projects)

| Project | missing-default | cannot-build | jq-error | assertion | other |
|---------|----------------|--------------|----------|-----------|-------|
| 000_rnix_dump_20260118 |   | X |   |   |   |
| 001_collect_locks_20260118 | X |   |   |   |   |
| 001_dump_nix_20260118 | X |   |   |   |   |
| 002a_grep_references_20260118 |   |   |   |   | X |
| 002a_inputs_only_20260118 |   |   |   |   | X |
| 002b_inputs_and_description_20 |   |   |   |   | X |
| 002_binstore_locator_20260118 |   | X |   |   |   |
| 002c_collected_locks_derivatio |   | X |   |   |   |
| 002d_processed_lock_files_2026 |   | X |   |   |   |
| 002_process_locks_20260118 |   |   |   |   | X |
| 003_generate_bow_histogram_202 |   | X |   |   |   |
| 003_generate_virtual_packages_ |   |   |   | X |   |
| 003_sanitize_extracted_data_20 |   | X |   |   |   |
| 004_fold_to_matrix_20260118 |   |   |   | X |   |
| 004_nix_to_solana_translator_2 |   | X |   |   |   |
| 005_final_report_20260118 |   |   |   | X |   |
| 03_20260118 | X |   |   |   |   |
| 09_20260118 |   |   |   |   | X |
| 10_20260118 | X |   |   |   |   |
| 10_20260118 | X |   |   |   |   |
| 12_20260118 |   |   |   |   | X |
| 13_20260118 |   | X |   |   |   |
| 14_20260118 |   |   |   |   |   |
| 1-build-system_20260118 | X |   |   |   |   |
| 1inch_Router_20260118 |   | X |   |   |   |
| 2025-01-27-build-time-gemini-c |   |   |   |   | X |
| 2025-01-27-gemini-hello-world_ |   | X |   |   |   |
| 2025_20260118 | X |   |   |   |   |
| 22_20260118 |   |   |   |   | X |
| Aave_V3_Pool_20260118 |   | X |   |   |   |
| act_20260118 |   |   |   |   |   |
| agda_20260118 |   | X |   |   |   |
| aggregator_20260118 | X |   |   |   |   |
| ai-workflow_20260118 |   |   |   |   | X |
| article-wrapper_20260118 |   |   |   |   | X |
| audit-flakes_20260118 |   |   |   | X |   |
| audit-with-rust_20260118 |   |   |   |   | X |
| bag-of-words-generator_2026011 | X |   |   |   |   |
| Balancer_Vault_20260118 |   | X |   |   |   |
| base_20260118 | X |   |   |   |   |
| bench_20260118 |   | X |   |   |   |
| binstore-prime-md-indexes_2026 |   |   |   |   | X |
| bootstrap-mycology-schedule-fl |   |   |   |   | X |
| brainfuck_20260118 |   |   |   |   | X |
| bridge-pattern_20260118 |   |   |   |   | X |
| c4-mycology-diagram_20260118 |   |   |   |   | X |
| c4-use-cases_20260118 |   |   |   |   | X |
| cargo-watch_20260118 |   | X |   |   |   |
| colosseum_20260118 |   | X |   |   |   |
| colosseum-mirror_20260118 | X |   |   |   |   |

## Statistics

- **Total failed builds**: 240
- **Unique error types**: 5
- **Most common error**: other
