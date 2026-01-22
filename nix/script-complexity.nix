{
  description = "Script complexity analysis via perf - measure reception complexity";
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
    in {
      packages.${system} = {
        # Job 1: Analyze Nix scripts via nix parse
        nix-script-complexity = pkgs.stdenv.mkDerivation {
          name = "nix-script-complexity";
          src = ./.;
          
          buildInputs = [ pkgs.linuxPackages.perf pkgs.nix ];
          
          buildPhase = ''
            mkdir -p $out/{perf,analysis}
            
            # Find all nix files
            find . -maxdepth 3 -name "*.nix" \
              ! -path "./.git/*" \
              ! -path "./nix/flakes/*" \
              ! -path "./submodules/*" > nix-files.txt
            
            # Analyze each nix file
            while read -r nix_file; do
              name=$(echo "$nix_file" | tr '/' '_' | sed 's/^_//')
              
              echo "Analyzing: $nix_file"
              
              # Record nix parse
    # Use: perf-lib from github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
                -F 997 -g \
                nix-instantiate --parse "$nix_file" 2>/dev/null || true
              
              # Record nix eval
    # Use: perf-lib from github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
                -F 997 -g \
                nix-instantiate --eval "$nix_file" 2>/dev/null || true
              
              # Extract complexity metrics
              if [ -f "$out/perf/parse-$name.perf.data" ]; then
                SAMPLES=$(perf script -i "$out/perf/parse-$name.perf.data" | wc -l)
                UNIQUE_IPS=$(perf script -i "$out/perf/parse-$name.perf.data" -F ip | sort -u | wc -l)
                
                echo "$nix_file,$SAMPLES,$UNIQUE_IPS,parse" >> $out/analysis/nix-complexity.csv
              fi
              
              if [ -f "$out/perf/eval-$name.perf.data" ]; then
                SAMPLES=$(perf script -i "$out/perf/eval-$name.perf.data" | wc -l)
                UNIQUE_IPS=$(perf script -i "$out/perf/eval-$name.perf.data" -F ip | sort -u | wc -l)
                
                echo "$nix_file,$SAMPLES,$UNIQUE_IPS,eval" >> $out/analysis/nix-complexity.csv
              fi
            done < nix-files.txt
            
            # Add header
            sed -i '1i file,samples,unique_ips,operation' $out/analysis/nix-complexity.csv
            
            echo "Nix script complexity analysis complete"
          '';
        };
        
        # Job 2: Analyze Bash scripts via bash parse + shellcheck
        bash-script-complexity = pkgs.stdenv.mkDerivation {
          name = "bash-script-complexity";
          src = ./.;
          
          buildInputs = [ pkgs.linuxPackages.perf pkgs.bash pkgs.shellcheck ];
          
          buildPhase = ''
            mkdir -p $out/{perf,analysis}
            
            # Find all bash files
            find . -maxdepth 3 -name "*.sh" \
              ! -path "./.git/*" \
              ! -path "./submodules/*" > bash-files.txt
            
            # Analyze each bash file
            while read -r bash_file; do
              name=$(echo "$bash_file" | tr '/' '_' | sed 's/^_//')
              
              echo "Analyzing: $bash_file"
              
              # Record bash parse (syntax check)
    # Use: perf-lib from github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
                -F 997 -g \
                bash -n "$bash_file" 2>/dev/null || true
              
              # Record shellcheck
    # Use: perf-lib from github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
                -F 997 -g \
                shellcheck "$bash_file" 2>/dev/null || true
              
              # Extract complexity metrics
              if [ -f "$out/perf/parse-$name.perf.data" ]; then
                SAMPLES=$(perf script -i "$out/perf/parse-$name.perf.data" | wc -l)
                UNIQUE_IPS=$(perf script -i "$out/perf/parse-$name.perf.data" -F ip | sort -u | wc -l)
                
                echo "$bash_file,$SAMPLES,$UNIQUE_IPS,parse" >> $out/analysis/bash-complexity.csv
              fi
              
              if [ -f "$out/perf/shellcheck-$name.perf.data" ]; then
                SAMPLES=$(perf script -i "$out/perf/shellcheck-$name.perf.data" | wc -l)
                UNIQUE_IPS=$(perf script -i "$out/perf/shellcheck-$name.perf.data" -F ip | sort -u | wc -l)
                
                echo "$bash_file,$SAMPLES,$UNIQUE_IPS,shellcheck" >> $out/analysis/bash-complexity.csv
              fi
            done < bash-files.txt
            
            # Add header
            sed -i '1i file,samples,unique_ips,operation' $out/analysis/bash-complexity.csv
            
            echo "Bash script complexity analysis complete"
          '';
        };
        
        # Job 3: Unified complexity analysis
        script-complexity-unified = pkgs.stdenv.mkDerivation {
          name = "script-complexity-unified";
          
          buildInputs = [
            self.packages.${system}.nix-script-complexity
            self.packages.${system}.bash-script-complexity
            pkgs.python3
          ];
          
          buildPhase = ''
            mkdir -p $out/{analysis,visualization}
            
            # Combine all complexity data
            cat ${self.packages.${system}.nix-script-complexity}/analysis/nix-complexity.csv > $out/analysis/all-scripts.csv
            tail -n +2 ${self.packages.${system}.bash-script-complexity}/analysis/bash-complexity.csv >> $out/analysis/all-scripts.csv
            
            # Compute complexity scores
            python3 <<'PYTHON'
            import csv
            import json
            
            # Read complexity data
            scripts = []
            with open('$out/analysis/all-scripts.csv') as f:
                reader = csv.DictReader(f)
                for row in reader:
                    scripts.append({
                        'file': row['file'],
                        'samples': int(row['samples']),
                        'unique_ips': int(row['unique_ips']),
                        'operation': row['operation'],
                        'complexity_score': int(row['samples']) * int(row['unique_ips'])
                    })
            
            # Sort by complexity
            scripts.sort(key=lambda x: x['complexity_score'], reverse=True)
            
            # Save JSON
            with open('$out/analysis/complexity-ranked.json', 'w') as f:
                json.dump(scripts, f, indent=2)
            
            # Generate report
            with open('$out/analysis/complexity-report.txt', 'w') as f:
                f.write("Script Complexity Analysis\n")
                f.write("==========================\n\n")
                f.write(f"Total scripts: {len(scripts)}\n\n")
                f.write("Top 10 most complex:\n")
                for i, script in enumerate(scripts[:10], 1):
                    f.write(f"{i}. {script['file']}\n")
                    f.write(f"   Samples: {script['samples']}\n")
                    f.write(f"   Unique IPs: {script['unique_ips']}\n")
                    f.write(f"   Complexity: {script['complexity_score']}\n\n")
            
            print(f"Analyzed {len(scripts)} scripts")
            PYTHON
            
            echo "Unified complexity analysis complete"
          '';
        };
        
        # Job 4: Reception complexity (how hard to parse/understand)
        reception-complexity = pkgs.stdenv.mkDerivation {
          name = "reception-complexity";
          
          buildInputs = [
            self.packages.${system}.script-complexity-unified
            pkgs.python3
          ];
          
          buildPhase = ''
            mkdir -p $out/reception
            
            # Reception complexity = complexity of parsing/analyzing the script
            # Higher perf samples = harder to receive/understand
            
            python3 <<'PYTHON'
            import json
            
            # Load complexity data
            with open('${self.packages.${system}.script-complexity-unified}/analysis/complexity-ranked.json') as f:
                scripts = json.load(f)
            
            # Compute reception complexity
            # Reception = how much work to parse/analyze
            for script in scripts:
                # Normalize by file size (if available)
                script['reception_complexity'] = script['complexity_score']
                
                # Classify
                if script['reception_complexity'] > 10000:
                    script['reception_class'] = 'very_high'
                elif script['reception_complexity'] > 5000:
                    script['reception_class'] = 'high'
                elif script['reception_complexity'] > 1000:
                    script['reception_class'] = 'medium'
                else:
                    script['reception_class'] = 'low'
            
            # Save
            with open('$out/reception/reception-complexity.json', 'w') as f:
                json.dump(scripts, f, indent=2)
            
            # Generate report
            with open('$out/reception/reception-report.txt', 'w') as f:
                f.write("Reception Complexity Analysis\n")
                f.write("=============================\n\n")
                f.write("Reception complexity = computational cost to parse/analyze script\n\n")
                
                classes = {}
                for script in scripts:
                    cls = script['reception_class']
                    classes[cls] = classes.get(cls, 0) + 1
                
                f.write("Distribution:\n")
                for cls, count in sorted(classes.items()):
                    f.write(f"  {cls}: {count}\n")
                
                f.write("\nHighest reception complexity:\n")
                for i, script in enumerate(scripts[:5], 1):
                    f.write(f"{i}. {script['file']} ({script['reception_class']})\n")
            
            print("Reception complexity computed")
            PYTHON
            
            echo "Reception complexity analysis complete"
          '';
        };
        
        # Complete analysis
        default = pkgs.stdenv.mkDerivation {
          name = "script-complexity-complete";
          
          buildInputs = [
            self.packages.${system}.nix-script-complexity
            self.packages.${system}.bash-script-complexity
            self.packages.${system}.script-complexity-unified
            self.packages.${system}.reception-complexity
          ];
          
          buildPhase = ''
            mkdir -p $out/{nix,bash,unified,reception,perf}
            
            # Collect all outputs
            cp -r ${self.packages.${system}.nix-script-complexity}/analysis/* $out/nix/
            cp -r ${self.packages.${system}.nix-script-complexity}/perf/* $out/perf/
            
            cp -r ${self.packages.${system}.bash-script-complexity}/analysis/* $out/bash/
            cp -r ${self.packages.${system}.bash-script-complexity}/perf/* $out/perf/
            
            cp -r ${self.packages.${system}.script-complexity-unified}/analysis/* $out/unified/
            cp -r ${self.packages.${system}.reception-complexity}/reception/* $out/reception/
            
            # Generate summary
            cat > $out/summary.txt <<EOF
            Script Complexity Analysis Complete
            ====================================
            
            Nix scripts: $(tail -n +2 $out/nix/nix-complexity.csv | wc -l)
            Bash scripts: $(tail -n +2 $out/bash/bash-complexity.csv | wc -l)
            
            Perf traces: $(ls $out/perf/*.perf.data | wc -l)
            
            Reception complexity classes:
            $(cat $out/reception/reception-report.txt | grep -A5 "Distribution:")
            
            Files:
            - nix/nix-complexity.csv
            - bash/bash-complexity.csv
            - unified/all-scripts.csv
            - unified/complexity-ranked.json
            - reception/reception-complexity.json
            - reception/reception-report.txt
            - perf/*.perf.data
            EOF
            
            cat $out/summary.txt
          '';
        };
      };
    };
}
