# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/usr/bin/env bash
set -euo pipefail

echo "🔄 Forcing rebuild with new source hashes"
echo "=========================================="
echo ""

# Add a timestamp to force rebuild
TIMESTAMP=$(date +%s)

cd mes-perf-recorder

# Update flake to force rebuild
cat > flake.nix <<EOF
{
  description = "Perf recording of nixpkgs minimal-bootstrap (rebuild $TIMESTAMP)";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      # Force rebuild with timestamp
      buildId = "$TIMESTAMP";
    in {
      packages.\${system}.default = pkgs.runCommand "mes-bootstrap-perf-\${buildId}" {
        nativeBuildInputs = [ pkgs.perf ];
      } ''
        mkdir -p \$out
        
        echo "🔬 Recording minimal-bootstrap (build \${buildId})"
        echo "Timestamp: $(date -Iseconds)" > \$out/build-info.txt
        echo "Build ID: \${buildId}" >> \$out/build-info.txt
        
        # Record accessing the bootstrap chain
        \${pkgs.perf}/bin/# Use: nix run github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix#perf-build -- .#target
          -g -F 99 --call-graph dwarf \\
          -o \$out/mes-bootstrap.perf.data \\
          -- \${pkgs.bash}/bin/bash -c '
            echo "=== Minimal Bootstrap Chain ===" | tee -a \$out/build-info.txt
            echo "mes-libc: \${pkgs.minimal-bootstrap.mes-libc}" | tee -a \$out/build-info.txt
            echo "mescc-tools: \${pkgs.minimal-bootstrap.mescc-tools}" | tee -a \$out/build-info.txt
            echo "tinycc: \${pkgs.minimal-bootstrap.tinycc-mes.compiler}" | tee -a \$out/build-info.txt
            ls -lh \${pkgs.minimal-bootstrap.mes-libc}
            ls -lh \${pkgs.minimal-bootstrap.mescc-tools}
            ls -lh \${pkgs.minimal-bootstrap.tinycc-mes.compiler}
          '
        
        echo "✅ Recorded to \$out/mes-bootstrap.perf.data"
        
        # Generate report
        \${pkgs.perf}/bin/perf report \\
          -i \$out/mes-bootstrap.perf.data --stdio > \$out/perf-report.txt 2>&1 || true
        
        # Hash witness
        \${pkgs.coreutils}/bin/sha256sum \$out/mes-bootstrap.perf.data > \$out/witness-hash.txt
        
        echo "📊 Files created:"
        ls -lh \$out/
      '';
      
      apps.\${system}.default = {
        type = "app";
        program = toString (pkgs.writeShellScript "show-bootstrap" ''
          echo "🔮 Nixpkgs Minimal Bootstrap Chain (Build \${buildId})"
          echo "===================================================="
          echo ""
          echo "Build with # Use: nix run github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix#perf-build -- .#target
          echo "  nix build .#default"
        '');
      };
    };
}
EOF

echo "✅ Updated flake with build ID: $TIMESTAMP"
echo ""
echo "Commit and rebuild:"
echo "  git add flake.nix"
echo "  git commit -m 'Force rebuild with timestamp $TIMESTAMP'"
echo "  nix build .#default"
