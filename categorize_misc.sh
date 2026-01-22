#!/bin/bash
# Categorize misc files into lore/history/games/ideas

cd /mnt/data1/meta-introspector/docs/misc

# Lore (mythology, rituals, mystical)
mv BURN_RITUAL.md ../lore/ 2>/dev/null
mv CURSED_EVIL_RUNESTONES.md ../lore/ 2>/dev/null
mv RUNESTONE_STONEHENGE.md ../lore/ 2>/dev/null
mv THE_SHIRE.md ../lore/ 2>/dev/null
mv PLATOS_CAVE_MINING.md ../lore/ 2>/dev/null
mv cpu_singing.md ../lore/ 2>/dev/null
mv emoji_tape_proof.md ../lore/ 2>/dev/null
mv poem.md ../lore/ 2>/dev/null
mv poem2.md ../lore/ 2>/dev/null

# History (evolution, timeline, archaeology)
mv EVOLUTION_SUMMARY.md ../history/ 2>/dev/null
mv TIMELINE.md ../history/ 2>/dev/null
mv HISTORICAL_PROJECTS.md ../history/ 2>/dev/null
mv EMACS_CREATED_GCC.md ../history/ 2>/dev/null
mv GIT_COMPRESSION_ARCHAEOLOGY.md ../history/ 2>/dev/null
mv SOURCEFORGE_LINEAGE.md ../history/ 2>/dev/null
mv PROGRAM_EVOLUTION.md ../history/ 2>/dev/null

# Games (game generator, markets, prediction)
mv GAME_GENERATOR.md ../games/ 2>/dev/null
mv BRANCH_PREDICTION_MARKET.md ../games/ 2>/dev/null
mv MEME_MARKETPLACE.md ../games/ 2>/dev/null

# Ideas (proposals, plans, designs)
mv DAO_COORDINATION.md ../ideas/ 2>/dev/null
mv FEDERAL_DAO_INTEGRATION.md ../ideas/ 2>/dev/null
mv DISCOVERY_NETWORK.md ../ideas/ 2>/dev/null
mv MYCELIUM_NETWORK.md ../ideas/ 2>/dev/null
mv LLM_MYCELIUM_NETWORK.md ../ideas/ 2>/dev/null
mv P2P_GIT_MIRROR_DESIGN.md ../ideas/ 2>/dev/null
mv LIVING_MEME_SYSTEM.md ../ideas/ 2>/dev/null
mv HORIZONTAL_MEME_TRANSFER.md ../ideas/ 2>/dev/null
mv INTENT_PREDICTION.md ../ideas/ 2>/dev/null
mv INTELLIGENT_FLAKE_LIFECYCLE.md ../ideas/ 2>/dev/null
mv SERVERLESS_SENATOR_PLUGIN.md ../ideas/ 2>/dev/null
mv THRESHOLD_RECONSTRUCTION.md ../ideas/ 2>/dev/null
mv TIERED_ACCESS.md ../ideas/ 2>/dev/null
mv SAFE_MULTICHAIN_WALLET.md ../ideas/ 2>/dev/null
mv SAFE_WALLET_LOADER.md ../ideas/ 2>/dev/null
mv MULTICHAIN_P2P_ZK_HME.md ../ideas/ 2>/dev/null
mv ONCHAIN_TOP100_CALCULATOR.md ../ideas/ 2>/dev/null
mv INVESTOR_PITCH.md ../ideas/ 2>/dev/null
mv MIGRATION_PLAN.md ../ideas/ 2>/dev/null
mv UNIFICATION_PLAN.md ../ideas/ 2>/dev/null
mv REPRODUCIBLE_BUILD_PLAN.md ../ideas/ 2>/dev/null
mv SINGULARITY_DESIGN.md ../ideas/ 2>/dev/null
mv UNIFIED_ARCHITECTURE.md ../ideas/ 2>/dev/null

echo "✅ Categorized misc files"
echo ""
echo "Summary:"
echo "  docs/lore/: $(ls ../lore/*.md 2>/dev/null | wc -l) files"
echo "  docs/history/: $(ls ../history/*.md 2>/dev/null | wc -l) files"
echo "  docs/games/: $(ls ../games/*.md 2>/dev/null | wc -l) files"
echo "  docs/ideas/: $(ls ../ideas/*.md 2>/dev/null | wc -l) files"
echo "  docs/misc/: $(ls *.md 2>/dev/null | wc -l) files (remaining)"
