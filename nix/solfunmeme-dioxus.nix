{
  description = "SOLFUNMEME Dioxus - Built from local git mirror";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    solfunmeme-dioxus = {
      url = "github:meta-introspector/solfunmeme-dioxus/feature/block-collector";
      flake = true;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, solfunmeme-dioxus }:
    let
      system = "x86_64-linux";
    in {
      packages.${system} = {
        default = solfunmeme-dioxus.packages.${system}.default;
      };
      
      devShells.${system}.default = solfunmeme-dioxus.devShells.${system}.default;
    };
}
