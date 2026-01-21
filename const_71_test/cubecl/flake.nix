{
  description = "CubeCL const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      name = "cubecl-71";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
  };
}
