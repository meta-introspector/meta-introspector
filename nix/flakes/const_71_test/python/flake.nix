{
  description = "Python const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-python";
        
        src = pkgs.writeText "main.py" ''
          X = 71
          print(f"x = {X}")
        '';
        
        buildInputs = [ pkgs.python3 ];
      dontUnpack = true;
        
        unpackPhase = "true";
        
        buildPhase = ''
          python3 -m py_compile $src
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          echo '#!/usr/bin/env python3' > $out/bin/const-71-python
          cat $src >> $out/bin/const-71-python
          chmod +x $out/bin/const-71-python
        '';
      };
    };
}
