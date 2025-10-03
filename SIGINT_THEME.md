# SIGINT Workbench UI Transformation

## Overview
The UI has been completely transformed from a modern, rounded, purple/blue theme to a professional, military-grade SIGINT (Signals Intelligence) workbench aesthetic.

## Key Visual Changes

### Color Palette
**Before:**
- Purple/blue accent colors (lch(70% 65 260))
- Soft, rounded aesthetic
- Consumer-friendly appearance

**After:**
- Tactical green primary (lch(75% 65 140))
- Amber warnings (lch(75% 70 80))
- Cyan data indicators (lch(75% 50 200))
- High-contrast military-grade colors
- Sharp, angular tactical design

### Typography
**Before:**
- Inter, SF Pro Display (modern sans-serif)
- Rounded, friendly appearance

**After:**
- Share Tech Mono (monospace, technical)
- Orbitron (display headers, futuristic)
- Rajdhani (clean technical sans)
- All-caps emphasis for technical readouts
- Increased letter-spacing for readability

### UI Elements

#### Buttons
**Before:** Rounded pills with soft shadows
**After:** Angular tactical switches with:
- Inset beveled edges
- Tactical border glow
- Military-grade activation states
- All-caps labels with wide letter spacing

#### Panels
**Before:** Rounded corners (16px), soft borders
**After:** 
- Sharp corners (0px radius)
- Tactical corner brackets (top-left, bottom-right)
- Inset border highlights
- Professional military panel styling

#### Charts
**Before:** Clean minimal backgrounds
**After:**
- Grid overlay patterns (oscilloscope-style)
- Scanline effects
- Tactical color scheme (green for TX, cyan for RX)
- Radar/scope aesthetic
- Technical readout styling

#### Badges & Status Indicators
**Before:** Rounded badges with soft colors
**After:**
- Angular tactical indicators
- Pulsing glow animations
- All-caps labels
- Military status color coding

#### Tables
**Before:** Subtle rounded styling
**After:**
- Sharp technical readout format
- Accent-colored headers with arrows
- Hover states with tactical highlighting
- Monospace data cells

### Background Effects
**Before:** Soft radial gradients
**After:**
- Grid overlay (40px squares)
- Scanline effects (subtle horizontal lines)
- Tactical corner accents
- Professional workbench appearance

### Animations
**Before:** Smooth, organic transitions
**After:**
- Pulsing tactical indicators
- Sharp, precise transitions
- Glow effects on active elements
- Military-grade status animations

## Technical Implementation

### CSS Changes
- Replaced all rounded corners with sharp angles
- Updated color variables to tactical palette (LCH color space)
- Added grid and scanline overlays using repeating gradients
- Implemented corner bracket pseudo-elements on panels and nodes
- Enhanced box-shadow effects for tactical depth and glow
- Added tactical status utility classes
- Imported Google Fonts: Share Tech Mono, Orbitron, Rajdhani

### Color Standards and Enforcement

**MANDATORY: All CSS colors MUST use LCH color space**

The SIGINT Workbench theme strictly enforces the use of LCH (Lightness, Chroma, Hue) color space for all CSS color definitions. This ensures:

- **Perceptual uniformity**: Equal numerical differences in LCH produce equal perceptual differences in color
- **Consistent brightness**: All colors at the same lightness level appear equally bright to the human eye
- **Predictable blending**: Color gradients and transitions appear smooth and natural
- **Professional appearance**: Military-grade interfaces use perceptually uniform color spaces

#### CSS Color Rules

1. **All CSS colors MUST be defined using LCH format:**
   ```css
   /* ✅ CORRECT */
   --accent: lch(75% 65 140);
   --text-primary: lch(92% 8 140);
   background: lch(8% 2 180);
   
   /* ❌ INCORRECT - DO NOT USE */
   --accent: #00ff00;
   --text: rgb(200, 200, 200);
   background: rgba(10, 10, 15, 0.9);
   ```

2. **Variable naming convention:**
   - Use semantic names: `--tactical-green`, `--accent`, `--panel-border`
   - Avoid hex or RGB values in variable names
   - Include alpha channel when needed: `lch(75% 65 140 / 0.5)`

3. **Gradients must use LCH colors:**
   ```css
   /* ✅ CORRECT */
   background: linear-gradient(135deg, lch(75% 65 140) 0%, lch(70% 60 120) 100%);
   
   /* ❌ INCORRECT */
   background: linear-gradient(135deg, #00ff00 0%, #00cc00 100%);
   ```

4. **No hardcoded RGB/hex colors allowed in CSS files**
   - Exception: `transparent` keyword is allowed
   - All other colors must use LCH or CSS variables

#### Plotters/Rust Color Mapping

While the Plotters library requires RGB colors for rendering, these values are derived from the LCH tactical palette:

- **Tactical Green** (TX): `lch(75% 65 140)` → `RGBColor(120, 220, 150)`
- **Tactical Cyan** (RX): `lch(75% 50 200)` → `RGBColor(120, 200, 240)`
- **Tactical Amber** (warnings): `lch(75% 70 80)` → `RGBColor(~240, ~200, ~120)`

These RGB values in Rust code are acceptable as they represent the closest RGB approximation of the LCH tactical colors. When adding new chart colors:

1. Define the color in LCH space first (in CSS or documentation)
2. Convert to RGB for Plotters using a color converter
3. Document the LCH → RGB mapping in comments

### Rust/Plotters Changes
- Updated chart caption fonts to Share Tech Mono
- Changed constellation point colors:
  - TX: Tactical green RGB(120, 220, 150) ≈ LCH(75% 65 140)
  - RX: Tactical cyan RGB(120, 200, 240) ≈ LCH(75% 50 200)
- Updated mesh grid colors to tactical green palette
- Modified default accent colors for line charts
- All RGB values in Plotters are approximations of LCH tactical colors

### HTML
- Updated page title to "CHIMERA | SIGINT Workbench"
- Added meta description: "Advanced Low Probability of Intercept & Detection Signal Processing Training"

## Design Inspiration
The new design draws from:
- Military radar/sonar displays
- Intelligence agency workstations (NSA, GCHQ style)
- Oscilloscope/spectrum analyzer UIs
- Terminal/command-line interfaces
- Tactical operations centers (TOC)
- SIGINT (Signals Intelligence) equipment
- Mil-spec user interfaces

## Result
A professional, high-information-density interface that looks like it belongs in a classified military signals intelligence facility, perfectly matching the advanced signal processing nature of the Chimera project. The interface now conveys:

- **Authority**: Military-grade professionalism
- **Precision**: Technical accuracy and attention to detail
- **Intelligence**: Sophisticated data analysis capabilities
- **Stealth**: Covert operations aesthetic matching the LPI/LPD protocol
- **Power**: Advanced signal processing capabilities

The transformation successfully aligns the visual presentation with the project's focus on signals intelligence, covert communications, and advanced DSP techniques.
