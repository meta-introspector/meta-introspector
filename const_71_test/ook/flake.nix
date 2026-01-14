{
  description = "ook: const x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "ook-71" ''
      echo "ook: x = 71"
    '';
  };
}
