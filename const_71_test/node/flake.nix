{
  description = "Node.js const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-node";
        
        src = pkgs.writeText "main.js" ''
          const x = 71;
          console.log(`x = ''${x}`);
        '';
        
        buildInputs = [ pkgs.nodejs ];
      dontUnpack = true;
        
        unpackPhase = "true";
        
        buildPhase = "true";
        
        installPhase = ''
          mkdir -p $out/bin
          echo '#!/usr/bin/env node' > $out/bin/const-71-node
          cat $src >> $out/bin/const-71-node
          chmod +x $out/bin/const-71-node
        '';
      };
    };
}
