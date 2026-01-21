{
  description = "CMake build system: const 71";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "cmake-71";
      
      src = pkgs.runCommand "cmake-src" {} ''
        mkdir -p $out
        cat > $out/CMakeLists.txt << 'EOF'
cmake_minimum_required(VERSION 3.10)
project(const71)
add_executable(const71 main.c)
EOF
        cat > $out/main.c << 'EOF'
#include <stdio.h>
int main() {
  printf("71\n");
  printf("Built with: CMake\n");
  return 0;
}
EOF
      '';
      
      nativeBuildInputs = [ pkgs.cmake ];
      
      buildPhase = ''
        cmake .
        make
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp const71 $out/bin/cmake-71
      '';
    };
  };
}
