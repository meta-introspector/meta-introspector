{
  description = "Meta-analysis: Apply 4 tools to 236 Rust executables";
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Import the 4 analysis tools
      keywords = import ../001_keywords { inherit nixpkgs; };
      primes = import ../002_primes { inherit nixpkgs; };
      harmonic = import ../003_harmonic_filter { inherit nixpkgs; };
      markov = import ../004_markov_model { inherit nixpkgs; };
      
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "meta-analysis-236-executables";
        src = ../..;
        
        buildInputs = [ 
          pkgs.python3 
          pkgs.ripgrep
          keywords.packages.${system}.default
          primes.packages.${system}.default
          harmonic.packages.${system}.default
          markov.packages.${system}.default
        ];
        
        buildPhase = ''
          mkdir -p $out/{executables,analysis,reports}
          
          # Step 1: Find all 236 executables
          python3 <<'PYTHON'
          import re
          from pathlib import Path
          import json
          
          executables = []
          
          # Find all .rs files with main() in root
          for rs_file in Path('.').glob('*.rs'):
              try:
                  content = rs_file.read_text()
                  if 'fn main(' in content:
                      executables.append({
                          'name': rs_file.stem,
                          'path': str(rs_file),
                          'lines': len(content.split('\n')),
                          'size': len(content)
                      })
              except:
                  pass
          
          # Save list
          with open('$out/executables/list.json', 'w') as f:
              json.dump(executables, f, indent=2)
          
          print(f"Found {len(executables)} executables")
          PYTHON
          
          # Step 2: Apply keywords analysis to each executable
          python3 <<'PYTHON'
          import json
          from pathlib import Path
          from collections import Counter
          import re
          
          # Load executables
          with open('$out/executables/list.json') as f:
              executables = json.load(f)
          
          # Load term-to-prime mapping from primes tool
          # (Would load from ${primes}/primes/term-to-prime.json)
          
          results = []
          
          for exe in executables:
              try:
                  content = Path(exe['path']).read_text()
                  
                  # Extract terms
                  terms = re.findall(r'\b[a-z_][a-z_0-9]*\b', content.lower())
                  term_counts = Counter(terms)
                  
                  # Compute Gödel number (simplified)
                  godel = 1
                  cursed_primes = [37, 157, 191, 223, 227, 229, 233, 239, 241, 251]
                  fake_terms = ['fake', 'dummy', 'test', 'example', 'foo', 'bar']
                  
                  has_fake = any(term in fake_terms for term in terms)
                  
                  # Harmonic analysis: name vs impl complexity
                  name_complexity = len(exe['name']) + len(exe['name'].split('_')) * 10
                  impl_complexity = exe['lines'] + len(terms)
                  
                  if impl_complexity > 0:
                      harmony_ratio = name_complexity / impl_complexity
                      harmonic = 0.05 < harmony_ratio < 0.15
                  else:
                      harmony_ratio = 0
                      harmonic = False
                  
                  # Markov analysis: check for natural term sequences
                  # (Would use ${markov}/model/markov-transitions.json)
                  
                  results.append({
                      'name': exe['name'],
                      'terms': len(term_counts),
                      'unique_terms': len(set(terms)),
                      'has_fake_terms': has_fake,
                      'name_complexity': name_complexity,
                      'impl_complexity': impl_complexity,
                      'harmony_ratio': harmony_ratio,
                      'harmonic': harmonic,
                      'top_terms': dict(term_counts.most_common(5))
                  })
              except:
                  pass
          
          # Save results
          with open('$out/analysis/all-executables.json', 'w') as f:
              json.dump(results, f, indent=2)
          
          print(f"Analyzed {len(results)} executables")
          PYTHON
          
          # Step 3: Generate reports
          python3 <<'PYTHON'
          import json
          
          # Load analysis
          with open('$out/analysis/all-executables.json') as f:
              results = json.load(f)
          
          # Find issues
          fake_executables = [r for r in results if r['has_fake_terms']]
          disharmonic = [r for r in results if not r['harmonic']]
          
          # Generate report
          with open('$out/reports/summary.txt', 'w') as f:
              f.write("Meta-Analysis: 236 Rust Executables\n")
              f.write("=" * 50 + "\n\n")
              
              f.write(f"Total analyzed: {len(results)}\n")
              f.write(f"With fake terms: {len(fake_executables)}\n")
              f.write(f"Disharmonic (name/impl mismatch): {len(disharmonic)}\n\n")
              
              f.write("Fake Executables:\n")
              for exe in fake_executables[:10]:
                  f.write(f"  - {exe['name']}\n")
              
              f.write("\nDisharmonic Executables:\n")
              for exe in disharmonic[:10]:
                  f.write(f"  - {exe['name']} (ratio: {exe['harmony_ratio']:.3f})\n")
              
              f.write("\nTop Terms Across All Executables:\n")
              all_terms = {}
              for exe in results:
                  for term, count in exe['top_terms'].items():
                      all_terms[term] = all_terms.get(term, 0) + count
              
              for term, count in sorted(all_terms.items(), key=lambda x: x[1], reverse=True)[:20]:
                  f.write(f"  {term}: {count}\n")
          
          print("Report generated")
          PYTHON
          
          # Step 4: Prioritize for conversion
          python3 <<'PYTHON'
          import json
          
          # Load analysis
          with open('$out/analysis/all-executables.json') as f:
              results = json.load(f)
          
          # Score each executable for conversion priority
          for exe in results:
              score = 0
              
              # High complexity = high priority
              if exe['impl_complexity'] > 500:
                  score += 10
              
              # Harmonic = good candidate
              if exe['harmonic']:
                  score += 5
              
              # No fake terms = good candidate
              if not exe['has_fake_terms']:
                  score += 5
              
              # Many unique terms = interesting
              if exe['unique_terms'] > 50:
                  score += 3
              
              exe['conversion_priority'] = score
          
          # Sort by priority
          results.sort(key=lambda x: x['conversion_priority'], reverse=True)
          
          # Save prioritized list
          with open('$out/analysis/conversion-priority.json', 'w') as f:
              json.dump(results[:20], f, indent=2)
          
          # Generate conversion plan
          with open('$out/reports/conversion-plan.txt', 'w') as f:
              f.write("Top 20 Executables for Nix Conversion\n")
              f.write("=" * 50 + "\n\n")
              
              for i, exe in enumerate(results[:20], 5):
                  f.write(f"{i:03d}. {exe['name']}\n")
                  f.write(f"     Priority: {exe['conversion_priority']}\n")
                  f.write(f"     Complexity: {exe['impl_complexity']}\n")
                  f.write(f"     Harmonic: {exe['harmonic']}\n")
                  f.write(f"     Nix: analysis/{i:03d}_{exe['name']}/\n\n")
          
          print("Conversion plan generated")
          PYTHON
          
          cat $out/reports/summary.txt
          echo ""
          cat $out/reports/conversion-plan.txt
        '';
      };
    };
}
