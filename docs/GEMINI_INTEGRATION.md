# Gemini CLI Integration

## Overview

Gemini CLI is integrated into our nix flake for LLM-powered code analysis and context generation.

## Installation

```bash
# Build gemini CLI
nix build .#gemini

# Or use in dev shell
nix develop
gemini --help
```

## Use Cases

### 1. Generate LLM Context for Nix Flakes

```bash
# Generate context for our flake
gemini --include-directories . --model gemini-2.5-flash

# With checkpointing
gemini --include-directories . --checkpointing --model gemini-2.5-flash
```

### 2. Query About Code Structure

```bash
# Ask about rustc branch predictions
gemini "Explain the branch prediction mining system in demo_branch_mining.rs"

# Ask about Markov chains
gemini "How does the Markov chain mining map to rustc compiler branches?"
```

### 3. Automated Mining Analysis

```bash
# Analyze mining results
gemini "Analyze the blockchain from demo_swarm_hunt and suggest optimizations"

# Compare compression ratios
gemini "Compare the compression ratios between demo_block_market and demo_content_store"
```

### 4. Generate Documentation

```bash
# Generate docs for a demo
gemini "Generate comprehensive documentation for demo_lattice.rs including usage examples"

# Create tutorial
gemini "Create a step-by-step tutorial for running the branch prediction mining demo"
```

## Integration with Mining Demos

### Branch Prediction Mining + Gemini

```bash
# Run branch mining
cargo run --release --bin demo_branch_mining > branch_results.txt

# Analyze with Gemini
gemini "Analyze these branch predictions and suggest which branches are most critical: $(cat branch_results.txt)"
```

### Markov Chain Mining + Gemini

```bash
# Run Markov mining
cargo run --release --bin demo_markov_mining > markov_results.txt

# Get insights
gemini "What grammar patterns are most common in these Markov transitions: $(cat markov_results.txt)"
```

### Lattice Proof + Gemini

```bash
# Run lattice proof
cargo run --release --bin demo_lattice > lattice_proof.txt

# Verify with LLM
gemini "Verify this lattice proof and explain the uniqueness property: $(cat lattice_proof.txt)"
```

## LLM Context Generation Scripts

Located in: `/mnt/data1/nix/vendor/rust/cargo2nix/ai-ml-zk-ops/source/tools/github/meta-introspector/streamofrandom/2025/09/`

### Generate Monster Group Context

```bash
cd /mnt/data1/nix/vendor/rust/cargo2nix/ai-ml-zk-ops/source/tools/github/meta-introspector/streamofrandom/2025/09/22/nix-llm-context/

./generate_monster_group_llm_txt.sh \
  --symbol "Monster Group" \
  --html-file-name monster_group.html \
  --keywords-script ./extract_keywords.sh \
  --links-file-name links.txt \
  --tutorials-pattern "*.md" \
  --main-project /mnt/data1/meta-introspector \
  --output-dir ./output
```

### Generate OEIS Context

```bash
./generate_oeis_llm_txt.sh \
  --sequence A000001 \
  --output-dir ./output
```

## Advanced Usage

### Batch Processing

```bash
# Process all demo binaries
for demo in demo_*.rs; do
  echo "Analyzing $demo..."
  gemini "Summarize the purpose and key algorithms in $demo" > "docs/analysis_${demo%.rs}.md"
done
```

### Context-Aware Mining

```bash
# Use Gemini to suggest mining targets
TARGETS=$(gemini "Based on the rustc source code, suggest 10 functions that would benefit from branch prediction mining")

# Feed to mining system
echo "$TARGETS" | cargo run --release --bin demo_branch_mining --stdin
```

### Automated Documentation

```bash
# Generate README for each mining system
gemini "Generate a comprehensive README.md for the branch prediction mining system" > docs/BRANCH_MINING.md
gemini "Generate a comprehensive README.md for the Markov chain mining system" > docs/MARKOV_MINING.md
```

## Configuration

Gemini CLI uses environment variables:

```bash
export GEMINI_API_KEY="your-api-key"
export GEMINI_MODEL="gemini-2.5-flash"  # or gemini-pro
```

## Integration with HuggingFace

```bash
# Generate dataset descriptions
gemini "Generate a dataset card for introspector/rust/branch-predictions/" > dataset_card.md

# Upload to HuggingFace
huggingface-cli upload introspector/rust dataset_card.md --repo-type dataset
```

## Tips

1. **Use checkpointing** for long-running analysis
2. **Include context** from multiple files for better understanding
3. **Iterate** - refine prompts based on responses
4. **Combine** with mining results for deeper insights
5. **Document** - save useful prompts and responses

## Examples

See `examples/gemini/` for complete examples of:
- Mining analysis workflows
- Documentation generation
- Code optimization suggestions
- Dataset curation
