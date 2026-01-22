{
  description = "AMD x86-64 ASM const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "amd-asm-71";
      src = pkgs.writeText "const71.asm" ''
        section .data
            msg db '71', 10
            len equ $ - msg
        
        section .text
            global _start
        
        _start:
            mov rax, 1      ; sys_write
            mov rdi, 1      ; stdout
            mov rsi, msg
            mov rdx, len
            syscall
            
            mov rax, 60     ; sys_exit
            xor rdi, rdi
            syscall
      '';
      
      nativeBuildInputs = [ pkgs.nasm ];
      
      dontUnpack = true;
      
      buildPhase = ''
        nasm -f elf64 $src -o const71.o
        ld const71.o -o const71
        ./const71 > output.txt
        grep -q "71" output.txt || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp const71 $out/bin/amd-asm-71
      '';
    };
  };
}
