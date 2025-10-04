# Security & Configuration Setup Checklist

## 🎯 Overview

This checklist guides you through configuring GitHub repository settings to enable safe parallel agent development for the Chimera project.

**Last Updated:** 2025-10-04  
**Status:** ⚠️ Manual configuration required

---

## ✅ Completed (Automated)

These have been committed to the repository:

- ✅ **CODEOWNERS file** (`.github/CODEOWNERS`)
- ✅ **Dependabot configuration** (`.github/dependabot.yml`)
- ✅ **Auto-merge workflow** (`.github/workflows/auto-merge.yml`)
- ✅ **Contract enforcement workflow** (`.github/workflows/contract-enforcement.yml`)
- ✅ **Quality gate workflows** (6 workflows)
- ✅ **Contract files** (`contracts/node-types.ts`, `contracts/node-trait.rs`)
- ✅ **Custom Copilot instructions** (`.github/copilot-instructions.md`)
- ✅ **Documentation** (agent-instructions.md, parallel-task-strategy.md, etc.)

---

## ⏳ Pending (Manual Configuration)

### Priority 1: Unblock GitHub Copilot (CRITICAL)

**Issue:** Firewall blocking Copilot from accessing external resources

**Solution:** Add URLs to Copilot allowlist

**Steps:**
1. Go to: https://github.com/ArrEssJay/chimera/settings/copilot
2. Click **"Policies"** tab
3. Find **"URL Content Access"** section
4. Add these URLs:
   ```
   npmjs.com
   registry.npmjs.org
   storybook.js.org
   chromatic.com
   crates.io
   static.crates.io
   playwright.dev
   jestjs.io
   docs.rs
   doc.rust-lang.org
   reactflow.dev
   rustwasm.github.io
   developer.mozilla.org
   typescript-lang.org
   ```
5. Click **"Save changes"**

**Verification:**
```bash
# Test if Copilot can now access storybook.js.org
# Assign issue #46 to Copilot and monitor for firewall errors
```

---

### Priority 2: Branch Protection via Rulesets

**Purpose:** Protect main branch while allowing parallel work

**Steps:**

#### Option A: Via GitHub UI

1. Go to: https://github.com/ArrEssJay/chimera/settings/rules
2. Click **"New ruleset"** → **"New branch ruleset"**
3. Create 5 rulesets using configurations from `docs/github-rulesets-configuration.md`:
   - **Contract Protection** - Blocks contract modifications
   - **Main Branch Protection** - Enforces quality on main
   - **Feature Branch Workflow** - Flexible for feature work
   - **Agent PR Requirements** - Metadata for agent PRs
   - **Critical File Protection** - Review for infrastructure

#### Option B: Via GitHub CLI (Faster)

```bash
# Navigate to repo root
cd /Users/rowan/VSCode/chimera

# Run the setup script from docs/github-rulesets-configuration.md
# (Extract the bash script from that file and run it)

# Or create rulesets one by one:
gh api repos/ArrEssJay/chimera/rulesets --method POST --input - <<EOF
{
  "name": "Contract Protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/*"]
    }
  },
  "rules": [
    {
      "type": "file_path_restriction",
      "parameters": {
        "restricted_file_paths": ["contracts/**"]
      }
    }
  ]
}
EOF

# Repeat for other rulesets...
```

**Verification:**
```bash
# List all rulesets
gh api repos/ArrEssJay/chimera/rulesets | jq '.[] | {name, enforcement}'

# Expected output: 5 rulesets with enforcement="active"
```

---

### Priority 3: Enable Auto-Merge

**Purpose:** Allow agents to merge their own PRs when checks pass

**Steps:**
1. Go to: https://github.com/ArrEssJay/chimera/settings
2. Scroll to **"Pull Requests"** section
3. Check ✅ **"Allow auto-merge"**
4. Select merge method: **"Allow squash merging"**
5. Uncheck "Allow merge commits" and "Allow rebase merging"
6. Click **"Save changes"**

**Verification:**
```bash
# Check if auto-merge is enabled
gh api repos/ArrEssJay/chimera | jq '.allow_auto_merge'
# Expected: true
```

---

### Priority 4: GitHub Actions Permissions

**Purpose:** Allow workflows to create PRs and add comments

**Steps:**
1. Go to: https://github.com/ArrEssJay/chimera/settings/actions
2. Under **"Workflow permissions"**:
   - Select ⚪ **"Read and write permissions"**
   - Check ✅ **"Allow GitHub Actions to create and approve pull requests"**
3. Under **"Fork pull request workflows"**:
   - Select ⚪ **"Require approval for first-time contributors"**
4. Click **"Save"**

**Verification:**
```bash
# Check Actions permissions
gh api repos/ArrEssJay/chimera | jq '.permissions'
```

---

### Priority 5: Code Security Settings

**Purpose:** Enable automated security scanning

**Steps:**
1. Go to: https://github.com/ArrEssJay/chimera/settings/security_analysis
2. Enable:
   - ✅ **Dependabot alerts**
   - ✅ **Dependabot security updates**
   - ✅ **Secret scanning**
   - ✅ **Push protection** (prevents committing secrets)
3. Click **"Enable"** for each

**Verification:**
```bash
# Check security settings
gh api repos/ArrEssJay/chimera/vulnerability-alerts
gh api repos/ArrEssJay/chimera/automated-security-fixes
```

---

### Priority 6: MCP Server Configuration (Optional)

**Purpose:** Enhance agent capabilities with additional tools

**Note:** This is optional but recommended for advanced agent features.

**Steps:**
1. Review `docs/mcp-server-configuration.md`
2. Create `.github/copilot-mcp.json` if you want custom MCP servers
3. Implement custom MCP servers for:
   - Contract validation
   - Wave management
   - Code analysis

