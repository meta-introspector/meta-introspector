{
  description = "Zig const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "zig-71";
      src = pkgs.writeText "const71.zig" ''
        const std = @import("std");
        
        pub fn main() void {
            std.debug.print("71\n", .{});
        }
      '';
      
      nativeBuildInputs = [ pkgs.zig ];
      
      dontUnpack = true;
      
      buildPhase = ''
        ${pkgs.zig}/bin/zig build-exe $src -O ReleaseFast -femit-bin=const71
        ./const71 2>&1 | tee output.txt
        grep -q "71" output.txt || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp const71 $out/bin/zig-71
      '';
    };
  };
}
