#!/usr/bin/env bash
# Execute repository reorganization per architectural plan
# Phase 2: Systematic File Relocation

set -euo pipefail

echo "🏗️  Meta-Introspector Repository Reorganization"
echo "================================================"
echo ""

# Phase 1: Create scaffolding (if not exists)
echo "Phase 1: Creating directory structure..."
mkdir -p src/{telemetry,analysis,tools,core}
mkdir -p scripts/{build,analysis,git,maintenance}
mkdir -p nix/{flakes,expressions}
mkdir -p research/{blockchain,mathematical,experimental}
mkdir -p config/{dev,qa,prod}

# Phase 2: Move Rust source files
echo "Phase 2: Relocating Rust source files..."

# Core application logic
for file in access_pattern_profiler.rs all_commits_collector.rs \
            code_duplication_scanner.rs canonical_data_store.rs \
            telemetry_lib.rs markov_analyzer.rs; do
  [ -f "$file" ] && git mv "$file" src/core/ 2>/dev/null || true
done

# Telemetry
for file in telemetry_*.rs rustc_trace_*.rs; do
  [ -f "$file" ] && git mv "$file" src/telemetry/ 2>/dev/null || true
done

# Analysis tools
for file in *_analyzer.rs *_scanner.rs query_*.rs; do
  [ -f "$file" ] && git mv "$file" src/analysis/ 2>/dev/null || true
done

# Research/experimental
for file in automorphic_orbit_71.rs homotopy_classifier.rs \
            compiler_as_compression.rs moonshine_*.rs; do
  [ -f "$file" ] && git mv "$file" research/mathematical/ 2>/dev/null || true
done

# Move scripts
echo "Phase 2: Relocating scripts..."

# Build scripts
for file in build*.sh nix_builder.sh bootstrap.sh; do
  [ -f "$file" ] && git mv "$file" scripts/build/ 2>/dev/null || true
done

# Git management
for file in add_remotes.sh clone_*.sh *_git_*.sh; do
  [ -f "$file" ] && git mv "$file" scripts/git/ 2>/dev/null || true
done

# Analysis scripts
for file in analyze_*.sh analyze_*.py; do
  [ -f "$file" ] && git mv "$file" scripts/analysis/ 2>/dev/null || true
done

# Maintenance
for file in reorganize.sh fix_*.sh cleanup_*.sh; do
  [ -f "$file" ] && git mv "$file" scripts/maintenance/ 2>/dev/null || true
done

# Move Nix files
echo "Phase 2: Relocating Nix expressions..."
[ -f "flake.nix" ] && git mv flake.nix nix/ 2>/dev/null || true
[ -f "default.nix" ] && git mv default.nix nix/ 2>/dev/null || true
[ -f "shell-cross.nix" ] && git mv shell-cross.nix nix/ 2>/dev/null || true
[ -d "const_71_test" ] && git mv const_71_test nix/flakes/ 2>/dev/null || true

# Move blockchain research
echo "Phase 2: Relocating blockchain research..."
for item in decompile_solana_contracts.sh blockchain_blocks smart_contracts; do
  [ -e "$item" ] && git mv "$item" research/blockchain/ 2>/dev/null || true
done

# Move config files
echo "Phase 2: Relocating configuration..."
[ -d "envs" ] && git mv envs/* config/ 2>/dev/null || true
[ -f "clippy.toml" ] && git mv clippy.toml config/ 2>/dev/null || true

# Consolidate Rust crates
echo "Phase 2: Consolidating Rust crates..."
for dir in libs telemetry_lib bach demos; do
  if [ -d "$dir" ]; then
    # Move contents to src/
    find "$dir" -name "*.rs" -exec git mv {} src/core/ \; 2>/dev/null || true
    # Remove empty dir
    rmdir "$dir" 2>/dev/null || true
  fi
done

# Move specialized tools
for dir in build-logs-to-parquet query_ast_types; do
  [ -d "$dir" ] && git mv "$dir" src/tools/ 2>/dev/null || true
done

echo ""
echo "✅ Phase 2 complete: Files relocated"
echo ""
echo "Next steps:"
echo "  1. Review changes: git status"
echo "  2. Run Phase 3: ./scripts/maintenance/update_paths.sh"
echo "  3. Test: cargo build --all"
echo "  4. Commit: git commit -m 'Phase 2: Systematic file relocation'"
