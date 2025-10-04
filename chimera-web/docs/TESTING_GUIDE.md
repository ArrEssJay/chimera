# Manual Testing Guide

## Overview

This guide provides step-by-step instructions for manual testing of responsive design and accessibility features in Chimera.

## Pre-Testing Setup

### Required Tools

1. **Modern Browsers**
   - Chrome/Edge (latest)
   - Firefox (latest)
   - Safari (latest, macOS only)

2. **Screen Readers** (choose based on OS)
   - **NVDA** (Windows) - Free download from nvaccess.org
   - **VoiceOver** (macOS/iOS) - Built-in
   - **JAWS** (Windows) - Trial available
   - **TalkBack** (Android) - Built-in

3. **Testing Devices** (if available)
   - iPhone or Android phone
   - iPad or Android tablet
   - Various screen sizes (13", 15", 27"+)

4. **Browser Extensions**
   - axe DevTools (accessibility testing)
   - WAVE (accessibility evaluation)
   - Lighthouse (performance & accessibility)

### Test Environment

```bash
# Start development server
npm run dev

# Open in browser
# Navigate to: http://localhost:5173
```

## Responsive Design Testing

### 1. Mobile Testing (< 640px)

#### Browser DevTools Method

1. Open Chrome DevTools (F12)
2. Click "Toggle Device Toolbar" (Ctrl+Shift+M / Cmd+Opt+M)
3. Select device presets:
   - iPhone SE (375x667)
   - iPhone 12 Pro (390x844)
   - Samsung Galaxy S20 (360x800)

#### What to Check

- [ ] **Layout**
  - Single column layout
  - No horizontal scrolling
  - Content fits viewport width
  - Proper spacing (12-16px padding)

- [ ] **Touch Targets**
  - All buttons ≥44x44px
  - Adequate spacing between buttons (8px minimum)
  - Links easily tappable
  - No accidental taps

- [ ] **Typography**
  - Text readable at default zoom
  - Font sizes appropriate (14px base)
  - Line height comfortable (1.5)

- [ ] **Navigation**
  - Skip link appears on focus
  - Header stacks vertically
  - Footer stacks vertically

- [ ] **Interactive Elements**
  - Buttons full width or properly sized
  - Dropdowns accessible
  - Forms easy to fill
  - Modals display properly

#### Manual Mobile Device Test

If you have a physical device:

1. Navigate to app URL on device
2. Test portrait and landscape orientations
3. Test scrolling performance
4. Test form inputs (keyboard behavior)
5. Test button taps (no mis-taps)

### 2. Tablet Testing (640px - 1024px)

#### Browser DevTools Method

1. Select tablet presets:
   - iPad (768x1024)
   - iPad Pro (1024x1366)
   - Surface Pro 7 (912x1368)

2. Test both portrait and landscape

#### What to Check

- [ ] **Layout**
  - Two-column grids where appropriate
  - Proper use of horizontal space
  - No wasted space
  - Balanced layout

- [ ] **Touch Targets**
  - Still ≥44x44px
  - Good spacing maintained

- [ ] **Typography**
  - Text size appropriate for viewing distance
  - Comfortable reading

- [ ] **Adaptive Features**
  - More content visible than mobile
  - Better use of screen real estate

### 3. Desktop Testing (> 1024px)

#### Test at Multiple Widths

1. 1024px (minimum desktop)
2. 1366px (common laptop)
3. 1920px (Full HD)
4. 2560px+ (large desktop)

#### What to Check

- [ ] **Layout**
  - Multi-column layout (up to 5 columns for node graph)
  - Max width: 1600px (centered)
  - Proper grid layouts
  - No excessive whitespace

- [ ] **Hover States**
  - Buttons change on hover
  - Links underline on hover
  - Tooltips appear on hover
  - Interactive feedback clear

- [ ] **Mouse Interactions**
  - Precise clicking
  - Context menus (if applicable)
  - Drag and drop (if applicable)

- [ ] **Keyboard Navigation**
  - All elements keyboard accessible
  - Focus indicators visible
  - Tab order logical

### 4. Zoom Testing

#### 200% Zoom Test

1. Set browser zoom to 200% (Ctrl/Cmd + +)
2. Navigate through entire app
3. Check for:
   - [ ] No horizontal scrolling
   - [ ] All content visible
   - [ ] Text remains readable
   - [ ] No layout breaks
   - [ ] Buttons still accessible

#### 400% Zoom Test (optional)

1. Set zoom to 400%
2. Verify critical functionality remains accessible

### 5. Orientation Testing

For mobile/tablet devices:

1. Start in portrait mode
2. Rotate to landscape
3. Verify:
   - [ ] Layout adapts properly
   - [ ] No content cut off
   - [ ] Interactive elements still accessible
   - [ ] No layout jank/flickering

## Accessibility Testing

### 1. Keyboard Navigation Testing

#### Basic Navigation

**Test Steps:**
1. Open the app
2. Press Tab (do NOT use mouse)
3. Navigate through ALL interactive elements

**Check:**
- [ ] Skip link appears first
- [ ] Focus indicator clearly visible
- [ ] Tab order is logical (left-to-right, top-to-bottom)
- [ ] All buttons/links reachable
- [ ] No keyboard traps (can always Tab away)
- [ ] Shift+Tab goes backward

#### Keyboard Shortcuts

Test these interactions:

| Element | Keys | Expected Behavior |
|---------|------|-------------------|
| Button | Enter, Space | Activates button |
| Link | Enter | Follows link |
| Dropdown | Arrow keys | Navigate options |
| Dropdown | Escape | Closes dropdown |
| Modal | Escape | Closes modal |
| Skip Link | Tab then Enter | Jumps to main content |

**Checklist:**
- [ ] Enter activates buttons
- [ ] Space activates buttons
- [ ] Arrow keys work in dropdowns
- [ ] Escape closes modals/dropdowns
- [ ] Enter activates links
- [ ] Tab order makes sense

### 2. Focus Indicator Testing

**Test Steps:**
1. Tab through all interactive elements
2. Verify focus indicator on each

**Check:**
- [ ] Focus outline visible (2px solid)
- [ ] Focus outline has sufficient contrast
- [ ] Focus offset provides spacing (2px)
- [ ] Focus not hidden by other elements
- [ ] Custom components have focus styles

**Focus Colors:**
- Primary: Blue outline (--accent)
- High contrast: Stronger outline

### 3. Screen Reader Testing

#### NVDA (Windows)

**Setup:**
1. Download NVDA from nvaccess.org
2. Install and launch
3. Open app in browser

**Navigation Commands:**
- `H` - Navigate by heading
- `B` - Navigate by button
- `F` - Navigate by form field
- `L` - Navigate by link
- `T` - Navigate by table
- `Arrow keys` - Read line by line
- `Ctrl` - Stop reading

**Test Checklist:**
- [ ] Page title announced
- [ ] Headings announced with level (H1, H2, etc.)
- [ ] Links announced with destination
- [ ] Buttons announced with purpose
- [ ] Form fields announced with labels
- [ ] Error messages announced
- [ ] Loading states announced (aria-busy)
- [ ] Dynamic content changes announced

#### VoiceOver (macOS)

**Setup:**
1. Press Cmd+F5 to enable VoiceOver
2. Open app in Safari

**Navigation Commands:**
- `VO + Right Arrow` - Next item
- `VO + Left Arrow` - Previous item
- `VO + H` - Next heading
- `VO + Space` - Activate button/link
- `VO + Shift + Down` - Enter group
- `VO + Shift + Up` - Exit group

**Test Checklist:**
- [ ] Rotor navigation works (headings, links, buttons)
- [ ] All content accessible
- [ ] Proper element roles announced
- [ ] Labels associated with inputs

#### Testing Script

Read this aloud with screen reader active:

1. "Navigate to main heading"
   - Should hear: "Heading level 1, Chimera - React + TypeScript"

2. "Find the welcome heading"
   - Should hear: "Heading level 2, Welcome"

3. "Navigate to the first button"
   - Should hear button purpose and state

4. "Tab to skip link"
   - Should hear: "Link, Skip to main content"

5. "Activate skip link"
   - Should jump to main content

### 4. Color Contrast Testing

#### Using Browser DevTools

**Chrome DevTools Method:**
1. Right-click element
2. Select "Inspect"
3. Look at "Accessibility" pane
4. Check contrast ratio

**Required Ratios:**
- Normal text (< 18px): ≥4.5:1
- Large text (≥ 18px): ≥3:1
- UI components: ≥3:1

#### Using axe DevTools

1. Install axe DevTools extension
2. Open DevTools → axe tab
3. Click "Scan All of My Page"
4. Review "Color Contrast" issues
5. Fix all violations

#### Manual Contrast Check

Use WebAIM Contrast Checker:
1. Go to: https://webaim.org/resources/contrastchecker/
2. Enter foreground color (text)
3. Enter background color
4. Verify pass for WCAG AA

**Test These Combinations:**
- [ ] Body text on background
- [ ] Headings on background
- [ ] Button text on button background
- [ ] Link text on background
- [ ] Muted text on background
- [ ] Success/Warning/Error text

### 5. ARIA Attribute Testing

#### Using Browser DevTools

**Inspect ARIA Attributes:**
1. Right-click element
2. Select "Inspect"
3. Check "Accessibility" pane
4. Verify:
   - Computed role is correct
   - Name is descriptive
   - States are accurate (expanded, selected, etc.)

#### Common ARIA Patterns

**Button:**
```html
<button aria-busy="true">Loading</button>
```
- [ ] Has button role (implicit)
- [ ] aria-busy when loading
- [ ] Not aria-disabled (use disabled attribute)

**Select/Dropdown:**
```html
<div role="combobox" aria-expanded="false" aria-haspopup="listbox">
```
- [ ] Has combobox role
- [ ] aria-expanded reflects state
- [ ] aria-haspopup indicates dropdown

**Panel/Accordion:**
```html
<div role="button" aria-expanded="true">
```
- [ ] Collapsible header has button role
- [ ] aria-expanded reflects state

### 6. Touch Target Testing

#### Visual Inspection

Use browser DevTools to measure:
1. Right-click button → Inspect
2. Check Computed styles
3. Verify min-height and min-width

**Requirements:**
- Minimum size: 44x44px
- Minimum spacing: 8px between targets

#### Physical Device Test

On actual phone/tablet:
1. Try tapping buttons
2. Try tapping links
3. Verify:
   - [ ] No mis-taps
   - [ ] Easy to tap accurately
   - [ ] Adequate spacing prevents accidental taps

### 7. Skip Navigation Testing

**Test Steps:**
1. Load page
2. Press Tab (focus should be on skip link)
3. Verify skip link visible
4. Press Enter
5. Verify focus moves to main content

**Check:**
- [ ] Skip link is first focusable element
- [ ] Skip link visible when focused
- [ ] Skip link text is clear: "Skip to main content"
- [ ] Activating skip link moves focus to main
- [ ] Main content has id="main-content"

## Automated Testing

### Run Test Suite

```bash
# Run all tests
npm test

# Run accessibility tests specifically
npm test -- src-react/tests/accessibility.test.tsx

# Run with coverage
npm test -- --coverage
```

### Lighthouse Audit

1. Open Chrome DevTools
2. Click "Lighthouse" tab
3. Select "Accessibility" category
4. Click "Generate report"
5. Target: ≥90 score

**Common Issues:**
- Missing alt text on images
- Insufficient color contrast
- Missing ARIA attributes
- Missing form labels

### axe DevTools Scan

1. Install axe DevTools extension
2. Open DevTools → axe tab
3. Click "Scan All of My Page"
4. Review violations by severity:
   - Critical (fix immediately)
   - Serious (fix soon)
   - Moderate (fix eventually)
   - Minor (nice to fix)

## Issue Reporting

### When You Find an Issue

**Document:**
1. **Description**: What's wrong?
2. **Steps to Reproduce**: How to see the issue?
3. **Expected Behavior**: What should happen?
4. **Actual Behavior**: What actually happens?
5. **Environment**:
   - Browser & version
   - Device & OS
   - Screen size
   - Zoom level
6. **Screenshots**: Visual evidence
7. **Severity**: Critical/High/Medium/Low

### Issue Template

```markdown
## Accessibility Issue

**Type:** [Keyboard/Screen Reader/Color Contrast/Touch Target]

**Description:**
[Clear description of the issue]

**Steps to Reproduce:**
1. Navigate to [page]
2. [Action]
3. [Observe issue]

**Expected:**
[What should happen]

**Actual:**
[What actually happens]

**Environment:**
- Browser: Chrome 120
- OS: Windows 11
- Screen Reader: NVDA 2023.3
- Device: Desktop

**Severity:** High

**Screenshots:**
[Attach screenshots]
```

## Regression Testing

Before each release:

1. [ ] Run full test suite
2. [ ] Run Lighthouse audit (score ≥90)
3. [ ] Run axe scan (0 violations)
4. [ ] Test keyboard navigation (all elements)
5. [ ] Test screen reader (NVDA or VoiceOver)
6. [ ] Test mobile device (real device)
7. [ ] Test tablet device (real device)
8. [ ] Test at 200% zoom
9. [ ] Test skip navigation
10. [ ] Verify color contrast (all combinations)

## Quick Check (5 minutes)

For rapid validation:

1. **Keyboard Test** (2 min)
   - Tab through page
   - Verify focus indicators
   - Test button activation

2. **Zoom Test** (1 min)
   - Zoom to 200%
   - Check for breaks

3. **Mobile Test** (1 min)
   - Resize to 375px
   - Check layout

4. **Screen Reader Test** (1 min)
   - Turn on screen reader
   - Navigate to heading
   - Navigate to button

5. **axe Scan** (30 sec)
   - Run automated scan
   - Check for critical issues

## Resources

### Testing Tools
- [NVDA](https://www.nvaccess.org/) - Free screen reader (Windows)
- [axe DevTools](https://www.deque.com/axe/devtools/) - Accessibility testing
- [WAVE](https://wave.webaim.org/) - Web accessibility evaluation
- [Lighthouse](https://developers.google.com/web/tools/lighthouse) - Audit tool
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/) - Contrast testing

### Learning Resources
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [WebAIM](https://webaim.org/) - Accessibility tutorials
- [A11y Project](https://www.a11yproject.com/) - Community resources
- [Inclusive Components](https://inclusive-components.design/) - Accessible patterns

## Need Help?

If you're unsure about a test result:
1. Check the documentation (ACCESSIBILITY.md)
2. Review WCAG 2.1 guidelines
3. Ask in team chat
4. File an issue for clarification

Remember: **Accessibility is everyone's responsibility!**
