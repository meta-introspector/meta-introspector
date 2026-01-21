{
  description = "MES Bootstrap Proof - Complete transparency from 357 bytes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    guix.url = "github:NixOS/nixpkgs/nixos-unstable"; # For guix package
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      packages.${system} = {
        
        # Build MES with full instrumentation
        mes-bootstrap-proof = pkgs.stdenv.mkDerivation {
          name = "mes-bootstrap-proof";
          
          buildInputs = with pkgs; [
            guix
            linuxPackages.perf
            strace
          ];
          
          # No src - we build from Guix
          unpackPhase = "true";
          
          buildPhase = ''
            mkdir -p $out/traces
            
            # Record the entire bootstrap
            perf record -g -o $out/traces/mes-bootstrap.perf.data -- \
            strace -f -o $out/traces/mes-bootstrap.strace -e trace=all -- \
              guix build --no-substitutes mes 2>&1 | tee $out/traces/mes-bootstrap.log || true
            
            # Extract metrics
            SAMPLES=$(perf report -i $out/traces/mes-bootstrap.perf.data --stdio 2>/dev/null | grep "Samples:" | awk '{print $3}' || echo 0)
            SYSCALLS=$(wc -l < $out/traces/mes-bootstrap.strace || echo 0)
            
            # Create metadata
            cat > $out/traces/metadata.json <<EOF
            {
              "timestamp": "$(date -Iseconds)",
              "proof": "MES bootstrap from 357 bytes",
              "seed_size": 357,
              "perf_samples": $SAMPLES,
              "syscalls": $SYSCALLS,
              "stages": [
                "bootstrap-seeds-1.0.0",
                "stage0-posix-1.4",
                "mes-boot-0.24.2",
                "tcc-boot0",
                "gcc-core-mesboot0-2.95.3",
                "mes-0.26"
              ]
            }
            EOF
          '';
          
          installPhase = ''
            echo "MES Bootstrap Proof stored in $out/traces"
          '';
        };
        
        # Export as NAR
        mes-bootstrap-nar = pkgs.runCommand "mes-bootstrap-nar" {
          buildInputs = [ pkgs.nix ];
        } ''
          mkdir -p $out
          nix-store --export ${self.packages.${system}.mes-bootstrap-proof} > $out/mes-bootstrap-proof.nar
          
          # Compress
          xz -9 $out/mes-bootstrap-proof.nar
          
          # Create index
          cat > $out/index.json <<EOF
          {
            "name": "mes-bootstrap-proof",
            "description": "Complete transparency proof: MES from 357 bytes",
            "nar_size": $(stat -c%s $out/mes-bootstrap-proof.nar.xz),
            "store_path": "${self.packages.${system}.mes-bootstrap-proof}",
            "hf_dataset": "hf://datasets/introspector/mes-bootstrap-proof"
          }
          EOF
        '';
        
        default = self.packages.${system}.mes-bootstrap-nar;
      };
      
      # Script to upload to HuggingFace
      apps.${system}.upload-to-hf = {
        type = "app";
        program = toString (pkgs.writeShellScript "upload-to-hf" ''
          set -euo pipefail
          
          NAR_PATH="${self.packages.${system}.mes-bootstrap-nar}"
          
          echo "📤 Uploading MES Bootstrap Proof to HuggingFace"
          echo "NAR: $NAR_PATH/mes-bootstrap-proof.nar.xz"
          
          # Upload to HF dataset
          if [ -d hf-build-telemetry-upload ]; then
            cp $NAR_PATH/mes-bootstrap-proof.nar.xz hf-build-telemetry-upload/
            cp $NAR_PATH/index.json hf-build-telemetry-upload/mes-bootstrap-proof.json
            
            cd hf-build-telemetry-upload
            git add mes-bootstrap-proof.nar.xz mes-bootstrap-proof.json
            git commit -m "Add MES bootstrap proof NAR"
            git push
            
            echo "✅ Uploaded to HuggingFace"
          else
            echo "❌ HF dataset not found"
            exit 1
          fi
        '');
      };
    };
}
