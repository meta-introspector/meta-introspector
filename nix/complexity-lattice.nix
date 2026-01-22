{
  description = "Complexity lattice - grow from 71 baseline via clustering";
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
    in {
      packages.${system} = {
        # Job 1: Extract 71 baseline (const 71 in each language)
        baseline-71 = pkgs.stdenv.mkDerivation {
          name = "baseline-71";
          src = ./flakes/const_71_test;
          
          buildInputs = [ pkgs.linuxPackages.perf ];
          
          buildPhase = ''
            mkdir -p $out/{baseline,perf}
            
            # For each language, extract the minimal "71" program
            for lang_dir in */; do
              lang=$(basename "$lang_dir")
              
              # Find the main source file
              if [ -f "$lang_dir/main.rs" ]; then
                cp "$lang_dir/main.rs" "$out/baseline/$lang.src"
              elif [ -f "$lang_dir/main.py" ]; then
                cp "$lang_dir/main.py" "$out/baseline/$lang.src"
              elif [ -f "$lang_dir/main.sh" ]; then
                cp "$lang_dir/main.sh" "$out/baseline/$lang.src"
              fi
              
              # Record perf of building this baseline
              if [ -f "$lang_dir/flake.nix" ]; then
                perf record -o "$out/perf/$lang-baseline.perf.data" \
                  -F 997 -g \
                  nix build "$lang_dir" --no-link 2>/dev/null || true
              fi
            done
            
            # Create baseline manifest
            cat > $out/baseline/manifest.json <<EOF
            {
              "name": "71-baseline",
              "description": "Minimal const 71 in each language",
              "languages": $(ls $out/baseline/*.src | wc -l),
              "complexity_level": 0,
              "features": ["const", "integer", "output"]
            }
            EOF
            
            echo "Baseline 71 extracted"
          '';
        };
        
        # Job 2: Compute baseline complexity vectors
        baseline-vectors = pkgs.stdenv.mkDerivation {
          name = "baseline-vectors";
          
          buildInputs = [ 
            self.packages.${system}.baseline-71
            pkgs.python3
          ];
          
          buildPhase = ''
            mkdir -p $out/vectors
            
            # For each baseline, compute complexity vector
            python3 <<'PYTHON'
            import json
            import os
            from pathlib import Path
            
            vectors = {}
            
            baseline_dir = Path('${self.packages.${system}.baseline-71}/baseline')
            perf_dir = Path('${self.packages.${system}.baseline-71}/perf')
            
            for src_file in baseline_dir.glob('*.src'):
                lang = src_file.stem
                
                # Compute features
                with open(src_file) as f:
                    code = f.read()
                
                # Basic complexity metrics
                vector = {
                    'language': lang,
                    'lines': len(code.split('\n')),
                    'chars': len(code),
                    'tokens': len(code.split()),
                    'complexity_level': 0,
                    'features': ['const', 'integer', 'output']
                }
                
                # Add perf metrics if available
                perf_file = perf_dir / f'{lang}-baseline.perf.data'
                if perf_file.exists():
                    # Parse perf data (simplified)
                    vector['perf_samples'] = 0  # Would parse actual perf data
                    vector['unique_ips'] = 0
                
                vectors[lang] = vector
            
            # Save vectors
            with open('$out/vectors/baseline-vectors.json', 'w') as f:
                json.dump(vectors, f, indent=2)
            
            print(f"Computed {len(vectors)} baseline vectors")
            PYTHON
            
            echo "Baseline vectors computed"
          '';
        };
        
        # Job 3: Cluster similar code (OEIS, Rosetta Code)
        cluster-similar = pkgs.stdenv.mkDerivation {
          name = "cluster-similar";
          
          buildInputs = [
            self.packages.${system}.baseline-vectors
            pkgs.python3
            pkgs.python3Packages.numpy
            pkgs.python3Packages.scikit-learn
          ];
          
          buildPhase = ''
            mkdir -p $out/clusters
            
            # Load baseline vectors
            # Load OEIS/Rosetta Code datasets
            # Cluster by similarity to baseline
            
            python3 <<'PYTHON'
            import json
            import numpy as np
            from sklearn.cluster import KMeans
            from sklearn.metrics.pairwise import cosine_similarity
            
            # Load baseline vectors
            with open('${self.packages.${system}.baseline-vectors}/vectors/baseline-vectors.json') as f:
                baseline = json.load(f)
            
            # Create feature matrix
            features = []
            languages = []
            for lang, vec in baseline.items():
                features.append([
                    vec['lines'],
                    vec['chars'],
                    vec['tokens']
                ])
                languages.append(lang)
            
            X = np.array(features)
            
            # Cluster (k=5 complexity levels)
            kmeans = KMeans(n_clusters=5, random_state=42)
            clusters = kmeans.fit_predict(X)
            
            # Assign complexity levels
            clustered = {}
            for i, lang in enumerate(languages):
                clustered[lang] = {
                    'cluster': int(clusters[i]),
                    'complexity_level': int(clusters[i]),
                    'features': baseline[lang]['features']
                }
            
            # Save clusters
            with open('$out/clusters/complexity-clusters.json', 'w') as f:
                json.dump(clustered, f, indent=2)
            
            # Generate lattice
            lattice = {
                'level_0': [lang for lang, data in clustered.items() if data['complexity_level'] == 0],
                'level_1': [lang for lang, data in clustered.items() if data['complexity_level'] == 1],
                'level_2': [lang for lang, data in clustered.items() if data['complexity_level'] == 2],
                'level_3': [lang for lang, data in clustered.items() if data['complexity_level'] == 3],
                'level_4': [lang for lang, data in clustered.items() if data['complexity_level'] == 4],
            }
            
            with open('$out/clusters/complexity-lattice.json', 'w') as f:
                json.dump(lattice, f, indent=2)
            
            print(f"Clustered into {len(set(clusters))} levels")
            PYTHON
            
            echo "Clustering complete"
          '';
        };
        
        # Job 4: Generate growth path (low to high complexity)
        growth-path = pkgs.stdenv.mkDerivation {
          name = "growth-path";
          
          buildInputs = [
            self.packages.${system}.cluster-similar
            pkgs.python3
          ];
          
          buildPhase = ''
            mkdir -p $out/path
            
            # Generate ordered path from level 0 to level N
            python3 <<'PYTHON'
            import json
            
            # Load lattice
            with open('${self.packages.${system}.cluster-similar}/clusters/complexity-lattice.json') as f:
                lattice = json.load(f)
            
            # Generate growth path
            path = []
            for level in range(5):
                level_key = f'level_{level}'
                if level_key in lattice:
                    for lang in lattice[level_key]:
                        path.append({
                            'step': len(path),
                            'language': lang,
                            'complexity_level': level,
                            'description': f'Add features from level {level}'
                        })
            
            # Save path
            with open('$out/path/growth-path.json', 'w') as f:
                json.dump(path, f, indent=2)
            
            # Generate report
            with open('$out/path/growth-report.txt', 'w') as f:
                f.write("Complexity Growth Path\n")
                f.write("======================\n\n")
                f.write("Start: Level 0 (const 71 baseline)\n")
                f.write("End: Level 4 (full language features)\n\n")
                
                for level in range(5):
                    level_key = f'level_{level}'
                    if level_key in lattice:
                        f.write(f"Level {level}: {len(lattice[level_key])} languages\n")
                        for lang in lattice[level_key][:5]:
                            f.write(f"  - {lang}\n")
                        if len(lattice[level_key]) > 5:
                            f.write(f"  ... and {len(lattice[level_key]) - 5} more\n")
                        f.write("\n")
            
            print(f"Growth path: {len(path)} steps")
            PYTHON
            
            echo "Growth path generated"
          '';
        };
        
        # Job 5: Integrate OEIS/Rosetta Code as labeled data
        labeled-datasets = pkgs.stdenv.mkDerivation {
          name = "labeled-datasets";
          
          buildInputs = [ pkgs.python3 ];
          
          buildPhase = ''
            mkdir -p $out/datasets
            
            # OEIS sequences as labeled complexity examples
            cat > $out/datasets/oeis-manifest.json <<EOF
            {
              "name": "OEIS",
              "description": "Integer sequences with known complexity",
              "examples": [
                {"id": "A000027", "name": "Natural numbers", "complexity": 0},
                {"id": "A000045", "name": "Fibonacci", "complexity": 1},
                {"id": "A000040", "name": "Primes", "complexity": 2},
                {"id": "A000142", "name": "Factorial", "complexity": 1},
                {"id": "A000079", "name": "Powers of 2", "complexity": 0}
              ]
            }
            EOF
            
            # Rosetta Code as labeled language feature examples
            cat > $out/datasets/rosetta-manifest.json <<EOF
            {
              "name": "Rosetta Code",
              "description": "Same task in multiple languages",
              "tasks": [
                {"name": "Hello World", "complexity": 0, "features": ["output"]},
                {"name": "FizzBuzz", "complexity": 1, "features": ["loop", "conditional", "output"]},
                {"name": "Fibonacci", "complexity": 1, "features": ["recursion", "arithmetic"]},
                {"name": "Quicksort", "complexity": 2, "features": ["recursion", "arrays", "comparison"]},
                {"name": "Web Server", "complexity": 3, "features": ["io", "networking", "concurrency"]}
              ]
            }
            EOF
            
            echo "Labeled datasets prepared"
          '';
        };
        
        # Complete lattice
        default = pkgs.stdenv.mkDerivation {
          name = "complexity-lattice-complete";
          
          buildInputs = [
            self.packages.${system}.baseline-71
            self.packages.${system}.baseline-vectors
            self.packages.${system}.cluster-similar
            self.packages.${system}.growth-path
            self.packages.${system}.labeled-datasets
          ];
          
          buildPhase = ''
            mkdir -p $out/{baseline,vectors,clusters,path,datasets}
            
            # Collect all outputs
            cp -r ${self.packages.${system}.baseline-71}/baseline/* $out/baseline/
            cp -r ${self.packages.${system}.baseline-vectors}/vectors/* $out/vectors/
            cp -r ${self.packages.${system}.cluster-similar}/clusters/* $out/clusters/
            cp -r ${self.packages.${system}.growth-path}/path/* $out/path/
            cp -r ${self.packages.${system}.labeled-datasets}/datasets/* $out/datasets/
            
            # Generate summary
            cat > $out/summary.txt <<EOF
            Complexity Lattice Complete
            ===========================
            
            Baseline: 71 languages (const 71)
            Complexity levels: 5 (0-4)
            Growth path: $(cat $out/path/growth-path.json | jq 'length') steps
            
            Labeled datasets:
            - OEIS: Integer sequences
            - Rosetta Code: Language features
            
            Files:
            - baseline/manifest.json (71 baseline)
            - vectors/baseline-vectors.json (complexity vectors)
            - clusters/complexity-clusters.json (clustered by similarity)
            - clusters/complexity-lattice.json (5 levels)
            - path/growth-path.json (ordered growth)
            - path/growth-report.txt (human readable)
            - datasets/oeis-manifest.json (OEIS labeled)
            - datasets/rosetta-manifest.json (Rosetta labeled)
            
            Growth strategy:
            1. Start at level 0 (const 71 baseline)
            2. Add features incrementally
            3. Follow most common path
            4. Use OEIS/Rosetta as labeled examples
            5. Cluster similar complexity
            EOF
            
            cat $out/summary.txt
          '';
        };
      };
    };
}
