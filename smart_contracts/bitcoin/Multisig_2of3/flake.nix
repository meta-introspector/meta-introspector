{
  description = "Bitcoin script: Multisig_2of3 (Multisig)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.writeShellScriptBin "Multisig_2of3" ''
        #!/usr/bin/env bash
        # Bitcoin Script: Multisig_2of3 (Multisig)
        
        echo "Script Type: Multisig"
        echo "Name: Multisig_2of3"
        
        # Example script pattern
        case "Multisig" in
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
            echo "Script: Multisig"
            ;;
        esac
      '';
    };
}
