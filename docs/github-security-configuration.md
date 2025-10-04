# GitHub Repository Security Configuration for Agentic Coding

## 🔒 Required Security Settings

### 1. GitHub Copilot Allowlist (Repository Settings)

**Path:** Repository Settings → Copilot → Coding agent settings

**Required URLs to Whitelist:**

```
# Package Registries
npmjs.com
registry.npmjs.org
registry.yarnpkg.com

# CDNs and Asset Hosts
cdn.jsdelivr.net
unpkg.com
esm.sh

# Storybook (for UI component development)
storybook.js.org
chromatic.com

# Rust/Cargo
crates.io
static.crates.io
github.com/rust-lang

# WASM Tools
rustwasm.github.io

# Documentation Sites
developer.mozilla.org
docs.rs
doc.rust-lang.org

# Testing/Build Tools
playwright.dev
jestjs.io
```

**How to Add:**
1. Go to https://github.com/ArrEssJay/chimera/settings/copilot
2. Scroll to "Copilot coding agent settings"
3. Click "Edit allowlist"
4. Add each URL above
5. Click "Save"

---

### 2. Branch Protection Rules

**Path:** Repository Settings → Branches → Branch protection rules

#### For `main` branch:

```yaml
Branch name pattern: main

Required settings:
  ✅ Require a pull request before merging
    ✅ Require approvals: 0 (for agent auto-merge)
    ⬜ Dismiss stale pull request approvals when new commits are pushed
    ✅ Require review from Code Owners (optional)
  
  ✅ Require status checks to pass before merging
    ✅ Require branches to be up to date before merging
    Required status checks:
      - detect-conflicts
      - verify-contracts-locked
      - validate-component (for UI PRs)
      - rust-validation (for Rust PRs)
      - node-unit-test (for node PRs)
  
  ✅ Require conversation resolution before merging
  
  ⬜ Require signed commits (optional but recommended)
  
  ✅ Require linear history (prevents merge commits)
  
  ⬜ Require deployments to succeed before merging (if using preview deployments)
  
  ❌ Do not allow bypassing the above settings
  
  ⬜ Allow force pushes (DISABLE for safety)
  
  ⬜ Allow deletions (DISABLE for safety)
```

**Commands to set via CLI:**
```bash
# Install GitHub CLI if needed
brew install gh

# Set branch protection
gh api repos/ArrEssJay/chimera/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["detect-conflicts","verify-contracts-locked"]}' \
  --field enforce_admins=false \
  --field required_pull_request_reviews='{"dismiss_stale_reviews":false,"require_code_owner_reviews":false,"required_approving_review_count":0}' \
  --field restrictions=null \
  --field required_linear_history=true \
  --field allow_force_pushes=false \
  --field allow_deletions=false
```

---

#### For feature branches (Wave isolation):

```yaml
Branch name pattern: feature/*

Required settings:
  ✅ Require status checks to pass before merging
    Required status checks:
      - detect-conflicts
      - verify-contracts-locked
  
  ❌ Do not require pull request reviews (agents merge to feature branches)
  
  ✅ Allow force pushes (for feature branch rebasing)
  
  ⬜ Allow deletions (after merge to main)
```

---

### 3. GitHub Actions Permissions

**Path:** Repository Settings → Actions → General

```yaml
Actions permissions:
  ✅ Allow all actions and reusable workflows
  
Workflow permissions:
  ✅ Read and write permissions
    ✅ Allow GitHub Actions to create and approve pull requests
  
Fork pull request workflows:
  ✅ Require approval for first-time contributors
```

**Why:** Agents need to comment on PRs, update status checks, and potentially auto-merge.

---

### 4. Code Security Settings

**Path:** Repository Settings → Security

#### Code scanning:
```yaml
✅ Enable Dependabot alerts
✅ Enable Dependabot security updates
✅ Enable Dependabot version updates

Create .github/dependabot.yml:
```

```yaml
# .github/dependabot.yml
version: 2
updates:
  # npm dependencies
  - package-ecosystem: "npm"
    directory: "/chimera-web"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
    labels:
      - "dependencies"
      - "automated"
  
  # Cargo dependencies
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
    labels:
      - "dependencies"
      - "automated"
  
  # GitHub Actions
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    labels:
      - "dependencies"
      - "automated"
```

#### Secret scanning:
```yaml
✅ Enable secret scanning
✅ Enable push protection
```

---

### 5. Collaborators & Access

**Path:** Repository Settings → Collaborators and teams

```yaml
Recommended:
  - GitHub Copilot: Write access (automatic with Enterprise)
  - CI/CD bots: Write access
  - Human maintainers: Admin access
  
Access levels:
  - Read: Can view and clone
  - Triage: Can manage issues/PRs without write access
  - Write: Can push to non-protected branches
  - Maintain: Can manage repo settings
  - Admin: Full access
```

---

### 6. Repository Rulesets (Modern Alternative to Branch Protection)

**Path:** Repository Settings → Rules → Rulesets

**Create Ruleset: "Contract Enforcement"**
```yaml
Name: Contract Enforcement
Enforcement: Active
Target: All branches

Rules:
  ✅ Block force pushes
  ✅ Require status checks to pass
    - verify-contracts-locked
    - detect-conflicts
  ✅ Require pull request before merging
  ✅ Require file path restrictions
    Blocked paths:
      - contracts/** (cannot be modified)
```

