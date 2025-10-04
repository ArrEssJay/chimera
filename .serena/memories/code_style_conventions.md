# Chimera Code Style and Conventions

## Rust Code Style

### General Principles
- Follow standard Rust conventions (rustfmt default)
- Use `nightly` toolchain (see rust-toolchain.toml)
- **Critical**: NO PANICS in core code (`chimera-core/src/`)
  - Never use `.unwrap()` or `.expect()`
  - Always return `Result<T, E>` for fallible operations
  - Use `?` operator for error propagation

### Naming Conventions
- **Modules**: snake_case (e.g., `config`, `decoder`, `diagnostics`)
- **Structs/Enums**: PascalCase (e.g., `SimulationOutput`, `NodeDefinition`)
- **Functions**: snake_case (e.g., `run_simulation`, `build_modulation_audio`)
- **Constants**: SCREAMING_SNAKE_CASE

### Error Handling Pattern
```rust
// ❌ WRONG - Will fail CI
fn process(&self, inputs: Vec<Data>) -> Result<Vec<Data>, Error> {
    let first = inputs.get(0).unwrap(); // FORBIDDEN
    // ...
}

// ✅ CORRECT
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

## TypeScript Code Style (Epic #40)

### General Principles
- Use TypeScript strict mode
- Prefer arrow functions for React components
- Use `type` over `interface` for props
- Import types from `contracts/` (never modify contracts)

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

```css
/* ❌ WRONG */
.button {
  color: #007bff;
  padding: 16px;
}

/* ✅ CORRECT */
.button {
  color: var(--primary-color);
  padding: var(--spacing-md);
}
```

## Testing Conventions

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
});
```

## Contract-First Development

### Golden Rules
1. **NEVER modify files in `contracts/` directory**
2. Always import types from contracts
3. If contract is unclear, ask for clarification
4. TypeScript/Rust type checking will catch violations

### Example
```typescript
// ✅ CORRECT
import type { NodeDefinition } from '../../contracts/node-types';

export function createNode(def: NodeDefinition) {
  // Implementation
}

// ❌ WRONG - Don't create your own type
interface MyNodeDefinition { /* ... */ }
```

## File Organization

### Rust Workspace
```
chimera/
├── chimera-core/       # Core DSP logic (NO PANICS!)
├── chimera-cli/        # CLI tools
├── chimera-web/        # WASM frontend
└── contracts/          # Locked API contracts
```

### React Structure (Epic #40)
```
src-react/
├── components/         # UI components
├── hooks/             # Custom hooks
├── types/             # Type definitions (import from contracts!)
└── utils/             # Utility functions
```

## AI Agent Guidelines
See `.github/copilot-instructions.md` for detailed agent instructions including:
- File ownership strategy
- Parallel development protocol
- Testing requirements (≥80% coverage)
- No panics policy for Rust core
