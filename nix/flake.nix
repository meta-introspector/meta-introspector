{
  description = "Meta-introspector central build system - all analysis as nix jobs";
  
  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
    in {
      # Job 1: Build all 71 languages
      packages.${system}.languages = import ./const_71_test/languages.nix { inherit pkgs; };
      
      # Job 2: Extract build graph (depends on Job 1)
      packages.${system}.build-graph = pkgs.stdenv.mkDerivation {
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
      packages.${system}.perf-analysis = pkgs.stdenv.mkDerivation {
        name = "perf-analysis";
        buildInputs = [ self.packages.${system}.languages ];
        
        buildPhase = ''
          mkdir -p $out/analysis
          
          # Extract all perf data
          find ${self.packages.${system}.languages} -name "*.perf.data" > $out/analysis/perf-files.txt
          
          # Run perf-complexity on all traces
          for perf_file in $(cat $out/analysis/perf-files.txt); do
            perf-complexity --perf-data "$perf_file" --output $out/analysis/
          done
        '';
      };
      
      # Job 4: Topological function matrix (depends on Job 2 + Job 3)
      packages.${system}.topological-matrix = pkgs.stdenv.mkDerivation {
        name = "topological-matrix";
        buildInputs = [ 
          self.packages.${system}.build-graph
          self.packages.${system}.perf-analysis
        ];
        
        buildPhase = ''
          mkdir -p $out/matrix
          
          # Build function matrix from build order + perf data
          topological-function-matrix \
            --build-order ${self.packages.${system}.build-graph}/graphs/build-order.txt \
            --perf-analysis ${self.packages.${system}.perf-analysis}/analysis/ \
            --output $out/matrix/
        '';
      };
      
      # Job 5: Harmonic analysis (depends on Job 3)
      packages.${system}.harmonic-analysis = pkgs.stdenv.mkDerivation {
        name = "harmonic-analysis";
        buildInputs = [ self.packages.${system}.perf-analysis ];
        
        buildPhase = ''
          mkdir -p $out/harmonics
          
          # Run harmonic analyzer on all perf traces
          for perf_file in $(cat ${self.packages.${system}.perf-analysis}/analysis/perf-files.txt); do
            harmonic-analyzer "$perf_file" > $out/harmonics/$(basename "$perf_file").harmonics
          done
        '';
      };
      
      # Job 6: Model training (depends on Job 3 + Job 4 + Job 5)
      packages.${system}.model-training = pkgs.stdenv.mkDerivation {
        name = "model-training";
        buildInputs = [ 
          self.packages.${system}.perf-analysis
          self.packages.${system}.topological-matrix
          self.packages.${system}.harmonic-analysis
        ];
        
        buildPhase = ''
          mkdir -p $out/models
          
          # Train mes-transformer on all analysis
          mes-transformer-train \
            --perf-data ${self.packages.${system}.perf-analysis}/analysis/ \
            --topology ${self.packages.${system}.topological-matrix}/matrix/ \
            --harmonics ${self.packages.${system}.harmonic-analysis}/harmonics/ \
            --output $out/models/checkpoint.bin
        '';
      };
      
      # Job 7: Complete system (depends on all jobs)
      packages.${system}.default = pkgs.stdenv.mkDerivation {
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
          mkdir -p $out/{languages,graphs,analysis,matrix,harmonics,models,logs}
          
          # Collect all outputs
          cp -r ${self.packages.${system}.languages}/* $out/languages/
          cp -r ${self.packages.${system}.build-graph}/* $out/graphs/
          cp -r ${self.packages.${system}.perf-analysis}/* $out/analysis/
          cp -r ${self.packages.${system}.topological-matrix}/* $out/matrix/
          cp -r ${self.packages.${system}.harmonic-analysis}/* $out/harmonics/
          cp -r ${self.packages.${system}.model-training}/* $out/models/
          
          # Generate metadata
          cat > $out/.meta-introspector/metadata.json <<EOF
          {
            "version": "1.0",
            "timestamp": "$(date -Iseconds)",
            "commit": "${self.rev or "dirty"}",
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
      };
      
      # Build all jobs individually
      packages.${system}.all-jobs = pkgs.symlinkJoin {
        name = "all-jobs";
        paths = [
          self.packages.${system}.languages
          self.packages.${system}.build-graph
          self.packages.${system}.perf-analysis
          self.packages.${system}.topological-matrix
          self.packages.${system}.harmonic-analysis
          self.packages.${system}.model-training
        ];
      };
    };
}
