{
  description = "Build logs to Parquet converter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "build-logs-to-parquet";
        version = "0.1.0";
        
        src = ./.;
        
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ ];
        
        meta = {
          description = "Convert Nix build logs to Parquet format";
        };
      };
    };
}
