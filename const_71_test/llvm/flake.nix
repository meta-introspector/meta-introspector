{
  description = "LLVM/Clang const x = 71 test";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-llvm";
        
        src = pkgs.writeText "main.cpp" ''
          #include <iostream>
          
          int main() {
              const int x = 71;
              std::cout << "x = " << x << std::endl;
              return 0;
          }
        '';
        
        buildInputs = [ pkgs.clang pkgs.llvm ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          clang++ -O0 -g $src -o const-71-llvm
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp const-71-llvm $out/bin/
        '';
      };
    };
}
