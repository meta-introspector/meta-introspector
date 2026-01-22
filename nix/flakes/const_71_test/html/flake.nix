{
  description = "HTML const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "html-71";
      src = pkgs.writeText "const71.html" ''
        <!DOCTYPE html>
        <html>
        <head><title>Const 71</title></head>
        <body>
          <script>const x = 71; document.write(x);</script>
        </body>
        </html>
      '';
      
      dontUnpack = true;
      
      buildPhase = ''
        grep -q "71" $src || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out
        cp $src $out/const71.html
        echo "71" > $out/result.txt
      '';
    };
  };
}
