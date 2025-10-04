# Chimera Deployment Guide

This document describes the deployment process for the Chimera web dashboard.

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Deployment Process](#deployment-process)
- [Environment Variables](#environment-variables)
- [Build Optimization](#build-optimization)
- [Rollback Procedures](#rollback-procedures)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)

---

## Overview

Chimera has two web applications that can be deployed:

1. **React App** (`src-react/`) - Modern React-based UI (Primary)
2. **Yew App** (`src/`) - Rust/WASM-based UI (Legacy)

Both applications are deployed to **GitHub Pages** with automatic CI/CD pipelines.

### Deployment URLs

- **Production (React)**: https://impermanent.io (via GitHub Pages)
- **Production (Yew)**: https://impermanent.io/yew (if deployed separately)

---

## Architecture

### React Application

```
┌─────────────────┐
│   GitHub Repo   │
│    (main branch)│
└────────┬────────┘
         │
         │ Push to main
         ▼
┌─────────────────┐
│  GitHub Actions │
│   (CI Pipeline) │
│  - Type Check   │
│  - Lint         │
│  - Test (80%+)  │
│  - Build        │
└────────┬────────┘
         │
         │ All checks pass
         ▼
┌─────────────────┐
│  GitHub Actions │
│ (Deploy Pipeline)│
│  - Build Prod   │
│  - Optimize     │
│  - Upload       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  GitHub Pages   │
│  (Static Host)  │
│  impermanent.io │
└─────────────────┘
```

---

## Deployment Process

### Automatic Deployment (Recommended)

Deployments happen automatically when code is pushed to the `main` branch:

1. **Push to Main**
   ```bash
   git push origin main
   ```

2. **CI Validation** (`.github/workflows/ci-react.yml`)
   - TypeScript type checking
   - ESLint code quality checks
   - Unit tests with ≥80% coverage
   - Development build verification
   - Production build verification

3. **Deployment** (`.github/workflows/deploy-react.yml`)
   - Install dependencies
   - Build production bundle with optimizations
   - Verify build artifacts
   - Check bundle size
   - Deploy to GitHub Pages

4. **Verification**
   - Check deployment status in GitHub Actions
   - Visit https://impermanent.io to verify

### Manual Deployment

To trigger a manual deployment:

1. Go to **Actions** tab in GitHub
2. Select **Deploy React App** workflow
3. Click **Run workflow**
4. Select `main` branch
5. Click **Run workflow**

### Local Production Build

To build and test the production bundle locally:

```bash
cd chimera-web

# Install dependencies
npm ci

# Build production bundle
npm run build -- --config vite.config.prod.ts

# Preview production build
npm run preview
```

This will:
- Build optimized production bundle in `dist-react/`
- Apply all production optimizations (minification, code splitting, etc.)
- Start local server at http://localhost:4173

---

## Environment Variables

### Configuration Files

- `.env.example` - Template with all available variables
- `.env.production` - Production environment overrides
- `.env` (local only) - Local development overrides

### Available Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `VITE_APP_ENV` | Application environment | `development` |
| `VITE_API_URL` | API endpoint URL (future use) | - |
| `VITE_ENABLE_ANALYTICS` | Enable analytics tracking | `false` |
| `VITE_ENABLE_DEBUG` | Enable debug logging | `true` |
| `VITE_BUILD_VERSION` | Build version (auto-set by CI) | - |
| `VITE_BUILD_TIME` | Build timestamp (auto-set by CI) | - |
| `VITE_GIT_COMMIT` | Git commit hash (auto-set by CI) | - |

### Adding New Variables

1. Add to `.env.example` with documentation
2. Add to `.env.production` with production value
3. Add TypeScript definition to `src-react/env.d.ts`:
   ```typescript
   interface ImportMetaEnv {
     readonly VITE_YOUR_VARIABLE: string;
   }
   ```
4. Use in code: `import.meta.env.VITE_YOUR_VARIABLE`

### Security Notes

⚠️ **Never commit sensitive data**:
- `.env` is gitignored (local only)
- Only `VITE_*` prefixed variables are exposed to client
- Use GitHub Secrets for sensitive CI/CD values

---

## Build Optimization

### Code Splitting

The production build automatically splits code into chunks:

```typescript
// vite.config.prod.ts
manualChunks: {
  'react-vendor': ['react', 'react-dom'],      // ~140KB
  'state-management': ['zustand'],              // ~4KB
  'charts': ['recharts'],                       // ~120KB
}
```

### Minification

- **JavaScript**: Terser with aggressive compression
  - Removes console.log statements
  - Removes debugger statements
  - Optimizes variable names
  
- **CSS**: Built-in Vite optimization
  - Removes unused styles
  - Minifies and combines files

- **HTML**: Minified with whitespace removal

### Asset Optimization

All assets are automatically optimized:

1. **Hashed Filenames** - For cache busting
   - `assets/main-[hash].js`
   - `assets/styles-[hash].css`

2. **Bundle Analysis** - Check bundle size in CI logs
   ```bash
   npm run build -- --config vite.config.prod.ts
   # Check dist-react/ folder size
   ```

3. **Target Modern Browsers** - ES2020+ for smaller bundles

### Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Total Bundle Size | < 10 MB | ✅ Enforced in CI |
| Initial Load Time | < 3s (Fast 3G) | 📊 Monitor |
| First Contentful Paint | < 2s | 📊 Monitor |
| Time to Interactive | < 5s | 📊 Monitor |

---

## Rollback Procedures

### Emergency Rollback (Immediate)

If a critical bug is deployed to production:

**Option 1: Revert via GitHub UI (Recommended)**

1. Go to repository **Code** tab
2. Find the problematic commit
3. Click **"..."** → **Revert**
4. Create PR and merge immediately
5. Auto-deployment will restore previous version

**Option 2: Revert via Git**

```bash
# Find the bad commit
git log --oneline

# Revert the commit (creates new commit)
git revert <commit-hash>

# Push to main
git push origin main

# Deployment happens automatically
```

**Option 3: Manual Revert via GitHub Pages**

1. Go to **Settings** → **Pages**
2. Select previous deployment from dropdown
3. Click **Redeploy**

### Planned Rollback

For non-critical issues, use a planned rollback:

1. Create a new branch from the last known good commit:
   ```bash
   git checkout -b rollback/fix-issue <good-commit-hash>
   ```

2. Cherry-pick any needed fixes:
   ```bash
   git cherry-pick <fix-commit-hash>
   ```

3. Create PR and follow normal review process

4. Merge to main for automatic deployment

### Rollback Testing

Before rolling back:

1. Check CI logs for the failure reason
2. Download artifact from failing build
3. Test locally if possible
4. Document the issue and rollback reason

### Post-Rollback

1. **Incident Report**: Document what went wrong
2. **Root Cause Analysis**: Why did it happen?
3. **Prevention**: How to avoid in the future?
4. **Fix Forward**: Plan to resolve and re-deploy

---

## Monitoring

### Deployment Status

Check deployment health:

1. **GitHub Actions**
   - Go to **Actions** tab
   - Check workflow status (green = success)
   - Review logs for any warnings

2. **GitHub Pages Status**
   - Go to **Settings** → **Pages**
   - Verify deployment URL is active
   - Check "Your site is live at..." message

### Build Metrics

Monitor in GitHub Actions summary:

- **Bundle Size**: Total size of dist-react/
- **Chunk Sizes**: Size of individual JS files
- **Build Time**: Time to complete build
- **Test Coverage**: Must be ≥80%

### Production Health Checks

Manual verification after deployment:

1. **Smoke Test**
   ```bash
   curl -I https://impermanent.io
   # Should return 200 OK
   ```

2. **Visual Check**
   - Visit https://impermanent.io
   - Verify UI loads correctly
   - Check browser console for errors

3. **Functionality Test**
   - Test key user flows
   - Verify no broken links
   - Check responsive design

---

## Troubleshooting

### Build Failures

**Problem**: TypeScript errors

```
Solution:
1. Run `npm run typecheck` locally
2. Fix type errors in src-react/
3. Commit and push
```

**Problem**: Test coverage below 80%

```
Solution:
1. Run `npm run test:coverage` locally
2. Add tests to increase coverage
3. Focus on untested files shown in report
```

**Problem**: Bundle size exceeds 10MB

```
Solution:
1. Check bundle size report in CI logs
2. Identify large dependencies
3. Use dynamic imports for large libraries
4. Consider code splitting improvements
```

### Deployment Failures

**Problem**: "Resource not accessible by integration"

```
Solution:
1. Check Settings → Actions → General
2. Ensure "Read and write permissions" is enabled
3. Enable "Allow GitHub Actions to create PRs"
```

**Problem**: "Deploy to GitHub Pages failed"

```
Solution:
1. Verify GitHub Pages is enabled
2. Check source is set to "GitHub Actions"
3. Verify custom domain (CNAME) is correct
```

**Problem**: 404 errors after deployment

```
Solution:
1. Check that base path is correct in vite config
2. Verify CNAME file is copied to dist/
3. Check GitHub Pages settings for custom domain
```

### Runtime Issues

**Problem**: White screen / blank page

```
Solution:
1. Check browser console for errors
2. Verify JavaScript files loaded correctly
3. Check for CORS or CSP issues
4. Test in incognito mode (cache issues)
```

**Problem**: Environment variables not working

```
Solution:
1. Ensure variable has VITE_ prefix
2. Rebuild application (env vars are build-time)
3. Check .env.production file exists
4. Verify TypeScript definitions in env.d.ts
```

---

## CI/CD Pipeline Details

### Workflow Files

- `.github/workflows/ci-react.yml` - Quality checks for all PRs
- `.github/workflows/deploy-react.yml` - Deployment to GitHub Pages
- `.github/workflows/test-deploy.yml` - Test deployment for PRs

### Quality Gates

All checks must pass before deployment:

- ✅ TypeScript compilation (zero errors)
- ✅ ESLint (zero errors, warnings allowed)
- ✅ Unit tests (80%+ coverage required)
- ✅ Development build (must succeed)
- ✅ Production build (must succeed)

### Deployment Triggers

Deployments run on:

- Push to `main` branch
- Manual workflow dispatch
- Changes to React source files or configs

---

## Best Practices

### Before Deploying

- [ ] All tests pass locally
- [ ] No TypeScript errors
- [ ] No ESLint errors
- [ ] Coverage ≥80%
- [ ] Tested in development mode
- [ ] Tested in production mode (preview)
- [ ] Verified on multiple browsers
- [ ] Checked responsive design

### After Deploying

- [ ] Verify deployment succeeded in Actions
- [ ] Check production site loads
- [ ] Test critical user flows
- [ ] Monitor for errors (first 30 minutes)
- [ ] Document any issues
- [ ] Communicate to team

### Regular Maintenance

- [ ] Monitor bundle size trends
- [ ] Update dependencies monthly
- [ ] Review and optimize slow pages
- [ ] Check for console warnings
- [ ] Audit lighthouse scores
- [ ] Review and update documentation

---

## Support

### Resources

- **GitHub Actions**: https://github.com/ArrEssJay/chimera/actions
- **GitHub Pages**: https://github.com/ArrEssJay/chimera/settings/pages
- **Documentation**: `/docs` folder in repository
- **Issues**: https://github.com/ArrEssJay/chimera/issues

### Getting Help

1. Check this documentation first
2. Review CI/CD logs for error messages
3. Check troubleshooting section
4. Create GitHub issue with:
   - Error messages
   - Steps to reproduce
   - Expected vs actual behavior
   - CI/CD logs (if applicable)

---

**Last Updated**: 2024-01-09  
**Version**: 1.0.0  
**Maintained by**: Chimera Development Team
