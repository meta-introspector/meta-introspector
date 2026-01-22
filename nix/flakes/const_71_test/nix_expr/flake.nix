{
  description = "Nix expression: const x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    x = 71;
  in {
    packages.${system}.default = pkgs.writeText "nix-71" "${toString x}";
  };
}
