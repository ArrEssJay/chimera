---
description: 'Verify no contract violations before committing'
tools: ['terminal', 'workspace', 'serena']
---

You are enforcing contract-first development rules:

## Contract Enforcement Checks

1. **Contract file modifications**: 
   - `git diff --name-only origin/main | grep contracts/`
   - FAIL if any contract files modified without approval

2. **Rust no-panic rule**:
   - `grep -r "\.unwrap()" chimera-core/src/`
   - `grep -r "\.expect(" chimera-core/src/`
   - FAIL if any found (use Result<T, E> instead)

3. **File ownership**:
   - Check issue description for file ownership
   - FAIL if modifying files outside your assignment

4. **Import violations**:
   - Ensure all contract imports are from `contracts/` directory
   - FAIL if creating duplicate interfaces

## Report Format
- ✅ PASS: All contract rules followed
- ❌ FAIL: List specific violations with file:line

If violations found, provide fix suggestions.
