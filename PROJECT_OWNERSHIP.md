# Project Ownership Analysis

## Summary

**141 failed projects analyzed:**

| Owner | Count | Percentage |
|-------|-------|------------|
| **Local (No Git Remote)** | 107 | 75.9% |
| **meta-introspector** | 34 | 24.1% |

## Interpretation

### Local Projects (107 - 76%)

These are **experimental flakes** in `streamofrandom/` with no git remote:
- Not tracked in separate repositories
- Part of the meta-introspector monorepo
- Mostly experiments and prototypes
- Examples: audit-flakes, proof systems, composite flakes

**These are YOUR experiments!**

### meta-introspector Projects (34 - 24%)

These have `meta-introspector` as the GitHub organization:
- Tracked in separate repositories
- Part of the meta-introspector organization
- More formal projects
- Examples: feature flakes, composite flakes, CRQ projects

**These are also YOURS** (meta-introspector org)

## Conclusion

**100% of failed projects are yours!**

- 76% are local experiments (no separate repo)
- 24% are in meta-introspector GitHub org
- 0% are from external organizations

All 141 failed projects are part of the meta-introspector ecosystem.

## meta-introspector Projects (34)

| Project | Type |
|---------|------|
| 03, 09, 12, 14 | Numbered experiments |
| composite-2-3-5-7-11-13-... | Composite feature flakes |
| feature-2-nix-base | Feature: Nix base |
| feature-3-home-dir-creds | Feature: Home credentials |
| feature-5-oauth-creds | Feature: OAuth |
| feature-7-telemetry | Feature: Telemetry |
| feature-11-llm-output-capture | Feature: LLM output |
| feature-13-makefile-input | Feature: Makefile |
| feature-17-yolo-approval | Feature: YOLO approval |
| feature-19-self-source-input | Feature: Self-source |
| crq-document-check | CRQ: Document check |
| crq-search-lattice | CRQ: Search lattice |
| gemini-integration | Gemini integration |
| meta-introspector | Main project |
| nix-ngram-indexer | N-gram indexer |
| workflow-tasks | Workflow tasks |
| zos | ZOS system |
| ... and 17 more | Various |

## Local Experiments (107)

**Categories:**

### Audit Flakes (~20)
- 000_rnix_dump, 001_dump_nix, 002_process_locks
- 002a_grep_references, 002a_inputs_only, 002b_inputs_and_description
- 003_generate_virtual_packages, 004_fold_to_matrix, 005_final_report

### Proof Systems (~10)
- proof, metacoq, bench, brainfuck

### ZOS Tasks (~10)
- act, observe, orient, decide, run-zos-tasks

### LLM Integration (~15)
- llm-data-extractor-flake, gemini-prompt-flake
- keyword-searcher, prompt-template-flake

### Composite Flakes (~8)
- Already listed in meta-introspector section

### Other Experiments (~44)
- Various experimental flakes in streamofrandom/

## Recommendation

Since all projects are yours:

1. **Keep valuable experiments** - Move to proper repos
2. **Archive old experiments** - Move to incomplete_experiments/
3. **Delete abandoned** - Clean up streamofrandom/
4. **Focus on meta-introspector org** - These are more formal

**Priority:** Fix the 34 meta-introspector org projects first (more important than local experiments)
