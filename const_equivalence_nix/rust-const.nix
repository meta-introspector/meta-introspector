{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage {
  pname = "const-test-rust";
  version = "0.1.0";
  src = pkgs.writeTextDir "src/main.rs" ''
    fn main() {
        const X: i32 = 71;
        println!("{}", X);
    }
  '';
  cargoLock.lockFile = pkgs.writeText "Cargo.lock" "";
  cargoSha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
}
