{
  description = "Unity: Central control flake for all meta-introspector repos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      
      # Unity version
      version = "v1";
      
      # Standard Rust build function
      buildRustPackage = { pname, src, cargoLock ? "${src}/Cargo.lock" }:
        pkgs.rustPlatform.buildRustPackage {
          inherit pname src;
          version = "unity-${version}";
          cargoLock.lockFile = cargoLock;
        };
      
      # Standard tools available to all repos
      unityTools = pkgs.buildEnv {
        name = "unity-tools";
        paths = with pkgs; [
          git
          nix
          rustc
          cargo
        ];
      };
      
    in {
      # Export for other flakes to use
      lib = {
        inherit version buildRustPackage unityTools;
        
        # Standard package builder
        mkPackage = { src, pname ? "repo" }:
          if builtins.pathExists "${src}/Cargo.toml"
          then buildRustPackage { inherit pname src; }
          else pkgs.stdenv.mkDerivation {
            name = pname;
            inherit src;
            installPhase = "mkdir -p $out && cp -r . $out/";
          };
      };
      
      packages.${system} = {
        unity-tools = unityTools;
      };
      
      # Standard dev shell for all repos
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [ unityTools ];
        shellHook = ''
          echo "🌐 Unity ${version} - Meta-Introspector Control System"
        '';
      };
    };
}
