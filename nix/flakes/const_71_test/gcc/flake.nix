{
  description = "GCC const x = 71 test";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-gcc";
        
        src = pkgs.writeText "main.c" ''
          #include <stdio.h>
          
          int main() {
              const int x = 71;
              printf("x = %d\n", x);
              return 0;
          }
        '';
        
        buildInputs = [ pkgs.gcc ];
      dontUnpack = true;
        
        unpackPhase = "true";
        
        buildPhase = ''
          gcc -O0 -g $src -o const-71-gcc
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp const-71-gcc $out/bin/
        '';
      };
    };
}
