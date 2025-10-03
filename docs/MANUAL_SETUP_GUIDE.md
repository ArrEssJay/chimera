# Quick Setup Guide: GitHub Rulesets & Security

## 🎯 What You Need to Do

GitHub Rulesets cannot be fully created via CLI - you must use the **GitHub UI**. This guide walks you through the manual setup.

**Time required:** ~15 minutes

---

## ✅ Step 1: View Current Rulesets

```bash
cd /Users/rowan/VSCode/chimera
gh ruleset list
```

**Current rulesets:**
- "Copilot review for default branch" (already exists)

---

## 🔧 Step 2: Add Main Branch Protection

### Go to GitHub UI:
https://github.com/ArrEssJay/chimera/settings/rules

### Click "New ruleset" → "New branch ruleset"

### Configure:

**Ruleset name:** `Main Branch Protection`

**Enforcement status:** ✅ Active

**Target branches:**
- ✅ Add target
- Select: `Default branch`

**Rules to enable:**

1. ✅ **Restrict deletions**
2. ✅ **Require linear history**
3. ✅ **Require a pull request before merging**
   - Required approvals: `0` (for auto-merge)
   - ✅ Require review from Code Owners
4. ✅ **Require status checks to pass**
   - ✅ Require branches to be up to date
   - Add required checks:
     - `detect-conflicts`
     - `verify-contracts-locked`

**Bypass list:**
- Leave empty (no bypasses)

### Click "Create"

---

## 🔧 Step 3: Add Feature Branch Workflow

### Click "New ruleset" → "New branch ruleset"

### Configure:

**Ruleset name:** `Feature Branch Workflow`

**Enforcement status:** ✅ Active

**Target branches:**
- ✅ Add target
- Select: `Include by pattern`
- Patterns:
  - `feature/*`
  - `wave-*`

**Rules to enable:**

1. ✅ **Require a pull request before merging**
   - Required approvals: `0`
   - ⬜ Require review from Code Owners (unchecked)
2. ✅ **Require status checks to pass**
   - ⬜ Require branches to be up to date (unchecked - more flexible)
   - Add required checks:
     - `detect-conflicts`

### Click "Create"

---

## 📋 Step 4: Verify Setup

```bash
# List all rulesets
gh ruleset list

# Expected output:
# - Copilot review for default branch (active)
# - Main Branch Protection (active)
# - Feature Branch Workflow (active)

# Check what rules apply to main
gh ruleset check main

# Check what rules apply to a feature branch
gh ruleset check feature/test
```

---

## 🔒 Contract Protection (Already Done ✅)

Contract files are protected via **CODEOWNERS** file:

```bash
cat .github/CODEOWNERS
```

This ensures:
- `/contracts/**` requires @ArrEssJay approval
- `/.github/workflows/**` requires @ArrEssJay approval
- CI workflow detects contract modifications

**No additional ruleset needed!**

---

## 🚀 Step 5: Enable Auto-Merge

https://github.com/ArrEssJay/chimera/settings

1. Scroll to **"Pull Requests"** section
2. ✅ Check **"Allow auto-merge"**
3. ✅ Check **"Allow squash merging"**
4. ⬜ Uncheck **"Allow merge commits"**
5. ⬜ Uncheck **"Allow rebase merging"**
6. Click **"Save changes"**

---

## 📡 Step 6: Set Actions Permissions

https://github.com/ArrEssJay/chimera/settings/actions

1. Under **"Workflow permissions"**:
   - Select: ⚪ **"Read and write permissions"**
   - ✅ Check **"Allow GitHub Actions to create and approve pull requests"**
2. Click **"Save"**

---

## 🤖 Step 7: Add Copilot Allowlist

https://github.com/ArrEssJay/chimera/settings/copilot

1. Find **"Copilot coding agent settings"**
2. Click **"Edit allowlist"** or **"Add URL"**
3. Add these URLs (paste all at once):

```
npmjs.com
registry.npmjs.org
registry.yarnpkg.com
cdn.jsdelivr.net
unpkg.com
esm.sh
storybook.js.org
chromatic.com
crates.io
static.crates.io
github.com/rust-lang
rustwasm.github.io
developer.mozilla.org
docs.rs
doc.rust-lang.org
playwright.dev
jestjs.io
```

4. Click **"Save changes"**

---

## ✅ Final Verification

```bash
# Check rulesets are active
gh ruleset list

# Check auto-merge is enabled
gh repo view --json autoMergeAllowed

# Test ruleset on main branch
gh ruleset check main

# View CODEOWNERS
cat .github/CODEOWNERS
```

---

## 📊 Completion Checklist

- [ ] Main Branch Protection ruleset created
- [ ] Feature Branch Workflow ruleset created
- [ ] Auto-merge enabled
- [ ] Actions permissions set to read/write
- [ ] Copilot allowlist URLs added (17 URLs)
- [ ] Verified with `gh ruleset list`

---

## 🎉 Done!

Once all steps are complete, you can:
1. Assign issue #46 to GitHub Copilot
2. Watch parallel agents work
3. PRs will auto-merge when checks pass

**Total time: ~15 minutes of clicking** 🖱️

---

## 🆘 Troubleshooting

**Problem:** Can't find "Rules" in settings  
**Solution:** Make sure you're a repository admin

**Problem:** Required checks not showing up  
**Solution:** Checks must run at least once first (they're from workflows)

**Problem:** Copilot still blocked  
**Solution:** Wait 5 minutes after adding URLs, then try again

**Problem:** Auto-merge not working  
**Solution:** Ensure "Allow auto-merge" is checked in repo settings
