{
  description = "Canonical build command library for Nix";
  
  outputs = { self, nixpkgs }: {
    lib = {
      # Canonical nix build wrapper
      nixBuild = { pkgs, target, args ? [] }: ''
        ${pkgs.nix}/bin/nix build ${target} ${builtins.concatStringsSep " " args}
      '';
      
      # Canonical cargo build wrapper
      cargoBuild = { pkgs, args ? [] }: ''
        ${pkgs.cargo}/bin/cargo build ${builtins.concatStringsSep " " args}
      '';
      
      # Canonical flake update wrapper
      flakeUpdate = { pkgs, args ? [] }: ''
        ${pkgs.nix}/bin/nix flake update ${builtins.concatStringsSep " " args}
      '';
    };
  };
}
