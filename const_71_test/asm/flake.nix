{
  description = "Assembly x86_64 const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-asm";
        
        src = pkgs.writeText "main.asm" ''
          section .data
              msg db "x = 71", 10
              len equ $ - msg

          section .text
              global _start

          _start:
              ; write(1, msg, len)
              mov rax, 1
              mov rdi, 1
              mov rsi, msg
              mov rdx, len
              syscall
              
              ; exit(0)
              mov rax, 60
              xor rdi, rdi
              syscall
        '';
        
        buildInputs = [ pkgs.nasm ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          nasm -f elf64 $src -o const-71.o
          ld const-71.o -o const-71-asm
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp const-71-asm $out/bin/
        '';
      };
    };
}