**Create Ruleset: "Main Branch Protection"**
```yaml
Name: Main Branch Protection
Enforcement: Active
Target: main branch

Rules:
  ✅ Require linear history
  ✅ Block force pushes
  ✅ Require status checks
  ✅ Require pull request
  ✅ Restrict deletions
```

---

### 7. Auto-merge Settings

**Enable auto-merge for agent PRs:**

Add to `.github/workflows/auto-merge.yml`:

```yaml
name: Auto-merge Agent PRs

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  auto-merge:
    runs-on: ubuntu-latest
    if: |
      github.event.pull_request.user.login == 'github-actions[bot]' ||
      github.event.pull_request.user.login == 'copilot[bot]'
    
    steps:
      - name: Wait for status checks
        uses: fountainhead/action-wait-for-check@v1.1.0
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          checkName: 'detect-conflicts,verify-contracts-locked'
          ref: ${{ github.event.pull_request.head.sha }}
      
      - name: Enable auto-merge
        if: success()
        run: |
          gh pr merge --auto --squash "${{ github.event.pull_request.number }}"
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

### 8. CODEOWNERS File

**Create `.github/CODEOWNERS`:**

```
# Global owners
* @ArrEssJay

# Contract files require explicit approval
/contracts/** @ArrEssJay

# Critical infrastructure
/.github/workflows/** @ArrEssJay
/Cargo.toml @ArrEssJay
/rust-toolchain.toml @ArrEssJay

# Agent-owned files (no human review required)
/chimera-web/src/components/** 
/chimera-core/src/nodes/** 

# Documentation
/docs/** @ArrEssJay
```

This ensures:
- Contract changes require human approval
- Infrastructure changes require human approval
- Agent-generated code can auto-merge

---

### 9. GitHub Copilot Workspace Settings

**Path:** Repository Settings → Copilot → Workspace settings

```yaml
✅ Enable GitHub Copilot for this repository

Allowed features:
  ✅ Code completions
  ✅ Chat
  ✅ Pull request summaries
  ✅ Workspace agent (for issue assignment)

Code review settings:
  ✅ Enable automated code reviews
  ⬜ Require human review for critical changes
```

---

### 10. Secrets and Variables

**Path:** Repository Settings → Secrets and variables → Actions

**Required secrets:**
```yaml
# None required for basic setup

# Optional for enhanced features:
CHROMATIC_TOKEN          # For visual regression testing
CODECOV_TOKEN            # For coverage reports
SLACK_WEBHOOK_URL        # For notifications
```

**Required variables:**
```yaml
# Example configuration variables
AGENT_AUTO_MERGE=true
REQUIRED_COVERAGE=80
CONTRACT_ENFORCEMENT=strict
```

---

## 🚀 Quick Setup Script

Run this to configure everything at once:

```bash
#!/bin/bash
# setup-repo-security.sh

REPO="ArrEssJay/chimera"

echo "🔒 Configuring repository security for agentic coding..."

# 1. Enable auto-merge
gh api repos/$REPO/update \
  --field allow_auto_merge=true \
  --field allow_squash_merge=true \
  --field allow_merge_commit=false \
  --field allow_rebase_merge=false

# 2. Set branch protection for main
gh api repos/$REPO/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["detect-conflicts","verify-contracts-locked"]}' \
  --field required_pull_request_reviews='{"required_approving_review_count":0}' \
  --field required_linear_history=true \
  --field allow_force_pushes=false

# 3. Enable Dependabot
gh api repos/$REPO/vulnerability-alerts --method PUT
gh api repos/$REPO/automated-security-fixes --method PUT

# 4. Set Actions permissions
gh api repos/$REPO --method PATCH \
  --field allow_actions=true \
  --field actions_permissions=all

echo "✅ Repository security configured!"
echo ""
echo "⚠️  Manual steps still required:"
echo "1. Add URLs to Copilot allowlist: https://github.com/$REPO/settings/copilot"
echo "2. Create CODEOWNERS file"
echo "3. Review and test auto-merge workflow"
```

---

## 🔍 Verification Checklist

After configuration, verify:

- [ ] Copilot can access storybook.js.org (check agent logs)
- [ ] Branch protection rules are active
- [ ] Required status checks are configured
- [ ] Auto-merge is enabled
- [ ] CODEOWNERS file exists
- [ ] Dependabot alerts are enabled
- [ ] Secret scanning is active
- [ ] Workflow permissions allow PR comments
- [ ] Contract enforcement workflow runs on PRs
- [ ] Feature branches have appropriate permissions

---

## 🆘 Troubleshooting

### Issue: "Firewall rules blocked me from connecting"
**Solution:** Add URLs to Copilot allowlist (see section 1)

### Issue: "Required status checks not found"
**Solution:** Wait for first PR to run workflows, then add checks to branch protection

### Issue: "Auto-merge not available"
**Solution:** Enable in repo settings → General → "Allow auto-merge"

### Issue: "Copilot can't create PRs"
**Solution:** Check Actions permissions → "Allow GitHub Actions to create PRs"

### Issue: "Contract files were modified"
**Solution:** Ensure contract-enforcement.yml workflow is active and contracts/ is protected

---

## 📋 Summary

**Critical Settings:**
1. ✅ Copilot allowlist configured
2. ✅ Branch protection on `main`
3. ✅ Auto-merge enabled
4. ✅ Contract enforcement active
5. ✅ CODEOWNERS file created

**Result:** Agents can work in parallel, auto-merge safe changes, while critical files remain protected.
