{
  description = "ZOS - Zero Operating System Foundation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    unity.url = "github:meta-introspector/meta-introspector/v1?dir=zos/unity";
  };

  outputs = { self, nixpkgs, unity }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      
      # Build all ZOS components
      zos-server = pkgs.rustPlatform.buildRustPackage {
        pname = "zos-server";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
      
      # Build all SO plugins
      zos-plugins = pkgs.stdenv.mkDerivation {
        name = "zos-plugins";
        src = ./tools/so-plugins;
        buildPhase = ''
          for plugin in dns-server file-proxy github-proxy nix-proxy llm-proxy; do
            cd $plugin
            cargo build --release
            cd ..
          done
        '';
        installPhase = ''
          mkdir -p $out/lib
          cp target/release/*.so $out/lib/
        '';
      };
      
      # ZOS init replacement
      zos-init = pkgs.writeShellScriptBin "zos-init" ''
        #!/bin/sh
        echo "🚀 ZOS Init - PID 1"
        
        # Mount essential filesystems
        mount -t proc proc /proc
        mount -t sysfs sys /sys
        mount -t devtmpfs dev /dev
        
        # Setup SELinux
        ${pkgs.libselinux}/bin/load_policy
        
        # Start ZOS server with all plugins
        exec ${zos-server}/bin/zos_server \
          --plugin ${zos-plugins}/lib/libzos_dns_server.so \
          --plugin ${zos-plugins}/lib/libzos_file_proxy.so \
          --plugin ${zos-plugins}/lib/libzos_github_proxy.so \
          --plugin ${zos-plugins}/lib/libzos_nix_proxy.so \
          --plugin ${zos-plugins}/lib/libzos_llm_proxy.so
      '';
      
    in {
      packages.${system} = {
        default = zos-server;
        zos-server = zos-server;
        zos-plugins = zos-plugins;
        zos-init = zos-init;
        
        # Complete ZOS system
        zos-system = pkgs.buildEnv {
          name = "zos-system";
          paths = [
            zos-server
            zos-plugins
            zos-init
            pkgs.libselinux
            pkgs.iptables
            pkgs.cgroup-utils
          ];
        };
      };
      
      # NixOS module for ZOS
      nixosModules.zos = { config, lib, pkgs, ... }: {
        options.services.zos = {
          enable = lib.mkEnableOption "ZOS Foundation";
          replaceInit = lib.mkOption {
            type = lib.types.bool;
            default = false;
            description = "Replace systemd with ZOS init";
          };
        };
        
        config = lib.mkIf config.services.zos.enable {
          # Install ZOS
          environment.systemPackages = [ self.packages.${system}.zos-system ];
          
          # ZOS systemd services (layered)
          systemd.services.zos-root = {
            description = "ZOS Foundation - Root Layer (L1)";
            wantedBy = [ "multi-user.target" ];
            serviceConfig = {
              Type = "simple";
              ExecStart = "${zos-server}/bin/zos_server --layer root";
              User = "root";
              SELinuxContext = "system_u:system_r:zos_level1_t:s0";
            };
          };
          
          systemd.services.zos-user = {
            description = "ZOS Foundation - User Layer (L2)";
            wantedBy = [ "multi-user.target" ];
            after = [ "zos-root.service" ];
            serviceConfig = {
              Type = "simple";
              ExecStart = "${zos-server}/bin/zos_server --layer user";
              User = "zos";
              SELinuxContext = "system_u:system_r:zos_level2_t:s0";
            };
          };
          
          # Create zos user
          users.users.zos = {
            isSystemUser = true;
            group = "zos";
          };
          users.groups.zos = {};
          
          # Replace init (if enabled)
          boot.initrd.systemd.enable = lib.mkIf config.services.zos.replaceInit false;
          boot.kernelParams = lib.mkIf config.services.zos.replaceInit [
            "init=${zos-init}/bin/zos-init"
          ];
        };
      };
    };
}
