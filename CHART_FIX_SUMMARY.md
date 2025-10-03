# Chart Fix Summary

## Issue
The constellation and diagnostics plots were broken in the UI. Charts were rendering but missing critical axis labels that were supposed to be present according to the design documentation.

## Root Cause
The `LineChart` components in the diagnostics section were not passing the `x_label` and `y_label` props, even though:
1. The `LineChartProps` struct defined these properties
2. The `draw_line_chart_svg` function supported rendering axis labels
3. The documentation (UI_IMPROVEMENTS.md) specified what these labels should be

This meant the charts were rendering with axis ticks but no descriptive labels explaining what the axes represented.

## Changes Made

### 1. Fixed Missing Axis Labels (`chimera-web/src/ui.rs`)

Added proper axis labels to all 5 diagnostic LineChart components:

| Chart | X-Axis Label | Y-Axis Label |
|-------|-------------|--------------|
| Timing Error | Sample Index | Error (samples) |
| NCO Frequency Offset | Sample Index | Offset (Hz) |
| Clean Signal PSD | Frequency Bin | Power (dBFS) |
| Noisy Signal PSD | Frequency Bin | Power (dBFS) |
| Running BER | Symbol Index | BER |

### 2. Removed Duplicate Labels

- Changed chart titles from "Clean Signal PSD (dBFS)" and "Noisy Signal PSD (dBFS)" to just "Clean Signal PSD" and "Noisy Signal PSD"
- The unit "(dBFS)" is now only shown in the y-axis label, eliminating duplication
- Removed unused `HtmlCanvasElement` import

### 3. Added Playwright Testing Infrastructure

Created comprehensive end-to-end tests to prevent regression:

**New Files:**
- `chimera-web/playwright.config.ts` - Playwright configuration
- `chimera-web/tests/e2e/charts.spec.ts` - Chart validation tests
- `chimera-web/tests/e2e/README.md` - Testing documentation
- `chimera-web/package.json` - Node.js project configuration

**Test Coverage:**
- Verifies constellation charts render with I/Q axis labels
- Checks combined constellation includes TX/RX legend
- Validates all 5 diagnostic charts render with correct axis labels
- Confirms SVG backend is used (scalable and saveable)
- Ensures no duplicate labels appear

**Updated:**
- `.gitignore` - Excludes node_modules and test artifacts

## Verification

All existing Rust tests pass:
- 7 core library unit tests ✅
- 6 encoder acceptance tests ✅
- 4 pipeline integration tests ✅
- 1 doc test ✅

## How to Run Playwright Tests

```bash
cd chimera-web

# Install dependencies
npm install

# Install Playwright browsers
npx playwright install

# Run tests (requires trunk serve)
npm test
```

## Before and After

**Before:**
- Charts rendered but axes had no descriptive labels
- Users couldn't tell what the numbers on the axes represented
- PSD chart titles had redundant unit indicators

**After:**
- All diagnostic charts have clear, descriptive axis labels
- Constellation charts properly labeled with "In-Phase (I)" and "Quadrature (Q)"
- No duplicate labels or units
- Automated tests prevent this issue from recurring

## Technical Details

The fix was minimal and surgical:
- Modified only the LineChart component calls (lines 763-793 in ui.rs)
- Added two string properties per chart component
- Removed one unused import
- No changes to the rendering logic itself
- No changes to data processing or simulation code

The axis label rendering was already implemented in `draw_line_chart_svg()` at lines 1279-1282, using the `.x_desc()` and `.y_desc()` methods from the plotters library. The labels just needed to be passed through the component props.
