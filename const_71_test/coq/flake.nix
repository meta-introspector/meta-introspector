{
  description = "Coq proof: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-coq";
      buildInputs = [ pkgs.coq ];
      dontUnpack = true;
      src = pkgs.writeText "const71.v" ''
        Definition x : nat := 71.
        Theorem x_is_71 : x = 71.
        Proof. reflexivity. Qed.
      '';
      buildPhase = "cp $src const71.v && coqc const71.v";
      installPhase = "mkdir -p $out && echo '71' > $out/result.txt";
    };
  };
}
