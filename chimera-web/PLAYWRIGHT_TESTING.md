# Playwright Testing for Chimera Web

## Overview

This document outlines the approach for adding Playwright end-to-end tests to ensure UI responsiveness and proper operation of controls in the Chimera web dashboard.

## Prerequisites

To run Playwright tests, you'll need:

```bash
# Install Node.js and npm (if not already installed)
# Then install Playwright
npm init -y
npm install --save-dev @playwright/test
npx playwright install
```

## Test Structure

Create a `tests/e2e` directory for Playwright tests:

```
chimera-web/
  tests/
    e2e/
      playwright.config.ts
      ui-responsiveness.spec.ts
```

## Example Test Cases

### playwright.config.ts

```typescript
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://127.0.0.1:8080',
    trace: 'on-first-retry',
  },
  webServer: {
    command: 'trunk serve',
    url: 'http://127.0.0.1:8080',
    reuseExistingServer: !process.env.CI,
  },
});
```

### ui-responsiveness.spec.ts

```typescript
import { test, expect } from '@playwright/test';

test.describe('UI Responsiveness', () => {
  test('should load dashboard without auto-running simulation', async ({ page }) => {
    await page.goto('/');
    
    // Wait for page to load
    await expect(page.locator('h1')).toHaveText('Simulation Controls');
    
    // Check that initial state shows no pending changes
    const badge = page.locator('.badge-live.idle');
    await expect(badge).toBeVisible();
    await expect(badge).toHaveText('Up to date');
    
    // Run button should not be highlighted initially
    const runButton = page.locator('button.primary');
    await expect(runButton).not.toHaveClass(/highlight/);
  });

  test('should detect pending changes when user modifies plaintext', async ({ page }) => {
    await page.goto('/');
    
    // Find and modify the plaintext textarea
    const textarea = page.locator('textarea');
    await textarea.fill('Test message with changes');
    
    // Badge should change to show pending changes
    const pendingBadge = page.locator('.badge-pending');
    await expect(pendingBadge).toBeVisible();
    await expect(pendingBadge).toHaveText('Changes pending');
    
    // Run button should be highlighted
    const runButton = page.locator('button.primary');
    await expect(runButton).toHaveClass(/highlight/);
  });

  test('should detect pending changes when user modifies SNR', async ({ page }) => {
    await page.goto('/');
    
    // Find and modify the SNR input
    const snrInput = page.locator('input[type="range"]');
    await snrInput.fill('5');
    
    // Badge should show pending changes
    const pendingBadge = page.locator('.badge-pending');
    await expect(pendingBadge).toBeVisible();
    
    // Run button should be highlighted
    const runButton = page.locator('button.primary');
    await expect(runButton).toHaveClass(/highlight/);
  });

  test('should detect pending changes when user changes preset', async ({ page }) => {
    await page.goto('/');
    
    // Change preset
    const presetSelect = page.locator('select');
    await presetSelect.selectOption({ index: 1 });
    
    // Badge should show pending changes
    const pendingBadge = page.locator('.badge-pending');
    await expect(pendingBadge).toBeVisible();
  });

  test('should run simulation only when Run Now button is clicked', async ({ page }) => {
    await page.goto('/');
    
    // Modify plaintext
    const textarea = page.locator('textarea');
    await textarea.fill('Manual test message');
    
    // Wait a bit to ensure no auto-run happens
    await page.waitForTimeout(500);
    
    // Badge should still show pending changes (not running)
    const pendingBadge = page.locator('.badge-pending');
    await expect(pendingBadge).toBeVisible();
    
    // Click Run Now button
    const runButton = page.locator('button.primary');
    await runButton.click();
    
    // Badge should change to "Running…"
    const runningBadge = page.locator('.badge-live').filter({ hasText: 'Running…' });
    await expect(runningBadge).toBeVisible();
    
    // Wait for simulation to complete (timeout after 10 seconds)
    await expect(runningBadge).not.toBeVisible({ timeout: 10000 });
    
    // After completion, should show "Up to date"
    const upToDateBadge = page.locator('.badge-live.idle');
    await expect(upToDateBadge).toBeVisible();
    await expect(upToDateBadge).toHaveText('Up to date');
  });

  test('should remain responsive during control interactions', async ({ page }) => {
    await page.goto('/');
    
    // Rapidly change multiple controls
    const textarea = page.locator('textarea');
    const snrInput = page.locator('input[type="range"]');
    
    for (let i = 0; i < 5; i++) {
      await textarea.fill(`Message ${i}`);
      await snrInput.fill(`${i}`);
    }
    
    // UI should remain responsive - button should still be clickable
    const runButton = page.locator('button.primary');
    await expect(runButton).toBeEnabled();
    
    // Badge should show pending changes
    const pendingBadge = page.locator('.badge-pending');
    await expect(pendingBadge).toBeVisible();
  });

  test('should not auto-run after 300ms delay', async ({ page }) => {
    await page.goto('/');
    
    // Wait for initial load to complete
    await page.waitForLoadState('networkidle');
    
    // Modify plaintext
    const textarea = page.locator('textarea');
    await textarea.fill('Testing no auto-run');
    
    // Wait for more than 300ms
    await page.waitForTimeout(500);
    
    // Badge should still show pending changes (not running or completed)
    const pendingBadge = page.locator('.badge-pending');
    await expect(pendingBadge).toBeVisible();
    
    // No network requests should have been made for simulation
    // (This would require more sophisticated network monitoring)
  });
});
```

## Running the Tests

```bash
# Start the dev server (in one terminal)
cd chimera-web
trunk serve

# Run Playwright tests (in another terminal)
npx playwright test

# Run with UI mode for debugging
npx playwright test --ui

# Run specific test
npx playwright test ui-responsiveness.spec.ts
```

## CI Integration

Add to GitHub Actions workflow:

```yaml
- name: Install Playwright
  run: |
    cd chimera-web
    npm install --save-dev @playwright/test
    npx playwright install --with-deps

- name: Run Playwright tests
  run: |
    cd chimera-web
    npx playwright test
```

## Key Test Scenarios

1. **No Auto-Run**: Verify that changing controls does not automatically trigger simulation
2. **Change Detection**: Verify that UI correctly indicates when there are pending changes
3. **Button Highlight**: Verify that Run Now button is highlighted when there are changes
4. **Manual Run**: Verify that simulation only runs when user clicks Run Now
5. **UI Responsiveness**: Verify that rapid control changes don't lock up the UI
6. **State Transitions**: Verify proper badge transitions (Up to date → Changes pending → Running → Up to date)

## Implementation Notes

- These tests require a running instance of the web application (via `trunk serve`)
- Tests should be run in CI to catch regressions
- Consider adding visual regression testing for the button highlight animation
- Monitor test execution time to ensure tests remain fast
