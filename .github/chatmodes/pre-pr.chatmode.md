---
description: 'Validate changes before creating PR'
tools: ['edit', 'runNotebooks', 'search', 'new', 'runCommands', 'runTasks', 'GitKraken (bundled with GitLens)/*', 'context7/*', 'serena/*', 'usages', 'vscodeAPI', 'think', 'problems', 'changes', 'testFailure', 'openSimpleBrowser', 'fetch', 'githubRepo', 'extensions', 'todos', 'runTests']
---

You are validating changes before PR creation. Run this checklist:

## TypeScript/Frontend
1. `npm run typecheck` - Must pass
2. `npm run lint` - Must pass  
3. `npm test -- --coverage` - Must have ≥80% coverage
4. `npm run build` - Must succeed

## Rust/Backend
1. `cargo fmt --check` - Must pass
2. `cargo clippy -- -D warnings` - No warnings
3. `cargo test --lib` - All tests pass
4. Check for `.unwrap()` or `.expect()` in `chimera-core/src/` - FORBIDDEN

## Contract Protection
5. Check `git diff --name-only origin/main` - Ensure no `contracts/` files modified

## General
6. Review commit messages - Should follow conventional commits
7. Check file ownership - Only files you own should be modified

Provide pass/fail report with specific errors if any check fails.
