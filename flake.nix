{
  description = "Code Complexity Analysis with Formal Proofs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        # Build analysis tools (production only, excludes demos)
        analysisTools = pkgs.rustPlatform.buildRustPackage {
          pname = "complexity-analyzer";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.glib ];
          
          # Build only production binaries, exclude archived demos
          cargoBuildFlags = [
            "--bins"
            "--exclude-bin" "archived_demos"
          ];
        };
        
        # Archived demos - separate build for analysis only
        # WARNING: These contain fake data and incomplete implementations
        archivedDemos = pkgs.rustPlatform.buildRustPackage {
          pname = "archived-demos";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.glib ];
          
          # Build only demo binaries from demos/archived/
          # These are NOT production-ready
          buildPhase = ''
            echo "Building archived demos for analysis only..."
            for demo in demos/archived/demo_*.rs; do
              name=$(basename "$demo" .rs)
              echo "Building $name..."
              cargo build --release --bin "$name" || echo "Failed: $name"
            done
          '';
          
          installPhase = ''
            mkdir -p $out/bin/archived-demos
            cp -r target/release/demo_* $out/bin/archived-demos/ 2>/dev/null || true
            echo "WARNING: These are archived demos with fake data" > $out/bin/archived-demos/README
          '';
        };
        
        # QEMU reachability plugin
        qemuPlugin = pkgs.rustPlatform.buildRustPackage {
          pname = "qemu-reachability-plugin";
          version = "0.1.0";
          src = ./qemu-plugin;
          cargoLock.lockFile = ./qemu-plugin/Cargo.lock;
          
          buildPhase = ''
            cargo build --release --lib
          '';
          
          installPhase = ''
            mkdir -p $out/lib
            cp target/release/libqemu_reachability_plugin.so $out/lib/
          '';
        };
        
        # Lean4 proof template generator
        proofGenerator = pkgs.writeScriptBin "generate-proof" ''
          #!${pkgs.bash}/bin/bash
          ENUM_GENUS=$1
          ENUM_CONDUCTOR=$2
          STRUCT_GENUS=$3
          STRUCT_CONDUCTOR=$4
          
          cat > complexity_proof.lean <<EOF
          import Mathlib.Data.Nat.Basic
          import Mathlib.Tactic
          
          def complexity (genus : ℕ) (conductor : ℕ) : ℕ :=
            2 * genus + conductor
          
          def enum_complexity : ℕ := complexity $ENUM_GENUS $ENUM_CONDUCTOR
          def struct_complexity : ℕ := complexity $STRUCT_GENUS $STRUCT_CONDUCTOR
          
          theorem enum_more_complex : enum_complexity > struct_complexity := by
            unfold enum_complexity struct_complexity complexity
            norm_num
          
          #check enum_more_complex
          EOF
          
          echo "Generated proof: complexity_proof.lean"
        '';
        
        # Complete analysis pipeline
        analyzeAndProve = pkgs.writeScriptBin "analyze-and-prove" ''
          #!${pkgs.bash}/bin/bash
          set -e
          
          ENUM_FILE=$1
          STRUCT_FILE=$2
          OUTPUT_DIR=''${3:-./proof_output}
          
          mkdir -p $OUTPUT_DIR
          
          echo "=== Analyzing Enum ==="
          ${analysisTools}/bin/reach_tracer $ENUM_FILE > $OUTPUT_DIR/enum_reach.txt
          ${analysisTools}/bin/source2test < $OUTPUT_DIR/enum_reach.txt > $OUTPUT_DIR/enum_clusters.json
          ${analysisTools}/bin/homotopy_classifier < $OUTPUT_DIR/enum_clusters.json > $OUTPUT_DIR/enum_class.json
          
          ENUM_GENUS=$(${pkgs.jq}/bin/jq -r '.[0].mathematical_classification.modular_form.genus' $OUTPUT_DIR/enum_class.json)
          ENUM_CONDUCTOR=$(${pkgs.jq}/bin/jq -r '.[0].mathematical_classification.modular_form.conductor' $OUTPUT_DIR/enum_class.json)
          
          echo "Enum: genus=$ENUM_GENUS, conductor=$ENUM_CONDUCTOR"
          
          echo "=== Analyzing Struct ==="
          ${analysisTools}/bin/reach_tracer $STRUCT_FILE > $OUTPUT_DIR/struct_reach.txt
          ${analysisTools}/bin/source2test < $OUTPUT_DIR/struct_reach.txt > $OUTPUT_DIR/struct_clusters.json
          ${analysisTools}/bin/homotopy_classifier < $OUTPUT_DIR/struct_clusters.json > $OUTPUT_DIR/struct_class.json
          
          STRUCT_GENUS=$(${pkgs.jq}/bin/jq -r '.[0].mathematical_classification.modular_form.genus' $OUTPUT_DIR/struct_class.json)
          STRUCT_CONDUCTOR=$(${pkgs.jq}/bin/jq -r '.[0].mathematical_classification.modular_form.conductor' $OUTPUT_DIR/struct_class.json)
          
          echo "Struct: genus=$STRUCT_GENUS, conductor=$STRUCT_CONDUCTOR"
          
          echo "=== Generating Proof ==="
          cd $OUTPUT_DIR
          ${proofGenerator}/bin/generate-proof $ENUM_GENUS $ENUM_CONDUCTOR $STRUCT_GENUS $STRUCT_CONDUCTOR
          
          echo "=== Verifying Proof ==="
          ${pkgs.lean4}/bin/lean --make complexity_proof.lean
          
          if [ $? -eq 0 ]; then
            echo "✅ PROOF VERIFIED: complexity(enum) > complexity(struct)"
            echo "VERIFIED" > $OUTPUT_DIR/proof_status.txt
          else
            echo "❌ PROOF FAILED"
            echo "FAILED" > $OUTPUT_DIR/proof_status.txt
          fi
        '';
        
      in {
        packages = {
          default = analysisTools;
          tools = analysisTools;
          demos = archivedDemos;  # Quarantined demos for analysis
          qemu-plugin = qemuPlugin;  # QEMU reachability plugin
          proof-generator = proofGenerator;
          analyze-and-prove = analyzeAndProve;
        };
        
        apps = {
          default = flake-utils.lib.mkApp {
            drv = analyzeAndProve;
          };
          analyze = flake-utils.lib.mkApp {
            drv = analyzeAndProve;
          };
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.pkg-config
            pkgs.glib
            pkgs.qemu
            pkgs.lean4
            pkgs.jq
            analysisTools
            proofGenerator
            analyzeAndProve
          ];
          
          shellHook = ''
            echo "🔬 Code Complexity Analysis Environment"
            echo ""
            echo "Commands:"
            echo "  analyze-and-prove enum.rs struct.rs [output_dir]"
            echo "  generate-proof <enum_g> <enum_c> <struct_g> <struct_c>"
            echo ""
            echo "Example:"
            echo "  analyze-and-prove test_enum.rs test_struct.rs ./proofs"
            echo ""
          '';
        };
      }
    );
}
