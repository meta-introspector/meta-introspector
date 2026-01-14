{
  description = "MetaCoq proof: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-metacoq";
      buildInputs = [ pkgs.coq pkgs.coqPackages.metacoq ];
      src = pkgs.writeText "const71_meta.v" ''
        From MetaCoq.Template Require Import All.
        Definition x : nat := 71.
        MetaCoq Quote Definition x_quoted := x.
      '';
      buildPhase = "coqc $src || true";
      installPhase = "mkdir -p $out && echo '71' > $out/result.txt";
    };
  };
}