**Skip this step if:**
- You want to start with basic agent capabilities first
- You're not ready to implement custom MCP servers

---

## 🧪 Testing Configuration

After completing the manual steps, run these tests:

### Test 1: Copilot Access
```bash
# Assign issue #46 to GitHub Copilot
gh issue edit 46 --add-assignee @copilot

# Monitor for firewall errors
# Expected: No firewall errors
```

### Test 2: Branch Protection
```bash
# Try to modify a contract file directly on main
git checkout main
echo "// TEST" >> contracts/node-types.ts
git add contracts/node-types.ts
git commit -m "Test: Should be blocked"
git push

# Expected: Push rejected by ruleset
```

### Test 3: Auto-Merge
```bash
# Create a test PR from an agent
git checkout -b test/auto-merge
echo "// Test" >> chimera-web/src/components/Button.tsx
git add .
git commit -m "[Wave 2] Test auto-merge"
git push -u origin test/auto-merge

# Create PR
gh pr create --title "[Wave 2] Test Auto-Merge" \
  --body "Testing auto-merge functionality" \
  --label automated

# Enable auto-merge on the PR
gh pr merge --auto --squash

# Expected: PR auto-merges when checks pass
```

### Test 4: Workflows
```bash
# Trigger a workflow manually
gh workflow run contract-enforcement.yml

# Check status
gh run list --workflow=contract-enforcement.yml

# Expected: Workflow runs successfully
```

---

## 📊 Configuration Status Dashboard

| Component | Status | Priority | Docs |
|-----------|--------|----------|------|
| CODEOWNERS | ✅ Committed | P1 | `.github/CODEOWNERS` |
| Dependabot | ✅ Committed | P2 | `.github/dependabot.yml` |
| Auto-merge workflow | ✅ Committed | P1 | `.github/workflows/auto-merge.yml` |
| Contract enforcement | ✅ Committed | P1 | `.github/workflows/contract-enforcement.yml` |
| Quality gates | ✅ Committed | P2 | `.github/workflows/*.yml` |
| Copilot instructions | ✅ Committed | P1 | `.github/copilot-instructions.md` |
| Copilot allowlist | ⏳ Manual | **P1** | `docs/github-security-configuration.md` |
| Branch rulesets | ⏳ Manual | **P1** | `docs/github-rulesets-configuration.md` |
| Auto-merge enabled | ⏳ Manual | **P1** | Section in security docs |
| Actions permissions | ⏳ Manual | P2 | Section in security docs |
| Security scanning | ⏳ Manual | P2 | Section in security docs |
| MCP servers | ⏳ Optional | P3 | `docs/mcp-server-configuration.md` |

---

## 🚀 Quick Setup Script

For the fastest setup, run this bash script:

```bash
#!/bin/bash
# quick-setup.sh

echo "🔧 Chimera Security Setup"
echo "========================"
echo ""

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
  echo "❌ GitHub CLI not installed. Install with: brew install gh"
  exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
  echo "⚠️  Not authenticated with GitHub. Running: gh auth login"
  gh auth login
fi

REPO="ArrEssJay/chimera"

echo "📋 Repository: $REPO"
echo ""

# Enable auto-merge
echo "1. Enabling auto-merge..."
gh api repos/$REPO --method PATCH --field allow_auto_merge=true
echo "   ✅ Auto-merge enabled"

# Set Actions permissions
echo "2. Setting Actions permissions..."
gh api repos/$REPO --method PATCH \
  --field default_workflow_permissions=write \
  --field can_approve_pull_request_reviews=true
echo "   ✅ Actions permissions configured"

# Enable security features
echo "3. Enabling security features..."
gh api repos/$REPO/vulnerability-alerts --method PUT
gh api repos/$REPO/automated-security-fixes --method PUT
echo "   ✅ Security scanning enabled"

echo ""
echo "✅ Automated setup complete!"
echo ""
echo "⏳ Manual steps remaining:"
echo "   1. Add Copilot allowlist URLs (Priority 1)"
echo "   2. Create branch rulesets (Priority 1)"
echo ""
echo "📖 See docs/SECURITY_SETUP_CHECKLIST.md for details"
```

**To run:**
```bash
chmod +x quick-setup.sh
./quick-setup.sh
```

---

## ⚠️ Critical Path

**To unblock GitHub Copilot and start parallel development:**

1. **Add Copilot allowlist URLs** (5 minutes)
   - Fixes firewall blocking issue
   - Allows Copilot to access documentation

2. **Create branch rulesets** (10 minutes)
   - Protects contract files
   - Enables safe parallel work

3. **Enable auto-merge** (2 minutes)
   - Allows agent PRs to merge automatically
   - Reduces human bottleneck

4. **Test with issue #46** (5 minutes)
   - Assign to Copilot
   - Verify no firewall errors
   - Monitor PR creation

**Total time: ~22 minutes**

After these steps, parallel agent development can begin! 🚀

---

## 📞 Support

If you encounter issues:

1. **Check workflow logs:** https://github.com/ArrEssJay/chimera/actions
2. **Review documentation:** All docs in `docs/` directory
3. **Test configuration:** Run verification commands above
4. **Check ruleset conflicts:** Review ruleset priority in `docs/github-rulesets-configuration.md`

---

## 🎉 Success Criteria

You'll know configuration is complete when:

- ✅ Copilot can access external URLs (no firewall errors)
- ✅ Contracts are protected (can't push changes to contracts/ directly)
- ✅ Agent PRs can auto-merge when checks pass
- ✅ Workflows run successfully on PRs
- ✅ Issue #46 assigned to Copilot progresses without errors

**Next:** Begin Wave 2 parallel development with 5 agents! 🤖
