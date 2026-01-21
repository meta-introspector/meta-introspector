{
  description = "Brainfuck const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-bf";
        
        # Brainfuck program that prints "x = 71\n"
        src = pkgs.writeText "main.bf" ''
          +++++ +++++ +++++ +++++ +++++ +++++ +++++ +++++ +++++ +++++ +++++ +++++ .  x (120)
          ---.                                                                        space (32)
          +++++ +++++ +.                                                             = (61)
          ---.                                                                        space (32)
          +++++ +++++ +++++ +.                                                       7 (55)
          ++++.                                                                       1 (49)
          +++++ +++++ .                                                              newline (10)
        '';
        
        buildInputs = [ pkgs.bf ];
      dontUnpack = true;
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Create wrapper script that runs brainfuck
          cat > const-71-bf << 'EOF'
          #!/bin/sh
          exec ${pkgs.bf}/bin/bf ${"\${BASH_SOURCE[0]}.bf"}
          EOF
          chmod +x const-71-bf
          cp $src const-71-bf.bf
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp const-71-bf $out/bin/
          cp const-71-bf.bf $out/bin/
        '';
      };
    };
}
