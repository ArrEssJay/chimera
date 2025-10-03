# Chimera Web End-to-End Tests

This directory contains Playwright end-to-end tests for the Chimera Web UI.

## Purpose

These tests verify that:
- Constellation diagrams render correctly with proper axis labels (In-Phase (I) and Quadrature (Q))
- Combined constellation chart includes TX and RX symbols with legend
- Diagnostic charts (Timing Error, NCO Frequency Offset, PSD charts, Running BER) render with proper axis labels
- Charts use SVG backend and are scalable/saveable
- No duplicate labels appear in chart titles and axis labels

## Prerequisites

1. Install Node.js (v18 or higher)
2. Install dependencies:
   ```bash
   cd chimera-web
   npm install
   ```
3. Install Playwright browsers:
   ```bash
   npx playwright install
   ```
4. Install Trunk for serving the web app:
   ```bash
   cargo install trunk
   ```

## Running Tests

### Run all tests (headless)
```bash
npm test
```

### Run tests in headed mode (see the browser)
```bash
npm run test:headed
```

### Run tests in UI mode (interactive)
```bash
npm run test:ui
```

### View test report
```bash
npm run test:report
```

## Test Structure

- `charts.spec.ts` - Tests for chart rendering and axis labels
  - Verifies constellation charts render after simulation
  - Checks combined constellation has TX/RX legend
  - Validates all 5 diagnostic charts have correct axis labels
  - Confirms SVG backend is used
  - Ensures no duplicate labels

## How It Works

1. Playwright starts a local Trunk dev server (`trunk serve` on port 8080)
2. Tests navigate to the web app
3. Tests click the "Run" button to execute a simulation
4. Tests wait for charts to render (up to 30 seconds)
5. Tests verify SVG elements contain expected axis labels and content

## Debugging Failed Tests

If tests fail:
1. Check the screenshots in `test-results/` directory
2. Run with `--headed` flag to see what's happening
3. Use `--debug` flag to step through tests: `npx playwright test --debug`
4. View the HTML report: `npm run test:report`

## CI Integration

These tests can be run in CI by:
1. Installing Rust, Trunk, Node.js, and Playwright
2. Building the WASM bundle
3. Running `npm test`

Example GitHub Actions step:
```yaml
- name: Run Playwright tests
  run: |
    cd chimera-web
    npm ci
    npx playwright install --with-deps chromium
    npm test
```
