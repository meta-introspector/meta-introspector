{
  description = "RDF/Turtle const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "turtle-71";
      src = pkgs.writeText "const71.ttl" ''
        @prefix ex: <http://example.org/> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        
        ex:const_x a ex:Number ;
                   ex:value "71"^^xsd:integer .
      '';
      
      dontUnpack = true;
      
      buildPhase = ''
        grep -q "71" $src || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out
        cp $src $out/const71.ttl
        echo "71" > $out/result.txt
      '';
    };
  };
}
