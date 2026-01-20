{
  description = "Dioxus CLI v0.7";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      
      rustToolchain = pkgs.rust-bin.stable.latest.default;
      
    in {
      packages.${system} = {
        dioxus-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "dioxus-cli";
          version = "0.7.0";
          
          src = pkgs.fetchFromGitHub {
            owner = "DioxusLabs";
            repo = "dioxus";
            rev = "v${version}";
            sha256 = pkgs.lib.fakeSha256;
          };
          
          sourceRoot = "${src.name}/packages/cli";
          
          cargoHash = pkgs.lib.fakeHash;
          
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
          ];
          
          buildInputs = with pkgs; [
            openssl
          ];
        };
        
        default = self.packages.${system}.dioxus-cli;
      };
    };
}
