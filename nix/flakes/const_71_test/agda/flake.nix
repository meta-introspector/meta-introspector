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

        data Nat : Set where
          zero : Nat
          suc  : Nat → Nat

        {-# BUILTIN NATURAL Nat #-}

        const71 : Nat
        const71 = 71
      '';
      dontUnpack = true;
      buildPhase = ''
        mkdir -p /build
        cp $src /build/Const71.agda
        cd /build
        agda Const71.agda
      '';
      installPhase = "mkdir -p $out && echo '71' > $out/result.txt";
    };
  };
}
