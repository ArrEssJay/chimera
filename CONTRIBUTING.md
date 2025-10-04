# Contributing to Chimera

Thank you for your interest in contributing to Chimera! This guide will help you get started with development.

## 🚀 Quick Start

### Prerequisites

Install the following tools:

1. **Rust** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. **Node.js** (18 or later) and npm
   ```bash
   # Download from https://nodejs.org/
   # Or use nvm:
   nvm install 18
   ```

3. **Trunk** (WASM bundler)
   ```bash
   cargo install trunk
   ```

4. **wasm-pack** (optional, for CLI builds)
   ```bash
   cargo install wasm-pack
   ```

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/ArrEssJay/chimera.git
cd chimera

# Install Node dependencies
cd chimera-web
npm install --legacy-peer-deps

# Build Rust core (optional, trunk will do this automatically)
cd ../chimera-core
cargo build --release

# Run development server
cd ../chimera-web
npm run dev
```

## 🏗️ Project Structure

```
chimera/
├── chimera-core/          # Rust DSP engine
│   ├── src/
│   │   ├── lib.rs        # Core library exports
│   │   ├── ldpc/         # LDPC encoder/decoder
│   │   ├── modulation/   # QPSK modulation
│   │   └── channel/      # Channel simulation
│   └── tests/            # Integration tests
│
├── chimera-web/           # Web application
│   ├── src-react/        # React + TypeScript (current)
│   │   ├── components/   # UI components
│   │   ├── stores/       # Zustand state management
│   │   └── App.tsx       # Main app
│   ├── src/              # Legacy Yew code (being phased out)
│   ├── tests/            # E2E tests
│   └── package.json      # NPM config
│
├── chimera-cli/          # Command-line interface
├── contracts/            # Type definitions (TypeScript + Rust)
└── docs/                 # Documentation
```

## 🎯 Development Workflow

### 1. Choose Your Task

Check the [issues](https://github.com/ArrEssJay/chimera/issues) for:
- Issues labeled `good first issue` for beginners
- Issues labeled `help wanted` for contributions needed
- Current phase milestones

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation only
- `refactor/` - Code refactoring

### 3. Make Your Changes

#### For React Components

```bash
cd chimera-web

# Run development server with hot reload
npm run dev

# In another terminal, run Storybook
npm run storybook

# Run tests in watch mode
npm test
```

**Component checklist:**
- [ ] TypeScript interfaces defined with JSDoc
- [ ] Component implemented with accessibility (ARIA)
- [ ] CSS using design system variables
- [ ] Unit tests with ≥80% coverage
- [ ] Storybook stories showing all variants
- [ ] No hardcoded colors or spacing

#### For Rust Core

```bash
cd chimera-core

# Run tests
cargo test

# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build for WASM
cargo build --target wasm32-unknown-unknown
```

**Rust checklist:**
- [ ] No `.unwrap()` or `.expect()` in library code
- [ ] Use `Result<T, E>` for error handling
- [ ] Public APIs have rustdoc comments
- [ ] Tests cover happy and error paths
- [ ] Clippy passes with no warnings

### 4. Test Your Changes

```bash
# React tests
cd chimera-web
npm test -- --coverage
npm run lint
npm run build

# Rust tests
cd chimera-core
cargo test --lib
cargo test --test '*'

# E2E tests
cd chimera-web
npm run e2e
```

### 5. Commit Your Changes

Use clear, descriptive commit messages:

```bash
git add .
git commit -m "feat: add tooltip component with keyboard support"
```

Commit message format:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation change
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `style:` - Code style changes

### 6. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear title describing the change
- Description of what and why
- Screenshots for UI changes
- Link to related issue

## 📋 Code Style Guidelines

### TypeScript/React

- Use functional components with hooks
- Use TypeScript strict mode
- Export types/interfaces for public APIs
- Add JSDoc comments for all public functions
- Use CSS modules or component-scoped styles
- Follow existing component patterns

**Example:**

```typescript
/**
 * Button component with multiple variants.
 * Supports loading state and keyboard navigation.
 * 
 * @example
 * ```tsx
 * <Button variant="primary" onClick={handleClick}>
 *   Click Me
 * </Button>
 * ```
 */
