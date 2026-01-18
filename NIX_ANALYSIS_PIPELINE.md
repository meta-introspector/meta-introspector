# Nix Store Analysis Pipeline

## Vision
Closed feedback loop: Source → Nix Build → Binary Analysis → Parquet → New Flakes → Repeat

## Analysis Spectrum (7 Phases)

### Phase 1: Source Archive
**Input:** File list from git  
**Output:** `source_archive.parquet`  
**Columns:** file_path, git_repo, commit, size, hash  
**Status:** ✅ Done (FILE_GIT_MAPPING.csv exists)

### Phase 2: Bag of Words (Multi-layer N-gram)
**Input:** Source files  
**Output:** `ngrams.parquet`  
**Columns:** file_id, ngram, layer (1-5), frequency, context  
**Tool:** Existing n-gram indexer  
**Status:** 🔨 Need to apply to all sources

### Phase 3: Markov Model
**Input:** N-grams  
**Output:** `markov_transitions.parquet`  
**Columns:** symbol, next_symbol, probability, file_id  
**Tool:** Existing markov analyzer  
**Status:** 🔨 Need to apply to all sources

### Phase 4: Vector Embeddings
**Input:** Markov model  
**Output:** `embeddings.parquet`  
**Columns:** symbol, embedding_vector (array), similarity_score  
**Tool:** Existing eigenvector analysis  
**Status:** 🔨 Need to apply to all sources

### Phase 5: Cargo Build
**Input:** Rust projects  
**Output:** `cargo_builds.parquet`  
**Columns:** project, status, dependencies, features, build_time  
**Tool:** cargo build wrapper  
**Status:** ⚠️ Need wrapper

### Phase 6: Syn/HIR/MIR Trace
**Input:** Successful cargo builds  
**Output:** `rust_ir.parquet`  
**Columns:** file, syn_ast, hir_nodes, mir_blocks, macro_expansions  
**Tool:** rust-telemetry-driver + syn parser  
**Status:** ⚠️ Need integration

### Phase 7: Binary Analysis
**Input:** Built binaries in /nix/store  
**Output:** `binary_analysis.parquet`  
**Columns:** store_path, symbols, syscalls, elf_sections, moonshine_map  
**Tool:** Existing moonshine + perf analysis  
**Status:** ⚠️ Need to apply to 111 built packages

## Fallback Nix Template

For all 232 failed builds, create capture-only flake:

```nix
{
  description = "Fallback capture for ${project}";
  
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = pkgs.runCommand "${project}-capture" {} ''
      mkdir -p $out
      
      # Phase 1: Archive sources
      cp -r ${self} $out/source
      
      # Phase 2: N-gram analysis
      ${ngram-indexer}/bin/index $out/source > $out/ngrams.json
      
      # Phase 3: Markov model
      ${markov-analyzer}/bin/analyze $out/ngrams.json > $out/markov.json
      
      # Phase 4: Embeddings
      ${vector-embedder}/bin/embed $out/markov.json > $out/embeddings.json
      
      # Phase 5-7: Skip if not Rust or build fails
      # Status: captured at phase 4
      echo "phase_4_complete" > $out/status
    '';
  };
}
```

## Bootstrap Macros

Use existing macros to generate analysis pipeline:

```rust
// From existing work
macro_rules! analysis_phase {
    ($name:ident, $input:ty, $output:ty) => {
        pub struct $name {
            input: $input,
            status: PhaseStatus,
        }
        
        impl $name {
            pub fn run(&self) -> Result<$output> {
                // Run analysis
                // Save to parquet
                // Update status
            }
        }
    };
}

// Generate all 7 phases
analysis_phase!(SourceArchive, FileList, Parquet);
analysis_phase!(NgramAnalysis, SourceArchive, Parquet);
analysis_phase!(MarkovModel, NgramAnalysis, Parquet);
analysis_phase!(VectorEmbedding, MarkovModel, Parquet);
analysis_phase!(CargoBuild, SourceArchive, Parquet);
analysis_phase!(RustIRTrace, CargoBuild, Parquet);
analysis_phase!(BinaryAnalysis, CargoBuild, Parquet);
```

