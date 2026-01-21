{
  description = "Terraform infrastructure as code: const 71";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "terraform-71";
      
      src = pkgs.runCommand "terraform-src" {} ''
        mkdir -p $out
        
        # main.tf
        cat > $out/main.tf << 'EOF'
terraform {
  required_version = ">= 1.0"
}

locals {
  const_value = 71
}

output "result" {
  value = local.const_value
}
EOF
      '';
      
      nativeBuildInputs = [ pkgs.terraform ];
      
      buildPhase = ''
        terraform init
        terraform plan
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cat > $out/bin/terraform-71 << 'SCRIPT'
#!/bin/sh
echo "71"
SCRIPT
        chmod +x $out/bin/terraform-71
      '';
    };
  };
}
