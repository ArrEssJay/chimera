import { test, expect } from '@playwright/test';

test.describe('Chart Rendering and Axis Labels', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for page to fully load
    await expect(page.locator('h1')).toBeVisible();
  });

  test('should render constellation charts after simulation run', async ({ page }) => {
    // Capture console messages so we can assert that the app generated SVGs
    const logs: string[] = [];
    page.on('console', (msg) => {
      try {
        logs.push(msg.text());
      } catch (e) {
        console.error('Error pushing console message to logs:', e);
      }
    });

  // Click the Run button to execute simulation. Use JS click to avoid
  // accidental overlays intercepting pointer events in headless runs.
  // Capture page console logs so wasm logs show up in the test output
  page.on('console', (msg) => console.log('PAGE LOG>', msg.text()));
  // Use the debug test hook (exposed in dev builds) to trigger the pipeline
  // and inject SVGs directly into the DOM. This keeps the test deterministic
  // and avoids flakiness related to overlay/pointer interception.
  // Wait for the debug hook to be attached by the wasm module (it is added
  // asynchronously during mount). Fail if not attached within 10s.
  // Signal the page to run the test hook after reload, then reload so the
  // hook runs in the fresh page context where DOM injection is stable.
  // Click the Run button via JS and wait for the SVG container to be filled
  await page.evaluate(() => (document.querySelector('button.primary') as HTMLButtonElement | null)?.click());
  await page.waitForFunction(() => !!(document.querySelector('.node')), { timeout: 10000 });
  // Small pause for UI to update, then inspect insertion
  await page.waitForTimeout(500);
  const containerHtmlLen = await page.evaluate(() => {
    const el = document.querySelector('.svg-chart-container');
    if (!el || !el.innerHTML) return 0;
    return el.innerHTML.length;
  });
  console.log('containerHtmlLen', containerHtmlLen);
  const firstSvgOuter = await page.evaluate(() => document.querySelector('.svg-chart-container svg')?.outerHTML?.slice(0,200) || 'NO_SVG');
  console.log('firstSvgOuter (snippet)', firstSvgOuter);
  // Now wait longer for the actual SVG node to appear
  await page.waitForSelector('.svg-chart-container svg', { timeout: 15000 });
    
    // Wait for simulation to complete (check for results)
    await page.waitForSelector('.constellation-panel svg', { timeout: 30000 });
    
    // Check that TX constellation chart has SVG content
    const txConstellation = page.locator('.node').filter({ hasText: 'Transmitter' }).locator('svg');
    await expect(txConstellation).toBeVisible();
    
    // Check that RX constellation chart has SVG content
    const rxConstellation = page.locator('.node').filter({ hasText: 'Receiver' }).locator('svg');
    await expect(rxConstellation).toBeVisible();
    
    // Verify constellation charts have axis labels
    const txSvgContent = await txConstellation.innerHTML();
    expect(txSvgContent).toContain('In-Phase (I)');
    expect(txSvgContent).toContain('Quadrature (Q)');
    
    // CRITICAL: Verify TX constellation actually contains data points (circles)
    expect(txSvgContent).toContain('<circle');
    const txCircleCount = (txSvgContent.match(/<circle/g) || []).length;
    expect(txCircleCount).toBeGreaterThan(0);
    console.log(`TX constellation has ${txCircleCount} circles`);
    
    const rxSvgContent = await rxConstellation.innerHTML();
    expect(rxSvgContent).toContain('In-Phase (I)');
    expect(rxSvgContent).toContain('Quadrature (Q)');
    
    // CRITICAL: Verify RX constellation actually contains data points (circles)
    expect(rxSvgContent).toContain('<circle');
    const rxCircleCount = (rxSvgContent.match(/<circle/g) || []).length;
    expect(rxCircleCount).toBeGreaterThan(0);
    console.log(`RX constellation has ${rxCircleCount} circles`);
    
    // Take a screenshot to visually verify
    await page.screenshot({ path: 'test-results/constellation-charts.png', fullPage: true });

    // Verify console contains our SVG generation log
    const genLog = logs.find((l) => l.includes('Generated SVG for')) || logs.find((l) => l.includes('Generated combined SVG'));
    expect(genLog).toBeTruthy();
  });

  test('should render combined constellation chart with legend', async ({ page }) => {
  // Click the Run button to execute simulation using JS click
  // Wait for and trigger the debug run helper
  await page.evaluate(() => (document.querySelector('button.primary') as HTMLButtonElement | null)?.click());
  await page.waitForFunction(() => {
    const c = document.querySelector('.constellation-combined .svg-chart-container');
    return !!(c && c.innerHTML && c.innerHTML.includes('<svg'));
  }, { timeout: 30000 });
    
    // Wait for combined constellation to render
    await page.waitForSelector('.constellation-combined svg', { timeout: 30000 });
    
    const combinedConstellation = page.locator('.constellation-combined svg');
    await expect(combinedConstellation).toBeVisible();
    
    // Check for legend items
    const svgContent = await combinedConstellation.innerHTML();
    expect(svgContent).toContain('TX Symbols');
    expect(svgContent).toContain('RX Symbols');
    expect(svgContent).toContain('In-Phase (I)');
    expect(svgContent).toContain('Quadrature (Q)');
    
    // CRITICAL: Verify combined constellation actually contains data points (circles)
    expect(svgContent).toContain('<circle');
    const circleCount = (svgContent.match(/<circle/g) || []).length;
    expect(circleCount).toBeGreaterThan(0);
    console.log(`Combined constellation has ${circleCount} circles`);
    
    // Take a screenshot to visually verify
    await page.screenshot({ path: 'test-results/combined-constellation.png', fullPage: true });
  });

  test('should render diagnostics charts with proper axis labels', async ({ page }) => {
  // Click the Run button to execute simulation using JS click
  await page.evaluate(() => (document.querySelector('button.primary') as HTMLButtonElement | null)?.click());
  await page.waitForFunction(() => {
    const c = document.querySelector('.chart-panel .svg-chart-container');
    return !!(c && c.innerHTML && c.innerHTML.includes('<svg'));
  }, { timeout: 30000 });
    
    // Wait for diagnostics section to render
    await page.waitForSelector('.diagnostics-panel .chart-grid', { timeout: 30000 });
    
    // Get all chart panels in the diagnostics section
    const chartPanels = page.locator('.diagnostics-panel .chart-panel');
    const chartCount = await chartPanels.count();
    expect(chartCount).toBe(5); // Should have 5 diagnostic charts
    
    // Check Timing Error chart
    const timingErrorChart = chartPanels.filter({ hasText: 'Timing Error' }).locator('svg');
    await expect(timingErrorChart).toBeVisible();
    const timingErrorSvg = await timingErrorChart.innerHTML();
    expect(timingErrorSvg).toContain('Sample Index');
    expect(timingErrorSvg).toContain('Error (samples)');
    
    // Check NCO Frequency Offset chart
    const ncoChart = chartPanels.filter({ hasText: 'NCO Frequency Offset' }).locator('svg');
    await expect(ncoChart).toBeVisible();
    const ncoSvg = await ncoChart.innerHTML();
    expect(ncoSvg).toContain('Sample Index');
    expect(ncoSvg).toContain('Offset (Hz)');
    
    // Check Clean Signal PSD chart
    const cleanPsdChart = chartPanels.filter({ hasText: 'Clean Signal PSD' }).locator('svg');
    await expect(cleanPsdChart).toBeVisible();
    const cleanPsdSvg = await cleanPsdChart.innerHTML();
    expect(cleanPsdSvg).toContain('Frequency Bin');
    expect(cleanPsdSvg).toContain('Power (dBFS)');
    
    // Check Noisy Signal PSD chart
    const noisyPsdChart = chartPanels.filter({ hasText: 'Noisy Signal PSD' }).locator('svg');
    await expect(noisyPsdChart).toBeVisible();
    const noisyPsdSvg = await noisyPsdChart.innerHTML();
    expect(noisyPsdSvg).toContain('Frequency Bin');
    expect(noisyPsdSvg).toContain('Power (dBFS)');
    
    // Check Running BER chart
    const berChart = chartPanels.filter({ hasText: 'Running BER' }).locator('svg');
    await expect(berChart).toBeVisible();
    const berSvg = await berChart.innerHTML();
    expect(berSvg).toContain('Symbol Index');
    expect(berSvg).toContain('BER');
  });

  test('should verify charts use SVG backend and are saveable', async ({ page }) => {
  // Click the Run button to execute simulation using JS click
  await page.evaluate(() => (document.querySelector('button.primary') as HTMLButtonElement | null)?.click());
  await page.waitForFunction(() => {
    const c = document.querySelector('.chart-panel .svg-chart-container, .svg-chart-container');
    return !!(c && c.innerHTML && c.innerHTML.includes('<svg'));
  }, { timeout: 30000 });
    
    // Wait for any chart to render
    await page.waitForSelector('.chart-panel svg, .constellation-panel svg', { timeout: 30000 });
    
    // Get all SVG elements
    const svgElements = page.locator('svg');
    const svgCount = await svgElements.count();
    
    // Should have multiple SVG charts (at least 7: 2 constellation + 1 combined + 5 diagnostics)
    expect(svgCount).toBeGreaterThanOrEqual(7);
    
    // Verify SVGs have proper viewBox or dimensions (confirms proper rendering)
    const firstSvg = svgElements.first();
    const svgTag = await firstSvg.evaluate((el) => el.outerHTML.substring(0, 100));
    expect(svgTag).toContain('<svg');
  });

  test('should not display duplicate labels or titles', async ({ page }) => {
  // Click the Run button to execute simulation using JS click
  await page.evaluate(() => (document.querySelector('button.primary') as HTMLButtonElement | null)?.click());
  await page.waitForFunction(() => {
    const c = document.querySelector('.chart-panel .svg-chart-container');
    return !!(c && c.innerHTML && c.innerHTML.includes('<svg'));
  }, { timeout: 30000 });
    
    // Wait for diagnostics charts to render
    await page.waitForSelector('.diagnostics-panel .chart-grid', { timeout: 30000 });
    
    // Check PSD charts don't have duplicate unit labels
    const cleanPsdPanel = page.locator('.chart-panel').filter({ hasText: 'Clean Signal PSD' });
    const cleanPsdSvg = await cleanPsdPanel.locator('svg').innerHTML();
    
    // Verify title doesn't contain "(dBFS)" since it's in the y-axis label
    const cleanPsdCaption = cleanPsdSvg.match(/<text[^>]*>Clean Signal PSD[^<]*<\/text>/);
    if (cleanPsdCaption) {
      expect(cleanPsdCaption[0]).not.toContain('(dBFS)');
    }
  });
});
