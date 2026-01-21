{
  description = "SGML const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "sgml-71";
      src = pkgs.writeText "const71.sgml" ''
        <!DOCTYPE doc [
          <!ELEMENT doc (#PCDATA)>
          <!ENTITY const "71">
        ]>
        <doc>&const;</doc>
      '';
      
      dontUnpack = true;
      
      buildPhase = ''
        grep -q "71" $src || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out
        cp $src $out/const71.sgml
        echo "71" > $out/result.txt
      '';
    };
  };
}
