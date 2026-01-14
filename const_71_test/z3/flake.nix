{
  description = "Z3 SMT: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-z3";
      buildInputs = [ pkgs.z3 ];
      src = pkgs.writeText "const71.smt2" ''
        (declare-const x Int)
        (assert (= x 71))
        (check-sat)
        (get-value (x))
      '';
      buildPhase = "z3 $src > result.txt";
      installPhase = "mkdir -p $out && cp result.txt $out/";
    };
  };
}
