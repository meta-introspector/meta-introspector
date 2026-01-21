{
  description = "Perf recorder for Nix builds - captures EM witness of compilation";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.mkShell {
            buildInputs = [ pkgs.linuxPackages.perf pkgs.nix ];
            shellHook = ''
              echo "🎯 Perf Recorder for Nix Builds"
              echo "================================"
              echo ""
              echo "Usage:"
              echo "  perf-build <flake-ref> [build-args...]"
              echo ""
              echo "Examples:"
              echo "  perf-build .#default"
              echo "  perf-build github:NixOS/nixpkgs#hello"
              echo "  perf-build ../meta-introspector#default"
              echo ""
              echo "Output: perf.data in current directory"
              echo "Analyze: perf report"
              echo ""

              perf-build() {
                if [ "$#" -lt 1 ]; then
                  echo "❌ perf-build: need at least a flake ref (e.g. .#pkg)" >&2
                  return 1
                fi
                
                local target="$1"
                shift
                local timestamp=$(date +%Y%m%d_%H%M%S)
                local output="perf_build_''${timestamp}.data"

                echo "🔬 Recording perf profile of: nix build \"$target\" $*"
                echo "📊 Output: $output"
                echo ""
                
                # Record with call graph and frequency
                perf record \
                  -g \
                  -F 99 \
                  --call-graph dwarf \
                  -o "$output" \
                  -- nix build "$target" "$@" --print-build-logs
                
                local exit_code=$?
                
                if [ $exit_code -eq 0 ]; then
                  echo ""
                  echo "✅ Build complete!"
                  echo "📊 Perf data: $output"
                  echo "📈 Size: $(du -h "$output" | cut -f1)"
                  echo ""
                  echo "Analyze with:"
                  echo "  perf report -i $output"
                  echo "  perf script -i $output > trace.txt"
                else
                  echo ""
                  echo "❌ Build failed (exit $exit_code)"
                  echo "📊 Partial perf data may be in: $output"
                fi
                
                return $exit_code
              }
              
              perf-build-with-stats() {
                if [ "$#" -lt 1 ]; then
                  echo "❌ need flake ref" >&2
                  return 1
                fi
                
                local target="$1"
                shift
                local timestamp=$(date +%Y%m%d_%H%M%S)
                local perf_data="perf_build_''${timestamp}.data"
                local stats_file="perf_stats_''${timestamp}.txt"
                
                echo "🔬 Recording with stats: $target"
                
                perf stat \
                  -o "$stats_file" \
                  -d -d -d \
                  perf record \
                    -g -F 99 --call-graph dwarf \
                    -o "$perf_data" \
                    -- nix build "$target" "$@" --print-build-logs
                
                echo ""
                echo "✅ Complete!"
                echo "📊 Perf data: $perf_data"
                echo "📈 Stats: $stats_file"
                cat "$stats_file"
              }
            '';
          };
        });

      apps = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = self.apps.${system}.perf-build;
          
          perf-build = {
            type = "app";
            program = toString (pkgs.writeShellScript "perf-build" ''
              set -euo pipefail
              
              if [ "$#" -lt 1 ]; then
                echo "Usage: nix run .#perf-build -- <flake-ref> [build-args...]" >&2
                echo "" >&2
                echo "Examples:" >&2
                echo "  nix run .#perf-build -- .#default" >&2
                echo "  nix run .#perf-build -- github:NixOS/nixpkgs#hello" >&2
                exit 1
              fi
              
              target="$1"
              shift
              timestamp=$(date +%Y%m%d_%H%M%S)
              output="perf_build_''${timestamp}.data"
              
              echo "🔬 Recording perf profile of: nix build \"$target\" $*"
              echo "📊 Output: $output"
              echo ""
              
              exec ${pkgs.linuxPackages.perf}/bin/perf record \
                -g -F 99 --call-graph dwarf \
                -o "$output" \
                -- ${pkgs.nix}/bin/nix build "$target" "$@" --print-build-logs
            '');
          };
          
          perf-build-stats = {
            type = "app";
            program = toString (pkgs.writeShellScript "perf-build-stats" ''
              set -euo pipefail
              
              if [ "$#" -lt 1 ]; then
                echo "Usage: nix run .#perf-build-stats -- <flake-ref>" >&2
                exit 1
              fi
              
              target="$1"
              shift
              timestamp=$(date +%Y%m%d_%H%M%S)
              perf_data="perf_build_''${timestamp}.data"
              stats_file="perf_stats_''${timestamp}.txt"
              
              echo "🔬 Recording with detailed stats: $target"
              
              ${pkgs.linuxPackages.perf}/bin/perf stat \
                -o "$stats_file" \
                -d -d -d \
                ${pkgs.linuxPackages.perf}/bin/perf record \
                  -g -F 99 --call-graph dwarf \
                  -o "$perf_data" \
                  -- ${pkgs.nix}/bin/nix build "$target" "$@" --print-build-logs
              
              echo ""
              echo "✅ Complete!"
              echo "📊 Perf data: $perf_data"
              echo "📈 Stats: $stats_file"
              cat "$stats_file"
            '');
          };
        });
    };
}
