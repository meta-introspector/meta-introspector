{
  description = "OCaml const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-ocaml";
        
        src = pkgs.writeText "main.ml" ''
          let x = 71 in
          Printf.printf "x = %d\n" x
        '';
        
        buildInputs = [ pkgs.ocaml ];
      dontUnpack = true;
        
        unpackPhase = "true";
        
        buildPhase = ''
          ocamlopt -o const-71-ocaml $src
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp const-71-ocaml $out/bin/
        '';
      };
    };
}
