{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "zos-server";
  version = "0.1.0";

  src = ~/zos-qa;

  cargoLock = {
    lockFile = ~/zos-qa/Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    openssl
  ];

  # Build only the server binary
  cargoBuildFlags = [ "--bin" "zos_server" ];

  meta = with pkgs.lib; {
    description = "ZOS Self-Deploying Server";
    license = licenses.agpl3;
  };
}
