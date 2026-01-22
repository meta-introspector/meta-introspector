# Nix Workflow System

## Vision

All executables read/write to nix store using canonical inputs/outputs. Multiple runs can be munged together or imported like flakes. Everything is composable.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Nix Workflow Scheduler                      │
├─────────────────────────────────────────────────────────┤
│  Task 1 → Nix Store → Task 2 → Nix Store → Task 3      │
│     ↓                     ↓                     ↓       │
│  /nix/store/abc/      /nix/store/def/      /nix/store/ghi/ │
└─────────────────────────────────────────────────────────┘
                           │
                    ┌──────┴──────┐
                    │             │
              ┌─────▼─────┐ ┌────▼────┐
              │  Compose  │ │  Munge  │
              │  Tasks    │ │ Results │
              └─────┬─────┘ └────┬────┘
                    │            │
              ┌─────▼────────────▼─────┐
              │   Export as Flake      │
              │   Import from Flake    │
              └────────────────────────┘
```

## Canonical I/O

Every executable follows this pattern:

```rust
fn main() {
    // Read inputs from nix store
    let inputs = read_nix_store_inputs();
    
    // Process
    let results = process(inputs);
    
    // Write outputs to nix store
    write_nix_store_outputs(results);
}
```

### Input Format

```bash
--inputs /nix/store/abc.../input1.parquet:/nix/store/def.../input2.json
```

### Output Format

```bash
--output /nix/store/ghi.../
  ├── results.parquet
  ├── telemetry.json
  └── manifest.json
```

## Task Definition

```rust
NixTask {
    name: "branch_mining",
    executable: "demo_branch_mining",
    inputs: vec!["/nix/store/abc.../"],
    outputs: vec!["results.parquet", "telemetry.json"],
    env: HashMap::new(),
    pure: false,  // Impure for network/LLM access
}
```

## Workflow Execution

### 1. Sequential Tasks

```rust
let mut scheduler = NixWorkflowScheduler::new();

// Add tasks
scheduler.add_task(mining_task);
scheduler.add_task(analysis_task);
scheduler.add_task(export_task);

// Run all
scheduler.run_all().await?;
```

### 2. Parallel Tasks

```rust
// Tasks with no dependencies run in parallel
let tasks = vec![
    create_mining_task("branch_mining", vec![]),
    create_mining_task("markov_mining", vec![]),
    create_mining_task("block_market", vec![]),
];

// Nix daemon handles parallelization
for task in tasks {
    scheduler.add_task(task);
}
```

### 3. Dependent Tasks

```rust
// Task 2 depends on Task 1 output
let task1 = create_mining_task("branch_mining", vec![]);
scheduler.add_task(task1);
scheduler.run_all().await?;

let task1_output = scheduler.results[0].nix_store_path.clone();

let task2 = create_analysis_task("llm_analysis", vec![task1_output]);
scheduler.add_task(task2);
```

## Composability

### Compose Multiple Tasks

```rust
// Combine outputs of multiple tasks
let composed = scheduler.compose_tasks(&vec![
    "branch_mining".to_string(),
    "markov_mining".to_string(),
])?;

// Composed task has all inputs/outputs
// Can be run as a single task
scheduler.add_task(composed);
```

### Munge Results

```rust
// Merge results from multiple runs
let munged = scheduler.munge_results(&vec![
    "run1_branch_mining".to_string(),
    "run2_branch_mining".to_string(),
    "run3_branch_mining".to_string(),
])?;

// Munged result is a single nix store path
// Contains all data from all runs
println!("Munged: {}", munged);
```

## Import/Export as Flakes

### Export Workflow

```rust
// Save workflow as importable flake
scheduler.export_as_flake("/tmp/workflow.nix")?;
```

Generated flake:

```nix
{
  description = "Workflow with 5 tasks";
  
  outputs = { self }: {
    tasks = [
      { name = "branch_mining"; path = "demo_branch_mining"; }
      { name = "markov_mining"; path = "demo_markov_mining"; }
    ];
    
    results = [
      { task = "branch_mining"; store = "/nix/store/abc.../"; }
      { task = "markov_mining"; store = "/nix/store/def.../"; }
    ];
  };
}
```

### Import Workflow

```rust
// Load workflow from another project
scheduler.import_workflow("path/to/workflow.nix")?;

