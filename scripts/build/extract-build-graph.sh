#!/usr/bin/env bash
# Extract nix build graph as first ordering
# The dependency graph defines our topological order

set -euo pipefail

echo "📊 Extracting Nix Build Graph (First Ordering)"
echo "=============================================="
echo ""

# Build if needed
if [ ! -L "result" ]; then
    echo "Building system first..."
    ./bootstrap
    echo ""
fi

# Extract graph from result
echo "Extracting dependency graph..."
nix-store -q --graph result/ > build-graph.dot

# Count nodes and edges
NODES=$(grep -c "label=" build-graph.dot || echo "0")
EDGES=$(grep -c "\->" build-graph.dot || echo "0")

echo "  Nodes (derivations): $NODES"
echo "  Edges (dependencies): $EDGES"
echo ""

# Generate visualizations
echo "Generating visualizations..."

# Full graph
dot -Tpng build-graph.dot -o build-graph.png
echo "  ✓ build-graph.png (full graph)"

# Simplified graph (only our packages)
grep -E "meta-introspector|const_71" build-graph.dot > build-graph-simple.dot
echo "digraph {" > temp.dot
cat build-graph-simple.dot >> temp.dot
echo "}" >> temp.dot
mv temp.dot build-graph-simple.dot
dot -Tpng build-graph-simple.dot -o build-graph-simple.png 2>/dev/null || true
echo "  ✓ build-graph-simple.png (our packages only)"

# Extract topological order
echo ""
echo "Computing topological order..."
nix-store -q --references result/ | sort > build-order.txt
LEVELS=$(wc -l < build-order.txt)
echo "  ✓ build-order.txt ($LEVELS derivations)"

# Analyze levels
echo ""
echo "Analyzing dependency levels..."

cat > analyze-levels.py <<'EOF'
#!/usr/bin/env python3
import sys
import subprocess

# Get all derivations
result = subprocess.run(['nix-store', '-q', '--references', 'result/'], 
                       capture_output=True, text=True)
derivations = result.stdout.strip().split('\n')

# Build dependency map
deps = {}
for drv in derivations:
    refs = subprocess.run(['nix-store', '-q', '--references', drv],
                         capture_output=True, text=True)
    deps[drv] = set(refs.stdout.strip().split('\n')) - {drv}

# Compute levels (topological sort with levels)
levels = {}
remaining = set(derivations)

level = 0
while remaining:
    # Find nodes with no dependencies in remaining set
    current_level = {drv for drv in remaining 
                     if not (deps.get(drv, set()) & remaining)}
    
    if not current_level:
        break  # Cycle detected
    
    for drv in current_level:
        levels[drv] = level
        remaining.remove(drv)
    
    level += 1

# Print levels
print(f"Total levels: {level}")
print()

for i in range(level):
    level_drvs = [drv for drv, l in levels.items() if l == i]
    print(f"Level {i}: {len(level_drvs)} derivations")
    for drv in sorted(level_drvs)[:5]:  # Show first 5
        name = drv.split('/')[-1][:50]
        print(f"  - {name}")
    if len(level_drvs) > 5:
        print(f"  ... and {len(level_drvs) - 5} more")
    print()
EOF

chmod +x analyze-levels.py
python3 analyze-levels.py > build-levels.txt
cat build-levels.txt

# Create JSON representation
echo ""
echo "Creating JSON representation..."

cat > build-graph.json <<EOF
{
  "nodes": $NODES,
  "edges": $EDGES,
  "levels": $(grep "Total levels:" build-levels.txt | cut -d: -f2 | tr -d ' '),
  "graph_file": "build-graph.dot",
  "order_file": "build-order.txt",
  "levels_file": "build-levels.txt"
}
EOF

echo "  ✓ build-graph.json"

echo ""
echo "✅ Build graph extracted!"
echo ""
echo "Files created:"
echo "  build-graph.dot         - Full dependency graph (GraphViz)"
echo "  build-graph.png         - Full graph visualization"
echo "  build-graph-simple.png  - Simplified (our packages only)"
echo "  build-order.txt         - Topological order"
echo "  build-levels.txt        - Dependency levels"
echo "  build-graph.json        - Graph metadata"
echo ""
echo "First ordering: build-order.txt"
echo "  This defines the topological order of all derivations"
echo "  Core packages are at the bottom (level 0)"
echo "  Complete system is at the top (level N)"
