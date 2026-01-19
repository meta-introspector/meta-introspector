#!/bin/bash
# Process all meta-introspector repos using existing local clones

# Don't exit on errors - continue processing
set +e

BRANCH_NAME="nix-analysis-2026-01"
ANALYSIS_DIR="$HOME/.local/share/repo-analysis"
mkdir -p "$ANALYSIS_DIR"

echo "🔬 Processing meta-introspector repos from disk"
echo ""

# Find all meta-introspector repos on disk
grep "meta-introspector" /mnt/data1/meta-introspector/gitdirs.txt | sed 's|/.git$||' > "$ANALYSIS_DIR/meta-introspector-repos-on-disk.txt"

total=$(wc -l < "$ANALYSIS_DIR/meta-introspector-repos-on-disk.txt")
count=0
has_flake=0
broken_flake=0
no_flake=0

echo "Found $total meta-introspector repos on disk"
echo ""

while read repo_path; do
  ((count++))
  
  repo_name=$(basename "$repo_path")
  
  if [ ! -d "$repo_path" ]; then
    echo "[$count/$total] ⏭️  $repo_name - path not found"
    continue
  fi
  
  echo "[$count/$total] 🔍 $repo_name"
  echo "  📁 $repo_path"
  
  cd "$repo_path"
  
  # Check for existing flake.nix
  if [ -f "flake.nix" ]; then
    echo "  ✓ Has flake.nix"
    
    # Test if flake works
    if nix flake check 2>/dev/null; then
      echo "  ✅ Flake works - adding analysis"
      ((has_flake++))
      
      # Create analysis branch
      git checkout -b "$BRANCH_NAME" 2>/dev/null || git checkout "$BRANCH_NAME"
      
      # Add analysis wrapper
      cat > flake-analyzed.nix <<'EOF'
{
  inputs = {
    original.url = "path:.";
    meta-introspector.url = "github:meta-introspector/meta-introspector";
  };
  
  outputs = { self, original, meta-introspector }:
    let
      analyzers = meta-introspector.lib.analyzers;
    in {
      packages.x86_64-linux = {
        default = original.packages.x86_64-linux.default or original.defaultPackage.x86_64-linux;
        analyzed = analyzers.withFullAnalysis self.packages.x86_64-linux.default;
      };
    };
}
EOF
      
    else
      echo "  ⚠️  Flake broken - backing up and replacing"
      ((broken_flake++))
      
      git checkout -b "$BRANCH_NAME" 2>/dev/null || git checkout "$BRANCH_NAME"
      mv flake.nix flake.nix.broken
      
      # Add standard flake based on project type
      if [ -f "Cargo.toml" ]; then
        cp /mnt/data1/meta-introspector/templates/rust-flake.nix flake.nix
      elif [ -f "package.json" ]; then
        cp /mnt/data1/meta-introspector/templates/node-flake.nix flake.nix
      elif [ -f "setup.py" ] || [ -f "pyproject.toml" ]; then
        cp /mnt/data1/meta-introspector/templates/python-flake.nix flake.nix
      else
        cp /mnt/data1/meta-introspector/templates/generic-flake.nix flake.nix
      fi
    fi
    
  else
    echo "  ➕ No flake - adding standard one"
    ((no_flake++))
    
    git checkout -b "$BRANCH_NAME" 2>/dev/null || git checkout "$BRANCH_NAME"
    
    # Detect project type and add appropriate flake
    if [ -f "Cargo.toml" ]; then
      echo "  📦 Rust project"
      cp /mnt/data1/meta-introspector/templates/rust-flake.nix flake.nix
    elif [ -f "package.json" ]; then
      echo "  📦 Node project"
      cp /mnt/data1/meta-introspector/templates/node-flake.nix flake.nix
    elif [ -f "setup.py" ] || [ -f "pyproject.toml" ]; then
      echo "  📦 Python project"
      cp /mnt/data1/meta-introspector/templates/python-flake.nix flake.nix
    else
      echo "  📦 Generic project"
      cp /mnt/data1/meta-introspector/templates/generic-flake.nix flake.nix
    fi
  fi
  
  # Commit changes
  git add flake.nix flake-analyzed.nix 2>/dev/null || true
  git commit -m "Add Nix analysis on branch $BRANCH_NAME" 2>/dev/null || true
  
  # Queue for analysis
  echo "$repo_path" >> "$ANALYSIS_DIR/analysis-queue.txt"
  
  if [ $((count % 10)) -eq 0 ]; then
    echo ""
    echo "Progress: $count/$total"
    echo "  Has flake: $has_flake"
    echo "  Broken flake: $broken_flake"
    echo "  No flake: $no_flake"
    echo ""
  fi
  
done < "$ANALYSIS_DIR/meta-introspector-repos.txt"

echo ""
echo "✅ Processing complete!"
echo ""
echo "Summary:"
echo "  Total: $total"
echo "  Has working flake: $has_flake"
echo "  Broken flake (fixed): $broken_flake"
echo "  No flake (added): $no_flake"
echo ""
echo "Analysis queue: $ANALYSIS_DIR/analysis-queue.txt"
echo "Branch name: $BRANCH_NAME"
