# Summary of Changes to Fix UI Responsiveness Issue

## Problem Statement

The original implementation had a 300ms auto-preview timeout that would automatically trigger the simulation whenever any parameter changed. This led to an unresponsive UI, especially during rapid parameter adjustments or when making multiple changes.

## Solution

Removed the automatic simulation trigger and implemented manual simulation execution with visual feedback for pending changes.

## Changes Made

### 1. Removed Auto-Preview Mechanism (`chimera-web/src/ui.rs`)

**Before:**
- Used `use_effect_with` hook to watch for simulation input changes
- Created a 300ms timeout on every change
- Automatically ran simulation after timeout expired

**After:**
- Removed the entire auto-preview effect and timeout logic
- Removed `gloo_timers::callback::Timeout` import (no longer needed)
- Simulation now only runs when user explicitly clicks "Run Now" button

### 2. Added Change Detection State (`chimera-web/src/ui.rs`)

**New state variable:**
```rust
let last_run_input = use_state(|| None::<SimulationInput>);
```

**Change detection logic:**
```rust
let has_pending_changes = (*last_run_input).as_ref() != Some(&current_input);
```

This compares the current simulation input with the last executed input to determine if there are pending changes.

### 3. Updated Run Button Callback (`chimera-web/src/ui.rs`)

**Enhancement:**
- Now tracks the input that was last run
- Sets `last_run_input` state after simulation completes
- This enables proper change detection for future edits

### 4. Enhanced UI Visual Feedback (`chimera-web/src/ui.rs`)

**Badge States:**
- "Running…" - while simulation is executing
- "Changes pending" - when user has made changes but hasn't run simulation
- "Up to date" - when displayed results match current parameters

**Button Highlighting:**
- Run button gets `highlight` CSS class when there are pending changes
- Provides clear visual cue that user needs to run simulation

### 5. Updated UI Text (`chimera-web/src/ui.rs`)

**Panel header description:**
- **Before:** "Configure presets and channel parameters; the dashboard re-runs automatically after edits."
- **After:** "Configure presets and channel parameters, then click \"Run Now\" to execute the simulation."

**Control hint:**
- **Before:** "Changes trigger an auto-preview after 300 ms. Use \"Run Now\" to force an immediate rebuild."
- **After:** "Click \"Run Now\" to execute the simulation with the current parameters."

**Telemetry message:**
- **Before:** "Auto-preview will populate telemetry after the first run."
- **After:** "Run the simulation to populate telemetry data."

### 6. Added CSS Styling (`chimera-web/style.css`)

**New `.badge-pending` class:**
```css
.badge-pending {
  padding: 0.3rem 0.8rem;
  border-radius: 999px;
  border: 1px solid lch(70% 50 60 / 0.5);
  background: lch(60% 50 60 / 0.35);
  color: lch(90% 30 60);
  /* styling properties */
}
```

**New `.primary.highlight` class with animation:**
```css
button.primary.highlight {
  animation: pulse-highlight 1.5s ease-in-out infinite;
  box-shadow: 0 14px 42px var(--accent-glow), 0 0 30px lch(70% 50 60 / 0.4);
}

@keyframes pulse-highlight {
  0%, 100% {
    box-shadow: 0 14px 42px var(--accent-glow), 0 0 30px lch(70% 50 60 / 0.4);
  }
  50% {
    box-shadow: 0 14px 48px var(--accent-glow), 0 0 40px lch(70% 50 60 / 0.6);
  }
}
```

### 7. Added Tests (`chimera-web/tests/pipeline.rs`)

**New test:** `simulation_input_equality_for_change_detection`
- Verifies that `SimulationInput` equality works correctly
- Tests that changes to plaintext, SNR, and preset are properly detected
- Essential for the change detection mechanism to work

### 8. Created Playwright Testing Documentation

**New file:** `chimera-web/PLAYWRIGHT_TESTING.md`
- Comprehensive guide for setting up Playwright E2E tests
- Example test cases covering:
  - No auto-run verification
  - Change detection
  - Button highlighting
  - UI responsiveness
  - State transitions
- CI integration instructions

## Benefits

1. **Improved UI Responsiveness:** Users can make multiple rapid changes without triggering repeated simulations
2. **Better User Control:** Simulations only run when explicitly requested
3. **Clear Visual Feedback:** Users always know if their current parameters match the displayed results
4. **Better User Experience:** Highlighted button draws attention when action is needed
5. **Performance:** Reduces unnecessary computation by not running on every parameter change

## Testing

All existing tests pass:
- Core library tests: 7 passed
- Acceptance tests: 6 passed  
- Web tests: 4 passed (including new change detection test)

## Migration Notes

- Users will need to click "Run Now" to execute simulations
- This is a behavior change, but improves the overall experience
- No breaking API changes in the code structure
