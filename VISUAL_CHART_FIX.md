# Visual Chart Fix Documentation

## What Was Broken

Based on the issue screenshot and code analysis, the charts were rendering but missing critical information:

### Before the Fix

**Diagnostics Charts:**
```
┌─────────────────────────────┐
│   Timing Error              │  ← Title only
│                             │
│   [Chart with unlabeled     │  ← Axis ticks present
│    axes showing numbers     │     but no descriptions
│    but no context]          │
│                             │
└─────────────────────────────┘
```

Problems:
- Axes had numbers but no labels
- Users couldn't tell what the numbers represented
- No units shown on axes
- PSD charts had "(dBFS)" in title but not on axis

### After the Fix

**Diagnostics Charts:**
```
┌─────────────────────────────────────┐
│   Timing Error                      │  ← Clean title
│                                     │
│   Error (samples) │                 │  ← Y-axis labeled
│                   │                 │
│   [Chart with     │   *   *   *    │
│    clearly        │  *         *   │
│    labeled axes]  │ *           *  │
│                   │                 │
│                   └─────────────────│
│                     Sample Index    │  ← X-axis labeled
└─────────────────────────────────────┘
```

Benefits:
- X-axis clearly shows "Sample Index"
- Y-axis shows measurement type with units
- Professional appearance
- Self-explanatory charts

## Specific Chart Changes

### 1. Timing Error Chart
**Before:** 
- Title: "Timing Error"
- X-axis: Numbers only (0, 100, 200...)
- Y-axis: Numbers only (-0.5, 0, 0.5...)

**After:**
- Title: "Timing Error"
- X-axis: "Sample Index" (0, 100, 200...)
- Y-axis: "Error (samples)" (-0.5, 0, 0.5...)

### 2. NCO Frequency Offset Chart
**Before:**
- Title: "NCO Frequency Offset"
- X-axis: Numbers only
- Y-axis: Numbers only

**After:**
- Title: "NCO Frequency Offset"
- X-axis: "Sample Index"
- Y-axis: "Offset (Hz)"

### 3. Clean Signal PSD Chart
**Before:**
- Title: "Clean Signal PSD (dBFS)" ← Duplicate label
- X-axis: Numbers only
- Y-axis: Numbers only

**After:**
- Title: "Clean Signal PSD" ← No duplication
- X-axis: "Frequency Bin"
- Y-axis: "Power (dBFS)" ← Unit in proper place

### 4. Noisy Signal PSD Chart
**Before:**
- Title: "Noisy Signal PSD (dBFS)" ← Duplicate label
- X-axis: Numbers only
- Y-axis: Numbers only

**After:**
- Title: "Noisy Signal PSD" ← No duplication
- X-axis: "Frequency Bin"
- Y-axis: "Power (dBFS)" ← Unit in proper place

### 5. Running BER Chart
**Before:**
- Title: "Running BER"
- X-axis: Numbers only
- Y-axis: Numbers only

**After:**
- Title: "Running BER"
- X-axis: "Symbol Index"
- Y-axis: "BER"

## Code Changes

The fix was minimal and surgical. For each chart, we added two properties:

```rust
// Before
<LineChart
    title="Timing Error"
    values={timing_error.clone()}
    accent_rgb={Some((94, 214, 255))}
    tooltip={Some(AttrValue::from("..."))}
/>

// After
<LineChart
    title="Timing Error"
    values={timing_error.clone()}
    accent_rgb={Some((94, 214, 255))}
    x_label="Sample Index"              // ← Added
    y_label="Error (samples)"           // ← Added
    tooltip={Some(AttrValue::from("..."))}
/>
```

## Constellation Charts

The constellation charts (TX, RX, and Combined) were already working correctly with their I/Q axis labels:
- X-axis: "In-Phase (I)"
- Y-axis: "Quadrature (Q)"

These remain unchanged as they were already properly implemented.

## How the Rendering Works

The axis labels are rendered by the `draw_line_chart_svg()` function using the plotters library:

```rust
chart
    .configure_mesh()
    .x_desc(x_label)      // ← Uses the passed label
    .y_desc(y_label)      // ← Uses the passed label
    .label_style(("Inter", 13, &RGBColor(180, 180, 190)))
    .axis_desc_style(("Inter", 14, &RGBColor(200, 200, 210)))
    .draw()?;
```

The rendering infrastructure was already in place - we just needed to pass the labels through the component props.

## Impact

### User Experience
- Charts are now self-documenting
- No need to guess what axes represent
- Professional appearance matching the design docs
- Consistent with constellation chart styling

### Code Quality
- Matches the design specification in UI_IMPROVEMENTS.md
- Consistent with the SVG chart improvements already implemented
- No breaking changes
- Minimal code changes (surgical fix)

### Testing
- New Playwright tests ensure this won't break again
- Tests verify all axis labels are present
- Tests confirm no duplicate labels
- Tests run automatically in CI

## Summary

This was a simple but critical fix:
- **What broke:** Axis labels not being passed to chart components
- **Impact:** Charts were functional but confusing to users
- **Fix:** Added 10 lines of code (2 props per chart × 5 charts)
- **Result:** Professional, self-documenting charts
- **Prevention:** Automated tests ensure it stays fixed
