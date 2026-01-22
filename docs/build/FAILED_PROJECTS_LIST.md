# Failed Projects List

## Summary

Total failed: 144 projects

## By Category

### Missing-default (10 projects)
- 001_dump_nix
- 03
- eval-context
- gemini-prompt-flake
- main
- nix
- psyche
- test-env-var
- test-secrets-sops

**Status:** 2 fixed, 8 remaining

### Missing-attr (5 projects)
- bench
- brainfuck
- metacoq
- proof
- self-ngram-analyzer

**Status:** 0 fixed, 5 remaining

### Other (129 projects)

#### High Priority - Undefined Variables (32)
**Missing `lib` (9):**
- feature-19-self-source-input
- log-analysis-pipeline
- feature-5-oauth-creds
- feature-7-telemetry
- feature-11-llm-output
- feature-13-makefile-input
- feature-2-nix-base
- feature-3-home-dir-creds
- composite-2-3-nix-base-home

**Other undefined (23):**
- keyword-searcher (searchScript)
- llm-data-extractor-flake (prompt)
- gemini-prompt-flake (geminiPrompt)
- And 20 more...

#### Medium Priority - Flake Attributes (23)
- nix-ngram-indexer
- nix-llm-context
- zos
- workflow-tasks
- crq-document-check
- run-zos-tasks
- meta-orchestrator
- And 16 more...

**Status:** 8 fixed (self inputs), 15 remaining

#### Path Errors (12)
**Composite flakes (8):**
- composite-2-3-5-7-11-13-nix-base-home-oauth-telemetry-llm-output-makefile-input
- composite-2-3-5-7-11-nix-base-home-oauth-telemetry-llm-output
- composite-2-3-5-7-nix-base-home-oauth-telemetry
- composite-2-3-nix-base-home-creds
- composite-2-3-5-nix-base-home-oauth
- And 3 more...

**Other path errors (4):**
- typecheck
- 002a_grep_references
- 22
- And 1 more...

**Status:** 5 documented, 7 remaining

#### Other Issues (~62)
- Build failures
- Cannot find flake
- Coercion errors
- Duplicate attributes
- Various other errors

## Quick Reference

### Ready to Fix (High Impact)
1. **Undefined `lib`** (9 projects) - Add `lib = nixpkgs.lib;`
2. **Missing-attr** (5 projects) - Add missing attributes
3. **Remaining missing-default** (8 projects) - Add packages.default

### Needs Investigation
1. **Path errors** (7 remaining) - Check paths, update references
2. **Cannot find flake** (6 projects) - Find correct URLs
3. **Flake attributes** (15 remaining) - Fix attribute references

### Documented as Incomplete
1. **Undefined variables** (3 projects) - In incomplete_experiments/
2. **Path errors** (5 projects) - In incomplete_experiments/

## Full List (Alphabetical)

