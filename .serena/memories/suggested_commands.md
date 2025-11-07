# Chimera Development Commands

## Rust Core Development

### Testing
```bash
# Run all workspace tests
cargo test --workspace --all-features

# Run specific crate tests
cargo test -p chimera-core
cargo test -p chimera-cli

# Run with output
cargo test -- --nocapture
```

### Formatting & Linting
```bash
# Check formatting (don't modify)
cargo fmt --all -- --check

# Format code
cargo fmt --all

# Run Clippy (linter)
cargo clippy --lib --all-features -- -D warnings
cargo clippy --bins --all-features -- -D warnings
cargo clippy --tests --all-features -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Building
```bash
# Build release
cargo build --release

# Build for WASM
cd chimera-web
trunk build --release --public-url /chimera

# Build with trunk serve (dev server)
cd chimera-web
trunk serve
```

## Web Dashboard (Playwright Tests)

```bash
cd chimera-web

# Run tests
npm test

# Run with UI
npm run test:ui

# Run headed (visible browser)
npm run test:headed

# Show test report
npm run test:report
```

## Git Commands (macOS)

```bash
# List files
ls -la

# Find files
find . -name "*.rs"

# Search in files (macOS)
grep -r "pattern" .

# Git operations
git status
git diff
git log --oneline -10
git branch
```

## CI Validation (run locally before PR)

```bash
# Full validation suite
cargo test --workspace --all-features && \
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets --all-features -- -D warnings && \
cd chimera-web && trunk build --release

# Quick check
cargo test && cargo clippy --lib -- -D warnings
```

## GitHub CLI (for issue management)

```bash
# List issues
gh issue list --limit 50

# View issue
gh issue view <number>

# Create issue
gh issue create --title "..." --body "..."

# List PRs
gh pr list

# Check PR status
gh pr checks
```

## macOS-Specific Utilities

```bash
# File info
stat -f%z file.wasm  # Get file size

# Process management
ps aux | grep trunk

# Network
lsof -i :8080  # Check port usage
```
