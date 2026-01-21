{
  description = "SMT-LIB2: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-smt2";
      buildInputs = [ pkgs.cvc5 ];
      dontUnpack = true;
      src = pkgs.writeText "const71.smt2" ''
        (set-logic QF_LIA)
        (declare-const x Int)
        (assert (= x 71))
        (check-sat)
        (get-model)
      '';
      buildPhase = "cvc5 $src > result.txt";
      installPhase = "mkdir -p $out && cp result.txt $out/";
    };
  };
}
