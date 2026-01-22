{
  description = "Nix flake build system: const 71";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "nix-flake-71" ''
      # This IS a Nix flake, so just output 71
      echo "71"
    '';
  };
}
