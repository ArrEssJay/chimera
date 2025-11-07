# Chimera API Reference

This document describes the public APIs and components available in Chimera.

## 📚 Documentation Sources

Chimera uses multiple documentation systems:

- **Component Documentation**: Storybook (auto-generated from code)
- **TypeScript APIs**: JSDoc comments in source
- **Rust APIs**: Rustdoc comments (cargo doc)

## 🎨 React Components

All components are fully documented with JSDoc and TypeScript types. View them in Storybook or the source files.

### Running Storybook

```bash
cd chimera-web
npm run storybook
```

This opens an interactive component browser at http://localhost:6006

### Component Library

Located in `chimera-web/src-react/components/`

#### Button

Multi-variant button component with loading state and accessibility.

**Import**:
```typescript
import { Button, type ButtonProps } from '@/components/Button';
```

**Props**:
- `variant?: 'primary' | 'secondary' | 'danger'` - Visual style (default: 'primary')
- `size?: 'sm' | 'md' | 'lg'` - Button size (default: 'md')
- `loading?: boolean` - Show loading spinner (default: false)
- `icon?: ReactNode` - Optional icon before text
- `children: ReactNode` - Button content
- Plus all standard button HTML attributes

**Example**:
```tsx
<Button 
  variant="primary" 
  size="md"
  loading={isSubmitting}
  onClick={handleClick}
>
  Submit
</Button>
```

**Accessibility**:
- Fully keyboard accessible
- ARIA busy state when loading
- Focus visible styles

---

#### Select

Dropdown select component with keyboard navigation.

**Import**:
```typescript
import { Select, type SelectProps, type SelectOption } from '@/components/Select';
```

**Props**:
- `options: SelectOption[]` - Array of options
- `value?: string` - Selected value
- `onChange?: (value: string) => void` - Change handler
- `placeholder?: string` - Placeholder text (default: 'Select...')
- `disabled?: boolean` - Disable interaction (default: false)
- `className?: string` - Optional custom class

**SelectOption Type**:
```typescript
interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}
```

**Example**:
```tsx
const options = [
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3', disabled: true },
];

<Select
  options={options}
  value={selectedValue}
  onChange={setSelectedValue}
  placeholder="Choose option..."
/>
```

**Keyboard Support**:
- `Enter/Space` - Open/select
- `Arrow Up/Down` - Navigate options
- `Home/End` - Jump to first/last
- `Escape` - Close dropdown

---

#### Panel

Container component with optional header and collapsible content.

**Import**:
```typescript
import { Panel, type PanelProps } from '@/components/Panel';
```

**Props**:
- `title?: string` - Panel header title
- `children: ReactNode` - Panel content
- `footer?: ReactNode` - Optional footer content
- `collapsible?: boolean` - Enable collapse (default: false)
- `defaultCollapsed?: boolean` - Initial state (default: false)
- `className?: string` - Optional custom class

**Example**:
```tsx
<Panel 
  title="Settings" 
  collapsible
  footer={<Button>Save</Button>}
>
  <p>Panel content here</p>
</Panel>
```

---

#### Tooltip

Tooltip component with configurable placement.

**Import**:
```typescript
import { Tooltip, type TooltipProps } from '@/components/Tooltip';
```

**Props**:
- `content: ReactNode` - Tooltip content
- `placement?: 'top' | 'right' | 'bottom' | 'left'` - Position (default: 'top')
- `children: ReactNode` - Trigger element
- `className?: string` - Optional custom class

**Example**:
```tsx
<Tooltip content="Helpful explanation" placement="top">
  <button>Hover me</button>
</Tooltip>
```

**Behavior**:
- Shows on hover (200ms delay)
- Shows on focus
- Hides on mouse leave or blur
- ARIA compliant

---

#### Badge

Status indicator badge component.

**Import**:
```typescript
import { Badge, type BadgeProps } from '@/components/Badge';
```

**Props**:
- `status?: 'success' | 'warning' | 'error'` - Status type (default: 'success')
- `icon?: ReactNode` - Optional icon
- `children: ReactNode` - Badge text
- `className?: string` - Optional custom class

**Example**:
```tsx
<Badge status="success">Active</Badge>
<Badge status="warning" icon="⚠️">Pending</Badge>
<Badge status="error">Failed</Badge>
```

---

## 🦀 Rust Core API

The Rust core provides DSP functionality compiled to WebAssembly.

### Generating Rust Documentation

```bash
cd chimera-core
cargo doc --no-deps --open
```

This generates and opens rustdoc documentation in your browser.

### Key Modules

#### `chimera_core::modulation`

QPSK modulation and demodulation.

**Functions**:
- `modulate(bits: &[u8]) -> Vec<Complex<f32>>` - Convert bits to QPSK symbols
- `demodulate(symbols: &[Complex<f32>]) -> Vec<u8>` - Recover bits from symbols
- `compute_snr(signal: &[f32], noise: &[f32]) -> f32` - Calculate SNR in dB

#### `chimera_core::ldpc`

Low-density parity-check error correction.

**Functions**:
- `encode(message: &[u8], code_rate: f32) -> Result<Vec<u8>>` - LDPC encoding
- `decode(codeword: &[f32], max_iter: usize) -> Result<Vec<u8>>` - LDPC decoding
- `generate_matrix(n: usize, k: usize) -> Matrix` - Create LDPC matrix

