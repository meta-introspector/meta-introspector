{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {
  name = "const-test-c";
  src = pkgs.writeText "test.c" ''
    #include <stdio.h>
    int main() {
        const int x = 71;
        printf("%d\n", x);
        return 0;
    }
  '';
  buildInputs = [ pkgs.gcc ];
  buildPhase = ''
    gcc -g -O0 $src -o test
  '';
  installPhase = ''
    mkdir -p $out/bin
    cp test $out/bin/
  '';
}
