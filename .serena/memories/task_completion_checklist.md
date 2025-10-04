# Task Completion Checklist for Chimera

> **🎯 USER-FIRST IMPERATIVE**
> 
> **This checklist exists to protect users from our mistakes.**
> 
> Every item on this list is a guard rail between users and:
> - Bugs that waste their time
> - Crashes that lose their work
> - Inconsistencies that confuse them
> - Performance issues that frustrate them
> 
> **Skipping steps = shipping user pain.**
> 
> Do the checklist. Ship quality. Respect users.

---

## Before Submitting PR

### 1. Rust Changes

#### Run Tests
```bash
cargo test --workspace --all-features
```
**Must pass**: All tests green
**User impact**: Tests failing = bugs users will hit

#### Format Code
```bash
cargo fmt --all --check
```
**Must pass**: No formatting issues
**User impact**: Consistent code = maintainable = sustainable project = long-term support

If fails, run:
```bash
cargo fmt --all
```

#### Lint Code
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
**Must pass**: Zero warnings
**User impact**: Clippy catches bugs before users do

Common issues:
- Unused imports → Remove them
- `.unwrap()` in core code → Use `?` and `Result` (**prevents user crashes**)
- Missing docs on public APIs → Add `///` comments (**helps users learn**)

#### No Panics Check (Critical!)
```bash
grep -r "\.unwrap()" chimera-core/src/
grep -r "\.expect(" chimera-core/src/
```
**Must output**: Nothing (no matches)
**User impact**: Panics = browser crash = user loses work = BAD

If found, refactor to use proper error handling:
```rust
// Change this:
let value = map.get(key).unwrap();

// To this:
let value = map.get(key)
    .ok_or_else(|| Error::new("Key not found"))?;
```

---

### 2. Web/TypeScript Changes

#### Run Tests
```bash
cd chimera-web
npm test
```
**Must pass**: All tests green
**Must have**: ≥80% coverage
**User impact**: Untested code = bugs users find = poor experience

#### Type Check
```bash
npm run typecheck  # If configured
```
**Must pass**: No type errors
**User impact**: Type errors = runtime crashes = user frustration

#### Build Check
```bash
cd chimera-web
trunk build --release --public-url /chimera
```
**Must succeed**: Build completes without errors
**User impact**: Build failures = users can't access the tool

#### Accessibility Check
- Test keyboard navigation (Tab, Enter, Escape)
- Check focus indicators are visible
- Verify ARIA labels on interactive elements
- **User impact**: Not all users use a mouse - make it work for everyone

---

### 3. Contract Validation

#### Check No Contract Modifications
```bash
git diff contracts/
```
**Must output**: Nothing (no changes to contracts/)
**User impact**: Contract changes = breaking changes = instability = bad UX

If you modified contracts:
- Revert the changes
- Ask human reviewer if contract change is needed

---

### 4. File Ownership Check

#### Verify You Only Modified Your Files
```bash
git diff --name-only origin/main..HEAD
```

Compare with your issue assignment:
- ✅ Only files you OWN should appear
- ❌ If other files appear, ask for guidance

**User impact**: File conflicts = delayed merges = slower feature delivery

---

### 5. Documentation

#### Public APIs Documented
- All public functions have `///` doc comments
- Complex logic has inline comments
- README updated if new features added
- **User impact**: Good docs help users understand and learn

#### Tests Document Behavior
- Test names describe what they test
- Edge cases covered
- Error paths tested
- **User impact**: Tests are documentation of expected behavior

---

### 6. User Experience Validation

#### Ask Yourself:
- ✅ **Does this improve user experience?**
- ✅ **Could this confuse a beginner?**
- ✅ **Are error messages helpful?** (not just technical)
- ✅ **Is this accessible?** (keyboard, screen readers)
- ✅ **Is performance acceptable?** (test on slower devices)
- ✅ **Does this help users learn or build?**

If answer is "no" or "unsure" to any → fix before submitting

---

### 7. Git Hygiene

#### Commit Message Format
```
[Wave X] Brief description of user benefit

Detailed explanation if needed.

Closes #issue_number
```

Example:
```
[Wave 2] Add drag-and-drop for node graph

Users can now drag nodes from palette onto canvas,
making pipeline creation more intuitive.

Closes #52
```

#### Branch Naming
```
feature/component-name
feature/button-component
feature/bit-generator-node
```

---

## PR Checklist Template

```markdown
## 🎯 What This PR Does
Brief description of changes and **user benefit**

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
- [ ] **Tested user experience** (keyboard nav, error messages, performance)

## 🎯 User Impact
Describe how this improves user experience:
- What can users now do?
- What bugs are fixed?
- What's faster/easier?

## 🔗 Related Issue
Closes #XX

## 📝 Additional Notes
Any additional context
```

---

## Auto-Merge Requirements

Your PR will auto-merge if:
1. ✅ All CI checks pass
2. ✅ No contract violations
3. ✅ No file conflicts with other PRs
4. ✅ Tests have ≥80% coverage
5. ✅ No human review required flag

**User impact**: Fast merge = users get features faster

---

## If CI Fails

### Read the Error
Click on the failed check → View logs → Find error message
**Don't guess. Read the actual error.**

### Common Failures

**Test Failure**
- Fix the failing test
- Ensure test is correct
- Ensure implementation matches test
- **Remember**: This test protected users from a bug

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
- **Remember**: Clippy is trying to prevent user-facing bugs

**Coverage Failure**
- Add more tests
- Cover untested branches
- Test error paths
- **Remember**: Untested code = bugs users will find

**Contract Violation**
- Revert contract changes
- Import types from contracts instead
- **Remember**: This protects user experience stability

**File Conflict**
- Coordinate with other agent
- Wait for their PR to merge
- Rebase and resolve conflict

---

## When Task is Complete

### Final Checklist
1. ✅ Run full local validation
2. ✅ Create PR with proper format
3. ✅ **Verify user benefit is clear** in PR description
4. ✅ Ensure CI passes
5. ✅ Respond to any CI failures
6. ✅ Wait for auto-merge or human review
7. ✅ Move to next task in wave

### Celebrate! 🎉
You just made Chimera better for users. Every quality contribution matters.

---

## Emergency Contacts

If stuck:
- Check documentation in `docs/`
- Review agent instructions in `.github/copilot-instructions.md`
- Ask in PR comments
- Tag human reviewer for architecture questions

**Never ship untested code. Users deserve better.**

---

## Remember

> **Users gave us their trust when they opened Chimera.**
> 
> This checklist is how we honor that trust.
> 
> Every check we run, every test we write, every bug we prevent...
> 
> ...is us keeping our promise to users.
> 
> **Do the work. Ship quality. Make users proud they chose Chimera.**
