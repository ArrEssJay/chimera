# Tooltips Implementation Guide

This guide explains how to add and use tooltips in Chimera.

## 🎯 Overview

Chimera provides tooltips through:
1. **React Tooltip Component** - For React components
2. **CSS-based Tooltips** - For legacy Yew UI (existing implementation)
3. **ARIA Compliance** - For accessibility

## 📦 React Tooltip Component

### Basic Usage

Import and use the Tooltip component:

```tsx
import { Tooltip } from '@/components/Tooltip';

function MyComponent() {
  return (
    <Tooltip content="Helpful explanation" placement="top">
      <button>Hover me</button>
    </Tooltip>
  );
}
```

### Props

```typescript
interface TooltipProps {
  content: ReactNode;           // Tooltip text or JSX
  placement?: TooltipPlacement; // 'top' | 'right' | 'bottom' | 'left'
  children: ReactNode;          // Element to trigger tooltip
  className?: string;           // Optional custom styling
}
```

### Placement Options

```tsx
// Top (default)
<Tooltip content="Top tooltip">
  <button>Top</button>
</Tooltip>

// Right
<Tooltip content="Right tooltip" placement="right">
  <button>Right</button>
</Tooltip>

// Bottom
<Tooltip content="Bottom tooltip" placement="bottom">
  <button>Bottom</button>
</Tooltip>

// Left
<Tooltip content="Left tooltip" placement="left">
  <button>Left</button>
</Tooltip>
```

### With Rich Content

```tsx
<Tooltip
  content={
    <div>
      <strong>SNR (Signal-to-Noise Ratio)</strong>
      <p>Higher values = cleaner signal</p>
      <p>Typical range: 0-20 dB</p>
    </div>
  }
  placement="top"
>
  <span className="info-icon">ⓘ</span>
</Tooltip>
```

## 🎨 CSS-Based Tooltips (Existing)

The Yew-based UI uses CSS tooltips via the `data-tooltip` attribute:

```html
<!-- In Rust/Yew template -->
<p
  class="info-rollover"
  data-tooltip="Bit-error ratio after LDPC decoding"
  title="Bit-error ratio after LDPC decoding"
  tabindex="0"
>
  Post-FEC BER: 0.001
</p>
```

### CSS Classes

**`.info-rollover`** - Underlined text with tooltip:
```css
.info-rollover {
  border-bottom: 1px dashed;
  cursor: help;
}

.info-rollover::after {
  content: attr(data-tooltip);
  /* Positioning and styling */
}
```

**`.has-tooltip`** - Any element with tooltip (no underline):
```css
.has-tooltip::after {
  content: attr(data-tooltip);
}
```

## ♿ Accessibility

### Requirements

All tooltips must be accessible:

1. **Keyboard accessible**: Show on focus
2. **Screen reader friendly**: Use ARIA attributes
3. **Visible on hover AND focus**: Don't rely on mouse only
4. **Dismissable**: Close with Escape key

### React Component Accessibility

The Tooltip component handles accessibility automatically:

```tsx
// Generated markup
<div onMouseEnter={...} onFocus={...}>
  <div aria-describedby="tooltip-123">
    {children}
  </div>
  <div
    id="tooltip-123"
    role="tooltip"
    aria-hidden={!isVisible}
  >
    {content}
  </div>
</div>
```

### CSS Tooltip Accessibility

For CSS tooltips, ensure:

```html
<!-- Good: Focusable with title -->
<span
  class="info-rollover"
  data-tooltip="Explanation"
  title="Explanation"
  tabindex="0"
>
  Info text
</span>

<!-- Bad: Not focusable -->
<span data-tooltip="Explanation">
  Info text
</span>
```

## 📋 When to Use Tooltips

### ✅ Good Uses

- **Technical terms**: Explain DSP jargon
  ```tsx
  <Tooltip content="Signal-to-Noise Ratio in decibels">
    <span>SNR (dB)</span>
  </Tooltip>
  ```

- **Metric explanations**: Clarify what numbers mean
  ```tsx
  <Tooltip content="Ratio of incorrect bits to total bits">
    <span>BER: 0.001</span>
  </Tooltip>
  ```

- **Icon buttons**: Explain what action does
  ```tsx
  <Tooltip content="Run simulation">
    <button aria-label="Run simulation">▶️</button>
  </Tooltip>
  ```

- **Disabled controls**: Explain why disabled
  ```tsx
  <Tooltip content="Complete setup first">
    <button disabled>Run</button>
  </Tooltip>
  ```

### ❌ Bad Uses

- **Essential information**: Don't hide critical info in tooltips
- **Long instructions**: Use help text or modals instead
- **Redundant text**: Don't tooltip "Submit button"
- **On mobile**: Tooltips don't work well on touch devices

## 🛠️ Implementation Patterns

### With Form Controls

```tsx
function SettingsForm() {
  return (
    <div className="form-field">
      <label htmlFor="snr-input">
        SNR
        <Tooltip content="Signal-to-Noise Ratio (0-20 dB)">
          <span className="help-icon">ⓘ</span>
        </Tooltip>
      </label>
      <input
        id="snr-input"
        type="number"
        min={0}
        max={20}
      />
    </div>
  );
}
```

### With Metrics

```tsx
function MetricDisplay({ value }: { value: number }) {
  return (
    <Tooltip
      content="Pre-FEC Bit Error Rate: ratio of errors before error correction"
      placement="top"
    >
      <div className="metric">
        <span className="metric-label">Pre-FEC BER</span>
        <span className="metric-value">{value.toExponential(2)}</span>
      </div>
    </Tooltip>
  );
}
```

### With Complex Content

