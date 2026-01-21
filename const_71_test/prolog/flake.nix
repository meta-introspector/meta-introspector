{
  description = "Prolog: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-prolog";
      buildInputs = [ pkgs.swiProlog ];
      dontUnpack = true;
      src = pkgs.writeText "const71.pl" ''
        x(71).
        :- x(X), write(X), nl, halt.
      '';
      buildPhase = "swipl -q -s $src > result.txt";
      installPhase = "mkdir -p $out && cp result.txt $out/";
    };
  };
}
