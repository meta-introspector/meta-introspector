{
  description = "Perf recording of nixpkgs minimal-bootstrap";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      packages.${system}.default = pkgs.runCommand "mes-bootstrap-perf" {
        nativeBuildInputs = [ pkgs.perf ];
      } ''
        mkdir -p $out
        
        echo "🔬 Recording minimal-bootstrap packages"
        
        # Record accessing the bootstrap chain
        ${pkgs.perf}/bin/perf record \
          -g -F 99 --call-graph dwarf \
          -o $out/mes-bootstrap.perf.data \
          -- ${pkgs.bash}/bin/bash -c '
            echo "=== Minimal Bootstrap Chain ==="
            echo "mes-libc: ${pkgs.minimal-bootstrap.mes-libc}"
            echo "mescc-tools: ${pkgs.minimal-bootstrap.mescc-tools}"
            echo "tinycc: ${pkgs.minimal-bootstrap.tinycc-mes.compiler}"
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
          echo "🔮 Nixpkgs Minimal Bootstrap Chain"
          echo "=================================="
          echo ""
          echo "Available packages:"
          echo "  mes-libc: ${pkgs.minimal-bootstrap.mes-libc}"
          echo "  mescc-tools: ${pkgs.minimal-bootstrap.mescc-tools}"
          echo "  tinycc: ${pkgs.minimal-bootstrap.tinycc-mes.compiler}"
          echo ""
          echo "Build with perf recording:"
          echo "  nix build .#default"
        '');
      };
    };
}
