{
  description = "Mes-Transformer: Computational Omniscience Architecture";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mes-bootstrap.url = "path:../mes-bootstrap-proof";
  };
  
  outputs = { self, nixpkgs, mes-bootstrap }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system} = {
      # Tiny Transformer (CPU-only for now, GPU via manual build)
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "mes-transformer";
        version = "0.1.0";
        src = ./rust;
        cargoLock.lockFile = ./rust/Cargo.lock;
        
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl pkgs.zstd ];
        
        # Build without GPU feature for nix
        # For GPU: cargo build --release --features gpu
      };
    };
  };
}
