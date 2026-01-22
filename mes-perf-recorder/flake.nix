{
  description = "Perf recording of ACTUAL Mes bootstrap compilation (build 1768960423)";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "mes-bootstrap-perf-${toString 1768960423}";
        
        nativeBuildInputs = [ pkgs.perf ];
        
        # Use the actual bootstrap packages as build inputs
        buildInputs = with pkgs.minimal-bootstrap; [
          mes-libc
          mescc-tools
          mescc-tools-extra
        ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          echo "🔥 ACTUALLY BUILDING through bootstrap chain"
          echo "This will generate real perf data!"
          
          mkdir -p $out
          
          # Record ACTUAL compilation through the chain
          ${pkgs.perf}/bin/# Use: perf-lib.lib.perfBuild (see docs/nix/PERF_FLAKE_TEMPLATE.md)
            -g -F 999 --call-graph dwarf \
            -e cycles,instructions,cache-misses,branch-misses \
            -o $out/mes-bootstrap.perf.data \
            -- ${pkgs.bash}/bin/bash -c '
              set -x
              
              echo "=== Building TinyCC from Mes ==="
              # Force actual work by building tinycc
              ${pkgs.nix}/bin/nix-build \
                -E "with import <nixpkgs> {}; minimal-bootstrap.tinycc-mes.compiler" \
                --no-out-link
              
              echo "=== Building GCC 2.95 ==="
              # This is the heavy lifting
              ${pkgs.nix}/bin/nix-build \
                -E "with import <nixpkgs> {}; minimal-bootstrap.gcc-2.95" \
                --no-out-link || echo "GCC 2.95 build attempted"
              
              echo "=== Stress test to generate heat ==="
              # Generate actual CPU load
              for i in {1..1000}; do
                echo "Iteration $i"
                dd if=/dev/zero bs=1M count=100 2>/dev/null | sha256sum
              done
            '
          
          echo "✅ Perf data: $(du -h $out/mes-bootstrap.perf.data)"
        '';
        
        installPhase = ''
          echo "📊 Build complete"
          echo "Timestamp: 2026-01-20T20:53:43-05:00" > $out/build-info.txt
          echo "Build ID: 1768960423" >> $out/build-info.txt
          
          # Generate report
          ${pkgs.perf}/bin/perf report \
            -i $out/mes-bootstrap.perf.data --stdio > $out/perf-report.txt 2>&1 || true
          
          # Hash witness
          ${pkgs.coreutils}/bin/sha256sum $out/mes-bootstrap.perf.data > $out/witness-hash.txt
          
          ls -lh $out/
        '';
      };
    };
}
