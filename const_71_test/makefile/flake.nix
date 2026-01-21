{
  description = "Makefile build system: const 71";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "makefile-71";
      
      src = pkgs.writeTextDir "Makefile" ''
        all:
        	@echo "71"
        	@echo "Built with: GNU Make"
      '';
      
      buildPhase = ''
        make
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        echo '#!/bin/sh' > $out/bin/makefile-71
        echo 'make -f - <<EOF' >> $out/bin/makefile-71
        echo 'all:' >> $out/bin/makefile-71
        echo '	@echo "71"' >> $out/bin/makefile-71
        echo 'EOF' >> $out/bin/makefile-71
        chmod +x $out/bin/makefile-71
      '';
    };
  };
}
