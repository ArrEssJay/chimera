# Accessibility Guide

## Overview

Chimera is fully compliant with **WCAG 2.1 Level AA** accessibility standards. This document describes the accessibility features and testing procedures.

## WCAG 2.1 AA Compliance

### ✅ Perceivable

#### Color Contrast
- **Text contrast**: ≥4.5:1 ratio for normal text
- **UI component contrast**: ≥3:1 ratio for interactive elements
- All colors use CSS custom properties from the design system
- High contrast mode support via `@media (prefers-contrast: high)`

#### Text Alternatives
- All images have descriptive alt text
- Icons have ARIA labels when used without text
- Loading states announce via `aria-busy`
- Status indicators include text content, not just color

#### Adaptable Content
- Semantic HTML5 elements (`<button>`, `<nav>`, `<main>`, `<header>`, `<footer>`)
- Proper heading hierarchy (`h1` → `h2` → `h3`)
- ARIA landmarks for screen readers
- Responsive design that adapts to viewport size
- Content readable at 200% zoom

### ✅ Operable

#### Keyboard Navigation
- All interactive elements are keyboard accessible
- Visible focus indicators on all focusable elements
- Logical tab order throughout the application
- No keyboard traps
- Skip navigation link to main content

**Keyboard Shortcuts:**
- `Tab`: Move to next focusable element
- `Shift + Tab`: Move to previous focusable element
- `Enter` / `Space`: Activate buttons and links
- `Arrow keys`: Navigate within dropdowns, select components
- `Escape`: Close modals and dropdowns

#### Touch Targets
- Minimum touch target size: **44x44px** (WCAG 2.1 AA requirement)
- Adequate spacing between interactive elements on mobile
- All buttons and links meet minimum size requirements

#### Timing
- No time limits on user interactions
- Reduced motion support via `@media (prefers-reduced-motion: reduce)`

### ✅ Understandable

#### Readable
- Clear, concise language throughout
- Font size adjustable up to 200% without loss of functionality
- Line height and paragraph spacing optimized for readability

#### Predictable
- Consistent navigation across pages
- Consistent component behavior
- No unexpected context changes on focus or input

#### Input Assistance
- Form labels clearly associated with inputs
- Error messages are descriptive and actionable
- Required fields indicated with `*` and `aria-required`
- Form validation provides clear feedback

### ✅ Robust

#### Compatible
- Valid, semantic HTML5
- Proper ARIA roles, states, and properties
- Compatible with assistive technologies (NVDA, JAWS, VoiceOver)
- Cross-browser compatible (Chrome, Firefox, Safari, Edge)

## Responsive Design

### Breakpoints

| Device  | Width      | Columns | Layout          |
|---------|------------|---------|-----------------|
| Mobile  | < 640px    | 1       | Single column   |
| Tablet  | 640-1024px | 2       | Two columns     |
| Desktop | > 1024px   | 3-5     | Multi-column    |

### Mobile Optimizations

- **Touch targets**: All interactive elements ≥44x44px
- **Single-column layouts**: Easy vertical scrolling
- **Larger tap areas**: Buttons and links sized for fingers
- **Readable fonts**: Font sizes adjusted for mobile viewing
- **Horizontal scrolling**: Enabled for wide tables with `-webkit-overflow-scrolling: touch`

### Tablet Optimizations

- **Two-column layouts**: Better use of screen space
- **Flexible grids**: Adapt to portrait/landscape orientation
- **Touch-friendly**: Same touch target sizes as mobile

### Desktop Optimizations

- **Multi-column layouts**: Efficient use of wide screens
- **Keyboard navigation**: Enhanced for desktop users
- **Mouse interactions**: Hover states and tooltips
- **Max width**: 1600px for optimal readability

## Testing Checklist

### Manual Testing

#### Keyboard Navigation
- [ ] Tab through all interactive elements
- [ ] Verify visible focus indicators
- [ ] Check for keyboard traps
- [ ] Test skip navigation link
- [ ] Verify Escape key closes modals/dropdowns
- [ ] Test arrow key navigation in dropdowns

#### Screen Reader Testing

**NVDA (Windows):**
```bash
# Start NVDA
nvda
# Navigate by heading: H
# Navigate by form field: F
# Navigate by button: B
```

