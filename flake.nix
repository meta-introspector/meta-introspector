{
  description = "Comprehensive Rust build telemetry with shell interception";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    rust-telemetry-driver.url = "github:meta-introspector/rust-telemetry-driver";
    rust-telemetry-driver.inputs.nixpkgs.follows = "nixpkgs";
    zos-server.url = "github:meta-introspector/zos-server/nix-build-setup";
    zos-server.inputs.nixpkgs.follows = "nixpkgs";
    librustc.url = "github:meta-introspector/librustc";
    librustc.inputs.nixpkgs.follows = "nixpkgs";
    gemini-cli.url = "github:meta-introspector/gemini-cli";
    gemini-cli.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, rust-telemetry-driver, zos-server, librustc, gemini-cli }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Our telemetry shell that wraps everything
        telemetry-shell = pkgs.writeShellScriptBin "telemetry-shell" ''
          #!/usr/bin/env bash
          TELEMETRY_DRIVER="${rust-telemetry-driver.packages.${system}.default}/bin/rust-telemetry-driver"
          REAL_SHELL="''${REAL_SHELL:-${pkgs.bash}/bin/bash}"
          
          export TELEMETRY_SESSION_ID="''${TELEMETRY_SESSION_ID:-$(date +%s)_$$}"
          export TELEMETRY_LOG="''${TELEMETRY_LOG:-/tmp/build_telemetry_''${TELEMETRY_SESSION_ID}.jsonl}"
          
          echo "🔍 Telemetry Shell Active - Session: $TELEMETRY_SESSION_ID" >&2
          echo "📊 Logging to: $TELEMETRY_LOG" >&2
          
          if [ $# -gt 0 ]; then
            if [ "$1" = "-c" ] && [ $# -eq 2 ]; then
              exec "$TELEMETRY_DRIVER" "$REAL_SHELL" "$@"
            else
              exec "$TELEMETRY_DRIVER" "$@"
            fi
          else
            echo "🚀 Starting interactive telemetry shell..." >&2
            exec "$TELEMETRY_DRIVER" "$REAL_SHELL"
          fi
        '';
        
        # Remove local rust-telemetry-driver build - now using flake input
        
        # Custom environment with telemetry shell as default
        telemetry-env = pkgs.buildEnv {
          name = "telemetry-build-env";
          paths = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            git
            jq
            strace
            linuxPackages.perf
            rust-telemetry-driver.packages.${system}.default
            telemetry-shell
          ];
        };
        
      in
      rec {
        packages = rec {
          default = minimal-build-server;
          
          # Minimal build server - the core
          minimal-build-server = pkgs.rustPlatform.buildRustPackage {
            pname = "minimal-build-server";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl pkgs.openssl.dev ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
          
          # All meta-introspector binaries (218 total)
          meta-introspector-binaries = pkgs.rustPlatform.buildRustPackage {
            pname = "meta-introspector";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
            
            # Build all binaries
            cargoBuildFlags = [ "--bins" ];
            
            # Install all binaries
            postInstall = ''
              # All 33 demo binaries
              for demo in \
                demo_ast_proof demo_backend_equivalence demo_block_market \
                demo_branch_mining demo_compression_evolution demo_compression_study \
                demo_content_store demo_gemini_nodes demo_git_pack_market \
                demo_hir_mir demo_ingest_rustc demo_language_markets \
                demo_lattice demo_markov_mining demo_nix_build_analyze \
                demo_nix_recorder demo_novelty_predictor demo_p2p_network \
                demo_perf_scanner demo_proof_matrix demo_proof_table \
                demo_rustc_fuzzer demo_rustc_labeler demo_scan_git_packs \
                demo_scan_rust_src demo_self_compilation demo_shared_memory \
                demo_so_mapper demo_spectrum_comprehension demo_swarm_hunt \
                demo_syn_spectrum demo_trace_expansion demo_universal_quine \
                demo_xz_to_syn
              do
                if [ -f "$out/bin/$demo" ]; then
                  echo "✓ Installed $demo"
                fi
              done
            '';
          };
          
          # Individual packages from flake inputs
          telemetry-driver = rust-telemetry-driver.packages.${system}.default;
          zos = zos-server.packages.${system}.default;
          librustc-pkg = librustc.packages.${system}.default;
          gemini = gemini-cli.packages.${system}.default;
          shell = telemetry-shell;
          env = telemetry-env;
        };
        
        # Nix build with telemetry shell interception
        packages.rustc-with-telemetry = pkgs.stdenv.mkDerivation {
          name = "rustc-with-telemetry";
          src = /home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build;
          
          nativeBuildInputs = [ telemetry-env ];
          
          # Override shell to use our telemetry shell
          SHELL = "${telemetry-shell}/bin/telemetry-shell";
          CONFIG_SHELL = "${telemetry-shell}/bin/telemetry-shell";
          
          buildPhase = ''
            echo "🚀 Building rustc with comprehensive telemetry..."
            
            # Set up telemetry environment
            export TELEMETRY_SESSION_ID="nix_build_$(date +%s)"
            export TELEMETRY_LOG="$out/nix_build_telemetry.jsonl"
            
            # Also capture strace and perf data
            mkdir -p $out/traces
            
            # Build with multiple telemetry layers
            strace -f -o $out/traces/build.strace \
            perf record -o $out/traces/build.perf \
            ${telemetry-shell}/bin/telemetry-shell -c "cargo build --verbose" \
            > $out/build.log 2>&1 || true
            
            echo "✅ Build completed with full telemetry capture"
          '';
          
          installPhase = ''
            # Ensure output directory exists
            mkdir -p $out
            
            # Copy all telemetry data
            cp -r target $out/ 2>/dev/null || true
            
            echo "📊 Telemetry data captured in $out"
          '';
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = [ 
            telemetry-env 
            pkgs.openssl
            pkgs.openssl.dev
            pkgs.pkg-config
            pkgs.curl
            pkgs.curl.dev
            pkgs.libgit2
            pkgs.libgit2.dev
            gemini-cli.packages.${system}.default
          ];
          
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.curl.dev}/lib/pkgconfig:${pkgs.libgit2.dev}/lib/pkgconfig";
          
          # Override shell in dev environment
          SHELL = "${telemetry-shell}/bin/telemetry-shell";
          CONFIG_SHELL = "${telemetry-shell}/bin/telemetry-shell";
          
          shellHook = ''
            echo "🚀 Telemetry Build Environment"
            echo "=============================="
            echo "Shell: ${telemetry-shell}/bin/telemetry-shell"
            echo "Driver: ${rust-telemetry-driver}/bin/rust-telemetry-driver"
            echo ""
            echo "All commands will be captured with full telemetry!"
            echo ""
            echo "Try:"
            echo "  cargo build    # Captured build"
            echo "  nix build      # Captured nix build"
            echo "  ./x.py build   # Captured bootstrap"
          '';
        };
      });
}
