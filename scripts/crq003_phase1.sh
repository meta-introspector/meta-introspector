#!/usr/bin/env bash
# CRQ-003: OpenLightLLM → Rust Migration
# Phase 1: Rebase & Analyze

set -euo pipefail

LITELLM_DIR="/home/mdupont/projects/agentartificial/devops/vendor/litellm"
OUTPUT_DIR="data/crq003"

mkdir -p "$OUTPUT_DIR"

echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║         CRQ-003: OpenLightLLM → Rust Migration                       ║"
echo "║         Phase 1: Rebase & Analyze                                    ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo

# Step 1: Rebase
echo "📥 Step 1: Rebasing on upstream..."
cd "$LITELLM_DIR"
git fetch upstream
git status

echo
read -p "Rebase now? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git rebase upstream/main || {
        echo "❌ Rebase conflicts. Resolve manually."
        exit 1
    }
    echo "✅ Rebased successfully"
fi

# Step 2: Analyze
echo
echo "📊 Step 2: Analyzing codebase..."

# Count Python files
echo "Python files:"
find litellm -name "*.py" | wc -l

# Lines of code
echo
echo "Lines of code:"
cloc litellm/ --json > "$OUTPUT_DIR/cloc.json" 2>/dev/null || echo "Install cloc for detailed stats"

# Core components
echo
echo "Core components:"
ls -1 litellm/*.py | head -10

# Provider adapters
echo
echo "Provider adapters:"
ls -1 litellm/llms/ 2>/dev/null | head -10 || echo "Check litellm/llms/"

# Generate component inventory
echo
echo "📝 Generating component inventory..."
cat > "$OUTPUT_DIR/components.txt" << 'INVENTORY'
Core Components:
1. router.py - Request routing and provider selection
2. proxy/proxy_server.py - FastAPI server
3. llm_request.py - LLM request handling
4. token_counter.py - Rate limiting
5. budget_manager.py - Cost tracking

Provider Adapters:
- OpenAI
- Anthropic (Claude)
- Google (Gemini)
- AWS Bedrock
- Azure
- HuggingFace
- Cohere
- Replicate

Priority for Rust migration:
1. router.py (HIGH)
2. proxy_server.py (HIGH)
3. llm_request.py (HIGH)
4. token_counter.py (MEDIUM)
5. budget_manager.py (MEDIUM)
6. Provider adapters (MEDIUM)
INVENTORY

cat "$OUTPUT_DIR/components.txt"

echo
echo "✅ Analysis complete!"
echo "📁 Output: $OUTPUT_DIR/"
echo
echo "Next steps:"
echo "1. Review component inventory"
echo "2. Start Phase 2: Architecture design"
echo "3. Begin lifting with: python3 scripts/build/lift_python.py <file>"
