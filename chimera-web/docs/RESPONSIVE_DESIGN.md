# Responsive Design Guide

## Overview

Chimera uses a mobile-first responsive design approach that adapts seamlessly across all device sizes. This guide covers our responsive design strategy, breakpoints, and best practices.

## Breakpoints

We use three main breakpoints optimized for common device sizes:

| Breakpoint | Width Range  | Target Devices           | Layout Strategy    |
|------------|--------------|--------------------------|-------------------|
| Mobile     | < 640px      | Phones                   | Single column     |
| Tablet     | 640-1024px   | Tablets, small laptops   | 2-column grid     |
| Desktop    | > 1024px     | Laptops, desktops        | Multi-column grid |

### Additional Breakpoints

- **Large Desktop**: > 1600px - Max width container for readability
- **Landscape Mobile**: < 640px + landscape - Optimized 2-column layout

## CSS Custom Properties

All spacing, sizing, and typography use CSS custom properties for consistency:

```css
:root {
  /* Spacing */
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 12px;
  --spacing-lg: 16px;
  --spacing-xl: 24px;
  --spacing-2xl: 32px;
  --spacing-3xl: 48px;

  /* Touch targets */
  --touch-target-min: 44px;

  /* Font sizes */
  --font-size-xs: 11px;
  --font-size-sm: 12px;
  --font-size-base: 14px;
  --font-size-lg: 16px;
  --font-size-xl: 18px;
  --font-size-2xl: 24px;
}
```

## Mobile Design (< 640px)

### Layout Principles

1. **Single Column**: All content stacks vertically
2. **Full Width**: Content uses full width minus padding
3. **Touch Optimized**: All targets ≥44x44px
4. **Vertical Rhythm**: Consistent spacing between elements

### Example Mobile Layout

```css
@media (max-width: 640px) {
  /* Stack all grids */
  .control-grid,
  .chart-grid,
  .node-graph {
    grid-template-columns: 1fr;
  }

  /* Full-width buttons */
  .audio-actions button {
    width: 100%;
  }

  /* Larger touch targets */
  button {
    min-height: var(--touch-target-min);
    padding: var(--spacing-md) var(--spacing-lg);
  }
}
```

### Mobile Optimizations

- **Header**: Stacked layout, full-width search
- **Navigation**: Hamburger menu (if applicable)
- **Forms**: Full-width inputs, stacked fields
- **Tables**: Horizontal scroll with touch support
- **Images**: Responsive with `max-width: 100%`
- **Modals**: Full-screen on mobile

## Tablet Design (640px - 1024px)

### Layout Principles

1. **Two Column**: Better use of horizontal space
2. **Flexible Grids**: Adapt to portrait/landscape
3. **Touch-Friendly**: Maintain ≥44px touch targets
4. **Progressive Enhancement**: More features than mobile

### Example Tablet Layout

```css
@media (min-width: 640px) and (max-width: 1024px) {
  /* Two-column grids */
  .control-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  /* Flexible charts */
  .chart-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  /* Two-column node graph */
  .node-graph {
    grid-template-columns: repeat(2, 1fr);
  }
}
```

### Tablet Optimizations

- **Header**: Horizontal layout with room for controls
- **Navigation**: Always visible
- **Forms**: Two-column layout for related fields
- **Tables**: Full-width with better column distribution
- **Sidebars**: Can show/hide based on content

## Desktop Design (> 1024px)

### Layout Principles

1. **Multi-Column**: Efficient use of screen space
2. **Max Width**: 1600px for optimal readability
3. **Mouse Optimized**: Hover states, tooltips
4. **Rich Interactions**: Drag-and-drop, context menus

### Example Desktop Layout

```css
@media (min-width: 1024px) {
  /* Five-column node graph */
  .node-graph {
    grid-template-columns: repeat(5, 1fr);
  }

  /* Auto-fit charts */
  .chart-grid {
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  }

  /* Max width for readability */
  .main-grid,
  .app-header,
  .app-footer {
    max-width: 1600px;
    margin: 0 auto;
  }
}
```

### Desktop Optimizations

- **Header**: Full horizontal layout with all controls
- **Navigation**: Persistent sidebar or top nav
- **Forms**: Multi-column with inline validation
- **Tables**: Full feature set, sortable columns
- **Keyboard**: Enhanced keyboard shortcuts
- **Hover States**: Rich hover interactions

## Responsive Typography

### Font Sizes

Font sizes adapt based on viewport:

```css
/* Mobile */
@media (max-width: 640px) {
  html {
    font-size: 14px;
  }
  
  .logo-title {
    font-size: var(--font-size-lg); /* 16px */
  }
}

/* Desktop */
@media (min-width: 1024px) {
  html {
    font-size: 16px;
  }
  
  .logo-title {
    font-size: var(--font-size-xl); /* 18px */
  }
}
```

### Line Height

- **Body text**: 1.5 (optimal for readability)
- **Headings**: 1.2 (tighter for impact)
- **UI elements**: 1 (compact for controls)

## Responsive Images

### Best Practices

```html
<!-- Responsive image with srcset -->
<img 
  src="image.jpg" 
  srcset="image-320.jpg 320w, image-640.jpg 640w, image-1024.jpg 1024w"
  sizes="(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw"
  alt="Description"
/>

<!-- CSS for responsive images -->
<style>
  img {
    max-width: 100%;
    height: auto;
  }
</style>
```

## Responsive Grids

### CSS Grid

We use CSS Grid for flexible, responsive layouts:

```css
/* Auto-fit columns with minimum width */
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: var(--spacing-md);
}

/* Responsive column count */
@media (max-width: 640px) {
  .grid {
    grid-template-columns: 1fr;
  }
}

@media (min-width: 640px) and (max-width: 1024px) {
  .grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (min-width: 1024px) {
  .grid {
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  }
}
```

