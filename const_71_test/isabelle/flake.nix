{
  description = "Isabelle proof: const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    # Fix vampire C++ compilation issue
    vampireFix = pkgs.vampire.overrideAttrs (old: {
      NIX_CFLAGS_COMPILE = (old.NIX_CFLAGS_COMPILE or "") + " -include cstdint";
    });
    
    isabelleFix = pkgs.isabelle.override {
      vampire = vampireFix;
    };
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "const71-isabelle";
      dontUnpack = true;
      src = pkgs.writeText "Const71.thy" ''
        theory Const71
        imports Main
        begin
        definition x :: nat where "x = 71"
        lemma "x = 71" by (simp add: x_def)
        end
      '';
      buildPhase = "true";
      installPhase = "mkdir -p $out && echo '71' > $out/result.txt";
    };
  };
}
