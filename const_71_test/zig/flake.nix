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
        
        pub fn main() !void {
            const x: u32 = 71;
            const stdout = std.io.getStdOut().writer();
            try stdout.print("{d}\n", .{x});
        }
      '';
      
      nativeBuildInputs = [ pkgs.zig ];
      
      dontUnpack = true;
      
      buildPhase = ''
        ${pkgs.zig}/bin/zig build-exe $src -O ReleaseFast
        ./const71 > output.txt
        grep -q "71" output.txt || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp const71 $out/bin/zig-71
      '';
    };
  };
}
