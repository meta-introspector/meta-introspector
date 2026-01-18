# Meta-Introspector Action - Injection Template

## How to Add to Your Project

Add this workflow to contribute to the meta-introspector collective!

### Step 1: Create `.github/workflows/meta-introspector.yml`

```yaml
name: Contribute to Meta-Introspector Collective

on:
  push:
    branches: [main, master]
  pull_request:
  workflow_dispatch:

jobs:
  analyze:
    uses: meta-introspector/meta-introspector/.github/workflows/meta-introspector-action.yml@novel-code-analysis-clean
    with:
      project_name: ${{ github.event.repository.name }}
      upload_to_hf: true
    secrets:
      HF_TOKEN: ${{ secrets.HF_TOKEN }}
```

### Step 2: Commit and Push

```bash
git add .github/workflows/meta-introspector.yml
git commit -m "Add meta-introspector collective analysis"
git push
```

## What Gets Collected

1. **Markov models** - Character transition patterns
2. **Syn ASTs** - Syntax tree structures  
3. **HIR/MIR** - Compiler intermediate representations
4. **Binary analysis** - ELF structure, symbols
5. **Telemetry** - Build traces, syscalls

## Data Storage

- **GitHub Artifacts** (90 days)
- **HuggingFace**: https://huggingface.co/datasets/introspector/meta-introspector-collective

## Batch Injection for Mike's Repos

```bash
#!/bin/bash
# inject_into_mikes_repos.sh

REPOS=(
  "zos-production"
  "zos-bootstrap"
  "zos-qa"
)

for repo in "${REPOS[@]}"; do
  cd "/opt/$repo" || cd "/home/mdupont/$repo" || continue
  
  mkdir -p .github/workflows
  cat > .github/workflows/meta-introspector.yml << 'EOF'
name: Meta-Introspector Analysis
on:
  push:
  workflow_dispatch:
jobs:
  analyze:
    uses: meta-introspector/meta-introspector/.github/workflows/meta-introspector-action.yml@novel-code-analysis-clean
    with:
      project_name: ${{ github.event.repository.name }}
      upload_to_hf: true
EOF
  
  git add .github/workflows/meta-introspector.yml
  git commit -m "Add meta-introspector analysis"
  git push
done
```

## Benefits

- Build collective code knowledge base
- Identify unique vs duplicate code
- Discover novel algorithms
- Create training data for code AI
- Prove originality of your work

## View Collective Data

https://huggingface.co/datasets/introspector/meta-introspector-collective
