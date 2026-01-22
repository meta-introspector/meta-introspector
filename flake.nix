{
  description = "Meta-Introspector: Proven Nix builds with LMFDB orbit arithmetization";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    perf-lib.url = "github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix";
  };
  
  outputs = { self, nixpkgs, perf-lib }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Build with perf recording
      buildWithProof = name: buildCommand: perf-lib.lib.perfBuild {
        inherit name buildCommand;
        
        # Collect perf trace
        postBuild = ''
          mkdir -p $out/proofs
          cp perf.data $out/proofs/${name}.perf.data
          
          # Analyze for duplicates
          ${self.packages.${system}.analyzer}/bin/analyze-duplicates \
            $out/proofs/${name}.perf.data \
            > $out/proofs/${name}.duplicates.json
          
          # Compute LMFDB orbit
          ${self.packages.${system}.analyzer}/bin/compute-orbit \
            $out/proofs/${name}.perf.data \
            > $out/proofs/${name}.orbit.json
          
          # Generate ZK proof
          ${self.packages.${system}.analyzer}/bin/generate-proof \
            $out/proofs/${name}.orbit.json \
            > $out/proofs/${name}.proof.json
        '';
      };
      
    in {
      packages.${system} = {
        # The analyzer tools
        analyzer = pkgs.rustPlatform.buildRustPackage {
          name = "meta-introspector-analyzer";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          
          buildPhase = ''
            cargo build --release --bin analyze-duplicates
            cargo build --release --bin compute-orbit
            cargo build --release --bin generate-proof
            cargo build --release --bin driver
          '';
          
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/analyze-duplicates $out/bin/
            cp target/release/compute-orbit $out/bin/
            cp target/release/generate-proof $out/bin/
            cp target/release/driver $out/bin/
          '';
        };
        
        # Build driver with proof
        driver = buildWithProof "driver" ''
          cargo build --release --bin driver
          mkdir -p $out/bin
          cp target/release/driver $out/bin/
        '';
        
        # Build all analysis tools with proof
        analysis-tools = buildWithProof "analysis-tools" ''
          cargo build --release
          mkdir -p $out/bin
          cp target/release/* $out/bin/ || true
        '';
        
        # Default: build everything with proofs
        default = pkgs.symlinkJoin {
          name = "meta-introspector-proven";
          paths = [
            self.packages.${system}.driver
            self.packages.${system}.analysis-tools
          ];
          
          # Aggregate all proofs
          postBuild = ''
            mkdir -p $out/proofs/aggregate
            
            # Collect all perf traces
            find ${self.packages.${system}.driver} -name "*.perf.data" \
              -exec cp {} $out/proofs/aggregate/ \;
            find ${self.packages.${system}.analysis-tools} -name "*.perf.data" \
              -exec cp {} $out/proofs/aggregate/ \;
            
            # Analyze aggregate for duplicates
            ${self.packages.${system}.analyzer}/bin/analyze-duplicates \
              $out/proofs/aggregate/*.perf.data \
              > $out/proofs/aggregate/all-duplicates.json
            
            # Compute system-wide orbit
            ${self.packages.${system}.analyzer}/bin/compute-orbit \
              $out/proofs/aggregate/*.perf.data \
              > $out/proofs/aggregate/system-orbit.json
            
            # Generate final proof
            ${self.packages.${system}.analyzer}/bin/generate-proof \
              $out/proofs/aggregate/system-orbit.json \
              > $out/proofs/aggregate/system-proof.json
            
            # Verify no duplicates
            DUPLICATES=$(jq '.duplicates | length' $out/proofs/aggregate/all-duplicates.json)
            if [ "$DUPLICATES" -gt 0 ]; then
              echo "❌ Found $DUPLICATES duplicates - build failed"
              exit 1
            fi
            
            echo "✅ Zero duplicates - system is minimal"
            echo "✅ Orbit: $(jq -r '.orbit' $out/proofs/aggregate/system-orbit.json)"
            echo "✅ Proof: $(jq -r '.proof_hash' $out/proofs/aggregate/system-proof.json)"
          '';
        };
      };
      
      # Dev shell with all tools
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          self.packages.${system}.analyzer
          pkgs.perf
          pkgs.jq
        ];
        
        shellHook = ''
          echo "🚀 Meta-Introspector Development Environment"
          echo "=============================================="
          echo ""
          echo "Build with proof:"
          echo "  nix build .#driver"
          echo "  nix build .#analysis-tools"
          echo "  nix build .#default"
          echo ""
          echo "Proofs stored in: result/proofs/"
          echo ""
          echo "Verify:"
          echo "  jq . result/proofs/aggregate/system-proof.json"
          echo ""
        '';
      };
    };
}
