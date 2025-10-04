# Chimera Code Style and Conventions

> **🎯 USER-FIRST IMPERATIVE**
> 
> **Code quality exists to serve users.**
> 
> - Clean code = fewer bugs = better user experience
> - Good tests = reliability = users trust the tool
> - Consistent style = maintainable code = sustainable project = long-term user support
> - No panics = stable software = users don't lose work
> 
> **Every line of code we write should make the user's experience better or protect it from degrading.**

---

## Rust Code Style

### General Principles
- Follow standard Rust conventions (rustfmt default)
- Use `nightly` toolchain (see rust-toolchain.toml)
- **Critical**: NO PANICS in core code (`chimera-core/src/`)
  - Never use `.unwrap()` or `.expect()`
  - Always return `Result<T, E>` for fallible operations
  - Use `?` operator for error propagation
  - **Why**: Panics crash the user's browser tab and lose their work

### Naming Conventions
- **Modules**: snake_case (e.g., `config`, `decoder`, `diagnostics`)
- **Structs/Enums**: PascalCase (e.g., `SimulationOutput`, `NodeDefinition`)
- **Functions**: snake_case (e.g., `run_simulation`, `build_modulation_audio`)
- **Constants**: SCREAMING_SNAKE_CASE

### Error Handling Pattern
```rust
// ❌ WRONG - Will fail CI AND crash user's experience
fn process(&self, inputs: Vec<Data>) -> Result<Vec<Data>, Error> {
    let first = inputs.get(0).unwrap(); // FORBIDDEN - USER SEES CRASH
    // ...
}

// ✅ CORRECT - User gets helpful error message
fn process(&self, inputs: Vec<Data>) -> Result<Vec<Data>, Error> {
    let first = inputs.get(0)
        .ok_or_else(|| Error::new("Missing input"))?;
    // ...
}
```

### Documentation
- Public APIs require doc comments (`///`)
- Include examples for complex functions
- Document error conditions
- **Remember**: Users might read this code to learn DSP concepts

```rust
/// Processes input data and returns transformed output
///
/// # Arguments
/// * `input` - Raw input bytes
///
/// # Returns
/// * `Ok(Vec<u8>)` - Processed output
/// * `Err(String)` - Error message describing failure
///
/// # Example
/// ```
/// let output = process_data(&input)?;
/// ```
pub fn process_data(input: &[u8]) -> Result<Vec<u8>, String> {
    // ...
}
```

---

## TypeScript Code Style (Epic #40)

### General Principles
- Use TypeScript strict mode (catch errors before users do)
- Prefer arrow functions for React components
- Use `type` over `interface` for props
- Import types from `contracts/` (never modify contracts)
- **Focus**: User-facing components should be intuitive and accessible

### React Component Pattern
```typescript
// Component definition
export const Button: React.FC<ButtonProps> = ({ 
  variant = 'primary', 
  size = 'md',
  ...props 
}) => {
  return <button className={`btn btn-${variant}`} {...props} />;
};

// Props type
export type ButtonProps = {
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
};
```

### Import Organization
```typescript
// 1. React/external libs
import React from 'react';
import { useCallback } from 'react';

// 2. Contract imports
import type { NodeDefinition } from '../../contracts/node-types';

// 3. Local imports
import { Button } from './Button';
import styles from './Component.module.css';
```

### CSS Variables
- Use CSS variables from `chimera-web/style.css`
- Never hardcode colors, spacing, fonts
- **Why**: Consistent design = professional look = users take tool seriously

```css
/* ❌ WRONG - Inconsistent UI confuses users */
.button {
  color: #007bff;
  padding: 16px;
}

/* ✅ CORRECT - Consistent, professional UI */
.button {
  color: var(--primary-color);
  padding: var(--spacing-md);
}
```

---

## Testing Conventions

### Why We Test
- **Users don't want to find our bugs**
- **Tests document expected behavior** (helpful for learning)
- **Confidence to refactor** = faster feature delivery = more user value
- **≥80% coverage required** = quality gate for users

### Rust Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange
        let input = vec![1, 2, 3];
        
        // Act
        let result = process(input);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }
}
```

### TypeScript Tests (Jest/Playwright)
```typescript
describe('Button', () => {
  it('renders with correct variant class', () => {
    render(<Button variant="primary">Click</Button>);
    expect(screen.getByRole('button')).toHaveClass('btn-primary');
  });

  it('calls onClick handler when clicked', () => {
    const handleClick = jest.fn();
    render(<Button onClick={handleClick}>Click</Button>);
    fireEvent.click(screen.getByRole('button'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });
  
  // Test accessibility - users might use keyboard
  it('is keyboard accessible', () => {
    render(<Button>Click</Button>);
    const button = screen.getByRole('button');
    button.focus();
    expect(button).toHaveFocus();
  });
});
```

---

## Contract-First Development

### Golden Rules
1. **NEVER modify files in `contracts/` directory**
2. Always import types from contracts
3. If contract is unclear, ask for clarification
4. TypeScript/Rust type checking will catch violations

### Why Contracts Matter for Users
- **Parallel development** = faster feature delivery = users get features sooner
- **No breaking changes** = stable user experience
- **Type safety** = fewer runtime errors = users don't lose work

### Example
```typescript
// ✅ CORRECT - Respects contract, users get reliable behavior
import type { NodeDefinition } from '../../contracts/node-types';

export function createNode(def: NodeDefinition) {
  // Implementation
}

// ❌ WRONG - Creates instability, users hit bugs
interface MyNodeDefinition { /* ... */ }
```

---

## File Organization

### Rust Workspace
```
chimera/
├── chimera-core/       # Core DSP logic (NO PANICS! - protects user experience)
├── chimera-cli/        # CLI tools
├── chimera-web/        # WASM frontend (what users see)
└── contracts/          # Locked API contracts (stability for users)
```

### React Structure (Epic #40)
```
src-react/
├── components/         # UI components (user interface)
├── hooks/             # Custom hooks
├── types/             # Type definitions (import from contracts!)
└── utils/             # Utility functions
```

---

## Accessibility & UX

### Keyboard Navigation
- All interactive elements must be keyboard accessible
- Use semantic HTML (`<button>`, not `<div onclick>`)
- Provide focus indicators
- **Why**: Accessibility is not optional - all users deserve great UX

### Error Messages
- User-friendly language (not technical jargon)
- Actionable guidance ("Try reducing SNR" not "Invalid parameter")
- **Remember**: Error messages are part of the learning experience

### Performance
- Keep bundle size small (faster load = better first impression)
- Optimize render cycles (smooth UI = professional feel)
- Test on slower devices (not all users have fast computers)

---

## AI Agent Guidelines
See `.github/copilot-instructions.md` for detailed agent instructions including:
- File ownership strategy
- Parallel development protocol
- Testing requirements (≥80% coverage **protects users**)
- No panics policy for Rust core (**protects user experience**)

---

## Code Review Checklist

Before submitting code, ask:
1. ✅ **Does this improve user experience?**
2. ✅ **Could this crash or confuse a user?**
3. ✅ **Is this accessible to all users?**
4. ✅ **Are error messages helpful to users?**
5. ✅ **Is this performant enough for users?**
6. ✅ **Would a beginner understand the error messages?**
7. ✅ **Tests passing? (Users won't hit bugs)**
8. ✅ **No panics? (Users won't crash)**

---

## Remember

> **Users trust us with their time and attention.**
> 
> Every bug we prevent, every error we handle gracefully, every accessible feature we build...
> 
> ...is a promise kept to our users.
> 
> **Write code you'd be proud to show users.**
