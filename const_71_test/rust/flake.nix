{
  description = "Rust const x = 71 test";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "const-71-rust";
        version = "0.1.0";
        
        src = pkgs.writeTextDir "src/main.rs" ''
          fn main() {
              const X: i32 = 71;
              println!("x = {}", X);
          }
        '';
        
        cargoLock = {
          lockFile = pkgs.writeText "Cargo.lock" ''
            version = 3
            [[package]]
            name = "const-71-rust"
            version = "0.1.0"
          '';
        };
      };
    };
}
