# Chart and Visualization Improvements

This document details the chart rendering improvements made to address space efficiency and visualization quality issues.

## Problem Statement

The original issue (#2a3a40b4) requested:
1. Make the UI more compact to avoid scrolling between controls and outputs
2. Make graphs larger and zoomable
3. Add axis labels and ranges to graphs
4. Use vector SVG format for charts (saveable and scalable)
5. Use UI font in graphs
6. Combine TX and RX constellation plots

## Implementation Summary

### 1. SVG Backend Migration

**Before**: Charts used HTML Canvas via `plotters-canvas`
- Fixed raster resolution
- Cannot be saved as vector graphics
- No browser zoom capability beyond page zoom
- Limited export options

**After**: Charts use SVG via `plotters::backend::SVGBackend`
- Vector graphics that scale infinitely without quality loss
- Right-click save works natively in all browsers
- Browser zoom works perfectly
- Can be imported into design tools

**Technical Changes**:
- Added `svg_backend` feature to plotters dependency in `Cargo.toml`
- Created `draw_line_chart_svg()` and `draw_constellation_svg()` functions
- Components render SVG to string and inject via `dangerously_set_inner_html`
- Removed Canvas-specific code and dependencies

### 2. Axis Labels and Ranges

**Before**: Charts had titles but no axis labels or numeric ranges

**After**: All charts have comprehensive labeling:

| Chart | X-Axis Label | Y-Axis Label | Range Display |
|-------|-------------|--------------|---------------|
| Timing Error | Sample Index | Error (samples) | Auto-scaled ±5% padding |
| NCO Frequency Offset | Sample Index | Offset (Hz) | Auto-scaled ±5% padding |
| Clean Signal PSD | Frequency Bin | Power (dBFS) | Auto-scaled ±5% padding |
| Noisy Signal PSD | Frequency Bin | Power (dBFS) | Auto-scaled ±5% padding |
| Running BER | Symbol Index | BER | Auto-scaled ±5% padding |
| Constellation (single) | In-Phase (I) | Quadrature (Q) | -1.5 to 1.5 (fixed) |
| Constellation (combined) | In-Phase (I) | Quadrature (Q) | -1.5 to 1.5 (fixed) |

**Label Styling**:
- Axis labels: Inter font, 13px, color `lch(180, 180, 190)`
- Axis descriptions: Inter font, 14px, color `lch(200, 200, 210)`
- Grid lines: Bold `lch(60, 80, 110)` at 50% opacity, light at 30%
- 6-7 labels per axis for optimal readability

### 3. Increased Chart Sizes

**Before**:
- Line charts: 320×220px
- Constellation charts: 220×220px

**After**:
- Line charts: 500×280px (+56% width, +27% height)
- Single constellation: 400×400px (+82% in each dimension)
- Combined constellation: 500×450px (new, large unified view)

**CSS Updates**:
- `.chart-panel` min-height: 200px → 280px
- `.constellation-panel` min-height: 220px → 400px
- `.constellation-panel.constellation-combined` min-height: 450px (new)
- `.svg-chart-container` provides flexible responsive container
- SVG scales to 100% width while maintaining aspect ratio

### 4. Combined Constellation Component

**Before**: TX and RX constellations were shown separately in different pipeline nodes, requiring scrolling to compare

**After**: New `CombinedConstellation` component in dedicated section

**Visual Design**:
- **Reference Points**: QPSK ideal positions at (±√½, ±√½) shown as cyan/green halos (8px, 30% opacity)
- **TX Symbols**: Cyan/green circles (5px) representing ideal transmitted symbols
- **RX Symbols**: Pink/magenta circles (3px) representing received symbols after channel
- **Legend**: Text labels with matching color indicators at top right
- **Grid**: Full cartesian mesh with labeled axes and ranges

**Component Architecture**:
```rust
#[derive(Properties, PartialEq)]
pub struct CombinedConstellationProps {
    pub title: AttrValue,
    pub tx_i_samples: Vec<f64>,
    pub tx_q_samples: Vec<f64>,
    pub rx_i_samples: Vec<f64>,
    pub rx_q_samples: Vec<f64>,
}
```

**Usage**:
```html
<CombinedConstellation 
    title="TX vs RX Constellation" 
    tx_i_samples={tx_i.clone()} 
    tx_q_samples={tx_q.clone()} 
    rx_i_samples={rx_i.clone()} 
    rx_q_samples={rx_q.clone()} 
/>
```

### 5. Typography Consistency

**Before**: Charts used default plotters font (not matching UI)

**After**: All charts use "Inter" font family matching the UI:
- Chart titles: Inter, 18px
- Axis labels: Inter, 12-13px
- Axis descriptions: Inter, 14px
- Legend text: Inter, 13-14px

This creates visual consistency across the entire application.

### 6. Vertical Space Optimization

To accommodate larger charts while maintaining compact layout:

**Spacing Reductions**:
- Main grid gap: 20px → 16px
- Panel padding: 20px → 18px
- Panel header margin: 16px → 12px
- Node graph gap: 20px → 16px
- Chart grid gap: 16px → 12px
- Control grid gap: 20px → 16px
- Metrics grid gap: 20px → 16px
- Log viewer height: 180px → 150px

**Net Result**: Despite larger charts, overall page height is similar or reduced due to:
1. Combined constellation (2 separate → 1 unified view)
2. Tighter spacing throughout
3. Reduced log viewer height

## Files Modified

1. **chimera-web/Cargo.toml**
   - Added `svg_backend` feature to plotters dependency

2. **chimera-web/src/ui.rs**
   - Removed Canvas-specific imports
   - Added SVG backend imports
   - Created `draw_line_chart_svg()` with axis labels
   - Created `draw_constellation_svg()` with axis labels
   - Created `draw_combined_constellation_svg()` with legend
   - Added `CombinedConstellationProps` and `CombinedConstellation` component
   - Updated LineChart to use SVG rendering
   - Updated ConstellationChart to use SVG rendering
   - Added axis label props to LineChart
   - Injected combined constellation section in layout

3. **chimera-web/style.css**
   - Updated `.chart-panel` min-height
   - Updated `.constellation-panel` min-height
   - Added `.constellation-panel.constellation-combined` styles
   - Added `.svg-chart-container` styles for SVG elements
   - Reduced spacing throughout for compact layout

4. **UI_IMPROVEMENTS.md**
   - Documented new SVG rendering features
   - Documented combined constellation component
   - Updated spacing optimization table

## Benefits

1. **Better Visualization**: Larger charts with clear axis labels make data easier to interpret
2. **Exportability**: SVG format allows saving charts for reports/documentation
3. **Scalability**: Vector graphics scale perfectly on any display or zoom level
4. **Comparison**: Combined constellation enables direct TX/RX comparison
5. **Compactness**: Despite larger charts, reduced scrolling due to unified views and tighter spacing
6. **Consistency**: Inter font throughout creates cohesive design
7. **Professionalism**: Labeled axes and legends make charts publication-ready

## Testing

All tests pass:
```
cargo test --workspace
```

Results:
- Core tests: 7 passed
- Acceptance tests: 6 passed
- Web tests: 4 passed
- Doc tests: 1 passed

Build verified:
```
cargo check --workspace
```

## Browser Compatibility

SVG rendering works in all modern browsers:
- Chrome/Chromium 90+
- Firefox 88+
- Safari 14+
- Edge 90+

All WASM-compatible browsers support the SVG backend.

## Future Enhancements

Potential improvements:
- Interactive tooltips on hover showing exact values
- Zoom/pan controls for charts
- Export button with multiple format options (SVG, PNG, PDF)
- Collapsible chart sections
- Chart theme customization (colors, line styles)
- Downloadable chart data as CSV