#### `chimera_core::channel`

Channel simulation and impairment models.

**Functions**:
- `add_awgn(signal: &[Complex<f32>], snr_db: f32) -> Vec<Complex<f32>>` - Add white noise
- `apply_fading(signal: &[Complex<f32>]) -> Vec<Complex<f32>>` - Fading channel
- `compute_ber(original: &[u8], received: &[u8]) -> f32` - Bit error rate

### WASM Bindings

JavaScript-accessible functions (via wasm-bindgen).

**Core Functions**:
```typescript
// TypeScript definitions (auto-generated)

interface SimulationConfig {
  preset: string;
  plaintext: string;
  snr_db: number;
  audio_path?: string;
}

interface SimulationResult {
  frame_layout: FrameLayout;
  tx_symbols: Float32Array;
  rx_symbols: Float32Array;
  recovered_text: string;
  pre_fec_ber: number;
  post_fec_ber: number;
  residual_errors: number;
}

// Run simulation
function run_simulation(config: SimulationConfig): SimulationResult;

// Generate audio samples
function generate_audio(
  symbols: Float32Array,
  sample_rate: number,
  snr_db?: number
): Float32Array;
```

---

## 🗂️ State Management

Chimera uses Zustand for state management.

### Simulation Store

Located in `chimera-web/src-react/stores/simulationStore.ts`

**State**:
```typescript
interface SimulationState {
  config: SimulationConfig;
  result: SimulationResult | null;
  isRunning: boolean;
  error: string | null;
}
```

**Actions**:
```typescript
interface SimulationActions {
  setConfig(config: Partial<SimulationConfig>): void;
  runSimulation(): Promise<void>;
  reset(): void;
}
```

**Usage**:
```typescript
import { useSimulationStore } from '@/stores/simulationStore';

function MyComponent() {
  const { config, result, runSimulation } = useSimulationStore();
  
  return (
    <Button onClick={runSimulation}>
      Run Simulation
    </Button>
  );
}
```

---

## 🎨 Design System

### CSS Variables

Located in `chimera-web/style.css`

#### Colors
```css
--primary-color: /* Brand primary */
--text-primary: /* Primary text color */
--text-muted: /* Secondary text */
--text-soft: /* Tertiary text */
--border-color: /* Border/divider color */
--bg-overlay: /* Overlay background */
--accent: /* Accent/highlight */
--accent-glow: /* Accent shadow */
--panel: /* Panel background */
--warning: /* Warning color */
--error: /* Error color */
--success: /* Success color */
```

#### Spacing
```css
--spacing-xs: 4px;
--spacing-sm: 8px;
--spacing-md: 16px;
--spacing-lg: 24px;
--spacing-xl: 32px;
```

#### Typography
```css
--font-body: /* System font stack */
--font-mono: /* Monospace font */
```

#### Usage
```css
.my-component {
  color: var(--text-primary);
  padding: var(--spacing-md);
  border: 1px solid var(--border-color);
  font-family: var(--font-body);
}
```

---

## 📊 Type Definitions

### Contracts

Shared type definitions between TypeScript and Rust.

Located in `contracts/` directory:
- `contracts/node-types.ts` - TypeScript interfaces
- `contracts/node-trait.rs` - Rust traits

These files define the contract between frontend and backend. **Do not modify without approval.**

See `contracts/README.md` for detailed contract documentation.

---

## 🧪 Testing Utilities

### React Testing Library

**Render helper**:
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { Button } from './Button';

test('button renders with text', () => {
  render(<Button>Click Me</Button>);
  expect(screen.getByRole('button')).toHaveTextContent('Click Me');
});
```

### Custom Test Utilities

Located in `chimera-web/src-react/test-utils.ts` (if exists)

**Mock data factories**:
```typescript
import { createMockSimulationResult } from './test-utils';

const mockResult = createMockSimulationResult({
  pre_fec_ber: 0.01,
  post_fec_ber: 0.001,
});
```

---

## 📖 Additional Documentation

### Auto-Generated Docs

Generate and view documentation:

```bash
# Storybook (React components)
cd chimera-web
npm run storybook

# Rustdoc (Rust APIs)
cd chimera-core
cargo doc --no-deps --open

# TypeDoc (if configured)
cd chimera-web
npx typedoc src-react/
```

### Source Code Documentation

All public APIs are documented with:
- **TypeScript**: JSDoc comments
- **Rust**: Rustdoc comments

Read the source code in:
- `chimera-web/src-react/` - React components
- `chimera-core/src/` - Rust DSP core

### Related Documentation

- [User Guide](USER_GUIDE.md) - How to use the application
- [Contributing Guide](../CONTRIBUTING.md) - Development workflow
- [Technical Overview](chimera_technical_overview.md) - Architecture details
- [UI Controls Spec](ui_controls_specification.md) - Control reference

---

## 🔄 API Versioning

Current version: **0.1.0** (pre-release)

API stability:
- ✅ React components: Stable, semantic versioning
- ⚠️ WASM API: Subject to change before 1.0
- ⚠️ Rust internal API: May change between releases

Breaking changes will be documented in CHANGELOG.md

---

## 📞 Questions?

- **Component usage**: Check Storybook examples
- **Type definitions**: Read JSDoc or hover in VS Code
- **Rust functions**: See cargo doc output
- **Need help?**: Open a GitHub Discussion

**Last updated**: October 2025
