# Nix Build Failure Classification

## Summary

- **Total builds**: 396
- **Successful**: 252 (63.6%)
- **Failed**: 144 (36.4%)

## Failures by Category

- **other**: 129 (89.6%)
- **missing-default**: 10 (6.9%)
- **missing-attr**: 5 (3.5%)

## Detailed Breakdown

### missing-attr (5 projects)

- **bench**: Missing attribute
- **brainfuck**: Missing attribute
- **metacoq**: Missing attribute
- **proof**: Missing attribute
- **self-ngram-analyzer**: Missing attribute

### missing-default (9 projects)

- **001_dump_nix**: No packages.default attribute
- **03**: No packages.default attribute
- **eval-context**: No packages.default attribute
- **gemini-prompt-flake**: No packages.default attribute
- **main**: No packages.default attribute
- **nix**: No packages.default attribute
- **psyche**: No packages.default attribute
- **test-env-var**: No packages.default attribute
- **test-secrets-sops**: No packages.default attribute

### other (127 projects)

- **000_rnix_dump**: expect test failed
- **002_process_locks**: cannot coerce a set to a string: { generateBagOfWords = «thunk»; }
- **002a_grep_references**: Path '10/12/audit-flakes/002_extract_data/flake.nix' does not exist in Git repository "/mnt/data1/ni
- **002a_inputs_only**: flake attribute 'checks.x86_64-linux.healthcheck' is not a derivation
- **002b_inputs_and_description**: flake attribute 'checks.x86_64-linux.healthcheck' is not a derivation
- **002d_processed_lock_files**: Could not open file flake: No such file or directory
- **003_generate_virtual_packages**: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::ParsedURL::renderAut
- **003_sanitize_extracted_data**: Could not open file /nix/store/b1ayn0ln6n8bm2spz441csqc2ss66az3-hello-2.12.2/extracted-data.json: No
- **004_fold_to_matrix**: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::ParsedURL::renderAut
- **005_final_report**: [0mAssertion 'path.empty() || path.front().empty()' failed in std::string nix::ParsedURL::renderAut
- ... and 117 more

## Recommendations

### Fix missing-default (10 projects)
Add `packages.${{system}}.default` to flake.nix:
```nix
packages.${system}.default = pkgs.hello;
```
