import { test } from '@playwright/test';

test('visual inspection of constellation charts', async ({ page }) => {
  await page.goto('/');
  
  // Run simulation
  await page.evaluate(() => (document.querySelector('button.primary') as HTMLButtonElement | null)?.click());
  
  // Wait for charts
  await page.waitForSelector('.constellation-panel svg', { timeout: 30000 });
  
  // Wait a bit for rendering
  await page.waitForTimeout(2000);
  
  // Take full page screenshot
  await page.screenshot({ path: 'test-results/visual-full-page.png', fullPage: true });
  
  // Take screenshot of just TX constellation
  const txPanel = page.locator('.node').filter({ hasText: 'Transmitter' });
  await txPanel.screenshot({ path: 'test-results/visual-tx-constellation.png' });
  
  // Take screenshot of just RX constellation  
  const rxPanel = page.locator('.node').filter({ hasText: 'Receiver' });
  await rxPanel.screenshot({ path: 'test-results/visual-rx-constellation.png' });
  
  // Log the SVG content for debugging
  const txSvg = await txPanel.locator('svg').innerHTML();
  console.log('TX SVG length:', txSvg.length);
  console.log('TX SVG circles:', (txSvg.match(/<circle/g) || []).length);
  console.log('TX SVG sample:', txSvg.substring(0, 1000));
  
  // Keep browser open for 10 seconds for manual inspection
  await page.waitForTimeout(10000);
});