000_rnix_dump [other]: expect test failed
001_dump_nix [missing-default]: No packages.default attribute
002a_grep_references [other]: Path '10/12/audit-flakes/002_extract_data/flake.nix' does not exist in Git repos
002a_inputs_only [other]: flake attribute 'checks.x86_64-linux.healthcheck' is not a derivation
002b_inputs_and_description [other]: flake attribute 'checks.x86_64-linux.healthcheck' is not a derivation
002d_processed_lock_files [other]: Could not open file flake: No such file or directory
002_process_locks [other]: cannot coerce a set to a string: { generateBagOfWords = «thunk»; }
003_generate_virtual_packages [other]: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::
003_sanitize_extracted_data [other]: Could not open file /nix/store/b1ayn0ln6n8bm2spz441csqc2ss66az3-hello-2.12.2/ext
004_fold_to_matrix [other]: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::
005_final_report [other]: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::
03 [missing-default]: No packages.default attribute
09 [other]: path '«github:meta-introspector/time-2025/d670d37ce328808bfc0a8e8c6c7d49a61c11d8
12 [other]: flake 'git+file:///mnt/data1/nix/source/github/meta-introspector/streamofrandom/
14 [other]: flake 'self' attribute 'url' is not supported
2025-01-27-build-time-gemini-capture [other]: cannot find flake 'flake:geminiCredsDir' in the flake registries
2025-01-27-gemini-hello-world [other]: Cannot build '/nix/store/d6kqpry1pl9mlk665g46l3154pg1lmz2-gemini-hello-world-tes
22 [other]: Path '09/10/12/binstore/rnix-flake-ast.nar' does not exist in Git repository "/m
act [other]: flake 'self' attribute 'url' is not supported
ai-workflow [other]: undefined variable 'archiveDerivation'
article-wrapper [other]: expected flake output attribute 'packages.x86_64-linux.default' to be a derivati
audit-flakes [other]: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::
audit-with-rust [other]: Path '10/14/audit-flakes/001_collect_locks/flake.nix' does not exist in Git repo
bench [missing-attr]: Missing attribute
binstore-prime-md-indexes [other]: unable to download 'https://api.github.com/repos/meta-introspector/default/commi
bootstrap-mycology-schedule-flake [other]: cannot find flake 'flake:bridgeInstanceFlake' in the flake registries
brainfuck [missing-attr]: Missing attribute
bridge-pattern [other]: cannot find flake 'flake:consumer' in the flake registries
c4-mycology-diagram [other]: expected a string but got a thunk at /mnt/data1/nix/source/github/meta-introspec
c4-use-cases [other]: unable to download 'https://api.github.com/repos/meta-introspector/default/commi
composite-2-3-5-7-11-13-17-19-nix-base-home-oauth-telemetry-llm-output-makefile-input-yolo-self-source [other]: Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git reposit
composite-2-3-5-7-11-13-17-nix-base-home-oauth-telemetry-llm-output-makefile-input-yolo [other]: Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git reposit
composite-2-3-5-7-11-13-nix-base-home-oauth-telemetry-llm-output-makefile-input [other]: Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git reposit
composite-2-3-5-7-11-nix-base-home-oauth-telemetry-llm-output [other]: Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git reposit
composite-2-3-5-7-nix-base-home-oauth-telemetry [other]: Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git reposit
composite-2-3-5-nix-base-home-oauth [other]: Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git reposit
composite-2-3-nix-base-home-creds [other]: Path 'flakes/feature-3-home-dir-creds/default.nix' does not exist in Git reposit
consolidated-impure-gemini-telemetry-modules [other]: Path '10/08/09/27/7-concepts/6-qa-testing/tests/consolidated-impure-gemini-telem
consolidated-impure-gemini-telemetry [other]: undefined variable 'geminiPrompt'
consumer [other]: cannot find flake 'flake:hackathon-status-raw' in the flake registries
crq-document-check [other]: flake 'self' attribute 'url' is not supported
crq-search-lattice [other]: dynamic attribute 'aarch64-linux' already defined at /mnt/data1/nix/source/githu
data-header [other]: undefined variable 'packages'
data-lattice-builder [other]: flake 'self' attribute 'url' is not supported
decide [other]: flake 'self' attribute 'url' is not supported
document-single-flake-pipeline [other]: syntax error, unexpected invalid token, expecting ';'
dry-run-flake-evaluator [other]: expected flake output attribute 'packages.x86_64-linux.default' to be a derivati
dwim [other]: flake 'self' attribute 'url' is not supported
eval-context [missing-default]: No packages.default attribute
examples [other]: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::
feature-11-llm-output-capture [other]: undefined variable 'lib'
feature-13-makefile-input [other]: undefined variable 'lib'
feature-17-yolo-approval [other]: undefined variable 'lib'
feature-19-self-source-input [other]: undefined variable 'lib'
feature-2-nix-base [other]: undefined variable 'nix'
feature-3-home-dir-creds [other]: undefined variable 'lib'
feature-5-oauth-creds [other]: undefined variable 'lib'
feature-7-telemetry-capture [other]: undefined variable 'lib'
file-indexer [other]: undefined variable 'searchPath'
file-splitter [other]: undefined variable 'files'
flake-reconstruction-lattice [other]: path '«github:meta-introspector/time-2025/ef1039f2bbbd62cb802eab667078b4566d9f9b
gemini-integration [other]: attribute 'buildInputs' already defined at /mnt/data1/nix/source/github/meta-int
gemini-prompt-flake [missing-default]: No packages.default attribute
github-code-search [other]: unable to download 'https://api.github.com/repos/meta-introspector/default/commi
github-data-fetcher-flake [other]: undefined variable 'githubRepo'
hackathon-mycology-workflow-puml [other]: undefined variable 'monster_genome_data'
impure-wrapper [other]: undefined variable 'spec'
ipfs-store [other]: input 'flake-utils/systems' follows a non-existent input 'nixpkgs/lib/systems/fl
keyword-searcher [other]: undefined variable 'searchScript'
killerjoke [other]: follow cycle detected: [base -> base]
lean4-verifier [other]: unable to download 'https://api.github.com/repos/meta-introspector/lean4/commits
list-files [other]: expected flake output attribute 'packages.x86_64-linux.default' to be a derivati
llm-data-extractor-flake [other]: undefined variable 'prompt'
llm-nix-simulator-flake [other]: cannot find flake 'flake:llmApiWrapper' in the flake registries
llm-result-purifier-flake [other]: cannot find flake 'flake:impureLlmResult' in the flake registries
llm-task-template [other]: path '«github:meta-introspector/time-2025/ef1039f2bbbd62cb802eab667078b4566d9f9b
lmfdb2nix-implementation-task [other]: unable to download 'https://api.github.com/repos/meta-introspector/lean4/commits
lmfdb2nix [other]: flake 'self' attribute 'url' is not supported
locktoinput [other]: path '//data' is a symlink
log-analysis-pipeline [other]: undefined variable 'lib'
loop2 [other]: flake 'self' attribute 'url' is not supported
main [missing-default]: No packages.default attribute
mcts-solana-flake [other]: undefined variable 'promptTemplate'
metacoq [missing-attr]: Missing attribute
meta-indexer [other]: flake 'self' attribute 'url' is not supported
meta-introspector [other]: failed to run custom build command for `openssl-sys v0.9.111`
meta-orchestrator [other]: flake 'self' attribute 'url' is not supported
mina-zkp-integration-task [other]: input 'flake-utils/systems' follows a non-existent input 'nixpkgs/lib/systems/fl
mini-zkp-verifier [other]: input 'flake-utils/systems' follows a non-existent input 'nixpkgs/lib/systems/fl
monster-knot-calculator [other]: unable to download 'https://api.github.com/repos/meta-introspector/nar-similarit
mycology-workflow [other]: undefined variable 'geminiPrompt'
nar-binstore-builder [other]: flake 'self' attribute 'url' is not supported
nar-loader [other]: undefined variable 'narFile'
nar-locator [other]: undefined variable 'archiveDerivation'
nar [other]: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::
nar-similarity-search [other]: undefined variable 'keywords_json'
nix2 [other]: unable to download 'https://api.github.com/repos/meta-introspector/get-nix-file-
nix_concepts_and_facts [other]: function 'anonymous lambda' called without required argument 'lib'
nix-duplication-detector [other]: flake 'self' attribute 'url' is not supported
nix-llm-context [other]: flake 'self' attribute 'url' is not supported
nix-llm-task [other]: path '«github:meta-introspector/time-2025/ef1039f2bbbd62cb802eab667078b4566d9f9b
nix-log-sample-extractor-flake [other]: undefined variable 'lib'
nix [missing-default]: No packages.default attribute
nix-ngram-indexer [other]: flake 'self' attribute 'url' is not supported
number-searcher [other]: undefined variable 'searchScript'
number-searches [other]: undefined variable 'searchScript'
observe [other]: flake 'self' attribute 'url' is not supported
oeis-indexer [other]: flake 'self' attribute 'url' is not supported
orchestration-flake [other]: path '«github:meta-introspector/time-2025/d670d37ce328808bfc0a8e8c6c7d49a61c11d8
orient [other]: flake 'self' attribute 'url' is not supported
orient-test [other]: flake 'self' attribute 'url' is not supported
parser [other]: follow cycle detected: [base -> base]
preconditions [other]: unable to download 'https://api.github.com/repos/meta-introspector/streamofrando
predicate-analyzer [other]: undefined variable 'allFiles'
project-index-nar [other]: flake 'self' attribute 'flake' is not supported
project-scheduler-flake [other]: undefined variable 'promptTemplate'
prolog [other]: 'swiProlog' has been renamed to/replaced by 'swi-prolog'
proof [missing-attr]: Missing attribute
psyche [missing-default]: No packages.default attribute
pytorch [other]: 'pytorch' has been renamed to/replaced by 'torch'
qa-all-new-flakes [other]: undefined variable 'archiveDerivation'
qa-nar-similarity-pipeline [other]: invalid flake input attribute path element 'narSimilaritySearch.narLocatorFlake'
repo-analyzer [other]: attempt to call something which is not a function but a set: { lastModified = 17
response-007-cli-nar-output [other]: cargoHash, cargoVendorDir, cargoDeps, or cargoLock must be set
run-zos-tasks [other]: flake 'self' attribute 'url' is not supported
rust-audit [other]: expected a set but found a path: «github:meta-introspector/time-2025/24eff6126a7
search-results [other]: access to absolute path '/nix/store/10/11/nar-locator/flake.nix' is forbidden in
search-utils [other]: undefined variable 'searchUtils'
self-ngram-analyzer [missing-attr]: Missing attribute
solana-integration [other]: input 'flake-utils/systems' follows a non-existent input 'nixpkgs/lib/systems/fl
src [other]: Path 'src/Cargo.toml' does not exist in Git repository "/mnt/data1/nix".
telemetry-wrapper [other]: undefined variable 'geminiPrompt'
test-bug-repro-nix-2gram-indexer [other]: flake 'self' attribute 'url' is not supported
test-env-var [missing-default]: No packages.default attribute
test [other]: Path 'test/flake.nix' in the repository "/mnt/data1/nix" is not tracked by Git.
test-package-bag-of-words [other]: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::
test-secrets-sops [missing-default]: No packages.default attribute
typecheck [other]: Path '10/15/zos/ooda/tasks/act/flake.nix' does not exist in Git repository "/mnt
union [other]: flake attribute 'packages.x86_64-linux.voyager-modules-plugins-names' is not a d
workflow-tasks [other]: flake 'self' attribute 'url' is not supported
zos [other]: flake 'self' attribute 'flake' is not supported
