# The Nix Reproducibility Problem

## Current State: Partial Reproducibility

### What Nix Gives Us ✅
```bash
# Can rebuild from derivation
nix-store --realise /nix/store/72926zqphd96br2rrgqlxagzwq8sj5fx-Drift_Protocol.drv

# Derivation contains:
- Builder: /nix/store/...-bash-5.3p3/bin/bash
- Build inputs: /nix/store/...-solana-cli-3.0.12
- Build script: buildPhase, installPhase
- System: x86_64-linux
```

### What Nix DOESN'T Give Us ❌
```bash
# Missing critical metadata:
1. Source git commit    - Which version of code?
2. flake.lock snapshot  - Which input versions?
3. Build timestamp      - When was this built?
4. Build context        - Why was this built?
5. Rebuild instructions - How to rebuild from source?
```

## The Problem

**For 111 successful builds, I CANNOT tell you:**
- Which git commit produced each binary
- Which flake.lock was used
- How to rebuild from scratch (only from .drv)

**Example: Drift_Protocol**
```
Store path: /nix/store/n6haia96vypk03jcs68pazx1y6f58y9q-Drift_Protocol
Derivation: /nix/store/72926zqphd96br2rrgqlxagzwq8sj5fx-Drift_Protocol.drv

Can rebuild? YES (from .drv)
Know source? NO (no git commit)
Know inputs? PARTIAL (only store paths, not versions)
```

## The Solution: Metadata Wrapper

### New Standard: Every Build Includes Metadata

```nix
{
  description = "Reproducible build with full metadata";
  
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = pkgs.runCommand "project-with-metadata" {
      # Original build
      project = import ./default.nix { inherit pkgs; };
      
      # Metadata
      gitCommit = self.rev or "dirty";
      gitBranch = self.ref or "unknown";
      buildTime = builtins.currentTime;
      flakeLock = builtins.readFile ./flake.lock;
    } ''
      mkdir -p $out
      
      # Link original build
      ln -s $project $out/result
      
      # Save metadata
      cat > $out/metadata.json <<EOF
      {
        "git_commit": "$gitCommit",
        "git_branch": "$gitBranch",
        "build_time": $buildTime,
        "store_path": "$out",
        "derivation": "$project.drvPath",
        "rebuild_command": "nix build github:meta-introspector/project?rev=$gitCommit"
      }
      EOF
      
      # Save flake.lock
      echo "$flakeLock" > $out/flake.lock
      
      # Save rebuild script
      cat > $out/rebuild.sh <<EOF
      #!/bin/bash
      # Exact rebuild from source
      git clone https://github.com/meta-introspector/project
      cd project
      git checkout $gitCommit
      nix build
      EOF
      chmod +x $out/rebuild.sh
    '';
  };
}
```

### Result: Full Reproducibility

```bash
# For ANY binary in store:
ls /nix/store/n6haia96vypk03jcs68pazx1y6f58y9q-Drift_Protocol/

result/          # The actual binary
metadata.json    # Git commit, timestamp, rebuild command
flake.lock       # Exact input versions
rebuild.sh       # Script to rebuild from scratch
```

## Implementation: Wrap All 500 Projects

### Step 1: Create Universal Wrapper Template

```nix
# universal-wrapper.nix
{ pkgs, self, project }:

pkgs.runCommand "${project.name}-reproducible" {
  inherit project;
  gitCommit = self.rev or "dirty";
  gitBranch = self.ref or "unknown";
  gitRemote = self.sourceInfo.url or "unknown";
  buildTime = builtins.currentTime;
  nixVersion = builtins.nixVersion;
  system = builtins.currentSystem;
} ''
  mkdir -p $out
  
  # Link original
  ln -s $project $out/result
  
  # Full metadata
  cat > $out/metadata.json <<EOF
  {
    "project": "${project.name}",
    "git": {
      "commit": "$gitCommit",
      "branch": "$gitBranch",
      "remote": "$gitRemote"
    },
    "build": {
      "time": $buildTime,
      "system": "$system",
      "nix_version": "$nixVersion"
    },
    "store": {
      "path": "$out",
      "derivation": "${project.drvPath}"
    },
    "rebuild": {
      "from_drv": "nix-store --realise ${project.drvPath}",
      "from_source": "nix build $gitRemote?rev=$gitCommit"
    }
  }
  EOF
  
  # Save inputs
  ${pkgs.nix}/bin/nix-store -q --tree $project > $out/dependencies.txt
  ${pkgs.nix}/bin/nix-store -q --graph $project > $out/dependencies.dot
  
  # Rebuild script
  cat > $out/rebuild.sh <<'SCRIPT'
  #!/bin/bash
  set -e
  echo "Rebuilding from source..."
  nix build $gitRemote?rev=$gitCommit
  echo "Rebuilt: $(nix-build)"
  SCRIPT
  chmod +x $out/rebuild.sh
''
```

