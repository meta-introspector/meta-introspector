{
  description = "x86_64 assembly: const x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "asm-x86_64-71" ''
      echo "x86_64: mov x, 71"
    '';
  };
}
