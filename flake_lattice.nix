{
  description = "71 Language Complexity Lattice - All data in nix store";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    # Build harmonic analyzer once
    harmonic_analyzer = pkgs.rustPlatform.buildRustPackage {
      pname = "harmonic_analyzer";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      buildAndTestSubdir = ".";
      cargoBuildFlags = [ "--bin" "harmonic_analyzer" ];
    };
    
    # Build lattice builder once
    lattice_builder = pkgs.rustPlatform.buildRustPackage {
      pname = "lattice_builder";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      buildAndTestSubdir = ".";
      cargoBuildFlags = [ "--bin" "lattice_builder" ];
    };
    
    # Helper to create perf + analysis derivation for a language
    mkLanguageAnalysis = lang: pkgs.stdenv.mkDerivation {
      name = "${lang}-71-analysis";
      src = ./const_71_test/${lang};
      nativeBuildInputs = [ pkgs.perf harmonic_analyzer ];
      
      buildPhase = ''
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
        
        # Analyze
        ${harmonic_analyzer}/bin/harmonic_analyzer perf.data > analysis.txt || echo "Analysis failed" > analysis.txt
      '';
      
      installPhase = ''
        mkdir -p $out
        cp perf.data $out/${lang}_build.perf.data || true
        cp analysis.txt $out/${lang}_analysis.txt
      '';
    };
    
    # All 71 languages
    languages = [
      "bash" "python" "ruby" "rust" "nix_flake"
      "agda" "coq" "haskell" "isabelle" "lean4"
      # ... add all 71
    ];
    
    # Generate analysis for each language
    analyses = builtins.listToAttrs (map (lang: {
      name = "${lang}-analysis";
      value = mkLanguageAnalysis lang;
    }) languages);
    
  in {
    packages.${system} = analyses // {
      # Aggregate all analyses into lattice
      complexity-lattice = pkgs.stdenv.mkDerivation {
        name = "complexity-lattice";
        nativeBuildInputs = [ lattice_builder ];
        
        buildPhase = ''
          mkdir -p analyses
          
          # Copy all analyses
          ${pkgs.lib.concatMapStringsSep "\n" (lang: ''
            cp ${analyses."${lang}-analysis"}/${lang}_analysis.txt analyses/ || true
          '') languages}
          
          # Build lattice
          cd analyses
          ${lattice_builder}/bin/lattice_builder
        '';
        
        installPhase = ''
          mkdir -p $out
          cp analyses/complexity_lattice.json $out/
          cp analyses/complexity_lattice.dot $out/
          
          # Generate PNG if graphviz available
          ${pkgs.graphviz}/bin/dot -Tpng analyses/complexity_lattice.dot -o $out/complexity_lattice.png || true
        '';
      };
      
      # Convenience: all analyses in one place
      all-analyses = pkgs.symlinkJoin {
        name = "all-71-analyses";
        paths = map (lang: analyses."${lang}-analysis") languages;
      };
    };
    
    # Default: build the lattice
    packages.${system}.default = self.packages.${system}.complexity-lattice;
  };
}