```tsx
function ConstellationChart() {
  return (
    <div className="chart-container">
      <Tooltip
        placement="top"
        content={
          <div className="chart-help">
            <h4>QPSK Constellation</h4>
            <ul>
              <li>Four symbol positions (±1±j)</li>
              <li>Noise spreads clusters</li>
              <li>Tighter = better SNR</li>
            </ul>
            <a href="/docs/signal_processing_concepts.md#constellation">
              Learn more
            </a>
          </div>
        }
      >
        <div className="chart-header">
          TX Constellation <span className="help-icon">ⓘ</span>
        </div>
      </Tooltip>
      
      {/* Chart content */}
    </div>
  );
}
```

## 🎨 Styling Tooltips

### Using Design System

```tsx
<Tooltip
  content={<span style={{ color: 'var(--text-primary)' }}>Tooltip</span>}
  className="custom-tooltip"
>
  <button>Hover</button>
</Tooltip>
```

### Custom Tooltip Styles

```css
/* In component CSS */
.custom-tooltip .chimera-tooltip__content {
  background: var(--panel);
  border: 2px solid var(--accent);
  max-width: 300px;
  padding: var(--spacing-md);
}

.custom-tooltip .chimera-tooltip__content--visible {
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-8px); }
  to { opacity: 1; transform: translateY(0); }
}
```

## 📱 Mobile Considerations

Tooltips don't work well on touch devices. Consider alternatives:

### Alternative: Modal Help

```tsx
function MobileHelp({ content }: { content: ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  
  return (
    <>
      <button
        onClick={() => setIsOpen(true)}
        aria-label="Show help"
      >
        ⓘ
      </button>
      
      {isOpen && (
        <Modal onClose={() => setIsOpen(false)}>
          {content}
        </Modal>
      )}
    </>
  );
}
```

### Alternative: Expandable Help Text

```tsx
function ExpandableHelp({ content }: { content: string }) {
  const [isExpanded, setIsExpanded] = useState(false);
  
  return (
    <div className="help-expandable">
      <button onClick={() => setIsExpanded(!isExpanded)}>
        {isExpanded ? '▼' : '▶'} Help
      </button>
      {isExpanded && <p>{content}</p>}
    </div>
  );
}
```

## 🧪 Testing Tooltips

### Unit Tests

```typescript
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { Tooltip } from './Tooltip';

describe('Tooltip', () => {
  it('shows on hover', async () => {
    render(
      <Tooltip content="Help text">
        <button>Hover me</button>
      </Tooltip>
    );
    
    const trigger = screen.getByRole('button');
    fireEvent.mouseEnter(trigger);
    
    await waitFor(() => {
      expect(screen.getByRole('tooltip')).toBeVisible();
    });
  });
  
  it('shows on focus', async () => {
    render(
      <Tooltip content="Help text">
        <button>Focus me</button>
      </Tooltip>
    );
    
    const trigger = screen.getByRole('button');
    trigger.focus();
    
    await waitFor(() => {
      expect(screen.getByRole('tooltip')).toBeVisible();
    });
  });
  
  it('hides on blur', async () => {
    render(
      <Tooltip content="Help text">
        <button>Test</button>
      </Tooltip>
    );
    
    const trigger = screen.getByRole('button');
    trigger.focus();
    trigger.blur();
    
    await waitFor(() => {
      expect(screen.queryByRole('tooltip')).not.toBeVisible();
    });
  });
});
```

### E2E Tests

```typescript
import { test, expect } from '@playwright/test';

test('tooltips show on hover', async ({ page }) => {
  await page.goto('/');
  
  // Hover over element with tooltip
  await page.hover('[data-testid="snr-label"]');
  
  // Wait for tooltip to appear
  const tooltip = page.locator('[role="tooltip"]');
  await expect(tooltip).toBeVisible();
  await expect(tooltip).toContainText('Signal-to-Noise Ratio');
});
```

## 📊 Tooltip Inventory

Current tooltips in the application:

### Simulation Controls
- [x] Preset selection - explains preset types
- [x] SNR slider - definition and range
- [ ] Plaintext input - character limit explanation
- [ ] Run button - what happens when clicked

### Pipeline Visualization
- [x] Frame layout metrics - symbol type explanations
- [x] TX constellation - ideal QPSK positions
- [x] RX constellation - noise effects
- [x] Channel SNR - actual vs requested
- [x] LDPC decoder - iterations and convergence

### Audio Controls
- [ ] Play clean - original signal
- [ ] Play noisy - with channel noise
- [ ] Volume control - adjustment range

### Diagnostics
- [x] Pre-FEC BER - before error correction
- [x] Post-FEC BER - after error correction
- [x] Residual errors - uncorrected bits

## 🔧 Maintenance

### Adding New Tooltips

1. Identify element needing explanation
2. Choose appropriate pattern (component vs CSS)
3. Write clear, concise content
4. Test accessibility (keyboard + screen reader)
5. Update this guide if new pattern

### Updating Existing Tooltips

1. Verify tooltip is still accurate
2. Check for clarity and brevity
3. Ensure accessibility maintained
4. Test on multiple browsers

## 📚 Resources

- [React Tooltip Component](../chimera-web/src-react/components/Tooltip.tsx)
- [Tooltip Storybook](../chimera-web/src-react/components/Tooltip.stories.tsx)
- [CSS Tooltips](../chimera-web/style.css) (search for `.info-rollover`)
- [ARIA Tooltip Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/tooltip/)
- [WebAIM Tooltips](https://webaim.org/techniques/css/invisiblecontent/#tooltips)

---

**Need help?** Check the Tooltip component stories in Storybook or ask in GitHub Discussions.
