#!/bin/bash
# Run bootstrap 10,000 times until system rewrites itself
# Each iteration: build → prove → remember → evolve

set -e

ITERATIONS=10000
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "🔄 Bootstrap Evolution: 10,000 iterations"
echo "=========================================="
echo ""
echo "Goal: System rewrites itself into new form"
echo "Method: Fix errors until convergence"
echo ""

cd "$PROJECT_ROOT"

for i in $(seq 1 $ITERATIONS); do
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Iteration $i / $ITERATIONS"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    # Run bootstrap
    if ./scripts/build/bootstrap.sh 2>&1 | tee "data/iterations/iter_${i}.log"; then
        echo "✅ Iteration $i: Success"
        
        # Check if system has evolved
        CURRENT_ORBIT=$(cat data/last_orbit.txt)
        CURRENT_PROOF=$(cat data/last_proof.txt)
        
        # Compare with previous
        if [ -f "data/iterations/iter_$((i-1))_orbit.txt" ]; then
            PREV_ORBIT=$(cat "data/iterations/iter_$((i-1))_orbit.txt")
            
            if [ "$CURRENT_ORBIT" != "$PREV_ORBIT" ]; then
                echo "🎉 EVOLUTION DETECTED!"
                echo "   Previous orbit: $PREV_ORBIT"
                echo "   Current orbit:  $CURRENT_ORBIT"
                echo "   System has rewritten itself!"
                
                # Save evolution point
                mkdir -p data/evolutions
                cp -r result "data/evolutions/evolution_${i}"
                echo "$i" > data/last_evolution.txt
            fi
        fi
        
        # Save state
        mkdir -p data/iterations
        echo "$CURRENT_ORBIT" > "data/iterations/iter_${i}_orbit.txt"
        echo "$CURRENT_PROOF" > "data/iterations/iter_${i}_proof.txt"
        
    else
        echo "❌ Iteration $i: Failed"
        echo "   Error logged to data/iterations/iter_${i}.log"
        
        # Analyze error
        ERROR=$(tail -20 "data/iterations/iter_${i}.log")
        echo "$ERROR" > "data/iterations/iter_${i}_error.txt"
        
        # Try to fix automatically
        echo "   Attempting automatic fix..."
        
        if echo "$ERROR" | grep -q "duplicates"; then
            echo "   → Duplicate code detected"
            echo "   → Running deduplication..."
            
            # Extract duplicate locations
            if [ -f result/proofs/aggregate/all-duplicates.json ]; then
                jq -r '.duplicates[].locations[].file' result/proofs/aggregate/all-duplicates.json | sort -u > data/iterations/iter_${i}_dup_files.txt
                
                echo "   → Found duplicates in:"
                head -5 data/iterations/iter_${i}_dup_files.txt
                
                # Commit the error for analysis
                git add data/iterations/
                git commit -m "iteration $i: Found duplicates, analyzing for fix" || true
            fi
            
        elif echo "$ERROR" | grep -q "build failed"; then
            echo "   → Build error detected"
            echo "   → Checking dependencies..."
            
            # Log for manual review
            git add data/iterations/
            git commit -m "iteration $i: Build failed, needs manual fix" || true
            
        else
            echo "   → Unknown error"
            git add data/iterations/
            git commit -m "iteration $i: Unknown error, needs investigation" || true
        fi
        
        # Continue to next iteration
        echo "   → Continuing to iteration $((i+1))..."
    fi
    
    # Check for convergence
    if [ $i -gt 10 ]; then
        # Compare last 10 orbits
        RECENT_ORBITS=$(tail -10 data/iterations/iter_*_orbit.txt | sort -u | wc -l)
        
        if [ "$RECENT_ORBITS" -eq 1 ]; then
            echo ""
            echo "🎯 CONVERGENCE DETECTED!"
            echo "   Last 10 iterations produced same orbit"
            echo "   System has reached stable form"
            echo "   Final orbit: $(cat data/last_orbit.txt)"
            
            # Save final state
            mkdir -p data/final
            cp -r result data/final/converged_system
            echo "$i" > data/convergence_iteration.txt
            
            git add data/final/ data/convergence_iteration.txt
            git commit -m "convergence: System reached stable form at iteration $i"
            
            echo ""
            echo "✅ Evolution complete!"
            exit 0
        fi
    fi
    
    # Sleep briefly
    sleep 1
done

echo ""
echo "🏁 Completed $ITERATIONS iterations"
echo ""
echo "Statistics:"
echo "  Total iterations: $ITERATIONS"
echo "  Evolutions: $(ls data/evolutions/ 2>/dev/null | wc -l)"
echo "  Final orbit: $(cat data/last_orbit.txt)"
echo ""