## Closed Feedback Loop

```
┌─────────────────────────────────────────────────────────┐
│                    Source Files                         │
│              (FILE_GIT_MAPPING.csv)                     │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Split into Projects                        │
│         (group by git repo + directory)                 │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│           Generate Nix Flakes                           │
│    (template per project, fallback for failures)        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Nix Build (396 projects)                   │
│         Success: 111 → Phase 7 (binary)                 │
│         Failed: 232 → Phase 4 (embeddings)              │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│           Analysis Results (Parquet)                    │
│  - source_archive.parquet                               │
│  - ngrams.parquet                                       │
│  - markov_transitions.parquet                           │
│  - embeddings.parquet                                   │
│  - cargo_builds.parquet                                 │
│  - rust_ir.parquet                                      │
│  - binary_analysis.parquet                              │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│         Push to HuggingFace Dataset                     │
│      introspector/meta-introspector                     │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│      Generate New Flakes from Analysis                  │
│   (use embeddings to find similar projects)             │
│   (use markov to predict missing code)                  │
└────────────────────┬────────────────────────────────────┘
                     │
                     └──────────────┐
                                    │
                     ┌──────────────┘
                     ▼
              Back to Nix Build
```

## Implementation Steps

### Step 1: Split FILE_GIT_MAPPING.csv into Projects
```bash
# Group files by repo + top-level directory
cat FILE_GIT_MAPPING.csv | \
  awk -F',' '{print $2,$3}' | \
  sort | uniq -c > projects.txt
```

### Step 2: Generate Fallback Flakes for 232 Failed Builds
```bash
for project in $(cat failed_projects.txt); do
  cat > "$project/flake.nix" <<EOF
{
  description = "Capture analysis for $project";
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = 
      import ./capture-template.nix { inherit self nixpkgs; };
  };
}
EOF
done
```

### Step 3: Apply Phase 1-4 to All Projects
```bash
# Run analysis pipeline on all 343 projects
for project in $(ls -d */); do
  nix build "$project#default"
  # Saves to $out/{ngrams,markov,embeddings}.json
done
```

### Step 4: Apply Phase 5-7 to Rust Projects
```bash
# Only on projects with Cargo.toml
for project in $(find . -name Cargo.toml -exec dirname {} \;); do
  nix build "$project#rust-analysis"
  # Saves to $out/{cargo,syn,hir,mir,binary}.json
done
```

### Step 5: Convert All JSON to Parquet
```bash
# Use existing parquet converter
for json in $(find . -name "*.json"); do
  json-to-parquet "$json" "${json%.json}.parquet"
done
```

### Step 6: Push to HuggingFace
```bash
# Upload all parquet files
huggingface-cli upload \
  introspector/meta-introspector \
  analysis/*.parquet \
  --repo-type dataset
```

## Status Tracking

Each project has status file:
```json
{
  "project": "feature-2-nix-base",
  "phases": {
    "source_archive": "complete",
    "ngrams": "complete",
    "markov": "complete",
    "embeddings": "complete",
    "cargo_build": "failed",
    "rust_ir": "skipped",
    "binary_analysis": "skipped"
  },
  "outputs": {
    "source_archive": "/nix/store/...-source.parquet",
    "ngrams": "/nix/store/...-ngrams.parquet",
    "markov": "/nix/store/...-markov.parquet",
    "embeddings": "/nix/store/...-embeddings.parquet"
  }
}
```

## Next Actions

1. ✅ Create this spec
2. ⚠️ Find existing bootstrap macros
3. ⚠️ Create capture-template.nix
4. ⚠️ Generate fallback flakes for 232 failed builds
5. ⚠️ Run phase 1-4 on all 343 projects
6. ⚠️ Run phase 5-7 on 111 successful builds
7. ⚠️ Convert all to parquet
8. ⚠️ Push to HuggingFace
