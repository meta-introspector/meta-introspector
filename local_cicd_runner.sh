#!/bin/bash
# Local CI/CD runner for all 500+ repos
# Registers repos and runs Nix analysis on each

set -e

REGISTRY_FILE="data/repo_registry.json"
RESULTS_DIR="data/local-cicd-results"

mkdir -p "$RESULTS_DIR"

echo "🔧 LOCAL CI/CD RUNNER"
echo ""

# ============================================================================
# Step 1: Register all repos
# ============================================================================
register_repos() {
    echo "📋 Registering repositories..."
    
    cat > "$REGISTRY_FILE" << 'EOF'
{
  "repos": [
EOF
    
    # Mike's repos from mike_repos.rs
    local first=true
    for repo_path in /opt/zos-production /opt/zos-bootstrap /home/mdupont/zos-qa; do
        if [ -d "$repo_path" ]; then
            [ "$first" = false ] && echo "," >> "$REGISTRY_FILE"
            first=false
            
            cat >> "$REGISTRY_FILE" << REPO
    {
      "path": "$repo_path",
      "name": "$(basename $repo_path)",
      "owner": "mike-dupont",
      "has_nix": $([ -f "$repo_path/flake.nix" ] && echo "true" || echo "false"),
      "has_cargo": $([ -f "$repo_path/Cargo.toml" ] && echo "true" || echo "false")
    }
REPO
        fi
    done
    
    # Meta-introspector org repos from GitHub
    if [ -f ~/nix/index/github_meta-introspector_repos.json ]; then
        while IFS= read -r line; do
            name=$(echo "$line" | jq -r '.name')
            url=$(echo "$line" | jq -r '.url')
            
            [ "$first" = false ] && echo "," >> "$REGISTRY_FILE"
            first=false
            
            cat >> "$REGISTRY_FILE" << REPO
    {
      "name": "$name",
      "url": "$url",
      "owner": "meta-introspector",
      "registered": true
    }
REPO
        done < <(jq -c '.[]' ~/nix/index/github_meta-introspector_repos.json | head -500)
    fi
    
    echo "" >> "$REGISTRY_FILE"
    echo "  ]" >> "$REGISTRY_FILE"
    echo "}" >> "$REGISTRY_FILE"
    
    local count=$(jq '.repos | length' "$REGISTRY_FILE")
    echo "  ✅ Registered $count repositories"
}

# ============================================================================
# Step 2: Run CI/CD on each repo
# ============================================================================
run_cicd_on_repo() {
    local repo_path=$1
    local repo_name=$2
    
    echo ""
    echo "🔨 Running CI/CD: $repo_name"
    echo "   Path: $repo_path"
    
    if [ ! -d "$repo_path" ]; then
        echo "   ⚠️  Not found locally, skipping"
        return
    fi
    
    cd "$repo_path"
    
    # Use absolute path for results
    local result_file="/mnt/data1/meta-introspector/$RESULTS_DIR/${repo_name}.json"
    mkdir -p "/mnt/data1/meta-introspector/$RESULTS_DIR"
    
    # Initialize result
    cat > "$result_file" << EOF
{
  "repo": "$repo_name",
  "path": "$repo_path",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "steps": {}
}
EOF
    
    # Check for Nix flake
    if [ -f "flake.nix" ]; then
        echo "   📦 Running nix flake check..."
        if timeout 300 nix flake check 2>&1 | tee "/mnt/data1/meta-introspector/$RESULTS_DIR/${repo_name}_nix.log"; then
            jq '.steps.nix_check = "success"' "$result_file" > "$result_file.tmp" && mv "$result_file.tmp" "$result_file"
            echo "   ✅ Nix check passed"
        else
            jq '.steps.nix_check = "failed"' "$result_file" > "$result_file.tmp" && mv "$result_file.tmp" "$result_file"
            echo "   ❌ Nix check failed"
        fi
    fi
    
    # Check for Cargo
    if [ -f "Cargo.toml" ]; then
        echo "   🦀 Running cargo check..."
        if timeout 300 cargo check 2>&1 | tee "/mnt/data1/meta-introspector/$RESULTS_DIR/${repo_name}_cargo.log"; then
            jq '.steps.cargo_check = "success"' "$result_file" > "$result_file.tmp" && mv "$result_file.tmp" "$result_file"
            echo "   ✅ Cargo check passed"
        else
            jq '.steps.cargo_check = "failed"' "$result_file" > "$result_file.tmp" && mv "$result_file.tmp" "$result_file"
            echo "   ❌ Cargo check failed"
        fi
    fi
    
    # Run analysis if meta-introspector tools available
    if command -v concept_map_builder &> /dev/null; then
        echo "   🔍 Running concept map analysis..."
        if find . -name "*.rs" > /tmp/rs_files.txt && \
           timeout 60 concept_map_builder /tmp/rs_files.txt 2>&1 | tee "/mnt/data1/meta-introspector/$RESULTS_DIR/${repo_name}_analysis.log"; then
            jq '.steps.analysis = "success"' "$result_file" > "$result_file.tmp" && mv "$result_file.tmp" "$result_file"
            echo "   ✅ Analysis complete"
        else
            jq '.steps.analysis = "failed"' "$result_file" > "$result_file.tmp" && mv "$result_file.tmp" "$result_file"
            echo "   ⚠️  Analysis failed"
        fi
    fi
    
    echo "   💾 Results saved to $result_file"
}

# ============================================================================
# Step 3: Generate summary report
# ============================================================================
generate_report() {
    echo ""
    echo "📊 Generating CI/CD summary report..."
    
    local report="LOCAL_CICD_REPORT.md"
    
    cat > "$report" << 'EOF'
# Local CI/CD Report

**Generated**: $(date)

## Summary

EOF
    
    local total=$(jq '.repos | length' "$REGISTRY_FILE")
    local processed=$(ls -1 "$RESULTS_DIR"/*.json 2>/dev/null | wc -l)
    
    echo "- **Total repos registered**: $total" >> "$report"
    echo "- **Repos processed**: $processed" >> "$report"
    echo "" >> "$report"
    
    echo "## Results by Repository" >> "$report"
    echo "" >> "$report"
    echo "| Repository | Nix Check | Cargo Check | Analysis |" >> "$report"
    echo "|------------|-----------|-------------|----------|" >> "$report"
    
    for result in "$RESULTS_DIR"/*.json; do
        if [ -f "$result" ]; then
            local name=$(jq -r '.repo' "$result")
            local nix=$(jq -r '.steps.nix_check // "N/A"' "$result")
            local cargo=$(jq -r '.steps.cargo_check // "N/A"' "$result")
            local analysis=$(jq -r '.steps.analysis // "N/A"' "$result")
            
            echo "| $name | $nix | $cargo | $analysis |" >> "$report"
        fi
    done
    
    echo "" >> "$report"
    echo "## Logs" >> "$report"
    echo "" >> "$report"
    echo "All logs saved to: \`$RESULTS_DIR/\`" >> "$report"
    
    echo "  ✅ Report saved to $report"
}

# ============================================================================
# Main execution
# ============================================================================

main() {
    # Register all repos
    register_repos
    
    # Run CI/CD on local repos
    echo ""
    echo "🚀 Running CI/CD on local repositories..."
    
    jq -r '.repos[] | select(.path != null) | "\(.path)|\(.name)"' "$REGISTRY_FILE" | while IFS='|' read -r path name; do
        run_cicd_on_repo "$path" "$name"
    done
    
    # Generate report
    generate_report
    
    echo ""
    echo "✅ Local CI/CD complete!"
    echo ""
    echo "📊 View report: LOCAL_CICD_REPORT.md"
    echo "📁 View results: $RESULTS_DIR/"
}

# Run with optional parallel execution
if [ "$1" = "--parallel" ]; then
    echo "⚡ Running in parallel mode..."
    export -f run_cicd_on_repo
    jq -r '.repos[] | select(.path != null) | "\(.path)|\(.name)"' "$REGISTRY_FILE" | \
        parallel --colsep '|' run_cicd_on_repo {1} {2}
else
    main
fi