## Touch Targets

All interactive elements meet WCAG 2.1 AA standards:

### Minimum Size: 44x44px

```css
@media (max-width: 640px) {
  button,
  a,
  input[type="button"],
  .interactive-element {
    min-height: 44px;
    min-width: 44px;
    padding: var(--spacing-md) var(--spacing-lg);
  }
}
```

### Spacing Between Targets

```css
.button-group {
  display: flex;
  gap: var(--spacing-sm); /* 8px minimum */
}
```

## Testing Responsive Design

### Browser DevTools

1. Open Chrome DevTools (F12)
2. Toggle device toolbar (Ctrl+Shift+M)
3. Test common devices:
   - iPhone SE (375x667)
   - iPhone 12 Pro (390x844)
   - iPad (768x1024)
   - iPad Pro (1024x1366)

### Manual Testing

- **Physical Devices**: Test on real phones and tablets
- **Browser Resize**: Slowly resize browser window
- **Orientation**: Test portrait and landscape
- **Zoom**: Test at 200% zoom level

### Automated Testing

```bash
# Run responsive tests
npm test -- src-react/tests/responsive.test.tsx

# Visual regression testing (if configured)
npm run test:visual
```

## Common Responsive Patterns

### Responsive Navigation

```css
/* Mobile: Hamburger menu */
@media (max-width: 640px) {
  .nav-links {
    display: none;
  }
  
  .nav-links.open {
    display: flex;
    flex-direction: column;
  }
  
  .hamburger {
    display: block;
  }
}

/* Desktop: Horizontal menu */
@media (min-width: 640px) {
  .nav-links {
    display: flex;
    flex-direction: row;
  }
  
  .hamburger {
    display: none;
  }
}
```

### Responsive Cards

```css
.card-grid {
  display: grid;
  gap: var(--spacing-lg);
}

/* Mobile: 1 column */
@media (max-width: 640px) {
  .card-grid {
    grid-template-columns: 1fr;
  }
}

/* Tablet: 2 columns */
@media (min-width: 640px) and (max-width: 1024px) {
  .card-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

/* Desktop: 3+ columns */
@media (min-width: 1024px) {
  .card-grid {
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  }
}
```

### Responsive Tables

```css
/* Mobile: Horizontal scroll */
@media (max-width: 640px) {
  .table-wrapper {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }
  
  table {
    min-width: 600px;
  }
}

/* Desktop: Full width */
@media (min-width: 640px) {
  table {
    width: 100%;
  }
}
```

## Performance Considerations

### Mobile Performance

- **Lazy Loading**: Images load as needed
- **Code Splitting**: Load only necessary JavaScript
- **CSS Minification**: Reduce file size
- **HTTP/2**: Efficient resource loading

### Optimization Tips

```javascript
// Lazy load images
<img loading="lazy" src="image.jpg" alt="Description" />

// Responsive image loading
<picture>
  <source media="(max-width: 640px)" srcset="small.jpg" />
  <source media="(max-width: 1024px)" srcset="medium.jpg" />
  <img src="large.jpg" alt="Description" />
</picture>
```

## Print Styles

Don't forget print media:

```css
@media print {
  /* Hide non-essential elements */
  .app-header,
  .app-footer,
  button,
  .run-controls {
    display: none;
  }

  /* Optimize for print */
  body {
    background: white;
    color: black;
  }

  /* Avoid page breaks inside elements */
  .panel,
  .chart {
    break-inside: avoid;
  }
}
```

## Browser Support

We support modern browsers with graceful degradation:

- **Chrome**: Latest 2 versions
- **Firefox**: Latest 2 versions
- **Safari**: Latest 2 versions
- **Edge**: Latest 2 versions
- **Mobile Safari**: iOS 12+
- **Chrome Mobile**: Latest 2 versions

### Feature Detection

```css
/* Use feature queries for new CSS */
@supports (display: grid) {
  .container {
    display: grid;
  }
}

/* Fallback for older browsers */
@supports not (display: grid) {
  .container {
    display: flex;
    flex-wrap: wrap;
  }
}
```

## Best Practices

1. **Mobile First**: Start with mobile, enhance for larger screens
2. **Content First**: Focus on content hierarchy
3. **Touch Friendly**: Design for fingers, not just mouse
4. **Performance**: Optimize for slower mobile connections
5. **Test Real Devices**: Don't rely solely on emulators
6. **Flexible Units**: Use relative units (rem, %, vh/vw)
7. **Breakpoint Logic**: Group related breakpoint styles together
8. **Accessible**: Ensure responsive design maintains accessibility

## Tools & Resources

### Development Tools
- Chrome DevTools Device Mode
- Firefox Responsive Design Mode
- BrowserStack (cross-browser testing)
- LambdaTest (real device testing)

### Testing Services
- [Responsinator](http://www.responsinator.com/)
- [Am I Responsive](http://ami.responsivedesign.is/)
- [Mobile-Friendly Test](https://search.google.com/test/mobile-friendly)

### CSS Frameworks
- CSS Grid
- Flexbox
- CSS Custom Properties (Variables)

## Checklist

Before shipping responsive design:

- [ ] Test all three breakpoints (mobile, tablet, desktop)
- [ ] Verify touch targets ≥44x44px on mobile
- [ ] Check horizontal scrolling (should not occur)
- [ ] Test landscape orientation on mobile/tablet
- [ ] Verify images are responsive
- [ ] Check typography scaling
- [ ] Test on real devices
- [ ] Verify performance on 3G connection
- [ ] Check accessibility at all sizes
- [ ] Test print styles
