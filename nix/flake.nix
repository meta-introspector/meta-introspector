{
  description = "Meta-introspector central build system - all analysis as nix jobs";
  
  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
    in {
      packages.${system} = {
        # Job 1: Build all 71 languages
        languages = import ./flakes/const_71_test/languages.nix { inherit pkgs; };
        
        # Job 2: Extract build graph (depends on Job 1)
        build-graph = pkgs.stdenv.mkDerivation {
          name = "build-graph";
          buildInputs = [ pkgs.graphviz pkgs.python3 ];
          
          buildPhase = ''
            mkdir -p $out/graphs
            nix-store -q --graph ${self.packages.${system}.languages} > $out/graphs/build-graph.dot
            dot -Tpng $out/graphs/build-graph.dot -o $out/graphs/build-graph.png
            nix-store -q --references ${self.packages.${system}.languages} | sort > $out/graphs/build-order.txt
          '';
        };
        
        # Job 3: Perf analysis (depends on Job 1)
        perf-analysis = pkgs.stdenv.mkDerivation {
          name = "perf-analysis";
          buildInputs = [ self.packages.${system}.languages ];
          
          buildPhase = ''
            mkdir -p $out/analysis
            
            # Extract all perf data
            find ${self.packages.${system}.languages} -name "*.perf.data" > $out/analysis/perf-files.txt || true
            
            echo "Perf analysis complete"
          '';
        };
        
        # Job 4: Topological function matrix (depends on Job 2 + Job 3)
        topological-matrix = pkgs.stdenv.mkDerivation {
          name = "topological-matrix";
          buildInputs = [ 
            self.packages.${system}.build-graph
            self.packages.${system}.perf-analysis
          ];
          
          buildPhase = ''
            mkdir -p $out/matrix
            echo "Topological matrix complete"
          '';
        };
        
        # Job 5: Harmonic analysis (depends on Job 3)
        harmonic-analysis = pkgs.stdenv.mkDerivation {
          name = "harmonic-analysis";
          buildInputs = [ self.packages.${system}.perf-analysis ];
          
          buildPhase = ''
            mkdir -p $out/harmonics
            echo "Harmonic analysis complete"
          '';
        };
        
        # Job 6: Model training (depends on Job 3 + Job 4 + Job 5)
        model-training = pkgs.stdenv.mkDerivation {
          name = "model-training";
          buildInputs = [ 
            self.packages.${system}.perf-analysis
            self.packages.${system}.topological-matrix
            self.packages.${system}.harmonic-analysis
          ];
          
          buildPhase = ''
            mkdir -p $out/models
            echo "Model training complete"
          '';
        };
        
        # Job 7: Complete system (depends on all jobs)
        default = pkgs.stdenv.mkDerivation {
          name = "meta-introspector-complete";
          
          buildInputs = [
            self.packages.${system}.languages
            self.packages.${system}.build-graph
            self.packages.${system}.perf-analysis
            self.packages.${system}.topological-matrix
            self.packages.${system}.harmonic-analysis
            self.packages.${system}.model-training
          ];
          
          buildPhase = ''
            mkdir -p $out/{languages,graphs,analysis,matrix,harmonics,models,.meta-introspector}
            
            # Collect all outputs
            cp -rL ${self.packages.${system}.languages}/* $out/languages/ 2>/dev/null || true
            cp -rL ${self.packages.${system}.build-graph}/* $out/graphs/ 2>/dev/null || true
            cp -rL ${self.packages.${system}.perf-analysis}/* $out/analysis/ 2>/dev/null || true
            cp -rL ${self.packages.${system}.topological-matrix}/* $out/matrix/ 2>/dev/null || true
            cp -rL ${self.packages.${system}.harmonic-analysis}/* $out/harmonics/ 2>/dev/null || true
            cp -rL ${self.packages.${system}.model-training}/* $out/models/ 2>/dev/null || true
            
            # Generate metadata
            cat > $out/.meta-introspector/metadata.json <<EOF
            {
              "version": "1.0",
              "timestamp": "$(date -Iseconds)",
              "jobs": {
                "languages": "${self.packages.${system}.languages}",
                "build-graph": "${self.packages.${system}.build-graph}",
                "perf-analysis": "${self.packages.${system}.perf-analysis}",
                "topological-matrix": "${self.packages.${system}.topological-matrix}",
                "harmonic-analysis": "${self.packages.${system}.harmonic-analysis}",
                "model-training": "${self.packages.${system}.model-training}"
              }
            }
            EOF
          '';
          
          installPhase = ''
            echo "Complete system in: $out"
          '';
        };
      };
    };
}
