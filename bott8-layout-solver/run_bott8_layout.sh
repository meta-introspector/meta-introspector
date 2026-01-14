#!/usr/bin/env bash
# run_bott8_layout.sh
# Run MiniZinc optimization for 8D Bott manifold layout

set -euo pipefail

MODEL="bott8_optimal_layout.mzn"
DATA="bott8_layout_example.dzn"
OUTPUT="bott8_layout_solution.json"

echo "🍄 Running Bott[8] Optimal Layout Solver..."
echo ""
echo "Model: $MODEL"
echo "Data:  $DATA"
echo ""

# Check if minizinc is available
if ! command -v minizinc &> /dev/null; then
    echo "❌ Error: minizinc not found"
    echo "Install with: nix-shell -p minizinc"
    exit 1
fi

# Check if files exist
if [[ ! -f "$MODEL" ]]; then
    echo "❌ Error: Model file not found: $MODEL"
    exit 1
fi

if [[ ! -f "$DATA" ]]; then
    echo "❌ Error: Data file not found: $DATA"
    exit 1
fi

# Run MiniZinc solver
echo "⚙️  Solving..."
echo ""

# Use Gecode solver with time limit
minizinc \
    --solver Gecode \
    --time-limit 60000 \
    --output-mode json \
    --output-objective \
    "$MODEL" "$DATA" \
    > "$OUTPUT" 2>&1 || {
        echo "❌ Solver failed. Check output:"
        cat "$OUTPUT"
        exit 1
    }

echo "✅ Solution found!"
echo ""
echo "Output saved to: $OUTPUT"
echo ""

# Parse and display key results
if command -v jq &> /dev/null; then
    echo "📊 Key Results:"
    echo ""
    jq -r '
        if .output then
            .output.json
        else
            "No JSON output found"
        end
    ' "$OUTPUT" || {
        echo "Raw output:"
        cat "$OUTPUT"
    }
else
    echo "Install jq for formatted output: nix-shell -p jq"
    echo ""
    echo "Raw output:"
    cat "$OUTPUT"
fi

echo ""
echo "🧙♂️ Bott[8] layout optimization complete!"
