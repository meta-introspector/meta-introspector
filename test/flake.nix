{
  description = "Minimal test build";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.stdenv.mkDerivation {
      name = "test-71";
      
      src = nixpkgs.legacyPackages.x86_64-linux.writeText "hello.c" ''
        #include <stdio.h>
        int main() {
          printf("71\n");
          return 0;
        }
      '';
      
      unpackPhase = "true";
      
      buildPhase = ''
        $CC $src -o hello
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp hello $out/bin/
      '';
    };
  };
}
