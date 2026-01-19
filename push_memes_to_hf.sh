#!/bin/bash
# Push usage memes to HuggingFace dataset

DATASET="introspector/meta-meme"
LOCAL_DIR="hf-meta-meme"

echo "🎭 Pushing Usage Memes to HuggingFace"
echo "Dataset: $DATASET"
echo ""

# Clone or update dataset repo
if [ -d "$LOCAL_DIR" ]; then
    echo "📂 Updating existing repo..."
    cd "$LOCAL_DIR"
    git pull
else
    echo "📥 Cloning dataset repo..."
    git clone "https://huggingface.co/datasets/$DATASET" "$LOCAL_DIR"
    cd "$LOCAL_DIR"
fi

# Copy files
echo "📋 Copying files..."
cp ../usage_memes.json .
cp ../p2p_manifest.json .
cp ../nix_store_git_repos.txt .
cp ../nix_store_all_sources.json .

# Check if apt data is ready
if [ -f ../apt_git_repos.txt ]; then
    cp ../apt_git_repos.txt .
    cp ../apt_all_sources.json .
fi

# Create README
cat > README.md << 'EOF'
# meta-meme: Usage Meme Dataset

**Organization**: [introspector](https://huggingface.co/introspector)  
**Dataset**: [meta-meme](https://huggingface.co/datasets/introspector/meta-meme)  
**License**: AGPL-3.0

## 🎯 What is this?

The **meta-meme** dataset contains usage patterns of git repositories across system package managers, turning dependency data into shareable "memes" - cultural artifacts representing how software is actually used.

## 📊 Dataset Contents

### Core Files
- `usage_memes.json` - Complete usage graph with meme scores
- `p2p_manifest.json` - P2P distribution metadata
- `nix_store_git_repos.txt` - All git repos from Nix store (3,556 repos)
- `nix_store_all_sources.json` - Full Nix metadata
- `apt_git_repos.txt` - All git repos from apt packages
- `apt_all_sources.json` - Full apt metadata

### Statistics
- **Nix derivations analyzed**: 70,349
- **Unique git repositories**: 3,556+
- **Usage memes generated**: 2,769
- **Domains covered**: GitHub (91%), GNU Savannah (3%), GitLab (3%), Debian (2%), Kernel.org (1%)

## 🔥 Top Memes

1. **GRUB** (score: 89) - Bootloader, system-critical
2. **cargo-llvm-cov** (score: 41) - Rust tooling
3. **LLVM Project** (scores: 31-32) - Compiler infrastructure
4. **rusty_v8** (score: 36) - V8 bindings for Rust

## 📈 Meme Score Calculation

```
meme_score = base_usage + nix_bonus + apt_bonus + cross_system_bonus + domain_bonus

Where:
- base_usage: Number of packages using this repo
- nix_bonus: +10 if used by Nix
- apt_bonus: +10 if used by apt
- cross_system_bonus: +50 if used by both
- domain_bonus: +20 (compiler), +15 (system/kernel), +10 (rust), +5 (python/build)
```

## 🎭 Usage Meme Schema

```json
{
  "git_repo": "https://github.com/llvm/llvm-project",
  "used_by_nix": ["rustc", "clang", ...],
  "used_by_apt": ["llvm-dev", "clang", ...],
  "usage_count": 42,
  "domains": ["compiler", "toolchain"],
  "meme_score": 89.0,
  "first_seen": "2026-01-19T09:46:00Z",
  "last_seen": "2026-01-19T09:46:00Z"
}
```

## 🚀 Use Cases

- **Bootstrap datasets**: Complete git repo list for offline system reproduction
- **Dependency analysis**: Understand cross-system package relationships
- **Meme archaeology**: Study how software spreads through ecosystems
- **P2P distribution**: Share bootstrap datasets via IPFS/torrents
- **Cultural artifacts**: Package relationships as shareable memes

## 🔗 Related Datasets

- [meta-introspector](https://huggingface.co/datasets/introspector/meta-introspector) - 3M+ file index
- [solfunmeme-index](https://huggingface.co/datasets/introspector/solfunmeme-index) - Rust semantic analysis
- [git-activity](https://huggingface.co/datasets/introspector/git-activity) - Git activity tracking

## 📄 Citation

```bibtex
@dataset{meta_meme_2026,
  title={Meta-Meme: Usage Patterns as Cultural Artifacts},
  author={Meta-Introspector Team},
  year={2026},
  url={https://huggingface.co/datasets/introspector/meta-meme},
  note={Git repository usage patterns across Nix and apt ecosystems}
}
```

---

**Generated**: 2026-01-19T10:38:00+00:00  
**Project**: https://github.com/meta-introspector/meta-introspector  
**Organization**: https://huggingface.co/introspector
EOF

# Commit and push
echo "📤 Committing and pushing..."
git add .
git commit -m "Update usage memes: $(date -Iseconds)

- Nix: 3,556 git repos from 70,349 derivations
- Usage memes: 2,769 unique repos with scores
- Top meme: GRUB (score: 89)
- Domains: compiler, system, kernel, rust, python, build"

git push

echo ""
echo "✅ Done! View at: https://huggingface.co/datasets/$DATASET"
