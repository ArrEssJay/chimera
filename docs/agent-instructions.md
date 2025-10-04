# Agent Instructions for GitHub Copilot

## 🤖 How to Work in Parallel Safely

When assigned to an issue, follow these steps:

### 1. Read Your Contracts FIRST

Before writing any code, read the contract files:
- **TypeScript agents:** Read `contracts/node-types.ts`
- **Rust agents:** Read `contracts/node-trait.rs`

These define the interfaces you MUST implement. **DO NOT MODIFY CONTRACT FILES.**

### 2. Check Your File Ownership

Each agent is assigned specific files to create/modify. Check your issue description for:
- Files you OWN (you create and modify these)
- Files you READ (you can import from these but not modify)

**Example for Button Component Agent:**
```yaml
YOU OWN:
  - chimera-web/src/components/Button.tsx
  - chimera-web/src/components/Button.test.tsx
  - chimera-web/src/components/Button.stories.tsx

YOU READ (but don't modify):
  - chimera-web/style.css
  - contracts/node-types.ts
```

### 3. Create Your Branch

Branch naming pattern:
```bash
feature/[component-name]

Examples:
- feature/button-component
- feature/bit-generator-node
- feature/graph-validator
```

### 4. Implement Against Contracts

**For UI Components (TypeScript):**
```typescript
// ✅ CORRECT - Import from contracts
import type { NodeDefinition } from '../../contracts/node-types';

export interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  onClick?: () => void;
  disabled?: boolean;
  children: React.ReactNode;
}

export function Button({ variant = 'primary', size = 'md', ...props }: ButtonProps) {
  // Implementation
}
```

**For DSP Nodes (Rust):**
```rust
// ✅ CORRECT - Implement Node trait from contracts
use chimera_core::contracts::{Node, NodeDefinition, DataBuffer};

pub struct BitGeneratorNode {
    id: String,
}

impl Node for BitGeneratorNode {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "bit_generator".to_string(),
            name: "Bit Generator".to_string(),
            category: NodeCategory::Source,
            // ... rest of definition
        }
    }
    
    fn execute(
        &self,
        inputs: Vec<DataBuffer>,
        params: JsValue,
    ) -> Result<Vec<DataBuffer>, JsValue> {
        // MUST NOT PANIC - return Err() instead
        // Implementation here
    }
}
```

### 5. Write Comprehensive Tests

**Requirement:** ≥80% test coverage

```typescript
// Button.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { Button } from './Button';

describe('Button', () => {
  // Test all variants
  it('renders primary variant', () => { /* ... */ });
  it('renders secondary variant', () => { /* ... */ });
  it('renders danger variant', () => { /* ... */ });
  
  // Test all sizes
  it('renders small size', () => { /* ... */ });
  it('renders medium size', () => { /* ... */ });
  it('renders large size', () => { /* ... */ });
  
  // Test interactions
  it('calls onClick when clicked', () => { /* ... */ });
  it('does not call onClick when disabled', () => { /* ... */ });
  
  // Test accessibility
  it('is keyboard accessible', () => { /* ... */ });
  it('has proper ARIA attributes', () => { /* ... */ });
});
```

### 6. Run Local Validation

Before submitting PR, run:

```bash
# For TypeScript
cd chimera-web
npm run typecheck
npm run lint
npm test -- --coverage
npm run build

# For Rust
cargo fmt --check
cargo clippy -- -D warnings
cargo test --lib
```

### 7. Submit PR with Proper Format

**PR Title Pattern:**
```
[Wave X] Add [Component Name]

Examples:
- [Wave 2] Add Button Component
- [Wave 4] Add Bit Generator Node
```

**PR Description Template:**
```markdown
## 🎯 What This PR Does

Implements the Button component as part of Wave 2 - UI Components.

## 📋 Checklist

- [x] Implements contract interface
- [x] All tests pass (≥80% coverage)
- [x] TypeScript/Rust checks pass
- [x] No lint errors
- [x] Documentation complete
- [x] Only modified assigned files

## 🔗 Related Issue

Closes #46

## 📝 Notes

Implemented 3 variants (primary, secondary, danger) and 3 sizes (sm, md, lg).
All accessibility requirements met (keyboard nav, ARIA labels).
```

