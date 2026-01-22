{
  description = "Meta-analysis with ai-ml-zk-ops CRQ system integration";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    
    # Import ai-ml-zk-ops via git hash
    ai-ml-zk-ops = {
      url = "github:meta-introspector/ai-ml-zk-ops/e3551db";
      flake = false;  # Treat as source, not flake
    };
  };
  
  outputs = { self, nixpkgs, ai-ml-zk-ops }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "meta-analysis-with-crqs";
        
        # Both sources available
        src = ../..;
        crqSrc = ai-ml-zk-ops;
        
        buildInputs = [ pkgs.python3 pkgs.ripgrep pkgs.jq ];
        
        buildPhase = ''
          mkdir -p $out/{analysis,crqs,merged}
          
          echo "🔍 Analyzing meta-introspector executables..."
          
          # Step 1: Extract our 236 executables
          python3 <<'PYTHON'
          import json
          from pathlib import Path
          
          executables = []
          for rs_file in Path('.').glob('*.rs'):
              try:
                  content = rs_file.read_text()
                  if 'fn main(' in content:
                      executables.append({
                          'name': rs_file.stem,
                          'path': str(rs_file),
                          'lines': len(content.split('\n'))
                      })
              except:
                  pass
          
          with open('$out/analysis/executables.json', 'w') as f:
              json.dump(executables, f, indent=2)
          
          print(f"Found {len(executables)} executables")
          PYTHON
          
          echo "📋 Extracting CRQ system from ai-ml-zk-ops..."
          
          # Step 2: Extract CRQs and SOPs
          cd $crqSrc
          
          # Find all CRQs
          find . -name "CRQ*.md" -o -name "*crq*.md" | \
            grep -v ".git" > $out/crqs/crq-files.txt
          
          # Find all SOPs
          find . -name "SOP*.md" -o -name "*sop*.md" | \
            grep -v ".git" > $out/crqs/sop-files.txt
          
          # Find all flakes
          find . -name "flake.nix" | \
            grep -v ".git" > $out/crqs/flake-files.txt
          
          echo "Found CRQs: $(wc -l < $out/crqs/crq-files.txt)"
          echo "Found SOPs: $(wc -l < $out/crqs/sop-files.txt)"
          echo "Found Flakes: $(wc -l < $out/crqs/flake-files.txt)"
          
          echo "🔗 Merging/Zippering the two systems..."
          
          # Step 3: Create mapping between executables and CRQs
          python3 <<'PYTHON'
          import json
          from pathlib import Path
          
          # Load our executables
          with open('$out/analysis/executables.json') as f:
              executables = json.load(f)
          
          # Load CRQ files
          crq_files = Path('$out/crqs/crq-files.txt').read_text().strip().split('\n')
          
          # Create mapping
          mapping = {
              'meta_introspector': {
                  'executables': len(executables),
                  'top_20': [e['name'] for e in executables[:20]]
              },
              'ai_ml_zk_ops': {
                  'crqs': len(crq_files),
                  'governance': 'CRQ/SOP system'
              },
              'integration': {
                  'strategy': 'Each executable gets a CRQ for nix conversion',
                  'naming': 'CRQ_NNN_<executable_name>_nixification.md',
                  'location': 'analysis/NNN_<name>/CRQ.md'
              }
            }
          
          with open('$out/merged/integration-plan.json', 'w') as f:
              json.dump(mapping, f, indent=2)
          
          print("Integration plan created")
          PYTHON
          
          # Step 4: Generate CRQ template for executable conversion
          cat > $out/merged/CRQ_TEMPLATE.md <<'EOF'
# CRQ-NNN: Nixify <EXECUTABLE_NAME>

## Problem Statement
Convert <EXECUTABLE_NAME>.rs to pure nix build with reproducible I/O.

## Proposed Solution
Create analysis/NNN_<name>/flake.nix that:
1. Reads: Input files from nix store
2. Writes: Output to $out
3. Pure: No side effects

## Scope
- Single executable conversion
- Part of 236 → 20 priority conversion
- Follows meta-introspector governance

## Technical Details

### Current State
- Location: <EXECUTABLE_NAME>.rs
- Lines: <LINES>
- I/O: <INPUTS> → <OUTPUTS>

### Target State
- Location: analysis/NNN_<name>/flake.nix
- Inputs: Declared in flake inputs
- Outputs: $out/{analysis,results}

## Testing Plan
1. Build: nix build ./analysis/NNN_<name>
2. Verify: Check $out contents
3. Integrate: Add to bootstrap

## Rollback Plan
Keep original .rs file until verified.

## References
- Meta-Introspector Guide: docs/META_INTROSPECTOR_GUIDE.md
- CRQ System: ai-ml-zk-ops governance
- Bootstrap: scripts/build/bootstrap.sh
EOF
          
          echo "✅ Integration complete!"
          
          # Summary
          cat > $out/merged/SUMMARY.md <<'EOF'
# Meta-Analysis + CRQ System Integration

## Two Systems Merged

### Meta-Introspector
- 236 Rust executables
- 5 analysis jobs (keywords, primes, harmonic, markov, meta-analysis)
- Bootstrap pipeline
- Nix-based reproducible builds

### ai-ml-zk-ops
- CRQ (Change ReQuest) governance system
- SOP (Standard Operating Procedure) documentation
- Flake lattice architecture
- Git hash-based source management

## Integration Strategy

**Zipper Pattern:**
Each meta-introspector executable gets a CRQ for nixification.

**Naming Convention:**
- CRQ: CRQ_NNN_<executable>_nixification.md
- Nix: analysis/NNN_<executable>/flake.nix
- Both: Co-located in same directory

**Governance:**
- Follow ai-ml-zk-ops CRQ/SOP process
- Document in meta-introspector style
- Store in nix store via git hashes

## Next Steps

1. Generate CRQs for top 20 executables
2. Create nix flakes following CRQ specs
3. Add to bootstrap pipeline
4. Commit with CRQ references

## Benefits

- **Governance**: CRQ system ensures quality
- **Reproducibility**: Nix ensures builds
- **Documentation**: Both systems document
- **Traceability**: Git hashes track everything
EOF
          
          cat $out/merged/SUMMARY.md
        '';
      };
    };
}
