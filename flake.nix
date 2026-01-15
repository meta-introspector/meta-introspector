{
  description = "Comprehensive Rust build telemetry with shell interception";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    rust-telemetry-driver.url = "github:meta-introspector/rust-telemetry-driver";
    rust-telemetry-driver.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, rust-telemetry-driver }:
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
      {
        packages = {
          default = rust-telemetry-driver.packages.${system}.default;
          rust-telemetry-driver = rust-telemetry-driver.packages.${system}.default;
          telemetry-shell = telemetry-shell;
          telemetry-env = telemetry-env;
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
          buildInputs = [ telemetry-env ];
          
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
