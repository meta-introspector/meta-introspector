# Binary I/O Documentation

## Parquet Producers (Writers)

### Core Analysis Binaries

#### 1. `markov_resonance_analyzer`
**Writes**: `markov_symbol_scores.parquet` (106MB)
**Reads**: Source code files from nix store
**Purpose**: Symbol similarity analysis using Markov chains
**Schema**: symbol (string), file_id (uint64), score (float64), frequency (uint64)

#### 2. `nix_store_grammar` (lmfdb-self-analyzer)
**Writes**: `nix_store_grammars.parquet` (1.5MB, 49,655 rows)
**Reads**: `/nix/store` files
**Purpose**: Extract grammar patterns from nix store
**Schema**: path (string), grammar (string), size (uint64), compression_ratio (float64)

#### 3. `build-logs-to-parquet`
**Writes**: `nix_build_logs.parquet` (5.4KB)
**Reads**: `~/.local/share/nix-builder/logs/*.log`
**Purpose**: Convert build logs to structured format
**Schema**: project (string), build_status (string), exit_code (int32), timestamp (timestamp)

#### 4. `git_temporal_morphisms`
**Writes**: `git_temporal_morphisms.parquet`
**Reads**: Git repositories in `/mnt/data1/git/`
**Purpose**: Track temporal changes in git trees
**Schema**: source_tree (string), target_tree (string), commit_time (uint64), replaced_tree (string), witness_type (string)

#### 5. `bootstrap_arrow_chain`
**Writes**: `bootstrap_arrow_chain.parquet`
**Reads**: MES bootstrap chain
**Purpose**: Track bootstrap evolution
**Schema**: stage (string), replaced_arrow (string), new_arrow (string), byte_offset (uint64), timestamp (uint64), witness (string)

#### 6. `byte_provenance_tracker`
**Writes**: `byte_provenance.parquet`
**Reads**: Binary files, git objects
**Purpose**: Track byte-level provenance
**Schema**: git_object (string), byte_offset (uint64), process_id (string), code_byte (uint8), reach_depth (uint32), labeled_by (string)

### Supporting Binaries

#### 7. `file_index_service`
**Writes**: `data/patterns/*.parquet`
**Reads**: File system index
**Purpose**: Pattern analysis and caching

#### 8. `trading_node`
**Writes**: Portfolio data to parquet
**Reads**: Market data
**Purpose**: Trading simulation

#### 9. `rustc_auto_labeler`
**Writes**: Labeled rustc data
**Reads**: Rustc compilation traces
**Purpose**: Auto-label compilation patterns

## Parquet Consumers (Readers)

### Analysis Tools

#### 1. `query-parquet`
**Reads**: Any parquet file
**Purpose**: SQL queries on parquet data
**Usage**: `query-parquet nix_build_logs.parquet "SELECT * FROM nix_build_logs LIMIT 5"`

#### 2. Symbol Analysis Suite
**Reads**: `markov_symbol_scores.parquet`
**Binaries**:
- `symbol_eigenvector` - Eigenvector analysis
- `eigenvector_label_mapper` - Label mapping
- `automorphic_orbit_lmfdb` - LMFDB orbit analysis
- `elf_moonshine_detector` - Moonshine detection
- `binary_fingerprint_decoder` - Binary fingerprinting
- `markov_name_path_analyzer` - Path analysis
- `term_eigenvector_analyzer` - Term analysis

#### 3. Grammar Analysis Suite
**Reads**: `nix_store_grammars.parquet`
**Binaries**:
- `extract_code_tokens` - Token extraction
- `analyze_char_transitions` - Character analysis
- `analyze_transitions` - Transition analysis
- `find_word_sequences` - Sequence finding
- `markov_tree` - Tree analysis
- `markov_full_traversal` - Full traversal
- `merge_grammar` - Grammar merging
- `show_code_functions` - Function display
- `inspect_parquet` - Parquet inspection

## Standard Test Battery

### 1. Build and Generate Data
```bash
# Generate all parquet files
cargo run --release --bin markov_resonance_analyzer
cargo run --release --bin nix_store_grammar
cargo run --release --bin build-logs-to-parquet
cargo run --release --bin git_temporal_morphisms
cargo run --release --bin bootstrap_arrow_chain
cargo run --release --bin byte_provenance_tracker
```

### 2. Verify Outputs
```bash
# Check all parquet files exist
ls -lh *.parquet

# Verify schemas
cargo run --release --bin query-parquet -- \
  markov_symbol_scores.parquet \
  "SELECT * FROM markov_symbol_scores LIMIT 1"
```

### 3. Run Analysis Suite
```bash
# Symbol analysis
cargo run --release --bin symbol_eigenvector
cargo run --release --bin elf_moonshine_detector

# Grammar analysis
cargo run --release --bin extract_code_tokens
cargo run --release --bin analyze_transitions
```

### 4. Integration Test
```bash
# Full pipeline test
./test_parquet_pipeline.sh
```

## File Locations

### Inputs
- `/nix/store/*` - Nix store files
- `/mnt/data1/git/*` - Git repositories
- `~/.local/share/nix-builder/logs/*.log` - Build logs
- Source code files

### Outputs
- `markov_symbol_scores.parquet` - 106MB
- `nix_store_grammars.parquet` - 1.5MB
- `nix_build_logs.parquet` - 5.4KB
- `git_temporal_morphisms.parquet`
- `bootstrap_arrow_chain.parquet`
- `byte_provenance.parquet`
- `string_usage.parquet` - 107KB

### Cache
- `~/.local/share/nix-builder/cache/` - Build cache
- `data/patterns/*.parquet` - Pattern cache

## Adding New Parquet Binaries

1. **Create binary**: Add to `Cargo.toml` `[[bin]]` section
2. **Implement writer**: Use `arrow` and `parquet` crates
3. **Document I/O**: Add to this file
4. **Add to tests**: Update test battery
5. **Update README**: Add to Quick Start section
