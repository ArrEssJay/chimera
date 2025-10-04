# Task Completion Checklist for Chimera

## Before Submitting PR

### 1. Rust Changes

#### Run Tests
```bash
cargo test --workspace --all-features
```
**Must pass**: All tests green

#### Format Code
```bash
cargo fmt --all --check
```
**Must pass**: No formatting issues

If fails, run:
```bash
cargo fmt --all
```

#### Lint Code
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
**Must pass**: Zero warnings

Common issues:
- Unused imports → Remove them
- `.unwrap()` in core code → Use `?` and `Result`
- Missing docs on public APIs → Add `///` comments

#### No Panics Check (Critical!)
```bash
grep -r "\.unwrap()" chimera-core/src/
grep -r "\.expect(" chimera-core/src/
```
**Must output**: Nothing (no matches)

If found, refactor to use proper error handling:
```rust
// Change this:
let value = map.get(key).unwrap();

// To this:
let value = map.get(key)
    .ok_or_else(|| Error::new("Key not found"))?;
```

### 2. Web/TypeScript Changes

#### Run Tests
```bash
cd chimera-web
npm test
```
**Must pass**: All tests green
**Must have**: ≥80% coverage

#### Type Check
```bash
npm run typecheck  # If configured
```
**Must pass**: No type errors

#### Build Check
```bash
cd chimera-web
trunk build --release --public-url /chimera
```
**Must succeed**: Build completes without errors

### 3. Contract Validation

#### Check No Contract Modifications
```bash
git diff contracts/
```
**Must output**: Nothing (no changes to contracts/)

If you modified contracts:
- Revert the changes
- Ask human reviewer if contract change is needed

### 4. File Ownership Check

#### Verify You Only Modified Your Files
```bash
git diff --name-only origin/main..HEAD
```

Compare with your issue assignment:
- ✅ Only files you OWN should appear
- ❌ If other files appear, ask for guidance

### 5. Documentation

#### Public APIs Documented
- All public functions have `///` doc comments
- Complex logic has inline comments
- README updated if new features added

#### Tests Document Behavior
- Test names describe what they test
- Edge cases covered
- Error paths tested

### 6. Git Hygiene

#### Commit Message Format
```
[Wave X] Brief description

Detailed explanation if needed.

Closes #issue_number
```

#### Branch Naming
```
feature/component-name
feature/button-component
feature/bit-generator-node
```

## PR Checklist Template

```markdown
## 🎯 What This PR Does
Brief description of changes

## 📋 Checklist
- [ ] All tests pass (`cargo test` or `npm test`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] No lint errors (`cargo clippy -- -D warnings`)
- [ ] No panics in core code (if Rust)
- [ ] Coverage ≥80% (if TypeScript)
- [ ] Implements contract interface correctly
- [ ] Only modified assigned files
- [ ] Documentation complete
- [ ] No contract modifications

## 🔗 Related Issue
Closes #XX

## 📝 Additional Notes
Any additional context
```

## Auto-Merge Requirements

Your PR will auto-merge if:
1. ✅ All CI checks pass
2. ✅ No contract violations
3. ✅ No file conflicts with other PRs
4. ✅ Tests have ≥80% coverage
5. ✅ No human review required flag

## If CI Fails

### Read the Error
Click on the failed check → View logs → Find error message

### Common Failures

**Test Failure**
- Fix the failing test
- Ensure test is correct
- Ensure implementation matches test

**Format Failure**
```bash
cargo fmt --all
git add -u
git commit -m "Fix formatting"
git push
```

**Clippy Failure**
- Read warning carefully
- Fix the issue (don't suppress warning)
- Re-run locally before pushing

**Coverage Failure**
- Add more tests
- Cover untested branches
- Test error paths

**Contract Violation**
- Revert contract changes
- Import types from contracts instead

**File Conflict**
- Coordinate with other agent
- Wait for their PR to merge
- Rebase and resolve conflict

## When Task is Complete

1. ✅ Run full local validation
2. ✅ Create PR with proper format
3. ✅ Ensure CI passes
4. ✅ Respond to any CI failures
5. ✅ Wait for auto-merge or human review
6. ✅ Move to next task in wave

## Emergency Contacts

If stuck:
- Check documentation in `docs/`
- Review agent instructions in `.github/copilot-instructions.md`
- Ask in PR comments
- Tag human reviewer for architecture questions
