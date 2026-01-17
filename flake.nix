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
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, rust-telemetry-driver, zos-server, librustc }:
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
            # linuxPackages.perf  # Optional: uncomment for perf support
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
          
          # All meta-introspector binaries (220 total)
          meta-introspector-binaries = pkgs.rustPlatform.buildRustPackage {
            pname = "meta-introspector";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
            
            # Build all 220 binaries
            cargoBuildFlags = [ "--bins" ];
            
            meta = {
              description = "Meta-introspector: Comprehensive Rust build telemetry and analysis tools";
              longDescription = ''
                Collection of 220+ binaries for:
                - Build telemetry and instrumentation
                - Code analysis and compression
                - Repository mining and metrics
                - Nix integration and services
                - Blockchain and smart contract analysis
                - Demo applications and experiments
              '';
            };
          };
          
          # Individual packages from flake inputs
          telemetry-driver = rust-telemetry-driver.packages.${system}.default;
          zos = zos-server.packages.${system}.default;
          librustc-pkg = librustc.packages.${system}.default;
          shell = telemetry-shell;
          env = telemetry-env;
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
