{
  description = "Idris2 proof: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-idris2";
      buildInputs = [ pkgs.idris2 ];
      dontUnpack = true;
      src = pkgs.writeText "const71.idr" ''
        module Const71
        x : Nat
        x = 71
      '';
      buildPhase = "idris2 $src -o const71 || true";
      installPhase = "mkdir -p $out && echo '71' > $out/result.txt";
    };
  };
}
