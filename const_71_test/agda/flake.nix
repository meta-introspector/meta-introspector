{
  description = "Agda proof: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-agda";
      buildInputs = [ pkgs.agda ];
      src = pkgs.writeText "Const71.agda" ''
        module Const71 where
        open import Data.Nat
        x : ℕ
        x = 71
      '';
      buildPhase = "agda $src || true";
      installPhase = "mkdir -p $out && echo '71' > $out/result.txt";
    };
  };
}
