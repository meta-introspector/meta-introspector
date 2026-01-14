{
  description = "Rust libc interceptor with libbpf";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          pkg-config
          elfutils
          libelf
          zlib
        ];
        
        LIBELF_INCLUDE = "${pkgs.elfutils.dev}/include";
        LIBELF_LIB = "${pkgs.elfutils}/lib";
      };
    };
}
