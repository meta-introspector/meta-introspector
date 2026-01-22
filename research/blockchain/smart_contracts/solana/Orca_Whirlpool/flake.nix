{
  description = "Solana contract with solflake dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    solflake.url = "github:nasadorian/solflake";
  };

  outputs = { self, nixpkgs, solflake }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      contractName = builtins.baseNameOf ./.;
    in
    {
      packages.${system}.default = pkgs.writeTextDir "contract-info.json" (builtins.toJSON {
        name = contractName;
        blockchain = "solana";
        devShell = "nix develop";
      });

      devShells.${system}.default = solflake.devShells.${system}.default;
    };
}