**VoiceOver (macOS):**
```bash
# Start VoiceOver: Cmd + F5
# Navigate: VO + Arrow keys
# Interact: VO + Shift + Down
# Stop interacting: VO + Shift + Up
```

**Checklist:**
- [ ] All images have alt text announced
- [ ] Form labels are associated with inputs
- [ ] Button purpose is clear
- [ ] Error messages are announced
- [ ] Loading states are announced
- [ ] Status changes are announced

#### Visual Testing
- [ ] Text contrast meets 4.5:1 ratio (use WebAIM Contrast Checker)
- [ ] UI elements meet 3:1 contrast ratio
- [ ] Focus indicators are visible
- [ ] Touch targets are ≥44x44px on mobile
- [ ] Layout works at 320px width (mobile)
- [ ] Content readable at 200% zoom
- [ ] No horizontal scrolling at default zoom

#### Mobile Device Testing
- [ ] Test on iPhone (Safari)
- [ ] Test on Android (Chrome)
- [ ] Verify touch targets
- [ ] Test landscape orientation
- [ ] Check scrolling behavior

### Automated Testing

#### axe DevTools
```bash
# Install axe extension for Chrome/Firefox
# Open DevTools → axe tab
# Click "Scan All of My Page"
# Fix all violations
```

#### Lighthouse Audit
```bash
# In Chrome DevTools
# Go to Lighthouse tab
# Select "Accessibility" category
# Run audit
# Target: 90+ score
```

#### Jest/Vitest Tests
```bash
# Run accessibility test suite
npm test -- src-react/tests/accessibility.test.tsx

# Run all tests with coverage
npm test -- --coverage
```

## Component-Specific Accessibility

### Button
- Uses native `<button>` element
- Has `aria-busy` when loading
- Disabled via `disabled` attribute (not `aria-disabled`)
- Minimum size enforced on mobile

### Select
- Custom select with `role="combobox"`
- Keyboard navigable with arrow keys
- Has `aria-expanded` attribute
- Options have `role="option"`
- Announces selected value to screen readers

### Panel
- Collapsible header has `role="button"`
- Has `aria-expanded` attribute
- Keyboard accessible with Enter/Space
- Focus indicator visible

### Tooltip
- Proper `role="tooltip"`
- Associated with trigger element
- Accessible on keyboard focus
- Does not trap focus

### Badge
- Conveys information through text, not just color
- Appropriate color contrast
- Status communicated to screen readers

## Common Issues & Solutions

### Issue: Focus indicator not visible
**Solution:** Ensure `:focus-visible` styles are applied. Check CSS specificity.

### Issue: Screen reader not announcing changes
**Solution:** Use ARIA live regions (`aria-live="polite"` or `aria-live="assertive"`)

### Issue: Keyboard trap in modal
**Solution:** Implement focus trap and restore focus on close

### Issue: Poor color contrast
**Solution:** Use CSS variables from design system, test with contrast checker

### Issue: Touch targets too small on mobile
**Solution:** Apply `min-height: 44px` and `min-width: 44px` via responsive CSS

## Resources

### Testing Tools
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [axe DevTools](https://www.deque.com/axe/devtools/)
- [Lighthouse](https://developers.google.com/web/tools/lighthouse)
- [WAVE](https://wave.webaim.org/)

### Screen Readers
- [NVDA](https://www.nvaccess.org/) (Windows, free)
- [JAWS](https://www.freedomscientific.com/products/software/jaws/) (Windows)
- [VoiceOver](https://www.apple.com/accessibility/voiceover/) (macOS/iOS, built-in)
- [TalkBack](https://support.google.com/accessibility/android/answer/6283677) (Android, built-in)

### Guidelines
- [WCAG 2.1](https://www.w3.org/WAI/WCAG21/quickref/)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM](https://webaim.org/)

## Continuous Improvement

Accessibility is an ongoing commitment. Regular testing and user feedback help us improve:

1. **Automated tests** run on every commit
2. **Manual testing** performed before each release
3. **User feedback** collected and prioritized
4. **Accessibility audits** conducted quarterly

## Contact

If you encounter accessibility issues, please:
1. File an issue on GitHub
2. Include browser/assistive technology details
3. Describe expected vs. actual behavior
4. Provide screenshots if applicable

We aim to respond to accessibility issues within 48 hours.
