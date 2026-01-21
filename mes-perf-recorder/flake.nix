{
  description = "Perf recording of nixpkgs minimal-bootstrap (rebuild 1768960332)";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      # Force rebuild with timestamp
      buildId = "1768960332";
    in {
      packages.${system}.default = pkgs.runCommand "mes-bootstrap-perf-${buildId}" {
        nativeBuildInputs = [ pkgs.perf ];
      } ''
        mkdir -p $out
        
        echo "🔬 Recording minimal-bootstrap (build ${buildId})"
        echo "Timestamp: 2026-01-20T20:52:12-05:00" > $out/build-info.txt
        echo "Build ID: ${buildId}" >> $out/build-info.txt
        
        # Record accessing the bootstrap chain
        ${pkgs.perf}/bin/perf record \
          -g -F 99 --call-graph dwarf \
          -o $out/mes-bootstrap.perf.data \
          -- ${pkgs.bash}/bin/bash -c '
            echo "=== Minimal Bootstrap Chain ===" | tee -a $out/build-info.txt
            echo "mes-libc: ${pkgs.minimal-bootstrap.mes-libc}" | tee -a $out/build-info.txt
            echo "mescc-tools: ${pkgs.minimal-bootstrap.mescc-tools}" | tee -a $out/build-info.txt
            echo "tinycc: ${pkgs.minimal-bootstrap.tinycc-mes.compiler}" | tee -a $out/build-info.txt
            ls -lh ${pkgs.minimal-bootstrap.mes-libc}
            ls -lh ${pkgs.minimal-bootstrap.mescc-tools}
            ls -lh ${pkgs.minimal-bootstrap.tinycc-mes.compiler}
          '
        
        echo "✅ Recorded to $out/mes-bootstrap.perf.data"
        
        # Generate report
        ${pkgs.perf}/bin/perf report \
          -i $out/mes-bootstrap.perf.data --stdio > $out/perf-report.txt 2>&1 || true
        
        # Hash witness
        ${pkgs.coreutils}/bin/sha256sum $out/mes-bootstrap.perf.data > $out/witness-hash.txt
        
        echo "📊 Files created:"
        ls -lh $out/
      '';
      
      apps.${system}.default = {
        type = "app";
        program = toString (pkgs.writeShellScript "show-bootstrap" ''
          echo "🔮 Nixpkgs Minimal Bootstrap Chain (Build ${buildId})"
          echo "===================================================="
          echo ""
          echo "Build with perf recording:"
          echo "  nix build .#default"
        '');
      };
    };
}
