# Reproducible Build Plan: Folding into Nix

## Vision
Transform all meta-introspector projects into fully reproducible Nix builds with:
- Git-pinned inputs (flake.lock)
- Shared store paths (IPFS/HF/Archive.org/gossip)
- Build-time data collection
- Universal binary distribution

## Current State

### Build Statistics
- **Total projects**: 343
- **Successful builds**: 111 (32%)
- **Failed builds**: 232 (68%)
- **Store paths generated**: ~111 derivations

### Successful Package Categories

#### Solana Smart Contracts (20+)
- Jupiter_Aggregator
- Drift_Protocol
- Mango_Markets
- Marinade_Finance
- Orca_Whirlpool
- Phoenix_DEX
- Raydium_AMM
- Saber_StableSwap
- Serum_DEX
- Solend_Protocol

#### Bitcoin Scripts (5)
- P2PKH, P2SH, P2TR, P2WPKH, P2WSH
- Lightning_HTLC
- Multisig_2of3

#### Analysis Tools
- 001_nar_exporter
- 002_extract_raw_data
- 2025-01-27-minimal-qa-test

## Phase 1: Universal Build Wrapper

### Goal
Create a flake that applies analysis binaries to ALL projects during build.

### Components

#### 1. Build Observer Flake
```nix
{
  description = "Universal build observer - collects data during nix build";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs }: {
    lib.wrapWithObserver = { pkgs, derivation, collectors ? [] }:
      pkgs.runCommand "${derivation.name}-observed" {
        buildInputs = collectors;
      } ''
        # Collect build metadata
        echo "Build started: $(date -Iseconds)" > $out/build.json
        echo "Derivation: ${derivation}" >> $out/build.json
        echo "Store path: $out" >> $out/build.json
        
        # Run collectors
        ${pkgs.lib.concatMapStringsSep "\n" (c: "${c}/bin/collect") collectors}
        
        # Link original derivation
        ln -s ${derivation} $out/result
      '';
  };
}
```

#### 2. Data Collectors

**Git Provenance Collector**
```nix
pkgs.writeScriptBin "collect-git-provenance" ''
  #!/bin/bash
  echo "Git repo: $GIT_REPO"
  echo "Commit: $GIT_COMMIT"
  echo "Branch: $GIT_BRANCH"
  echo "Remote: $GIT_REMOTE"
''
```

**Build Telemetry Collector**
```nix
pkgs.writeScriptBin "collect-build-telemetry" ''
  #!/bin/bash
  echo "Build time: $(date -Iseconds)"
  echo "System: $(uname -a)"
  echo "Nix version: $(nix --version)"
  echo "Store path: $out"
''
```

**Dependency Graph Collector**
```nix
pkgs.writeScriptBin "collect-dependencies" ''
  #!/bin/bash
  nix-store -q --graph $out > $out/deps.dot
  nix-store -q --tree $out > $out/deps.tree
''
```

## Phase 2: Apply to All Flakes

### Auto-Wrapper Script
```bash
#!/bin/bash
# Apply build observer to all flakes

for flake in $(find /mnt/data1/nix/source -name "flake.nix"); do
  dir=$(dirname "$flake")
  
  # Add build-observer input
  sed -i '/inputs = {/a\    build-observer.url = "github:meta-introspector/build-observer";' "$flake"
  
  # Wrap packages with observer
  sed -i 's/packages\.\(.*\)\.default = \(.*\);/packages.\1.default = build-observer.lib.wrapWithObserver { inherit pkgs; derivation = \2; };/' "$flake"
done
```

## Phase 3: Store Distribution

### IPFS Integration
```nix
{
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = pkgs.runCommand "publish-to-ipfs" {} ''
      # Build package
      ${self.packages.x86_64-linux.myPackage}
      
      # Publish to IPFS
      ipfs add -r $out > $out/ipfs.cid
      
      # Announce to gossip network
      gossip-announce --cid $(cat $out/ipfs.cid) --type nix-store
    '';
  };
}
```

### HuggingFace Dataset Integration
```nix
{
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.publish-hf = pkgs.writeScriptBin "publish-hf" ''
      #!/bin/bash
      # Export store path metadata
      nix-store -q --json $out > metadata.json
      
      # Upload to HF dataset
      huggingface-cli upload \
        introspector/nix-store-paths \
        metadata.json \
        --repo-type dataset
    '';
  };
}
```

### Archive.org Integration
```nix
{
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.archive = pkgs.writeScriptBin "archive-org" ''
      #!/bin/bash
      # Create NAR archive
      nix-store --export $out > package.nar
      
      # Upload to Archive.org
      ia upload meta-introspector-${name} package.nar \
        --metadata="collection:opensource" \
        --metadata="mediatype:software"
    '';
  };
}
```

## Phase 4: Reproducibility Verification

### Build Attestation
```nix
{
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.attest = pkgs.writeScriptBin "attest-build" ''
      #!/bin/bash
      # Generate build attestation
      cat > attestation.json <<EOF
      {
        "store_path": "$out",
        "inputs": $(nix flake metadata --json),
        "system": "$(uname -a)",
        "timestamp": "$(date -Iseconds)",
        "builder": "nix-daemon"
      }
      EOF
      
      # Sign with GPG
      gpg --sign attestation.json
    '';
  };
}
```

## Implementation Roadmap

### Week 1: Foundation
- [ ] Create build-observer flake
- [ ] Implement 3 core collectors (git, telemetry, deps)
- [ ] Test on 10 successful packages

### Week 2: Integration
- [ ] Apply observer to all 111 successful builds
- [ ] Collect build data into unified dataset
- [ ] Push to HF dataset: introspector/build-telemetry

### Week 3: Distribution
- [ ] Set up IPFS node
- [ ] Publish 111 store paths to IPFS
- [ ] Create gossip network protocol
- [ ] Upload to Archive.org

### Week 4: Verification
- [ ] Implement build attestation
- [ ] Verify reproducibility (rebuild from flake.lock)
- [ ] Document all store paths
- [ ] Create public registry

## Success Metrics

- **100% reproducible**: All builds produce same store path
- **100% distributed**: All store paths in IPFS/HF/Archive.org
- **100% attested**: All builds have signed attestations
- **100% traceable**: All builds linked to git commits

## Benefits

1. **Reproducibility**: Anyone can rebuild exact same binaries
2. **Distribution**: Shared store reduces bandwidth/storage
3. **Provenance**: Every binary traced to source commit
4. **Collaboration**: Shared builds across network
5. **Archival**: Permanent storage in multiple locations
6. **Verification**: Cryptographic attestation of builds

## Next Steps

1. Create `build-observer` flake
2. Test on Jupiter_Aggregator (successful Solana contract)
3. Collect first build telemetry
4. Push to HF dataset
5. Iterate and expand to all 111 packages
