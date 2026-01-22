{
  description = "lua: const x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "lua-71";
      src = pkgs.writeText "const71.lua" ''
        local x = 71
        print("x = " .. x)
      '';
      nativeBuildInputs = [ pkgs.lua ];
      dontUnpack = true;
      buildPhase = ''
        lua $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.lua
        cat > $out/bin/lua-71 << 'EOF'
#!/bin/sh
${pkgs.lua}/bin/lua $(dirname $0)/const71.lua
EOF
        chmod +x $out/bin/lua-71
      '';
    };
  };
}
