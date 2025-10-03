# SIGINT Workbench Style Guide

## Overview

This document defines the mandatory styling standards for the Chimera SIGINT Workbench interface. All contributors must follow these guidelines to maintain the professional, military-grade aesthetic.

## Color System

### LCH Color Space Requirement

**ALL CSS colors MUST use the LCH (Lightness, Chroma, Hue) color space.**

#### Why LCH?

1. **Perceptual Uniformity**: Equal numerical differences produce equal perceptual differences
2. **Consistent Brightness**: All colors at the same lightness level appear equally bright
3. **Predictable Blending**: Smooth, natural gradients and transitions
4. **Professional Standard**: Used in military and professional interfaces for optimal readability

#### LCH Format

```css
lch(L% C H)           /* Without alpha */
lch(L% C H / A)       /* With alpha channel */
```

- **L (Lightness)**: 0% (black) to 100% (white)
- **C (Chroma)**: 0 (gray) to ~150 (vibrant)
- **H (Hue)**: 0-360 degrees (color wheel)
- **A (Alpha)**: 0 (transparent) to 1 (opaque)

### Tactical Color Palette

#### Primary Colors

```css
/* Core Interface Colors */
--bg: lch(8% 2 180);                    /* Deep military gray background */
--bg-overlay: lch(6% 3 180);            /* Darker overlay */
--panel: lch(12% 3 180);                /* Panel background */
--panel-border: lch(35% 20 140 / 0.4);  /* Panel borders */

/* Text Colors */
--text-primary: lch(92% 8 140);         /* Bright technical white */
--text-muted: lch(55% 12 140);          /* Muted text */
--text-soft: lch(70% 10 140);           /* Soft text */

/* Accent Colors */
--accent: lch(75% 65 140);              /* Tactical green */
--accent-strong: lch(85% 70 120);       /* Strong green */
--accent-glow: lch(70% 60 140 / 0.25);  /* Glow effect */

/* Tactical Status Colors */
--tactical-amber: lch(75% 70 80);       /* Warnings/alerts */
--tactical-green: lch(70% 60 140);      /* Active/success */
--tactical-cyan: lch(75% 50 200);       /* Data/information */

/* Semantic Colors */
--success: lch(72% 58 140);             /* Success states */
--danger: lch(62% 72 30);               /* Error/danger */
--warning: lch(75% 70 80);              /* Warning states */
```

#### Usage Guidelines

1. **Use CSS Variables**: Reference the color variables from `:root` whenever possible
   ```css
   /* ✅ CORRECT */
   background: var(--panel);
   border-color: var(--accent);
   
   /* ❌ AVOID - Define as variable first */
   background: lch(12% 3 180);
   ```

2. **Gradients Must Use LCH**:
   ```css
   /* ✅ CORRECT */
   background: linear-gradient(135deg, 
     lch(75% 65 140) 0%, 
     lch(70% 60 120) 100%
   );
   
   /* ❌ INCORRECT */
   background: linear-gradient(135deg, #00ff00 0%, #00cc00 100%);
   ```

3. **Alpha Transparency**:
   ```css
   /* ✅ CORRECT */
   border: 1px solid lch(35% 20 140 / 0.4);
   background: lch(75% 65 140 / 0.15);
   
   /* ❌ INCORRECT */
   border: 1px solid rgba(50, 150, 100, 0.4);
   ```

### Prohibited Color Formats

**NEVER use these in CSS:**

```css
/* ❌ Hex colors */
color: #00ff00;
background: #1a1a1f;

/* ❌ RGB/RGBA */
color: rgb(0, 255, 0);
background: rgba(10, 10, 15, 0.9);

/* ❌ HSL/HSLA */
color: hsl(120, 100%, 50%);
background: hsla(240, 20%, 10%, 0.9);

/* ❌ Named colors (except 'transparent') */
color: green;
background: black;
```

**Exception**: The keyword `transparent` is allowed where appropriate.

## Typography

### Font Families

```css
--font-body: "Share Tech Mono", "Consolas", "SF Mono", monospace;
--font-mono: "Share Tech Mono", "Consolas", "SF Mono", monospace;
--font-display: "Orbitron", "Share Tech Mono", "Rajdhani", monospace;
```

### Typography Rules

1. **Monospace for Data**: Use `--font-mono` for technical readouts, data tables, and logs
2. **Display for Headers**: Use `--font-display` for major headings and titles
3. **Letter Spacing**: Increase letter-spacing (0.03-0.15em) for technical readability
4. **All-Caps**: Use for technical labels and status indicators
5. **Font Weight**: 600-700 for emphasis, 400 for body text

## Layout & Geometry

### Angular Design Language

