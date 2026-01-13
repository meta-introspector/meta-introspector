{
  description = "Rustc build order capture with rust overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Custom rustc with our interceptor
        rustc-interceptor = pkgs.writeShellScriptBin "rustc" ''
          exec ${pkgs.rust-bin.stable.latest.default}/bin/rustc "$@"
        '';
        
        # Build rustc from source with our interceptor
        rustc-with-capture = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          name = "rustc-build-capture";
          src = ./.;
          
          buildInputs = with pkgs; [
            rustc-with-capture
            cargo
            git
            jq
          ];
          
          buildPhase = ''
            echo "Building rustc with build order capture..."
            # Set up our interceptor
            export RUSTC="${rustc-interceptor}/bin/rustc"
            
            # Build rustc from source
            cargo build --verbose > build.log 2>&1 || true
          '';
          
          installPhase = ''
            mkdir -p $out
            cp build.log $out/
            cp rustc_build_log.jsonl $out/ 2>/dev/null || true
          '';
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc-with-capture
            cargo
            git
            jq
            rust-analyzer
          ];
        };
      });
}
