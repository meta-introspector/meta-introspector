{
  description = "Meta-introspector with LMFDB analysis CI/CD";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        
        # All our analysis tools
        analysisTools = pkgs.rustPlatform.buildRustPackage {
          pname = "meta-introspector-analysis";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          
          buildInputs = with pkgs; [
            openssl
            pkg-config
          ];
        };
        
      in {
        # Packages: build all analysis tools
        packages = {
          default = analysisTools;
          
          # Individual tools
          concept-map = pkgs.runCommand "concept-map" {} ''
            mkdir -p $out/bin
            cp ${analysisTools}/bin/concept_map_builder $out/bin/
            cp ${analysisTools}/bin/enhanced_concept_map $out/bin/
          '';
          
          lmfdb-analysis = pkgs.runCommand "lmfdb-analysis" {} ''
            mkdir -p $out/bin
            cp ${analysisTools}/bin/nix_binary_lmfdb_analyzer $out/bin/
            cp ${analysisTools}/bin/lmfdb_instruction_classifier $out/bin/
          '';
          
          numerical-codebreaker = pkgs.runCommand "numerical-codebreaker" {} ''
            mkdir -p $out/bin
            cp ${analysisTools}/bin/numerical_codebreaker $out/bin/
          '';
        };
        
        # Checks: run all analysis on our own code
        checks = {
          # Build check
          build = analysisTools;
          
          # Run concept map analysis
          concept-map-analysis = pkgs.runCommand "concept-map-check" {
            buildInputs = [ analysisTools ];
          } ''
            cd ${./.}
            ${analysisTools}/bin/concept_map_builder
            test -f data/concept_map.json
            touch $out
          '';
          
          # Run LMFDB analysis on all binaries
          lmfdb-binary-analysis = pkgs.runCommand "lmfdb-check" {
            buildInputs = [ analysisTools ];
          } ''
            cd ${./.}
            for binary in ${analysisTools}/bin/*; do
              ${analysisTools}/bin/nix_binary_lmfdb_analyzer "$binary" || true
            done
            touch $out
          '';
          
          # Run numerical codebreaker
          numerical-analysis = pkgs.runCommand "numerical-check" {
            buildInputs = [ analysisTools ];
          } ''
            cd ${./.}
            ${analysisTools}/bin/concept_map_builder
            ${analysisTools}/bin/numerical_codebreaker
            test -f data/numerical_patterns.json
            touch $out
          '';
        };
        
        # Apps: run analysis tools
        apps = {
          concept-map = {
            type = "app";
            program = "${analysisTools}/bin/concept_map_builder";
          };
          
          lmfdb-analyze = {
            type = "app";
            program = "${analysisTools}/bin/nix_binary_lmfdb_analyzer";
          };
          
          codebreaker = {
            type = "app";
            program = "${analysisTools}/bin/numerical_codebreaker";
          };
          
          # Full analysis pipeline
          analyze-all = {
            type = "app";
            program = toString (pkgs.writeShellScript "analyze-all" ''
              set -e
              echo "🔍 Running full analysis pipeline..."
              
              echo "1. Building concept map..."
              ${analysisTools}/bin/concept_map_builder
              
              echo "2. Running LMFDB analysis..."
              for binary in ${analysisTools}/bin/*; do
                ${analysisTools}/bin/nix_binary_lmfdb_analyzer "$binary" || true
              done
              
              echo "3. Running numerical codebreaker..."
              ${analysisTools}/bin/numerical_codebreaker
              
              echo "✅ Analysis complete!"
              echo "Results in data/"
            '');
          };
        };
        
        # Dev shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
            openssl
            pkg-config
          ];
          
          shellHook = ''
            echo "🦀 Meta-introspector dev environment"
            echo "Run: nix flake check    # Run all analysis"
            echo "Run: nix run .#analyze-all  # Full pipeline"
          '';
        };
      }
    );
}
