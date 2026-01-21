{
  description = "MiniZinc constraint: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-minizinc";
      buildInputs = [ pkgs.minizinc ];
      dontUnpack = true;
      src = pkgs.writeText "const71.mzn" ''
        var 71..71: x;
        constraint x = 71;
        solve satisfy;
        output ["x = \(x)\n"];
      '';
      buildPhase = "minizinc $src > result.txt";
      installPhase = "mkdir -p $out && cp result.txt $out/";
    };
  };
}
