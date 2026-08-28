import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('home is semantic, keyboard-operable, and free of serious axe findings', async ({ page }) => {
  const consoleErrors: string[] = [];
  page.on('console', (message) => { if (message.type() === 'error') consoleErrors.push(message.text()); });
  await page.goto('/');
  await expect(page).toHaveTitle(/Migration Commit Witness/);
  await expect(page.locator('html')).toHaveAttribute('lang', 'en');
  await expect(page.locator('main')).toHaveCount(1);
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.getByRole('heading', { level: 1 })).toContainText('Your migration said');

  const secondTab = page.getByRole('tab', { name: '2. Command' });
  await page.getByRole('tab', { name: '1. Before' }).focus();
  await page.keyboard.press('ArrowRight');
  await expect(secondTab).toHaveAttribute('aria-selected', 'true');
  await expect(page.getByText('Exit 0', { exact: true })).toBeVisible();

  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((item) => ['serious', 'critical'].includes(item.impact ?? ''))).toEqual([]);
  expect(consoleErrors).toEqual([]);
});

test('license return is stored, stripped, verified, and unlocks the team kit', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/migration-commit-witness/verify?*', async (route) => {
    await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null }) });
  });
  await page.goto('/?license=test-license-token#pricing');
  await expect(page).toHaveURL(/\/#pricing$/);
  await expect(page.getByRole('button', { name: 'Download team rollout kit' })).toBeEnabled();
  const token = await page.evaluate(() => localStorage.getItem('sb_license:migration-commit-witness'));
  expect(token).toBe('test-license-token');
});

test('legal pages have one heading and pass serious axe checks', async ({ page }) => {
  for (const path of ['/privacy/', '/terms/']) {
    await page.goto(path);
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((item) => ['serious', 'critical'].includes(item.impact ?? ''))).toEqual([]);
  }
});

test('mobile layout has no horizontal document overflow', async ({ page, isMobile }) => {
  test.skip(!isMobile, 'mobile project only');
  await page.goto('/');
  const dimensions = await page.evaluate(() => ({ scroll: document.documentElement.scrollWidth, client: document.documentElement.clientWidth }));
  expect(dimensions.scroll).toBeLessThanOrEqual(dimensions.client + 1);
  await expect(page.locator('a.button-primary[href="#install"]').first()).toBeVisible();
});
