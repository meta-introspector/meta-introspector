{
  description = "Level 0: GNU Mes Bootstrap - Train MES Transformer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "mes-transformer-level0";
          version = "0.1.0";
          
          src = pkgs.fetchurl {
            url = "mirror://gnu/mes/mes-0.26.tar.gz";
            sha256 = "sha256-x9LvEKVe/6+RbNP/VVLbQvHfZKZJKJKJKJKJKJKJKJK=";
          };
          
          nativeBuildInputs = with pkgs; [ 
            linuxPackages.perf
            python3
          ];
          
          buildPhase = ''
            echo "🔬 Level 0: GNU Mes Bootstrap with Perf Recording"
            echo "=================================================="
            echo ""
            
            mkdir -p $out/perf $out/training
            
            # Record perf data during mes bootstrap
            echo "📊 Recording perf data for mes bootstrap..."
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
              -F 997 -g --call-graph dwarf \
              -- sh -c '
                ./configure --prefix=$out
                make
              ' 2>&1 | tee $out/perf/build.log
            
            echo ""
            echo "✅ Perf data captured: $(du -h $out/perf/mes-bootstrap.perf.data)"
            
            # Extract instruction pointers
            echo "📈 Extracting instruction pointers..."
            perf script -i $out/perf/mes-bootstrap.perf.data \
              -F ip | grep -v '^#' | sort -u > $out/training/ips.txt
            
            IP_COUNT=$(wc -l < $out/training/ips.txt)
            echo "   Unique IPs: $IP_COUNT"
            
            # Create training metadata
            cat > $out/training/meta.json << EOF
            {
              "level": 0,
              "name": "gnu-mes-bootstrap",
              "version": "0.26",
              "ip_count": $IP_COUNT,
              "perf_data": "$out/perf/mes-bootstrap.perf.data",
              "training_data": "$out/training/ips.txt",
              "timestamp": "$(date -Iseconds)",
              "derivation": "$out"
            }
            EOF
            
            echo ""
            echo "🎯 Level 0 Training Data Ready:"
            echo "   Perf: $out/perf/mes-bootstrap.perf.data"
            echo "   IPs:  $out/training/ips.txt ($IP_COUNT unique)"
            echo "   Meta: $out/training/meta.json"
          '';
          
          installPhase = ''
            # Install mes
            make install
            
            # Verify training data
            echo ""
            echo "📊 Training Data Summary:"
            echo "========================="
            cat $out/training/meta.json
            echo ""
            echo "✅ Level 0 complete - ready for transformer training"
          '';
        };
        
        # Extract training data from level 0
        packages.extract-training = pkgs.writeShellScriptBin "extract-level0" ''
          LEVEL0=$(nix build .#default --print-out-paths --no-link)
          echo "Level 0 derivation: $LEVEL0"
          echo "Training data: $LEVEL0/training/ips.txt"
          echo "Perf data: $LEVEL0/perf/mes-bootstrap.perf.data"
          cat $LEVEL0/training/meta.json
        '';
      }
    );
}