### 8. Respond to CI Feedback

If CI fails:
1. Read the error message carefully
2. Fix the issue locally
3. Run validation again
4. Push the fix
5. Wait for CI to re-run

**Common CI Failures:**
- ❌ Modified contract files → Revert changes to `contracts/`
- ❌ File conflicts → Coordinate with other agents
- ❌ Coverage <80% → Add more tests
- ❌ Type errors → Fix TypeScript/Rust types
- ❌ Lint errors → Run formatter

### 9. Don't Wait for Human Review

If all automated checks pass:
- ✅ Tests pass
- ✅ Coverage ≥80%
- ✅ Type checks pass
- ✅ No lint errors
- ✅ No contract violations
- ✅ No file conflicts

Your PR will be auto-merged! 🎉

## 🚫 Common Mistakes to Avoid

### ❌ DON'T: Modify Contract Files
```typescript
// ❌ WRONG - Never modify contracts/node-types.ts
export interface NodeDefinition {
  id: string;
  newField: string; // ❌ DON'T ADD FIELDS
}
```

### ❌ DON'T: Touch Files Outside Your Assignment
```typescript
// ❌ WRONG - If you're the Button agent, don't modify Select.tsx
// File: Select.tsx
export function Select() { /* ... */ }
```

### ❌ DON'T: Use .unwrap() or .expect() in Core Code
```rust
// ❌ WRONG - Will fail CI
fn execute(&self, inputs: Vec<DataBuffer>) -> Result<Vec<DataBuffer>, JsValue> {
    let data = inputs.get(0).unwrap(); // ❌ FORBIDDEN
}

// ✅ CORRECT - Use proper error handling
fn execute(&self, inputs: Vec<DataBuffer>) -> Result<Vec<DataBuffer>, JsValue> {
    let data = inputs.get(0)
        .ok_or_else(|| JsValue::from_str("Missing input"))?; // ✅ GOOD
}
```

### ❌ DON'T: Import from Unfinished Dependencies
```typescript
// ❌ WRONG - Don't import from nodes if Wave 4 isn't complete
import { BitGeneratorNode } from '../nodes/bit_generator'; // ❌ May not exist yet

// ✅ CORRECT - Use mocks for unfinished dependencies
import { MockGraphCore } from '../mocks/graph-core.mock'; // ✅ GOOD
```

## 📊 Dependency Waves

Know which wave you're in:

- **Wave 1:** ✅ Complete (React setup)
- **Wave 2:** UI Components (5 parallel agents)
- **Wave 3:** Node Graph Core (4 sequential agents)
- **Wave 4:** Built-in Nodes (15 parallel agents) - WAIT for Wave 3
- **Wave 5:** Graph Editor (6 agents) - WAIT for Wave 3, can use mocks for Wave 4
- **Wave 6:** Integration - WAIT for all above

**Rule:** Don't start work if your wave is blocked. Check issue labels:
- `wave-2-unlocked` ✅ Ready to work
- `wave-4-blocked` ⛔ Wait for Wave 3

## 🆘 If You Get Stuck

1. Check contract files - are you implementing the right interface?
2. Check CI logs - what specific error occurred?
3. Check other PRs - did someone else modify the same file?
4. Check issue comments - did requirements change?
5. Add a comment to your PR asking for help

## 🎯 Success Criteria

Your PR is ready when:
- ✅ All CI checks green
- ✅ Coverage ≥80%
- ✅ No contract violations
- ✅ No file conflicts
- ✅ Implements all requirements from issue
- ✅ Tests cover all acceptance criteria

**Then it will auto-merge!** 🚀

---

**Remember:** You're working in parallel with other agents. Follow the rules, stay in your lane, and we'll ship fast! 🤖⚡
