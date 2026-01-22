{
  description = "Meta-introspector complete system build";
  
  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Import all 71 language tests
      const71 = import ./const_71_test { inherit pkgs; };
      
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "meta-introspector-complete";
        
        buildInputs = [ pkgs.jq pkgs.graphviz pkgs.python3 ];
        
        buildPhase = ''
          mkdir -p $out/{bin,perf,logs,graphs,.meta-introspector}
          
          # Collect all 71 language outputs
          ${pkgs.lib.concatMapStringsSep "\n" (lang: ''
            if [ -d "${const71.${lang}}" ]; then
              cp -r ${const71.${lang}}/* $out/ 2>/dev/null || true
            fi
          '') (builtins.attrNames const71)}
          
          # Collect all perf data
          find ${const71} -name "*.perf.data" -exec cp {} $out/perf/ \; 2>/dev/null || true
          
          # Extract build graph (first ordering)
          echo "Extracting build graph..."
          nix-store -q --graph $out > $out/graphs/build-graph.dot
          
          # Generate visualization
          dot -Tpng $out/graphs/build-graph.dot -o $out/graphs/build-graph.png
          
          # Extract topological order
          nix-store -q --references $out | sort > $out/graphs/build-order.txt
          
          # Compute dependency levels
          cat > $out/graphs/analyze-levels.py <<'EOF'
#!/usr/bin/env python3
import subprocess
import json

# Get all derivations
derivations = open('$out/graphs/build-order.txt').read().strip().split('\n')

# Build dependency map
deps = {}
for drv in derivations:
    result = subprocess.run(['nix-store', '-q', '--references', drv],
                           capture_output=True, text=True)
    deps[drv] = set(result.stdout.strip().split('\n')) - {drv}

# Compute levels
levels = {}
remaining = set(derivations)
level = 0

while remaining:
    current_level = {drv for drv in remaining 
                     if not (deps.get(drv, set()) & remaining)}
    if not current_level:
        break
    
    for drv in current_level:
        levels[drv] = level
        remaining.remove(drv)
    
    level += 1

# Output
print(f"Total levels: {level}")
for i in range(level):
    level_drvs = [drv for drv, l in levels.items() if l == i]
    print(f"Level {i}: {len(level_drvs)} derivations")
    for drv in sorted(level_drvs)[:3]:
        print(f"  - {drv.split('/')[-1][:60]}")

# Save JSON
with open('$out/graphs/build-levels.json', 'w') as f:
    json.dump({'levels': level, 'derivations': levels}, f, indent=2)
EOF
          
          chmod +x $out/graphs/analyze-levels.py
          cd $out/graphs && python3 analyze-levels.py > build-levels.txt
          
          # Generate metadata
          cat > $out/.meta-introspector/metadata.json <<EOF
          {
            "version": "1.0",
            "timestamp": "$(date -Iseconds)",
            "commit": "${self.rev or "dirty"}",
            "languages": ${builtins.length (builtins.attrNames const71)},
            "perf_traces": $(find $out/perf -name "*.perf.data" | wc -l),
            "store_path": "$out",
            "graph": {
              "nodes": $(grep -c "label=" $out/graphs/build-graph.dot || echo 0),
              "edges": $(grep -c "\->" $out/graphs/build-graph.dot || echo 0),
              "levels": $(grep "Total levels:" $out/graphs/build-levels.txt | cut -d: -f2 | tr -d ' ')
            }
          }
          EOF
          
          # Create build log
          cat > $out/logs/build.log <<EOF
          Meta-introspector complete build
          =================================
          Timestamp: $(date -Iseconds)
          Commit: ${self.rev or "dirty"}
          Languages: ${builtins.length (builtins.attrNames const71)}
          Perf traces: $(find $out/perf -name "*.perf.data" | wc -l)
          
          Build Graph (First Ordering):
          - Nodes: $(grep -c "label=" $out/graphs/build-graph.dot || echo 0)
          - Edges: $(grep -c "\->" $out/graphs/build-graph.dot || echo 0)
          - Levels: $(grep "Total levels:" $out/graphs/build-levels.txt | cut -d: -f2 | tr -d ' ')
          
          Files:
          - graphs/build-graph.dot (GraphViz)
          - graphs/build-graph.png (visualization)
          - graphs/build-order.txt (topological order)
          - graphs/build-levels.txt (dependency levels)
          - graphs/build-levels.json (JSON format)
          EOF
        '';
        
        installPhase = ''
          echo "Complete system with build graph in: $out"
          echo "First ordering: $out/graphs/build-order.txt"
        '';
      };
    };
}
