{
  description = "Bitcoin script: P2PKH (Pay-to-PubKey-Hash)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.writeShellScriptBin "P2PKH" ''
        #!/usr/bin/env bash
        # Bitcoin Script: P2PKH (Pay-to-PubKey-Hash)
        
        echo "Script Type: Pay-to-PubKey-Hash"
        echo "Name: P2PKH"
        
        # Example script pattern
        case "Pay-to-PubKey-Hash" in
          "Pay-to-PubKey-Hash")
            echo "OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG"
            ;;
          "Pay-to-Script-Hash")
            echo "OP_HASH160 <scriptHash> OP_EQUAL"
            ;;
          "SegWit v0")
            echo "OP_0 <pubKeyHash>"
            ;;
          "Taproot")
            echo "OP_1 <witnessProgram>"
            ;;
          *)
            echo "Script: Pay-to-PubKey-Hash"
            ;;
        esac
      '';
    };
}
