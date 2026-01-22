{
  description = "Self-analysis: Run analysis binaries on meta-introspector itself";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        name = "self-analysis";
        src = ../..;
        cargoLock.lockFile = ../../Cargo.lock;
        
        buildPhase = ''
          mkdir -p $out/analysis
          
          echo "Running analysis binaries on meta-introspector..."
          
          # Analysis binaries that take git repo input
          cargo run --release --bin analyze_cargo_deps -- . > $out/analysis/cargo_deps.json 2>&1 || true
          cargo run --release --bin analyze_workspaces -- . > $out/analysis/workspaces.json 2>&1 || true
          cargo run --release --bin cascading-repo-analyzer -- . > $out/analysis/cascading.json 2>&1 || true
          
          # Analysis binaries that take parquet input (if exists)
          if ls *.parquet 2>/dev/null; then
            cargo run --release --bin analyze_char_transitions -- *.parquet > $out/analysis/char_transitions.txt 2>&1 || true
            cargo run --release --bin analyze_transitions -- *.parquet > $out/analysis/transitions.txt 2>&1 || true
          fi
          
          # Git analysis
          cargo run --release --bin all_commits_collector -- . > $out/analysis/commits.json 2>&1 || true
          
          # Binary classification (run on ourselves)
          python3 scripts/analysis/classify_binaries.py
          cp binary_classification.json $out/analysis/
          
          echo "Analysis complete. Results in $out/analysis/"
        '';
        
        installPhase = ''
          echo "Self-analysis results stored in $out/analysis/"
        '';
      };
      
      # Classify unknowns by similarity
      packages.${system}.classify-unknowns = pkgs.writeShellScriptBin "classify-unknowns" ''
        #!/usr/bin/env bash
        # Use analysis results to classify unknown binaries by similarity
        
        ANALYSIS_DIR="$1"
        
        if [ ! -d "$ANALYSIS_DIR" ]; then
          echo "Usage: classify-unknowns <analysis-dir>"
          exit 1
        fi
        
        ${pkgs.python3}/bin/python3 << 'PYTHON'
        import json
        import sys
        from pathlib import Path
        from collections import defaultdict
        
        # Load binary classification
        with open('binary_classification.json') as f:
            data = json.load(f)
        
        # Get unknowns
        unknowns = [b for b in data.get('categories', {}).get('unknown', [])]
        
        print(f"Classifying {len(unknowns)} unknown binaries by similarity...")
        
        # Load analysis results for similarity matching
        known_patterns = defaultdict(list)
        for category, binaries in data.get('categories', {}).items():
            if category == 'unknown':
                continue
            for binary in binaries:
                # Extract patterns
                inputs = tuple(sorted(binary.get('inputs', [])))
                outputs = tuple(sorted(binary.get('outputs', [])))
                known_patterns[(inputs, outputs)].append(category)
        
        # Classify unknowns by matching I/O patterns
        reclassified = defaultdict(list)
        for unknown in unknowns:
            inputs = tuple(sorted(unknown.get('inputs', [])))
            outputs = tuple(sorted(unknown.get('outputs', [])))
            
            # Find matching pattern
            if (inputs, outputs) in known_patterns:
                categories = known_patterns[(inputs, outputs)]
                # Use most common category
                category = max(set(categories), key=categories.count)
                reclassified[category].append(unknown['name'])
            else:
                reclassified['still_unknown'].append(unknown['name'])
        
        # Output results
        print("\nReclassification results:")
        for category, names in sorted(reclassified.items()):
            print(f"\n{category}: {len(names)}")
            for name in sorted(names)[:5]:
                print(f"  - {name}")
            if len(names) > 5:
                print(f"  ... and {len(names) - 5} more")
        
        # Save results
        with open('reclassified_binaries.json', 'w') as f:
            json.dump(dict(reclassified), f, indent=2)
        
        print(f"\nSaved to: reclassified_binaries.json")
        PYTHON
      '';
    };
}
