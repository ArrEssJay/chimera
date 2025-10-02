# Visual Changes Guide

This document illustrates the user-visible changes made to fix the UI responsiveness issue.

## UI State Flow

### Before (Auto-preview)

```
User edits parameter
    ↓
300ms timeout starts
    ↓
Timeout expires
    ↓
Simulation runs automatically
    ↓
UI shows "Updating…"
    ↓
Results update
```

**Problem:** Rapid changes would queue up multiple simulations, making UI unresponsive.

### After (Manual execution)

```
User edits parameter
    ↓
UI shows "Changes pending"
Button highlights (pulsing animation)
    ↓
User clicks "Run Now"
    ↓
UI shows "Running…"
    ↓
Results update
    ↓
UI shows "Up to date"
```

**Benefit:** User has full control; UI remains responsive during edits.

## Badge States

### 1. Initial Load / Up to Date
```
┌─────────────────┐
│ ⚪ Up to date   │
└─────────────────┘
```
- Gray background
- Subtle styling
- Indicates results match current parameters

### 2. Changes Pending
```
┌─────────────────────┐
│ 🟡 Changes pending │
└─────────────────────┘
```
- Amber/orange background  
- Brighter color
- Indicates user needs to run simulation

### 3. Running
```
┌──────────────┐
│ 🔵 Running… │
└──────────────┘
```
- Blue background
- Indicates simulation in progress
- Button disabled during this state

## Button States

### Normal State (No Changes)
```
┌─────────────┐
│  Run Now    │  ← Standard button styling
└─────────────┘
```

### Highlighted State (Changes Pending)
```
┌─────────────┐
│  Run Now    │  ← Pulsing glow animation
└─────────────┘
   ~~~~ ~~~~      Animated highlight
```
- Stronger box shadow
- Pulsing animation (1.5s cycle)
- Draws user attention

### Disabled State (Running)
```
┌─────────────┐
│  Running…   │  ← Faded appearance
└─────────────┘
```
- Reduced opacity
- Not clickable
- Shows progress

## Text Changes

### Panel Header

**Before:**
> Configure presets and channel parameters; the dashboard re-runs automatically after edits.

**After:**
> Configure presets and channel parameters, then click "Run Now" to execute the simulation.

### Control Hint

**Before:**
> Changes trigger an auto-preview after 300 ms. Use "Run Now" to force an immediate rebuild.

**After:**
> Click "Run Now" to execute the simulation with the current parameters.

### Empty Results Message

**Before:**
> Auto-preview will populate telemetry after the first run.

**After:**
> Run the simulation to populate telemetry data.

## User Interaction Flow Examples

### Example 1: Adjusting SNR

```
1. User moves SNR slider
   → Badge changes to "Changes pending" 🟡
   → Button highlights with pulsing glow

2. User adjusts more parameters
   → Badge remains "Changes pending" 🟡
   → No automatic simulation runs
   → UI stays responsive

3. User clicks "Run Now"
   → Badge changes to "Running…" 🔵
   → Button shows "Running…" and disables
   
4. Simulation completes
   → Badge changes to "Up to date" ⚪
   → Button returns to normal "Run Now"
   → Results update with new data
```

### Example 2: Changing Preset

```
1. User selects different preset
   → Badge changes to "Changes pending" 🟡
   → Button highlights
   → Preset defaults load (plaintext, SNR)
   
2. User clicks "Run Now"
   → Simulation executes with new preset
   → Results show new configuration
   → Badge shows "Up to date" ⚪
```

### Example 3: Typing Message

```
1. User types in plaintext area
   → Each keystroke updates state
   → Badge shows "Changes pending" 🟡
   → Button highlights
   → No simulation runs (UI remains responsive!)
   
2. User finishes typing
   → State still shows pending
   → User decides when to run
   
3. User clicks "Run Now"
   → Simulation runs with complete message
   → Single execution instead of many
```

## CSS Animation Details

### Pulse Highlight Animation

```css
@keyframes pulse-highlight {
  0%, 100% {
    box-shadow: 
      0 14px 42px var(--accent-glow), 
      0 0 30px lch(70% 50 60 / 0.4);
  }
  50% {
    box-shadow: 
      0 14px 48px var(--accent-glow), 
      0 0 40px lch(70% 50 60 / 0.6);
  }
}
```

- Duration: 1.5 seconds
- Timing: ease-in-out
- Loop: infinite
- Effect: Subtle breathing glow

### Color Palette

**Badge Colors (LCH color space):**
- Up to date: `lch(16% 12 260 / 0.85)` - Dark blue-gray
- Changes pending: `lch(60% 50 60 / 0.35)` - Warm amber
- Running: `lch(58% 40 260 / 0.35)` - Blue

**Button Highlight:**
- Base glow: `lch(75% 40 260 / 0.3)` - Cool blue
- Accent glow: `lch(70% 50 60 / 0.4→0.6)` - Warm accent

## Accessibility Considerations

1. **Color Independence:** State is indicated by both color AND text
2. **Clear Labels:** Badge text explicitly states current state
3. **Disabled State:** Button disabled during execution prevents confusion
4. **Animation:** Subtle pulse doesn't interfere with readability
5. **Timing:** No forced automatic actions; user controls when to act

## Performance Benefits

**Before (Auto-preview):**
- Typing "Hello" → 5 simulations triggered
- Slider adjustment → Continuous simulation runs
- Multiple parameter changes → Queue of simulations

**After (Manual execution):**
- Any number of edits → 0 simulations until user clicks
- User clicks "Run Now" → 1 simulation
- Clear performance improvement

## Browser Compatibility

The implementation uses modern web features:
- CSS animations (widely supported)
- LCH color space (fallback to RGB in older browsers)
- Standard HTML5 elements
- No vendor prefixes needed for target browsers

## Future Enhancements

Possible additions (not in current scope):
- Keyboard shortcut for "Run Now" (e.g., Cmd/Ctrl+Enter)
- Progress indicator showing simulation percentage
- Option to auto-save parameters to localStorage
- "Cancel" button to stop long-running simulations
