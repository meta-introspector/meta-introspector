{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {
  name = "const-test-cpp";
  src = pkgs.writeText "test.cpp" ''
    #include <iostream>
    int main() {
        const int x = 71;
        std::cout << x << std::endl;
        return 0;
    }
  '';
  buildInputs = [ pkgs.gcc ];
  buildPhase = ''
    g++ -g -O0 $src -o test
  '';
  installPhase = ''
    mkdir -p $out/bin
    cp test $out/bin/
  '';
}
