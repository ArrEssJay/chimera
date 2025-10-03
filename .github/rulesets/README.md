# GitHub Rulesets

This directory contains JSON definitions for GitHub repository rulesets.

## Rulesets

1. **main-branch-protection.json** - Protects the main branch
2. **feature-branch-workflow.json** - Rules for feature branches

## Import Rulesets

```bash
# From repo root
./scripts/import-rulesets.sh
```

## Manual Import

```bash
# Import main branch protection
gh api repos/ArrEssJay/chimera/rulesets \
  --method POST \
  --input .github/rulesets/main-branch-protection.json

# Import feature branch workflow
gh api repos/ArrEssJay/chimera/rulesets \
  --method POST \
  --input .github/rulesets/feature-branch-workflow.json
```

## View Rulesets

```bash
# List all rulesets
gh ruleset list

# View specific ruleset
gh ruleset view <ruleset-id>

# Check rules for a branch
gh ruleset check main
```
