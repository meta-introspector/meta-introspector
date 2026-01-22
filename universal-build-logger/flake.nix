{
  description = "Universal build logger - captures all build metadata in /nix/store";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      lib.wrap = { pkgs, self, project, projectName ? "unknown" }:
        let
          buildTime = "1737209000"; # Static timestamp
          gitCommit = self.rev or "dirty";
          gitBranch = self.ref or "unknown";
          
          # Create log derivation
          logDrv = pkgs.runCommand "${projectName}-build-log" {
            inherit buildTime gitCommit gitBranch;
          } ''
            mkdir -p $out/{1-upstream,2-fork-state,4-collected-info}
            
            # 1. Upstream state
            cat > $out/1-upstream/info.json <<EOF
            {
              "project": "${projectName}",
              "upstream_url": "unknown",
              "last_checked": ${toString buildTime}
            }
            EOF
            
            # 2. Fork state
            cat > $out/2-fork-state/info.json <<EOF
            {
              "branch": "$gitBranch",
              "commit": "$gitCommit",
              "build_time": ${toString buildTime}
            }
            EOF
            
            # 3. Build state (placeholder)
            cat > $out/3-build-state.json <<EOF
            {
              "status": "pending",
              "system": "${system}",
              "nix_version": "${builtins.nixVersion}"
            }
            EOF
            
            # 4. Collected info
            cat > $out/4-collected-info/info.json <<EOF
            {
              "analysis_phases": {
                "source_archive": "pending",
                "ngrams": "pending",
                "markov": "pending",
                "embeddings": "pending"
              }
            }
            EOF
            
            # 5. Missing info
            cat > $out/5-missing.json <<EOF
            {
              "missing": ["full analysis", "dependencies", "tests"]
            }
            EOF
            
            # Summary
            cat > $out/summary.json <<EOF
            {
              "project": "${projectName}",
              "git_commit": "$gitCommit",
              "log_derivation": "$out"
            }
            EOF
          '';
        in
        pkgs.runCommand "${projectName}-with-logs" {
          inherit logDrv;
          nativeBuildInputs = [ pkgs.linuxPackages.perf pkgs.strace ];
        } ''
          mkdir -p $out/perf-data
          
          # Try to build project with perf collection
          set +e
          if [ -d ${project} ]; then
            echo "Project is a directory: ${project}" > build.log
            if [ -f ${project}/bin/* ]; then
              # Collect perf data
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
                ${project}/bin/* --version 2>&1 | tee -a build.log
              
              # Collect strace
              strace -o $out/perf-data/strace.log -c ${project}/bin/* --version 2>&1 | tee -a build.log
            fi
          elif [ -f ${project} ]; then
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
              ${project} --version 2>&1 | tee build.log
            strace -o $out/perf-data/strace.log -c ${project} --version 2>&1 | tee -a build.log
          else
            echo "Unknown project type" > build.log
          fi
          BUILD_EXIT=$?
          set -e
          
          # Copy log template
          cp -r $logDrv/* $out/
          chmod -R u+w $out
          
          # Update build state
          cat > $out/3-build-state.json <<EOF
          {
            "status": $([ $BUILD_EXIT -eq 0 ] && echo '"success"' || echo '"failed"'),
            "exit_code": $BUILD_EXIT,
            "system": "${system}",
            "nix_version": "${builtins.nixVersion}",
            "build_time": ${toString buildTime}
          }
          EOF
          
          # Save build log
          cp build.log $out/build.log
          
          # Link result
          ln -s ${project} $out/result
          
          # Update summary
          cat > $out/summary.json <<EOF
          {
            "project": "${projectName}",
            "git_commit": "${gitCommit}",
            "build_status": $([ $BUILD_EXIT -eq 0 ] && echo '"success"' || echo '"failed"'),
            "log_derivation": "$out"
          }
          EOF
        '';

      # Test package
      packages.${system} = {
        test = self.lib.wrap {
          inherit pkgs self;
          project = pkgs.hello;
          projectName = "hello-test";
        };
        default = self.packages.${system}.test;
      };
    };
}
