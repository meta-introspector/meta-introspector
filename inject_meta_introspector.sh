#!/bin/bash
# Auto-discover and inject meta-introspector action into Mike's repos
# Adds Nix flake, labels, and complete DevOps toolchain

set -e

echo "🚀 META-INTROSPECTOR AUTO-INJECTION"
echo ""

# Mike's repos from mike_repos.rs output
REPOS=(
  "/opt/zos-production"
  "/opt/zos-bootstrap"
  "/home/mdupont/zos-qa"
)

inject_repo() {
  local repo_path=$1
  local repo_name=$(basename "$repo_path")
  
  echo "📦 Processing: $repo_name"
  cd "$repo_path" || return
  
  # 1. Auto-discover project type
  echo "  🔍 Auto-discovering..."
  local has_cargo=$([ -f "Cargo.toml" ] && echo "yes" || echo "no")
  local has_nix=$([ -f "flake.nix" ] && echo "yes" || echo "no")
  local has_rust=$(find . -name "*.rs" -type f | head -1 | wc -l)
  
  echo "    Cargo: $has_cargo, Nix: $has_nix, Rust files: $has_rust"
  
  # 2. Add Nix flake if missing
  if [ "$has_nix" = "no" ] && [ "$has_cargo" = "yes" ]; then
    echo "  📝 Creating flake.nix..."
    cat > flake.nix << 'EOF'
{
  description = "Auto-generated Nix flake for meta-introspector";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];
          
          shellHook = ''
            echo "🦀 Rust development environment"
            echo "Meta-introspector enabled"
          '';
        };
        
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "project";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );
}
EOF
    git add flake.nix
  fi
  
  # 3. Add meta-introspector workflow
  echo "  🔧 Adding meta-introspector workflow..."
  mkdir -p .github/workflows
  cat > .github/workflows/meta-introspector.yml << EOF
name: Meta-Introspector Analysis

on:
  push:
    branches: [main, master]
  pull_request:
  workflow_dispatch:
  schedule:
    - cron: '0 0 * * 0'  # Weekly

jobs:
  analyze:
    uses: meta-introspector/meta-introspector/.github/workflows/meta-introspector-action.yml@novel-code-analysis-clean
    with:
      project_name: $repo_name
      upload_to_hf: true
    secrets: inherit
EOF
  git add .github/workflows/meta-introspector.yml
  
  # 4. Add labels
  echo "  🏷️  Adding labels..."
  cat > .github/labels.yml << 'EOF'
- name: meta-introspector
  color: '0366d6'
  description: 'Part of meta-introspector collective'
- name: nix-enabled
  color: '7057ff'
  description: 'Nix flake available'
- name: telemetry-enabled
  color: 'fbca04'
  description: 'Build telemetry collection enabled'
- name: analysis-complete
  color: '0e8a16'
  description: 'Meta-introspector analysis completed'
EOF
  git add .github/labels.yml
  
  # 5. Add README badge
  echo "  📛 Adding README badge..."
  if [ -f "README.md" ]; then
    if ! grep -q "meta-introspector" README.md; then
      sed -i '1i [![Meta-Introspector](https://img.shields.io/badge/meta--introspector-enabled-blue)](https://github.com/meta-introspector/meta-introspector)\n' README.md
      git add README.md
    fi
  fi
  
  # 6. Add .meta-introspector config
  echo "  ⚙️  Adding config..."
  cat > .meta-introspector.toml << EOF
[project]
name = "$repo_name"
type = "rust"
owner = "mike-dupont"

[analysis]
markov = true
ast = true
hir = true
mir = true
binary = true
telemetry = true

[output]
github_artifacts = true
huggingface = true
archive_org = false

[collective]
contribute = true
share_patterns = true
share_duplicates = true
EOF
  git add .meta-introspector.toml
  
  # 7. Commit changes
  echo "  💾 Committing..."
  git commit -m "Add meta-introspector DevOps toolchain

- Auto-generated Nix flake for reproducible builds
- Meta-introspector analysis workflow
- Project labels and badges
- Telemetry and analysis configuration

Part of meta-introspector collective dataset initiative" || true
  
  echo "  ✅ Done: $repo_name"
  echo ""
}

# Process all repos
for repo in "${REPOS[@]}"; do
  if [ -d "$repo" ]; then
    inject_repo "$repo"
  else
    echo "⚠️  Not found: $repo"
  fi
done

echo ""
echo "📊 INJECTION SUMMARY"
echo ""
echo "Injected into:"
for repo in "${REPOS[@]}"; do
  if [ -d "$repo" ]; then
    echo "  ✅ $(basename $repo)"
  fi
done

echo ""
echo "🎯 NEXT STEPS"
echo ""
echo "1. Review changes in each repo"
echo "2. Push to trigger first analysis run"
echo "3. View results in GitHub Actions"
echo "4. Check HuggingFace for collective data"
echo ""
echo "Commands to push all:"
echo ""
for repo in "${REPOS[@]}"; do
  if [ -d "$repo" ]; then
    echo "  cd $repo && git push"
  fi
done
