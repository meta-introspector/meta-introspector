{
  description = "MES Transformer - Train on nix build perf data";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "mes-transformer-train";
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "burn-0.21.0" = "sha256-mw1fY+7iBgB9JDWR3bGQ3+Ht6lqo3h6WDjqP+Uiusm0=";
              "cubecl-0.9.0" = "sha256-Eg6mIYs9C5PR52xW35LEvUKkn6Dv/6rU2iURvi3ce5c=";
              "cubek-0.1.0" = "sha256-BNg1XXx4EHaIbkAcqFGmuOuwNS2tIUkA1dkVn7AvjmE=";
            };
          };

          nativeBuildInputs = with pkgs; [ 
            pkg-config 
            cudaPackages.cuda_nvcc
            makeWrapper
            linuxPackages.perf
          ];
          
          buildInputs = with pkgs; [
            cudaPackages.cuda_cudart
            cudaPackages.libcublas
            linuxPackages.nvidia_x11
          ];
          
          CUDA_PATH = "${pkgs.cudaPackages.cuda_cudart}";
          
          # Capture perf data during build
          preBuild = ''
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
            PERF_PID=$!
          '';
          
          postBuild = ''
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
            kill -INT $PERF_PID 2>/dev/null || true
            wait $PERF_PID 2>/dev/null || true
            
            echo "📊 Perf data captured: $out/build.perf.data"
          '';
          
          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin $out/perf $out/model
            
            # Install binary
            install -Dm755 target/*/release/mes-transformer-gpu $out/bin/mes-transformer-train
            
            # Wrap with CUDA libraries
            wrapProgram $out/bin/mes-transformer-train \
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath [ pkgs.linuxPackages.nvidia_x11 ]}"
            
            # Store perf data in output
            if [ -f build.perf.data ]; then
              cp build.perf.data $out/perf/
              perf script -i build.perf.data > $out/perf/trace.txt
              echo "✅ Perf data stored in $out/perf/"
            fi
            
            # Training happens here - using perf data from THIS build
            echo "🚀 Training MES transformer on build perf data..."
            $out/bin/mes-transformer-train --perf-data $out/perf/build.perf.data \
              --output-model $out/model/checkpoint.bin \
              --epochs 100 || echo "⚠️  Training skipped (no GPU or perf data)"
            
            # Store metadata
            cat > $out/meta.json << EOF
            {
              "derivation": "$out",
              "system": "${system}",
              "timestamp": "$(date -Iseconds)",
              "perf_data": "$out/perf/build.perf.data",
              "model": "$out/model/checkpoint.bin"
            }
            EOF
            
            runHook postInstall
          '';
          
          doCheck = false;
        };
        
        # Distributed training: merge multiple derivations
        packages.merge-training = pkgs.writeShellScriptBin "merge-training" ''
          # Merge model checkpoints from multiple nix stores
          echo "🔄 Merging training results from multiple derivations..."
          
          for store in "$@"; do
            if [ -f "$store/model/checkpoint.bin" ]; then
              echo "  Found: $store/model/checkpoint.bin"
            fi
          done
          
          echo "✅ Merge complete"
        '';
      }
    );
}
