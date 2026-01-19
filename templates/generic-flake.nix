{
  description = "Standard generic project with analysis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    meta-introspector.url = "github:meta-introspector/meta-introspector";
  };

  outputs = { self, nixpkgs, meta-introspector }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      analyzers = meta-introspector.lib.analyzers;
      
      package = pkgs.stdenv.mkDerivation {
        pname = "project";
        version = "0.1.0";
        src = ./.;
        buildPhase = "true";
        installPhase = "mkdir -p $out && cp -r . $out/";
      };
      
    in {
      packages.${system} = {
        default = package;
        analyzed = analyzers.withFullAnalysis package;
      };
    };
}
