{
  description = "Standard Rust project with analysis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    meta-introspector.url = "github:meta-introspector/meta-introspector";
  };

  outputs = { self, nixpkgs, meta-introspector }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      analyzers = meta-introspector.lib.analyzers;
      
      package = pkgs.rustPlatform.buildRustPackage {
        pname = "project";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
      
    in {
      packages.${system} = {
        default = package;
        analyzed = analyzers.withFullAnalysis package;
      };
    };
}
