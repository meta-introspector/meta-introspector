{
  description = "Bitcoin Core node for economic weight analysis";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.bitcoin;
      
      # Development shell with bitcoin tools
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          bitcoin
          bitcoind
        ];
        
        shellHook = ''
          echo "🪙 Bitcoin Core Environment"
          echo "=========================="
          echo "bitcoind version: $(bitcoind --version | head -1)"
          echo ""
          echo "Commands:"
          echo "  bitcoind -regtest -daemon  # Start regtest node"
          echo "  bitcoin-cli -regtest getblockchaininfo"
          echo "  bitcoin-cli -regtest stop"
          echo ""
          echo "Data dir: ~/.bitcoin"
        '';
      };
      
      # App to run bitcoind
      apps.${system}.default = {
        type = "app";
        program = "${pkgs.bitcoin}/bin/bitcoind";
      };
    };
}
