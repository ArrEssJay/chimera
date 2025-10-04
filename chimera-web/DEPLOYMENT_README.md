# Chimera Web - Deployment Quick Reference

## 🚀 Quick Commands

### Development
```bash
npm run dev              # Start development server
npm run preview          # Preview production build locally
npm run test             # Run tests
npm run test:coverage    # Run tests with coverage
npm run lint             # Check code quality
npm run typecheck        # Check TypeScript types
```

### Production Build
```bash
npm ci                   # Clean install dependencies
npm run build:prod       # Build optimized production bundle
```

The production build will be output to `dist-react/` with:
- Minified JavaScript (console.log removed)
- Code splitting for optimal loading
- Hashed filenames for cache busting
- Optimized CSS

### Preview Production Build
```bash
npm run build:prod       # Build first
npm run preview          # Preview at http://localhost:4173
```

---

## 📦 Build Outputs

| Build Type | Command | Output Dir | Size Target | Use Case |
|-----------|---------|------------|-------------|----------|
| Development | `npm run build` | `dist-react/` | No limit | Testing, debugging |
| Production | `npm run build:prod` | `dist-react/` | < 10 MB | Deployment to GitHub Pages |

---

## 🌍 Environment Variables

### Local Development
Create a `.env` file (gitignored):
```bash
VITE_APP_ENV=development
VITE_ENABLE_DEBUG=true
```

### Production
Uses `.env.production` (committed):
```bash
VITE_APP_ENV=production
VITE_ENABLE_DEBUG=false
```

### Available Variables
See `.env.example` for all available variables.

All variables must be prefixed with `VITE_` to be accessible in code:
```typescript
const env = import.meta.env.VITE_APP_ENV;
```

---

## 🔄 Deployment Process

### Automatic (Recommended)
1. Push to `main` branch
2. CI runs automatically (tests, linting, type checking)
3. If CI passes, deployment runs automatically
4. Site deploys to https://impermanent.io

### Manual Deployment
1. Go to **Actions** tab in GitHub
2. Select **Deploy React App** workflow
3. Click **Run workflow** → Select `main` → **Run workflow**

---

## ✅ Pre-Deployment Checklist

Before pushing to main:

```bash
# 1. Type check
npm run typecheck

# 2. Lint code
npm run lint

# 3. Run tests with coverage
npm run test:coverage
# Coverage must be ≥80%

# 4. Test production build
npm run build:prod
npm run preview
# Visit http://localhost:4173 and test manually

# 5. Check bundle size
du -sh dist-react/
# Should be < 10 MB
```

---

## 🐛 Troubleshooting

### Build fails with TypeScript errors
```bash
npm run typecheck
# Fix all errors shown
```

### Tests fail
```bash
npm run test:coverage
# Check which tests are failing
# Fix tests or code as needed
```

### Coverage below 80%
```bash
npm run test:coverage
# Check coverage report in coverage/index.html
# Add tests for uncovered files
```

### Bundle too large (> 10 MB)
```bash
npm run build:prod
du -h dist-react/assets/*.js | sort -rh
# Identify large files
# Consider:
# - Code splitting with dynamic imports
# - Removing unused dependencies
# - Lazy loading components
```

---

## 📚 Documentation

For complete deployment documentation, see:
- **[/docs/DEPLOYMENT.md](../docs/DEPLOYMENT.md)** - Comprehensive deployment guide
  - Environment variables
  - Build optimization
  - Rollback procedures
  - Monitoring
  - Troubleshooting

---

## 🔧 Configuration Files

| File | Purpose |
|------|---------|
| `vite.config.ts` | Development build configuration |
| `vite.config.prod.ts` | Production build configuration (optimized) |
| `vitest.config.ts` | Test runner configuration |
| `tsconfig.json` | TypeScript compiler configuration |
| `.env.example` | Environment variable template |
| `.env.production` | Production environment overrides |
| `src-react/env.d.ts` | TypeScript types for environment variables |

---

## 🎯 Quality Gates (CI/CD)

All must pass for deployment:

- ✅ TypeScript compilation (zero errors)
- ✅ ESLint (zero errors)
- ✅ Unit tests (80%+ coverage)
- ✅ Development build succeeds
- ✅ Production build succeeds
- ✅ Bundle size < 10 MB

---

## 📊 Bundle Analysis

After building, check bundle composition:

```bash
npm run build:prod

# Total size
du -sh dist-react/

# JavaScript files by size
find dist-react/assets -name "*.js" -exec du -h {} \; | sort -rh

# CSS files by size
find dist-react/assets -name "*.css" -exec du -h {} \;
```

Expected chunks:
- `react-vendor-[hash].js` - React & React-DOM (~140 KB)
- `state-management-[hash].js` - Zustand (~4 KB)
- `charts-[hash].js` - Recharts (~120 KB)
- `main-[hash].js` - Application code

---

## 🔗 Links

- **Production Site**: https://impermanent.io
- **GitHub Actions**: https://github.com/ArrEssJay/chimera/actions
- **GitHub Pages Settings**: https://github.com/ArrEssJay/chimera/settings/pages
