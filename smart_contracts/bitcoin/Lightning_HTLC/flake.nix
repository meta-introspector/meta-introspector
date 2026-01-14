{
  description = "Bitcoin script: Lightning_HTLC (Lightning)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.writeShellScriptBin "Lightning_HTLC" ''
        #!/usr/bin/env bash
        # Bitcoin Script: Lightning_HTLC (Lightning)
        
        echo "Script Type: Lightning"
        echo "Name: Lightning_HTLC"
        
        # Example script pattern
        case "Lightning" in
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
            echo "Script: Lightning"
            ;;
        esac
      '';
    };
}
