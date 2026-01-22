{
  description = "Analyze perf data from mes-bootstrap";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mes-perf.url = "path:../mes-perf-recorder";
  };

  outputs = { self, nixpkgs, mes-perf }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      packages.${system} = {
        default = self.packages.${system}.perf-analysis;
        
        perf-analysis = pkgs.runCommand "mes-perf-analysis" {
          nativeBuildInputs = [ pkgs.perf ];
          perfData = mes-perf.packages.${system}.default;
        } ''
          mkdir -p $out
          
          echo "📊 Analyzing perf data from: $perfData"
          
          # Copy perf data
          cp $perfData/mes-bootstrap.perf.data $out/
          cp $perfData/witness-hash.txt $out/
          
          # Generate detailed report
          ${pkgs.perf}/bin/perf report \
            -i $perfData/mes-bootstrap.perf.data \
            --stdio --no-children > $out/perf-report-full.txt 2>&1 || true
          
          # Generate script (trace)
          ${pkgs.perf}/bin/perf script \
            -i $perfData/mes-bootstrap.perf.data > $out/perf-trace.txt 2>&1 || true
          
          # Generate stats
          ${pkgs.perf}/bin/perf report \
            -i $perfData/mes-bootstrap.perf.data \
            --stdio --sort comm,dso > $out/perf-stats.txt 2>&1 || true
          
          # Create metadata
          cat > $out/metadata.json <<EOF
{
  "timestamp": "$(date -Iseconds)",
  "witness_hash": "$(cat $perfData/witness-hash.txt | cut -d' ' -f1)",
  "perf_data_size": $(stat -c%s $perfData/mes-bootstrap.perf.data),
  "bootstrap_chain": {
    "mes_libc": "${pkgs.minimal-bootstrap.mes-libc}",
    "mescc_tools": "${pkgs.minimal-bootstrap.mescc-tools}",
    "tinycc": "${pkgs.minimal-bootstrap.tinycc-mes.compiler}"
  },
  "nix_store_path": "$out"
}
EOF
          
          # Create README
          cat > $out/README.md <<'EOF'
# Mes Bootstrap Perf Analysis

## Files

- `mes-bootstrap.perf.data` - Raw # Use: perf-lib.lib.perfBuild (see docs/nix/PERF_FLAKE_TEMPLATE.md)
- `witness-hash.txt` - SHA256 hash of perf data
- `perf-report-full.txt` - Full perf report
- `perf-trace.txt` - Execution trace
- `perf-stats.txt` - Statistics by command/DSO
- `metadata.json` - Build metadata

## Witness Hash

This is the cryptographic witness of the Mes bootstrap chain access.

## Usage

\`\`\`bash
# View report
cat perf-report-full.txt

# Analyze with perf
perf report -i mes-bootstrap.perf.data
perf script -i mes-bootstrap.perf.data
\`\`\`
EOF
          
          echo "✅ Analysis complete"
          ls -lh $out/
        '';
        
        # Package for HuggingFace upload
        hf-archive = pkgs.runCommand "mes-perf-hf-archive" {
          analysis = self.packages.${system}.perf-analysis;
        } ''
          mkdir -p $out
          
          # Create NAR archive
          ${pkgs.nix}/bin/nix-store --dump $analysis > $out/mes-bootstrap-perf.nar
          
          # Compress
          ${pkgs.zstd}/bin/zstd -19 $out/mes-bootstrap-perf.nar -o $out/mes-bootstrap-perf.nar.zst
          
          # Hash
          ${pkgs.coreutils}/bin/sha256sum $out/mes-bootstrap-perf.nar.zst > $out/mes-bootstrap-perf.nar.zst.sha256
          
          # Copy metadata
          cp $analysis/metadata.json $out/
          cp $analysis/README.md $out/
          
          echo "📦 Archive created:"
          ls -lh $out/
        '';
      };
      
      apps.${system}.default = {
        type = "app";
        program = toString (pkgs.writeShellScript "show-analysis" ''
          echo "📊 Mes Bootstrap Perf Analysis"
          echo "=============================="
          echo ""
          echo "Build analysis:"
          echo "  nix build .#perf-analysis"
          echo ""
          echo "Create HF archive:"
          echo "  nix build .#hf-archive"
          echo ""
          echo "View report:"
          echo "  cat result/perf-report-full.txt"
        '');
      };
    };
}
