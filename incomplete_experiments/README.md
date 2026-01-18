# Incomplete Experiments

This directory contains flakes that are incomplete experiments and do not currently build.

## Purpose

Rather than failing CI or cluttering error reports, we:
1. Document what's incomplete
2. Explain what's needed to fix
3. Provide clear next steps
4. Keep them separate from working code

## Categories

### Undefined Variables

Projects with missing variable definitions:

- **keyword-searcher**: Missing `searchScript`
- **llm-data-extractor-flake**: Missing `prompt`
- **archive-flake**: Missing `archiveDerivation`
- **gemini-prompt-flake**: Missing `geminiPrompt`
- **prompt-template-flake**: Missing `promptTemplate`

## How to Complete an Experiment

1. Choose a project from above
2. Read its README.md
3. Implement the missing parts
4. Test: `nix build`
5. If successful, move back to main codebase
6. If abandoned, archive or delete

## Maintenance

- Review quarterly
- Archive abandoned experiments
- Complete or delete old experiments
- Keep this list updated

## Philosophy

**Incomplete experiments are OK!**

We document them rather than:
- ❌ Failing CI on experimental code
- ❌ Hiding errors with placeholder fixes
- ❌ Deleting potentially useful experiments

Instead we:
- ✅ Document what's incomplete
- ✅ Explain what's needed
- ✅ Keep experiments separate
- ✅ Allow easy completion later
