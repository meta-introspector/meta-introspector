{
  description = "Ethereum contract: Uniswap_V3_Router";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.writeTextDir "contract-info.json" (builtins.toJSON {
        name = "Uniswap_V3_Router";
        address = "0xE592427A0AEce92De3Edee1F18E0157C05861564";
        blockchain = "ethereum";
        type = "DEX Router";
      });
    };
}
