{
  description = "Bott[8] Optimal Layout Solver with Perf Integration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        # Solve the layout optimization
        layoutSolution = pkgs.stdenv.mkDerivation {
          name = "bott8-layout-solution";
          src = ./.;
          
          buildInputs = [ pkgs.minizinc ];
          
          buildPhase = ''
            echo "🍄 Solving Bott[8] layout..."
            minizinc \
              --solver Gecode \
              --time-limit 60000 \
              bott8_optimal_layout.mzn \
              bott8_layout_example.dzn \
              > solution.txt 2>&1 || true
          '';
          
          installPhase = ''
            mkdir -p $out
            cp solution.txt $out/
            echo "✅ Solution saved to $out/solution.txt"
          '';
        };
        
        # Run nix build with perf monitoring
        perfMonitor = pkgs.writeShellScriptBin "run-with-perf" ''
          set -euo pipefail
          
          OUTPUT_DIR="./perf_data"
          mkdir -p "$OUTPUT_DIR"
          
          TIMESTAMP=$(date +%s)
          PERF_DATA="$OUTPUT_DIR/perf_$TIMESTAMP.data"
          PERF_REPORT="$OUTPUT_DIR/perf_$TIMESTAMP.txt"
          PERF_JSON="$OUTPUT_DIR/perf_$TIMESTAMP.json"
          
          echo "🔥 Running nix build with perf monitoring..."
          echo "Output: $PERF_DATA"
          
          # Run perf stat with detailed metrics
          ${pkgs.linuxPackages.perf}/bin/perf stat \
            -e cycles,instructions,cache-references,cache-misses \
            -e branches,branch-misses \
            -e cpu-clock,task-clock \
            -e page-faults,context-switches \
            -o "$PERF_REPORT" \
            nix build .#layoutSolution 2>&1 || true
          
          echo ""
          echo "📊 Perf Report:"
          cat "$PERF_REPORT"
          
          # Parse perf output to JSON
          echo "🔄 Converting to JSON..."
          ${pkgs.python3}/bin/python3 ./parse_perf.py \
            "$PERF_REPORT" "$PERF_JSON"
          
          echo ""
          echo "✅ Perf data saved:"
          echo "  Raw:  $PERF_REPORT"
          echo "  JSON: $PERF_JSON"
          
          # Map to 8D
          echo ""
          echo "🌀 Mapping to 8D Bott manifold..."
          PERF_8D="$OUTPUT_DIR/perf_''${TIMESTAMP}_8d.json"
          ${pkgs.python3}/bin/python3 ./map_perf_to_8d.py \
            "$PERF_JSON" "$PERF_8D"
          
          echo ""
          echo "✅ Complete! Files:"
          echo "  Perf report: $PERF_REPORT"
          echo "  Perf JSON:   $PERF_JSON"
          echo "  8D mapping:  $PERF_8D"
        '';
        
        # Map perf data to 8D Bott manifold
        perfMapper = pkgs.writeShellScriptBin "map-perf-to-8d" ''
          set -euo pipefail
          
          if [ $# -lt 1 ]; then
            echo "Usage: map-perf-to-8d <perf_json_file>"
            exit 1
          fi
          
          PERF_JSON="$1"
          OUTPUT_JSON="''${PERF_JSON%.json}_8d.json"
          
          echo "🌀 Mapping perf data to 8D Bott manifold..."
          echo "Input:  $PERF_JSON"
          echo "Output: $OUTPUT_JSON"
          
          ${pkgs.python3}/bin/python3 ./map_perf_to_8d.py \
            "$PERF_JSON" "$OUTPUT_JSON"
          
          echo ""
          echo "✅ 8D mapping complete: $OUTPUT_JSON"
        '';
        
      in {
        packages = {
          default = layoutSolution;
          layoutSolution = layoutSolution;
          perfMonitor = perfMonitor;
          perfMapper = perfMapper;
        };
        
        apps = {
          default = {
            type = "app";
            program = "${perfMonitor}/bin/run-with-perf";
          };
          
          solve = {
            type = "app";
            program = "${pkgs.writeShellScript "solve" ''
              echo "🍄 Building layout solution..."
              nix build .#layoutSolution
              cat result/solution.txt
            ''}";
          };
          
          monitor = {
            type = "app";
            program = "${perfMonitor}/bin/run-with-perf";
          };
          
          map = {
            type = "app";
            program = "${perfMapper}/bin/map-perf-to-8d";
          };
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            minizinc
            linuxPackages.perf
            python3
            jq
          ];
          
          shellHook = ''
            echo "🧙♂️ Bott[8] Layout Solver + Perf Integration"
            echo ""
            echo "Commands:"
            echo "  nix run .#solve    - Solve layout optimization"
            echo "  nix run .#monitor  - Run with perf monitoring"
            echo "  nix run .#map      - Map perf data to 8D"
            echo ""
          '';
        };
      }
    );
}
