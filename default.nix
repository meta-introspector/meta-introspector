{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "meta-introspector";
  version = "0.1.0";

  src = builtins.filterSource
    (path: type: 
      let baseName = baseNameOf path;
      in !(baseName == "data" || 
           baseName == "target" ||
           baseName == ".git" ||
           baseName == "result"))
    ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    openssl
  ];

  # Build all workspace binaries
  cargoBuildFlags = [ "--workspace" "--bins" ];

  installPhase = ''
    mkdir -p $out/bin
    find target/release -maxdepth 1 -type f -executable ! -name "*.so" -exec cp {} $out/bin/ \;
  '';

  meta = with pkgs.lib; {
    description = "Meta-Introspector - All Binaries";
    license = licenses.agpl3;
  };
}