### Step 2: Apply to All Projects

```bash
#!/bin/bash
# wrap_all_projects.sh

for flake in $(find /mnt/data1/nix/source -name "flake.nix"); do
  dir=$(dirname "$flake")
  project=$(basename "$dir")
  
  echo "Wrapping: $project"
  
  # Backup original
  cp "$flake" "$flake.orig"
  
  # Add wrapper
  cat > "$flake" <<'EOF'
{
  description = "Reproducible build with metadata";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    wrapper.url = "github:meta-introspector/universal-wrapper";
  };
  
  outputs = { self, nixpkgs, wrapper }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      original = import ./flake.nix.orig { inherit self nixpkgs; };
    in {
      packages.x86_64-linux.default = 
        wrapper.lib.wrap {
          inherit pkgs self;
          project = original.packages.x86_64-linux.default;
        };
    };
}
EOF
done
```

### Step 3: Rebuild All 500 Projects

```bash
# Queue all projects with new wrapper
for flake in $(find /mnt/data1/nix/source -name "flake.nix"); do
  dir=$(dirname "$flake")
  ./nix_builder.sh queue "$dir"
done

# Start builder
./nix_builder.sh watch
```

### Step 4: Verify Reproducibility

```bash
# For each successful build:
for store_path in /nix/store/*-reproducible; do
  if [ -f "$store_path/metadata.json" ]; then
    commit=$(jq -r '.git.commit' "$store_path/metadata.json")
    echo "Testing: $store_path"
    
    # Rebuild from source
    bash "$store_path/rebuild.sh"
    
    # Compare hashes
    original_hash=$(nix-hash --type sha256 "$store_path/result")
    rebuilt_hash=$(nix-hash --type sha256 ./result)
    
    if [ "$original_hash" = "$rebuilt_hash" ]; then
      echo "✅ Reproducible: $store_path"
    else
      echo "❌ NOT reproducible: $store_path"
    fi
  fi
done
```

## Benefits

### Before (Current Nix)
```
/nix/store/abc123-project
├── bin/
└── lib/

Can rebuild? Only from .drv (if it exists)
Know source? No
Know inputs? Partial
```

### After (With Metadata Wrapper)
```
/nix/store/abc123-project-reproducible
├── result/              -> Original build
├── metadata.json        -> Full provenance
├── flake.lock          -> Exact inputs
├── dependencies.txt     -> Full dep tree
├── dependencies.dot     -> Graph visualization
└── rebuild.sh          -> Rebuild from scratch

Can rebuild? YES (from source OR .drv)
Know source? YES (git commit)
Know inputs? YES (flake.lock)
```

## Answer to Your Question

**"Can you tell me for all bins exactly how to rebuild them?"**

**Current answer:** NO - Only from .drv if it exists, but no source link

**After wrapper:** YES - Every binary has:
1. `metadata.json` with git commit
2. `flake.lock` with exact inputs
3. `rebuild.sh` script
4. Full dependency tree

**Command to rebuild ANY binary:**
```bash
# From any store path
cd /nix/store/abc123-project-reproducible
./rebuild.sh

# Or from metadata
commit=$(jq -r '.git.commit' metadata.json)
remote=$(jq -r '.git.remote' metadata.json)
nix build "$remote?rev=$commit"
```

## Next Steps

1. Create `universal-wrapper` flake
2. Test on 1 successful build (Drift_Protocol)
3. Apply to all 111 successful builds
4. Apply to all 232 failed builds (capture-only mode)
5. Verify reproducibility
6. Push metadata to HuggingFace dataset
