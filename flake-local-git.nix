# Nix Flake with Local Git Mirror

{
  description = "Build system using local git mirror";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Git config for local mirror
      localGitConfig = pkgs.writeText "local-git-config" ''
        [url "file:///mnt/data1/git/github.com/"]
          insteadOf = https://github.com/
          insteadOf = git@github.com:
        [url "file:///mnt/data1/git/gitlab.com/"]
          insteadOf = https://gitlab.com/
      '';
      
    in {
      # Overlay to inject local git config
      overlays.default = final: prev: {
        buildWithLocalGit = drv: drv.overrideAttrs (old: {
          GIT_CONFIG_GLOBAL = localGitConfig;
          GIT_CONFIG_SYSTEM = localGitConfig;
        });
      };
      
      # Dev shell with local git configured
      devShells.${system}.default = pkgs.mkShell {
        GIT_CONFIG_GLOBAL = localGitConfig;
        
        shellHook = ''
          echo "🔧 Using local git mirror at /mnt/data1/git"
          export GIT_CONFIG_GLOBAL=${localGitConfig}
        '';
        
        buildInputs = with pkgs; [
          git
          cargo
          rustc
        ];
      };
    };
}