1. **No Rounded Corners**: Use `border-radius: 0` or small values (2px max) for tactical feel
2. **Sharp Angles**: Prefer rectangular shapes and angular designs
3. **Corner Brackets**: Add tactical corner brackets using pseudo-elements
4. **Grid Alignment**: Align elements to a base grid (8px or 16px)

### Tactical UI Elements

#### Corner Brackets

```css
.tactical-panel::before {
  content: "";
  position: absolute;
  top: -1px;
  left: -1px;
  width: 8px;
  height: 8px;
  border-top: 2px solid var(--accent);
  border-left: 2px solid var(--accent);
  box-shadow: 0 0 4px var(--accent-glow);
}
```

#### Grid Overlays

```css
background: 
  repeating-linear-gradient(0deg, transparent, transparent 19px, var(--grid-color) 19px, var(--grid-color) 20px),
  repeating-linear-gradient(90deg, transparent, transparent 19px, var(--grid-color) 19px, var(--grid-color) 20px);
```

#### Scanlines

```css
.scanline-effect::before {
  content: "";
  position: absolute;
  inset: 0;
  background: repeating-linear-gradient(
    0deg,
    transparent,
    transparent 2px,
    var(--scanline-color) 2px,
    var(--scanline-color) 3px
  );
  pointer-events: none;
  opacity: 0.3;
}
```

## Effects

### Glow Effects

Use subtle glows for active/interactive elements:

```css
box-shadow: 0 0 12px var(--accent-glow);
text-shadow: 0 0 8px var(--accent-glow);
```

### Transitions

Keep transitions sharp and precise:

```css
transition: all 0.15s ease;  /* Quick, tactical feel */
```

Avoid slow, smooth transitions (>0.3s).

## Charts & Visualizations (Plotters/Rust)

While Plotters requires RGB colors, maintain consistency with the LCH palette:

### Color Mapping

Document the LCH → RGB conversion in comments:

```rust
// Tactical green for TX: lch(75% 65 140) → RGB approximation
let tx_color = RGBColor(120, 220, 150);

// Tactical cyan for RX: lch(75% 50 200) → RGB approximation  
let rx_color = RGBColor(120, 200, 240);

// Tactical amber for warnings: lch(75% 70 80) → RGB approximation
let warning_color = RGBColor(240, 200, 120);
```

### Mesh Grid Colors

Use tactical green shades for grid lines:

```rust
.bold_line_style(&RGBColor(80, 140, 100).mix(0.5))
.light_line_style(&RGBColor(60, 100, 80).mix(0.3))
```

## Enforcement

### Code Review Checklist

Before submitting changes:

- [ ] All CSS colors use LCH format (no hex, RGB, or HSL)
- [ ] CSS variables are used instead of hardcoded values
- [ ] Plotters RGB colors are documented with LCH equivalents
- [ ] Typography follows monospace/display font guidelines
- [ ] No rounded corners (except minimal 2px where necessary)
- [ ] Tactical elements (brackets, grids) are present where appropriate
- [ ] Transitions are quick (<0.2s) and precise
- [ ] Glow effects use defined color variables

### Validation

Run this check before committing CSS changes:

```bash
# Check for prohibited color formats in CSS
grep -E "(#[0-9a-fA-F]{3,6}|rgb\(|rgba\(|hsl\(|hsla\()" chimera-web/style.css
```

If any matches are found (except in comments), they must be converted to LCH.

## Tools & Resources

### LCH Color Converter

Use these tools to convert existing colors to LCH:

- [LCH Color Picker](https://lch.oklch.com/)
- [CSS Color Converter](https://colorjs.io/apps/convert/)

### Testing

Test colors in multiple scenarios:
- Light/dark mode contrast
- Color blindness simulation
- Low-light military environment simulation
- High ambient light conditions

## Examples

### Before (Incorrect)

```css
.button {
  background: #00ff00;
  border: 1px solid rgba(0, 255, 0, 0.5);
  color: rgb(200, 200, 200);
  border-radius: 16px;
  box-shadow: 0 4px 12px rgba(0, 255, 0, 0.3);
}
```

### After (Correct)

```css
.button {
  background: var(--accent);
  border: 1px solid lch(75% 65 140 / 0.5);
  color: var(--text-primary);
  border-radius: 0;
  box-shadow: 0 4px 12px var(--accent-glow);
}
```

## Questions?

Refer to:
- `SIGINT_THEME.md` - Overview of theme transformation
- `copilot_instructions.md` - Development guidelines
- This document - Detailed style specifications

For any questions about color choices or styling decisions, consult the existing codebase for examples and maintain consistency with the established SIGINT workbench aesthetic.