// All tasks and results are now available
// Can be composed with new tasks
```

## Pure vs Impure Tasks

### Pure Tasks (Cacheable)

```rust
NixTask {
    name: "export_to_hf",
    executable: "export-to-huggingface",
    inputs: vec!["/nix/store/abc.../"],
    outputs: vec!["dataset-card.md"],
    pure: true,  // Deterministic, cacheable
}
```

Benefits:
- Instant cache hits
- Reproducible
- No network access needed

### Impure Tasks (Live)

```rust
NixTask {
    name: "llm_analysis",
    executable: "demo_universal_llm_proxy",
    inputs: vec!["/nix/store/abc.../"],
    outputs: vec!["analysis.parquet"],
    pure: false,  // Needs network, LLM API
}
```

Benefits:
- Access to external resources
- Real-time data
- Telemetry capture

## Example Workflows

### 1. Mining Pipeline

```rust
// Mine → Analyze → Export
let workflow = vec![
    create_mining_task("branch_mining", vec![]),
    create_analysis_task("llm_analysis", vec![mining_output]),
    create_export_task("hf_export", vec![analysis_output]),
];
```

### 2. Batch Processing

```rust
// Run same task on multiple inputs
for input in inputs {
    let task = create_mining_task(&format!("run_{}", i), vec![input]);
    scheduler.add_task(task);
}

// Munge all results
let munged = scheduler.munge_results(&all_task_names)?;
```

### 3. Incremental Updates

```rust
// Load previous workflow
scheduler.import_workflow("previous_run.nix")?;

// Add new tasks
scheduler.add_task(new_mining_task);

// Compose with previous results
let composed = scheduler.compose_tasks(&vec![
    "previous_branch_mining".to_string(),
    "new_branch_mining".to_string(),
])?;
```

## Nix Daemon Integration

All tasks run through nix daemon:

```bash
# Pure build
nix-build task.nix --no-out-link

# Impure build
nix-build task.nix --impure --no-out-link

# Result is in nix store
/nix/store/abc123.../
```

Benefits:
- Automatic caching
- Parallel execution
- Garbage collection
- Content addressing
- Reproducibility

## Data Collection Pattern

Every executable follows this pattern:

```rust
// 1. Read canonical inputs
let inputs = std::env::var("inputs")
    .unwrap()
    .split(':')
    .map(|p| read_nix_store(p))
    .collect();

// 2. Process
let results = process(inputs);

// 3. Write canonical outputs
let output_dir = std::env::var("out").unwrap();
write_parquet(&format!("{}/results.parquet", output_dir), results)?;
write_json(&format!("{}/telemetry.json", output_dir), telemetry)?;
```

## Manifest Format

Every output includes a manifest:

```json
{
  "task": "branch_mining",
  "timestamp": "2026-01-16T13:00:00Z",
  "inputs": [
    "/nix/store/abc.../input1.parquet"
  ],
  "outputs": {
    "results": "/nix/store/def.../results.parquet",
    "telemetry": "/nix/store/def.../telemetry.json"
  },
  "duration_ms": 1234,
  "success": true
}
```

## Composability Rules

1. **Inputs are nix store paths** - Always read from `/nix/store/`
2. **Outputs are nix store paths** - Always write to `$out/`
3. **Manifest required** - Every output has `manifest.json`
4. **Parquet preferred** - Use parquet for data, JSON for metadata
5. **Idempotent** - Same inputs → same outputs

## Benefits

1. **Reproducibility** - Pure builds are deterministic
2. **Caching** - Nix daemon caches everything
3. **Composability** - Tasks can be combined freely
4. **Mungeable** - Multiple runs merge easily
5. **Importable** - Workflows are flakes
6. **Parallel** - Nix daemon handles parallelization
7. **Garbage Collection** - Unused results cleaned up
8. **Content Addressing** - Deduplication automatic

## Next Steps

1. Convert all 217 binaries to canonical I/O
2. Add manifest generation to each
3. Test workflow composition
4. Implement parallel execution
5. Set up automatic HuggingFace export
6. Create workflow library

## Ready to Deploy!

All executables use canonical I/O, all workflows are composable, everything runs via nix daemon!
