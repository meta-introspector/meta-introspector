# 📚 File Index - Meta-Introspector

## 🚀 Start Here

| File | Purpose | For Who |
|------|---------|---------|
| **QUICKSTART.md** | Get started in 5 minutes | New users |
| **README.md** | Project overview | Everyone |
| **docs/QA_DEPLOYMENT_REVIEW.md** | Deployment guide | DevOps |

## 📖 Core Documentation

### Getting Started
- `QUICKSTART.md` - New user guide (download → deploy → use)
- `README.md` - Main project documentation
- `NIX_BUILD_READY.md` - Nix build instructions
- `DEPLOYMENT.md` - Deployment strategy

### Build & Compilation
- `BUILD_SYSTEM.md` - Build system overview
- `docs/compilation-fixes-2026-01-17.md` - Recent compilation fixes
- `docs/BUILD_FIXING_GUIDE.md` - How to fix build errors

### Deployment
- `deploy-qa.sh` - Deploy QA server (one command)
- `tools/deploy.sh` - Universal deployment manager
- `self-deploy.sh` - Self-deployment chain
- `deploy-chain.sh` - Systemd deployment
- `docs/QA_DEPLOYMENT_REVIEW.md` - Deployment review
- `docs/DEPLOYMENT_SYSTEM.md` - Multi-environment deployment
- `docs/DEPLOYMENT_STATUS.md` - Current deployment status

### CI/CD
- `.github/workflows/build.yml` - GitHub Actions build
- `.github/workflows/release.yml` - Multi-platform releases
- `.gitlab-ci.yml` - GitLab CI/CD
- `Dockerfile` - Docker multi-stage build
- `docs/github-actions-setup.md` - GitHub Actions troubleshooting

## 🔧 Main Binaries

| Binary | Source | Purpose |
|--------|--------|---------|
| **minimal-build-server** | `minimal_build_server.rs` | Dev server - your control panel |
| **demo_shared_memory** | `demo_shared_memory.rs` | Shared memory demo |
| **nix_as_a_service** | `nix_as_a_service.rs` | Nix service API |
| **solana_as_a_service** | `solana_as_a_service.rs` | Solana integration |

## 📁 Directory Structure

```
meta-introspector/
├── *.rs                    # 220+ Rust binaries
├── *.md                    # Documentation files
├── *.sh                    # Build and deployment scripts
├── docs/                   # Detailed documentation
├── datasets/               # JSON datasets
├── tools/                  # Deployment tools
├── bootstrap-macros/       # Procedural macros
├── telemetry-macros/       # Telemetry macros
└── .github/workflows/      # CI/CD workflows
```

## 📝 Key Documentation Files

### Architecture & Design
- `COMPLETE_SYSTEM_SUMMARY.md` - System overview
- `MANIFEST.md` - Project manifest
- `docs/CANONICAL_STRUCTURE.md` - Canonical structure
- `docs/PROJECT_INDEX.md` - Project index

### Specialized Topics
- `COMPRESSION_CONFORMAL_FIELD.md` - Compression theory
- `PROGRAM_EVOLUTION.md` - Program evolution
- `MEME_MARKETPLACE.md` - Meme marketplace
- `ENUM_LATTICE.md` - Enum lattice theory
- `docs/PROOFCHAIN_SO.md` - Proof chain system
- `docs/COMPLETE_71_DISCOVERY.md` - 71 constant discovery

### Data & Datasets
- `datasets/json-data/` - 99 JSON files
- `datasets/upload-to-hf.sh` - Upload to HuggingFace

## 🛠️ Useful Scripts

### Build Scripts
- `build_all.sh` - Build all binaries
- `build_all_binaries.sh` - Build all binaries (alternative)
- `auto-fix.sh` - Auto-fix compilation errors

### Deployment Scripts
- `deploy-qa.sh` - Deploy QA server ⭐
- `setup-canonical.sh` - Setup canonical structure
- `prove-reproducibility.sh` - Prove reproducible builds

### Analysis Scripts
- `analyze_repos.sh` - Analyze repositories
- `quick-find.sh` - Quick file finder

## 🔍 Finding Things

### By Topic
```bash
# Find deployment files
grep -r "deploy" docs/*.md

# Find build scripts
ls *build*.sh

# Find server files
ls *server*.rs
```

### By Purpose
- **Want to deploy?** → `deploy-qa.sh` or `QUICKSTART.md`
- **Want to build?** → `NIX_BUILD_READY.md` or `cargo build`
- **Want to understand?** → `README.md` then `docs/`
- **Having errors?** → `docs/BUILD_FIXING_GUIDE.md`

## 📊 Statistics

- **Rust binaries**: 220+
- **Documentation files**: 50+
- **Shell scripts**: 100+
- **Total lines of code**: 500K+

## 🎯 Quick Actions

```bash
# Start dev server
./minimal-build-server

# Deploy QA
./deploy-qa.sh

# Build everything
nix build .#meta-introspector-binaries

# View docs
ls docs/*.md
```

## 📚 Documentation Hierarchy

1. **QUICKSTART.md** - Start here
2. **README.md** - Overview
3. **docs/** - Deep dives
4. **Specialized .md files** - Specific topics

## 🆘 Lost?

1. Read `QUICKSTART.md`
2. Check this index
3. Look in `docs/` directory
4. Search: `grep -r "your-topic" docs/`
