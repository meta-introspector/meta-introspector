#!/bin/bash
# Apply analysis to all tools in our toolchain

set -e

echo "🔬 Analyzing entire toolchain with parquet telemetry"
echo ""

ANALYSIS_DIR="$HOME/.local/share/toolchain-analysis"
mkdir -p "$ANALYSIS_DIR"

# List of core tools to analyze
TOOLS=(
  "rustc"
  "cargo"
  "nix"
  "git"
  "gcc"
  "clang"
  "python3"
  "bash"
)

echo "📊 Analyzing ${#TOOLS[@]} core tools..."
echo ""

for tool in "${TOOLS[@]}"; do
  echo "🔍 Analyzing: $tool"
  
  # Find tool in nix store
  tool_path=$(which "$tool" 2>/dev/null || echo "")
  
  if [ -z "$tool_path" ]; then
    echo "  ⚠️  Not found, skipping"
    continue
  fi
  
  # Get nix store path
  store_path=$(nix-store -q "$tool_path" 2>/dev/null || echo "")
  
  if [ -z "$store_path" ]; then
    echo "  ⚠️  Not in nix store, skipping"
    continue
  fi
  
  echo "  📦 Store path: $store_path"
  
  # Run analysis
  output_dir="$ANALYSIS_DIR/$tool"
  mkdir -p "$output_dir"
  
  # Source analysis
  if [ -d "$store_path" ]; then
    echo "  🔬 Running markov analysis..."
    cargo run --release --bin markov_resonance_analyzer -- \
      "$store_path" > "$output_dir/markov_symbol_scores.parquet" 2>/dev/null || true
  fi
  
  # Grammar extraction
  echo "  📝 Extracting grammar..."
  cargo run --release --bin nix_store_grammar -- \
    "$store_path" > "$output_dir/nix_store_grammars.parquet" 2>/dev/null || true
  
  # Binary analysis
  if [ -f "$tool_path" ]; then
    echo "  🔍 Binary provenance..."
    cargo run --release --bin byte_provenance_tracker -- \
      "$tool_path" > "$output_dir/byte_provenance.parquet" 2>/dev/null || true
  fi
  
  echo "  ✅ Analysis complete"
  echo ""
done

echo "📊 Toolchain Analysis Summary:"
echo ""
du -sh "$ANALYSIS_DIR"
find "$ANALYSIS_DIR" -name "*.parquet" -exec ls -lh {} \;

echo ""
echo "✅ Toolchain analysis complete!"
echo "📁 Results: $ANALYSIS_DIR"
