# React Migration - Developer Guide

## Overview

This document explains the dual frontend setup during the migration from Yew to React + TypeScript.

## Directory Structure

```
chimera-web/
├── src/              # Existing Yew implementation (Rust)
├── src-react/        # New React implementation (TypeScript)
├── index.html        # Yew entry point (Trunk)
├── index-react.html  # React entry point (Vite)
├── Trunk.toml        # Yew build config
├── vite.config.ts    # React build config
├── package.json      # React dependencies
└── Cargo.toml        # Rust dependencies
```

## Development Workflow

### Running the Yew App (Existing)

```bash
cd chimera-web
trunk serve
# Opens at http://localhost:8080
```

### Running the React App (New)

```bash
cd chimera-web
npm install  # First time only
npm run dev
# Opens at http://localhost:3000
```

### Running Both Simultaneously

You can run both apps at the same time on different ports:

```bash
# Terminal 1 - Yew
cd chimera-web && trunk serve

# Terminal 2 - React
cd chimera-web && npm run dev
```

## Testing

### Rust Tests (Existing)

```bash
cargo test
```

### React Tests (New)

```bash
cd chimera-web
npm run test          # Run once
npm run test:ui       # Interactive UI
npm run test:coverage # With coverage report
```

### Playwright E2E Tests

```bash
cd chimera-web
npm run e2e           # Headless
npm run e2e:ui        # Interactive
```

## Storybook

Run Storybook for component development and visual testing:

```bash
cd chimera-web
npm run storybook
# Opens at http://localhost:6006
```

## Building for Production

### Yew Build

```bash
cd chimera-web
trunk build --release
# Output: dist/
```

### React Build

```bash
cd chimera-web
npm run build
# Output: dist-react/
```

## Migration Strategy

### Phase 1: Foundation (PR #001) ✅
- Set up React + TypeScript + Vite
- Configure testing infrastructure
- Setup Storybook

### Phase 2: Components (PRs #002-004)
- Build base UI components
- Create WASM service layer
- Implement chart components

### Phase 3: Features (PRs #005-009)
- Migrate control components
- Migrate panel components
- Integrate state management
- Complete UI integration

### Phase 4: Deployment (PR #010)
- Switch production deployment to React
- Keep Yew version accessible

### Phase 5: Cleanup (PRs #011-016)
- Remove Yew code
- Optimize React build
- Documentation updates

## Key Points

- **Both frontends share the same Rust/WASM backend** (`chimera-core`)
- **No changes to signal processing logic** during migration
- **Incremental migration** - features added one PR at a time
- **No disruption** to existing Yew app during development
- **Full test coverage** before switching production deployment

## Troubleshooting

### npm install fails

Make sure you're in the `chimera-web` directory and have Node.js 18+ installed:

```bash
node --version  # Should be 18.x or higher
cd chimera-web
npm install
```

### Vite dev server won't start

Check if port 3000 is already in use:

```bash
lsof -ti:3000 | xargs kill -9  # macOS/Linux
```

### TypeScript errors

Make sure TypeScript can find the path aliases:

```bash
cd chimera-web
npx tsc --noEmit  # Type check without building
```

### WASM not loading

WASM integration comes in PR #003. Until then, use mocks for testing.

## Next Steps

1. **Read the technical proposal**: `docs/frontend-refactor-proposal.md`
2. **Review PR #002 spec**: `docs/github-prs/PR-002-ui-component-library.md`
3. **Check the epic**: GitHub issue #40
4. **Start building components** following the PR sequence

## Questions?

Refer to:
- **Technical Spec**: `docs/frontend-refactor-proposal.md`
- **Executive Summary**: `docs/frontend-refactor-summary.md`
- **Documentation Index**: `docs/INDEX.md`
- **GitHub Epic**: Issue #40