export function Button({ 
  variant = 'primary', 
  children,
  ...props 
}: ButtonProps) {
  // Implementation
}
```

### Rust

- Follow Rust API guidelines
- Use `Result<T, E>` for fallible operations
- Add rustdoc for all public items
- Avoid panics in library code
- Use `clippy` and `rustfmt`

**Example:**

```rust
/// Encodes a message using LDPC forward error correction.
///
/// # Arguments
/// * `message` - Input message bits
/// * `code_rate` - LDPC code rate (e.g., 0.5 for rate 1/2)
///
/// # Returns
/// Encoded codeword with parity bits, or error if encoding fails
///
/// # Errors
/// Returns `Err` if message length doesn't match code parameters
pub fn encode(message: &[u8], code_rate: f32) -> Result<Vec<u8>, EncoderError> {
    // Implementation
}
```

### CSS

Use CSS variables from `chimera-web/style.css`:

```css
.my-component {
  /* ✅ Good - use design system */
  color: var(--text-primary);
  padding: var(--spacing-md);
  border: 1px solid var(--border-color);
  
  /* ❌ Bad - hardcoded values */
  /* color: #333; */
  /* padding: 16px; */
}
```

## 🧪 Testing Guidelines

### Unit Tests

- Test all public APIs
- Test both success and error cases
- Test edge cases and boundary conditions
- Aim for ≥80% code coverage

### Component Tests

```typescript
describe('Button', () => {
  it('renders with text', () => {
    render(<Button>Click Me</Button>);
    expect(screen.getByRole('button')).toHaveTextContent('Click Me');
  });

  it('calls onClick when clicked', () => {
    const handleClick = vi.fn();
    render(<Button onClick={handleClick}>Click</Button>);
    fireEvent.click(screen.getByRole('button'));
    expect(handleClick).toHaveBeenCalledOnce();
  });

  it('is disabled when loading', () => {
    render(<Button loading>Click</Button>);
    expect(screen.getByRole('button')).toBeDisabled();
  });
});
```

### E2E Tests

See [PLAYWRIGHT_TESTING.md](chimera-web/PLAYWRIGHT_TESTING.md) for E2E testing guide.

## 🎨 Design System

### Colors

Use semantic color variables:
- `--primary-color` - Brand primary
- `--text-primary` - Primary text
- `--text-muted` - Secondary text
- `--border-color` - Borders and dividers
- `--bg-overlay` - Overlay backgrounds
- `--accent` - Accent/highlight color

### Spacing

- `--spacing-xs` - 4px
- `--spacing-sm` - 8px
- `--spacing-md` - 16px
- `--spacing-lg` - 24px
- `--spacing-xl` - 32px

### Typography

- `--font-body` - System font stack
- `--font-mono` - Monospace font

## 📦 Pull Request Process

1. **Ensure CI passes**: All tests, linting, and builds must succeed
2. **Update documentation**: Add/update docs for new features
3. **Add tests**: New code must have test coverage
4. **Follow style guide**: Match existing code patterns
5. **One concern per PR**: Keep PRs focused and reviewable

### PR Checklist

- [ ] Tests pass locally
- [ ] Linting passes
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if applicable)
- [ ] Screenshots added for UI changes
- [ ] No breaking changes (or clearly documented)
- [ ] Commit messages are clear

## 🐛 Bug Reports

When reporting bugs, include:

1. **Description**: Clear description of the issue
2. **Steps to reproduce**: Exact steps to trigger the bug
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Environment**: Browser, OS, Node/Rust versions
6. **Screenshots**: If applicable

## 💡 Feature Requests

When suggesting features:

1. **Use case**: What problem does this solve?
2. **Proposed solution**: How would you implement it?
3. **Alternatives**: Other approaches considered
4. **Additional context**: Mockups, examples, etc.

## 🤝 Code Review

### As a Reviewer

- Be respectful and constructive
- Suggest improvements, don't demand perfection
- Approve when ready, request changes if needed
- Comment on both strengths and areas for improvement

### As an Author

- Respond to all comments
- Ask questions if feedback is unclear
- Make requested changes or explain why not
- Thank reviewers for their time

## 📚 Additional Resources

- [React Documentation](https://react.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly Rust Guide](https://rustwasm.github.io/)
- [Storybook Documentation](https://storybook.js.org/docs)

## 🙋 Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/ArrEssJay/chimera/discussions)
- **Bugs**: Open a [GitHub Issue](https://github.com/ArrEssJay/chimera/issues)
- **Chat**: Join our community chat (link TBD)

## 📄 License

By contributing, you agree that your contributions will be licensed under the project's MIT License.

---

Thank you for contributing to Chimera! 🎉
