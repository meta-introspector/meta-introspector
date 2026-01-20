{
  description = "NixOps: Nix + Git + Rust operations framework";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        # Our Rust tools from this repo
        meta-introspector-tools = pkgs.rustPlatform.buildRustPackage {
          pname = "meta-introspector-tools";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        # NixOps environment with all tools
        nixops-env = pkgs.buildEnv {
          name = "nixops-env";
          paths = with pkgs; [
            # Core tools
            git
            nix
            
            # Our Rust binaries
            meta-introspector-tools
            
            # Monitoring & tracing
            strace
            linuxPackages.perf
            
            # Network tools
            dnsmasq
            mitmproxy
            
            # File server
            python3Packages.http-server
            
            # Git proxy
            nginx
          ];
        };

        # Atomic operation runner
        nixops-runner = pkgs.writeShellScriptBin "nixops-run" ''
          set -euo pipefail
          
          OPERATION="$1"
          REPO_PATH="$2"
          shift 2
          
          echo "🔧 NixOps: Running $OPERATION on $REPO_PATH"
          
          # Enter pure Nix environment
          nix develop ${self}#nixops --command bash -c "
            cd '$REPO_PATH'
            
            # Stash current state
            git stash push -m 'nixops-pre-$OPERATION' || true
            
            # Run operation atomically
            $OPERATION '$REPO_PATH' $@
            
            # Verify clean state
            if git diff --quiet; then
              echo '✅ Operation complete, no changes'
            else
              echo '📝 Changes detected, ready to commit'
            fi
          "
        '';

      in {
        packages = {
          default = nixops-env;
          nixops-runner = nixops-runner;
          tools = meta-introspector-tools;
        };

        devShells.nixops = pkgs.mkShell {
          name = "nixops-shell";
          
          buildInputs = [
            nixops-env
            nixops-runner
          ];
          
          shellHook = ''
            echo "🚀 NixOps Framework"
            echo "Tools: git, nix, rust, strace, perf, mitmproxy, dnsmasq"
            echo ""
            echo "Usage: nixops-run <operation> <repo-path> [args]"
          '';
        };

        apps.nixops = {
          type = "app";
          program = "${nixops-runner}/bin/nixops-run";
        };
      }
    );
}
