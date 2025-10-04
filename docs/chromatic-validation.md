# Chromatic Integration Validation

## Pre-Deployment Checklist

This document provides a checklist to validate the Chromatic integration before the first run.

### ✅ Package Installation

- [x] `chromatic` package installed in `devDependencies`
- [x] Version: ^13.3.0 (latest stable)

**Verify:**
```bash
cd chimera-web
npm list chromatic
# Should show: chromatic@13.3.0
```

### ✅ Configuration Files

- [x] `chromatic.config.json` created
- [x] Configuration includes:
  - `buildScriptName`: "build-storybook"
  - `exitZeroOnChanges`: true (don't fail on visual changes)
  - `exitOnceUploaded`: true (faster feedback)

**Verify:**
```bash
cd chimera-web
cat chromatic.config.json
```

### ✅ NPM Scripts

- [x] `chromatic` script added to package.json
- [x] Script command: `chromatic --exit-zero-on-changes`

**Verify:**
```bash
cd chimera-web
npm run chromatic -- --help
# Should show Chromatic CLI help
```

### ✅ GitHub Actions Workflow

- [x] Workflow file: `.github/workflows/chromatic.yml`
- [x] Triggers: push to main, pull_request to main
- [x] Steps include:
  - Checkout with full git history (fetch-depth: 0)
  - Node.js 20 setup with npm caching
  - Install dependencies with --legacy-peer-deps
  - Run Chromatic action with project token

**Verify:**
```bash
cat .github/workflows/chromatic.yml | grep -E "(fetch-depth|node-version|projectToken)"
```

### ✅ Storybook Configuration

- [x] Storybook v8.2.0 installed
- [x] Stories exist for all components:
  - Button (8 stories)
  - Select (multiple stories)
  - Badge (multiple stories)
  - Panel (multiple stories)
  - Tooltip (multiple stories)
- [x] Storybook builds successfully

**Verify:**
```bash
cd chimera-web
npm run build-storybook
# Should complete without errors
# Output directory: storybook-static/
```

### ✅ Documentation

- [x] README.md updated with Chromatic section
- [x] Comprehensive setup guide: `docs/chromatic-setup.md`
- [x] Validation checklist: `docs/chromatic-validation.md`

**Verify:**
```bash
grep -n "Chromatic" chimera-web/README.md
grep -n "Visual Regression Testing" docs/chromatic-setup.md
```

### ✅ .gitignore

- [x] `storybook-static/` already ignored
- [x] `node_modules/` already ignored
- [x] No Chromatic artifacts will be committed

**Verify:**
```bash
grep -E "(storybook-static|node_modules)" .gitignore
```

## Post-Deployment Setup

These steps require access to the Chromatic service and GitHub settings:

### 🔧 Chromatic Account Setup

1. Go to https://www.chromatic.com/
2. Sign in with GitHub account
3. Create or select project for "ArrEssJay/chimera"
4. Note the project token (format: `chpt_xxxxxxxxxxxxxx`)

### 🔧 GitHub Secret Configuration

1. Go to repository settings
2. Navigate to: Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Name: `CHROMATIC_PROJECT_TOKEN`
5. Value: Paste token from Chromatic
6. Click "Add secret"

### 🔧 First Baseline Capture

1. Push changes to main branch or create PR
2. Wait for workflow to run
3. Check Actions tab for "Visual Regression Testing" workflow
4. First run will capture baseline screenshots
5. View build in Chromatic dashboard
6. All baselines on main branch will auto-accept

## Testing the Integration

### Local Testing (Optional)

If you have the project token, you can test locally:

```bash
cd chimera-web
export CHROMATIC_PROJECT_TOKEN=your_token_here
npm run chromatic
```

Expected output:
```
✅ Build 1 published
ℹ  View the build details at: https://www.chromatic.com/build?appId=...
```

### CI Testing

1. Create a test PR with a small change
2. Check the "Visual Regression Testing" workflow runs
3. Verify the workflow completes successfully
4. Click the Chromatic build link in PR checks
5. Review any detected changes
6. Accept or reject changes in Chromatic UI

## Expected Coverage

After initial setup, Chromatic will test:

- **Button Component**: 8 visual states
  - Primary, Secondary, Danger variants
  - Small, Medium, Large sizes
  - Loading, Disabled states
  - With icon

- **Select Component**: Multiple states
  - Default, open, disabled
  - With options
  - Various sizes

- **Badge Component**: Multiple variants
  - Success, warning, error, info
  - Different sizes

- **Panel Component**: Multiple configurations
  - With/without header
  - Collapsible states
  - Different content

- **Tooltip Component**: Multiple positions
  - Top, bottom, left, right
  - Various content

**Total Stories**: ~35-40 visual test cases

## Troubleshooting Validation

### Issue: npm ci fails

```bash
# Solution: Use --legacy-peer-deps flag
npm ci --legacy-peer-deps
```

### Issue: Storybook build fails

```bash
# Check for errors
npm run build-storybook

# Verify story syntax
find src-react -name "*.stories.tsx"
```

### Issue: Workflow doesn't trigger

```bash
# Check workflow file syntax
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/chromatic.yml'))"

# Check triggers in workflow
grep -A5 "^on:" .github/workflows/chromatic.yml
```

### Issue: Token not found in CI

- Verify secret name is exactly: `CHROMATIC_PROJECT_TOKEN`
- Check secret is available in repository settings
- Ensure workflow uses correct secret reference: `${{ secrets.CHROMATIC_PROJECT_TOKEN }}`

## Success Criteria

All items below should be ✅ after deployment:

- [ ] GitHub secret `CHROMATIC_PROJECT_TOKEN` is set
- [ ] Workflow runs successfully on first push/PR
- [ ] Baseline screenshots captured in Chromatic dashboard
- [ ] PR checks show Chromatic build link
- [ ] Team can access Chromatic dashboard to review changes
- [ ] Visual changes can be accepted/rejected via Chromatic UI
- [ ] Baselines auto-accept on main branch
- [ ] Documentation is clear and team understands workflow

## Next Steps After Validation

1. **Team Training**: Share `docs/chromatic-setup.md` with team
2. **Review Process**: Establish who reviews visual changes
3. **Baseline Maintenance**: Plan for updating baselines intentionally
4. **Coverage Expansion**: Add more stories as components are added
5. **TurboSnap** (future): Enable `--only-changed` for faster builds
6. **Responsive Testing** (future): Add viewport stories for mobile/tablet

## Support Contacts

- **Chromatic Documentation**: https://www.chromatic.com/docs/
- **Storybook Documentation**: https://storybook.js.org/docs/
- **GitHub Actions Logs**: Check Actions tab in repository
- **Issue Tracker**: Open issue in repository with details
