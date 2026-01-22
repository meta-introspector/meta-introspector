{
  description = "JSON-LD const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "jsonld-71";
      src = pkgs.writeText "const71.jsonld" ''
        {
          "@context": "https://schema.org/",
          "@type": "Number",
          "value": 71,
          "name": "const_x"
        }
      '';
      
      nativeBuildInputs = [ pkgs.jq ];
      
      dontUnpack = true;
      
      buildPhase = ''
        ${pkgs.jq}/bin/jq '.value' $src | grep -q "71" || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out
        cp $src $out/const71.jsonld
        echo "71" > $out/result.txt
      '';
    };
  };
}
