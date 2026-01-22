{
  description = "UniMath const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    unimath = pkgs.stdenv.mkDerivation {
      name = "coq-unimath-${pkgs.coq.coq-version}";
      
      src = pkgs.fetchgit {
        url = "https://github.com/UniMath/UniMath.git";
        rev = "a2714eca29444a595cd280ea961ec33d17712009";
        sha256 = "0brhbslx4sxl8m9nxjbdl91gi99vcrikykl6b00f4cx5ww43csln";
      };
      
      buildInputs = [ pkgs.coq.ocaml pkgs.coq.camlp5 ];
      propagatedBuildInputs = [ pkgs.coq ];
      
      installFlags = [ "COQLIB=$(out)/lib/coq/${pkgs.coq.coq-version}/" ];
    };
    
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "unimath-71";
      buildInputs = [ pkgs.coq unimath ];
      dontUnpack = true;
      
      src = pkgs.writeText "const71.v" ''
        Require Import UniMath.Foundations.All.
        
        Definition x : nat := 71.
        
        Lemma x_is_71 : x = 71.
        Proof. reflexivity. Qed.
      '';
      
      buildPhase = ''
        cp $src const71.v
        coqc -R ${unimath}/lib/coq/${pkgs.coq.coq-version}/user-contrib/UniMath UniMath const71.v
      '';
      
      installPhase = ''
        mkdir -p $out
        echo "71" > $out/result.txt
      '';
    };
  };
}
