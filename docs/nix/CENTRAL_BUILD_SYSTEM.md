# Central Build System - All Analysis as Nix Jobs

The central build system schedules all analysis as nix jobs with automatic dependency management.

## Job Graph

```
Job 1: languages (71 languages)
  ↓
Job 2: build-graph (first ordering)
  ↓
Job 3: perf-analysis (perf traces)
  ↓
Job 4: topological-matrix (function matrix)
  ↓
Job 5: harmonic-analysis (harmonics)
  ↓
Job 6: model-training (NN models)
  ↓
Job 7: complete (all jobs)
```

## Build All Jobs

```bash
./bootstrap
```

This builds the complete system with all analysis.

## Build Individual Jobs

```bash
# Build just languages
nix build .#languages

# Build just build graph
nix build .#build-graph

# Build just perf analysis
nix build .#perf-analysis

# Build just topological matrix
nix build .#topological-matrix

# Build just harmonic analysis
nix build .#harmonic-analysis

# Build just model training
nix build .#model-training

# Build all jobs
nix build .#all-jobs
```

## Job Dependencies

Each job declares its dependencies:

```nix
packages.${system}.topological-matrix = pkgs.stdenv.mkDerivation {
  buildInputs = [ 
    self.packages.${system}.build-graph      # Depends on build-graph
    self.packages.${system}.perf-analysis    # Depends on perf-analysis
  ];
};
```

Nix automatically:
- Builds dependencies first
- Caches completed jobs
- Rebuilds only changed jobs
- Runs independent jobs in parallel

## Query Job Status

```bash
# List all jobs
nix flake show

# Check if job is cached
nix-store -q --references result/

# View job output
ls result/languages/
ls result/graphs/
ls result/analysis/
ls result/matrix/
ls result/harmonics/
ls result/models/
```

## Job Outputs

Each job produces outputs in nix store:

```
/nix/store/xxx-languages/        # Job 1 output
/nix/store/yyy-build-graph/      # Job 2 output
/nix/store/zzz-perf-analysis/    # Job 3 output
/nix/store/aaa-topological-matrix/  # Job 4 output
/nix/store/bbb-harmonic-analysis/   # Job 5 output
/nix/store/ccc-model-training/      # Job 6 output
/nix/store/ddd-complete/            # Job 7 output (all)
```

## Parallel Execution

Nix builds independent jobs in parallel:

```
Job 1 (languages)
  ↓
Job 2 (build-graph) ← Job 3 (perf-analysis)
  ↓                      ↓
  └──────────┬───────────┘
             ↓
Job 4 (topological-matrix) ← Job 5 (harmonic-analysis)
             ↓                      ↓
             └──────────┬───────────┘
                        ↓
             Job 6 (model-training)
                        ↓
             Job 7 (complete)
```

Jobs 2 and 3 run in parallel.
Jobs 4 and 5 run in parallel.

## Incremental Builds

Change detection:
- Change Job 1 → Rebuilds Jobs 2-7
- Change Job 3 → Rebuilds Jobs 4-7
- Change Job 6 → Rebuilds Job 7
- No changes → All cached

## Distributed Builds

Configure remote builders in `nix.conf`:

```
builders = ssh://builder1 x86_64-linux
           ssh://builder2 x86_64-linux
```

Nix distributes jobs across builders automatically.

## CI/CD Integration

GitHub Actions:

```yaml
- name: Build all jobs
  run: nix build .#all-jobs
  
- name: Upload artifacts
  uses: actions/upload-artifact@v3
  with:
    path: result/
```

## Monitoring

View build progress:

```bash
# Watch build logs
nix build --print-build-logs

# Check build status
nix-store -q --references result/

# View job graph
nix-store -q --graph result/ | dot -Tpng > jobs.png
```

## Benefits

1. **Declarative**: All jobs defined in flake.nix
2. **Automatic**: Dependency management by nix
3. **Parallel**: Independent jobs run simultaneously
4. **Incremental**: Only rebuild changed jobs
5. **Reproducible**: Same inputs = same outputs
6. **Distributed**: Can run on multiple machines
7. **Cached**: Completed jobs stored in nix store
8. **Queryable**: All outputs accessible via result/

## Implementation

See: `nix/flake.nix` - Central build system definition
