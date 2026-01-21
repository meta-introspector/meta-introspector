{
  description = "OpenTofu const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "opentofu-71";
      src = pkgs.writeText "const71.tf" ''
        variable "const_71" {
          type    = number
          default = 71
        }
        
        output "result" {
          value = var.const_71
        }
      '';
      nativeBuildInputs = [ pkgs.opentofu ];
      dontUnpack = true;
      
      buildPhase = ''
        ${pkgs.opentofu}/bin/tofu init
        ${pkgs.opentofu}/bin/tofu plan -var="const_71=71" > output.txt
        grep -q "71" output.txt || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.tf
        cat > $out/bin/opentofu-71 << 'SCRIPT'
#!/bin/sh
${pkgs.opentofu}/bin/tofu plan -var="const_71=71"
SCRIPT
        chmod +x $out/bin/opentofu-71
      '';
    };
  };
}
