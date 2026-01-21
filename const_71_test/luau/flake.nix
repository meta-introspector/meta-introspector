{
  description = "Luau (Roblox Lua): const 71";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "luau-71";
      
      src = pkgs.writeText "const71.lua" ''
        local const71 = 71
        print(const71)
      '';
      
      nativeBuildInputs = [ pkgs.luau ];
      dontUnpack = true;
      
      buildPhase = ''
        # Verify it compiles/runs
        luau $src > output.txt
        if ! grep -q "71" output.txt; then
          echo "ERROR: Luau didn't output 71"
          exit 1
        fi
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.lua
        cat > $out/bin/luau-71 << 'SCRIPT'
#!/bin/sh
${pkgs.luau}/bin/luau $(dirname $0)/const71.lua
SCRIPT
        chmod +x $out/bin/luau-71
      '';
    };
  };
}
