#!/bin/bash
# Further categorize remaining misc files

cd /mnt/data1/meta-introspector/docs/misc

# Sessions
mv SESSION_*.md ../sessions/ 2>/dev/null
mv TODO_SESSION_RESUME.md ../sessions/ 2>/dev/null

# Reference (READMEs, summaries, specs)
mv README_*.md ../reference/ 2>/dev/null
mv *_SUMMARY.md ../reference/ 2>/dev/null
mv CURRENT_SERVER_SPECS.md ../reference/ 2>/dev/null
mv POWER_REQUIREMENTS.md ../reference/ 2>/dev/null
mv RESOURCE_SCHEDULE.md ../reference/ 2>/dev/null
mv QUICKSTART.md ../reference/ 2>/dev/null
mv ROADMAP*.md ../reference/ 2>/dev/null
mv MANIFEST.md ../reference/ 2>/dev/null

# Move technical concepts to theory
mv COMBINATORIAL_COMPOSITION.md ../theory/ 2>/dev/null
mv COMPILATION_AS_WITNESS.md ../theory/ 2>/dev/null
mv COMPLETE_SINGULARITY.md ../theory/ 2>/dev/null
mv CONSTANT_SUBSTRATE.md ../theory/ 2>/dev/null
mv constraint_accumulation.md ../theory/ 2>/dev/null
mv cross_labeling_extraction.md ../theory/ 2>/dev/null
mv electron_flow.md ../theory/ 2>/dev/null
mv em_signature.md ../theory/ 2>/dev/null
mv fourier_instructions.md ../theory/ 2>/dev/null
mv quasifibers.md ../theory/ 2>/dev/null
mv semiosis.md ../theory/ 2>/dev/null
mv snark_witness.md ../theory/ 2>/dev/null

# Move meta-meme to its own category
mkdir -p ../meta-meme 2>/dev/null
mv META_MEME*.md ../meta-meme/ 2>/dev/null
mv METAMEME_CA.md ../meta-meme/ 2>/dev/null
mv my_meta_meme_badge.md ../meta-meme/ 2>/dev/null

# Move deployment/infrastructure
mkdir -p ../infrastructure 2>/dev/null
mv DEPLOYMENT*.md ../infrastructure/ 2>/dev/null
mv ZOS_*.md ../infrastructure/ 2>/dev/null
mv WASM_*.md ../infrastructure/ 2>/dev/null
mv ROBLOX_RUNTIME.md ../infrastructure/ 2>/dev/null

# Move analysis/tools
mkdir -p ../analysis 2>/dev/null
mv *_ANALYSIS.md ../analysis/ 2>/dev/null
mv FILE_INDEX*.md ../analysis/ 2>/dev/null
mv KEYWORD_ANALYSIS.md ../analysis/ 2>/dev/null
mv UNDEFINED_VARIABLE_ANALYSIS.md ../analysis/ 2>/dev/null
mv SELF_ATTRIBUTE_ERRORS.md ../analysis/ 2>/dev/null

# Move project management
mkdir -p ../projects 2>/dev/null
mv PROJECT_OWNERSHIP.md ../projects/ 2>/dev/null
mv PUBLIC_PRIVATE_SEPARATION.md ../projects/ 2>/dev/null
mv PUBLIC_SUBSTRATE.md ../projects/ 2>/dev/null
mv PYTHON_DELETE_LIST.md ../projects/ 2>/dev/null
mv PYTHON_TO_RUST_CONVERSION.md ../projects/ 2>/dev/null

# Move Solana/Jupiter specific
mkdir -p ../solana 2>/dev/null
mv SOLANA_*.md ../solana/ 2>/dev/null
mv JUPITER_*.md ../solana/ 2>/dev/null

echo "✅ Further categorization complete"
echo ""
echo "New categories:"
echo "  docs/sessions/: $(ls ../sessions/*.md 2>/dev/null | wc -l) files"
echo "  docs/reference/: $(ls ../reference/*.md 2>/dev/null | wc -l) files"
echo "  docs/meta-meme/: $(ls ../meta-meme/*.md 2>/dev/null | wc -l) files"
echo "  docs/infrastructure/: $(ls ../infrastructure/*.md 2>/dev/null | wc -l) files"
echo "  docs/analysis/: $(ls ../analysis/*.md 2>/dev/null | wc -l) files"
echo "  docs/projects/: $(ls ../projects/*.md 2>/dev/null | wc -l) files"
echo "  docs/solana/: $(ls ../solana/*.md 2>/dev/null | wc -l) files"
echo ""
echo "Remaining in misc: $(ls *.md 2>/dev/null | wc -l) files"
