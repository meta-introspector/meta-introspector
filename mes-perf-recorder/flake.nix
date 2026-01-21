{
  description = "Perf recording of GNU Mes bootstrap from Guix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          # The actual Mes bootstrap build
          mes-bootstrap = pkgs.stdenv.mkDerivation {
            name = "mes-bootstrap-recorded";
            
            src = pkgs.fetchurl {
              url = "https://git.savannah.gnu.org/cgit/mes.git/snapshot/mes-0.26.tar.gz";
              sha256 = "sha256-fake"; # TODO: get real hash
            };
            
            nativeBuildInputs = [ pkgs.linuxPackages.perf ];
            
            buildPhase = ''
              echo "🔬 Recording Mes bootstrap with perf"
              
              # Record the bootstrap
              perf record \
                -g -F 99 --call-graph dwarf \
                -o $out/mes-bootstrap.perf.data \
                -- bash -c '
                  # Stage 0: hex0
                  ./bootstrap.sh
                '
            '';
            
            installPhase = ''
              mkdir -p $out
              cp mes-bootstrap.perf.data $out/
              
              # Generate report
              perf report -i $out/mes-bootstrap.perf.data --stdio > $out/perf-report.txt
              
              # Extract stats
              perf script -i $out/mes-bootstrap.perf.data > $out/perf-trace.txt
              
              # Hash the witness
              sha256sum $out/mes-bootstrap.perf.data > $out/witness-hash.txt
              
              echo "✅ Perf data stored in $out"
            '';
          };
        });

      apps = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = self.apps.${system}.record-mes;
          
          record-mes = {
            type = "app";
            program = toString (pkgs.writeShellScript "record-mes-bootstrap" ''
              set -euo pipefail
              
              echo "🔮 Recording GNU Mes Bootstrap"
              echo "=============================="
              echo ""
              
              timestamp=$(date +%Y%m%d_%H%M%S)
              output_dir="mes-bootstrap-$timestamp"
              mkdir -p "$output_dir"
              
              cd "$output_dir"
              
              # Fetch Mes
              echo "📥 Fetching Mes 0.26..."
              ${pkgs.wget}/bin/wget -q https://git.savannah.gnu.org/cgit/mes.git/snapshot/mes-0.26.tar.gz
              ${pkgs.gnutar}/bin/tar xzf mes-0.26.tar.gz
              cd mes-0.26
              
              echo ""
              echo "🔬 Recording bootstrap with perf..."
              echo ""
              
              # Record the full bootstrap
              ${pkgs.linuxPackages.perf}/bin/perf record \
                -g -F 99 --call-graph dwarf \
                -o ../mes-bootstrap.perf.data \
                -- ${pkgs.bash}/bin/bash ./bootstrap.sh
              
              cd ..
              
              echo ""
              echo "✅ Bootstrap complete!"
              echo "📊 Perf data: $output_dir/mes-bootstrap.perf.data"
              echo "📈 Size: $(du -h mes-bootstrap.perf.data | cut -f1)"
              echo ""
              
              # Generate reports
              echo "📝 Generating reports..."
              ${pkgs.linuxPackages.perf}/bin/perf report \
                -i mes-bootstrap.perf.data --stdio > perf-report.txt
              
              ${pkgs.linuxPackages.perf}/bin/perf script \
                -i mes-bootstrap.perf.data > perf-trace.txt
              
              # Hash witness
              ${pkgs.coreutils}/bin/sha256sum mes-bootstrap.perf.data > witness-hash.txt
              
              echo "📄 Report: $output_dir/perf-report.txt"
              echo "📜 Trace: $output_dir/perf-trace.txt"
              echo "#️⃣  Hash: $(cat witness-hash.txt)"
              echo ""
              echo "Analyze with:"
              echo "  perf report -i $output_dir/mes-bootstrap.perf.data"
            '');
          };
          
          record-mes-stages = {
            type = "app";
            program = toString (pkgs.writeShellScript "record-mes-stages" ''
              set -euo pipefail
              
              echo "🔮 Recording Mes Bootstrap (Stage by Stage)"
              echo "==========================================="
              echo ""
              
              timestamp=$(date +%Y%m%d_%H%M%S)
              output_dir="mes-stages-$timestamp"
              mkdir -p "$output_dir"
              cd "$output_dir"
              
              # Fetch Mes
              echo "📥 Fetching Mes..."
              ${pkgs.wget}/bin/wget -q https://git.savannah.gnu.org/cgit/mes.git/snapshot/mes-0.26.tar.gz
              ${pkgs.gnutar}/bin/tar xzf mes-0.26.tar.gz
              cd mes-0.26
              
              # Record each stage separately
              stages=("hex0" "hex1" "hex2" "M0" "M2-Planet" "mes")
              
              for stage in "''${stages[@]}"; do
                echo ""
                echo "🔬 Recording stage: $stage"
                
                ${pkgs.linuxPackages.perf}/bin/perf record \
                  -g -F 99 --call-graph dwarf \
                  -o "../stage-$stage.perf.data" \
                  -- ${pkgs.bash}/bin/bash -c "./bootstrap-$stage.sh || true"
                
                echo "✅ Stage $stage recorded"
              done
              
              cd ..
              
              echo ""
              echo "✅ All stages recorded!"
              echo ""
              echo "Perf data files:"
              ls -lh stage-*.perf.data
              
              echo ""
              echo "Witness hashes:"
              ${pkgs.coreutils}/bin/sha256sum stage-*.perf.data | tee witness-hashes.txt
            '');
          };
        });

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              linuxPackages.perf
              wget
              gnutar
              bash
            ];
            
            shellHook = ''
              echo "🔮 GNU Mes Bootstrap Recorder"
              echo "============================"
              echo ""
              echo "Commands:"
              echo "  record-mes          - Record full bootstrap"
              echo "  record-mes-stages   - Record each stage separately"
              echo ""
              
              record-mes() {
                nix run .#record-mes
              }
              
              record-mes-stages() {
                nix run .#record-mes-stages
              }
            '';
          };
        });
    };
}
