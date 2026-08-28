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

test('skip navigation and focus treatment work from the keyboard', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  const skipLink = page.getByRole('link', { name: 'Skip to main content' });
  await expect(skipLink).toBeFocused();
  await expect(skipLink).toHaveCSS('outline-style', 'solid');
  await page.keyboard.press('Enter');
  await expect(page.locator('#main')).toBeFocused();
});

test('first load is local-only and writes no browser storage', async ({ page }) => {
  const origins = new Set<string>();
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  expect([...origins]).toEqual([new URL(page.url()).origin]);
  expect(await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length }))).toEqual({ local: 0, session: 0 });
});

test('service worker updates and keeps the mobile shell available offline', async ({ page, context }) => {
  test.skip(page.viewportSize()?.width !== 390, '390px offline/update contract');
  await page.goto('/');
  await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();
    if (!navigator.serviceWorker.controller) {
      await new Promise<void>((resolve) => navigator.serviceWorker.addEventListener('controllerchange', () => resolve(), { once: true }));
    }
  });
  await page.reload();
  await context.setOffline(true);
  await expect(page.getByText('Offline copy')).toBeVisible();
  await page.reload();
  await expect(page).toHaveTitle(/Migration Commit Witness/);
  await expect(page.locator('main')).toBeVisible();
  await expect(page.getByText('Offline copy')).toBeVisible();
  await context.setOffline(false);
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

test('a fresh invalid license verdict is reused for 24 hours', async ({ page }) => {
  let verifyRequests = 0;
  await page.route('https://api.sociobot.in/api/v1/products/migration-commit-witness/verify?*', async (route) => {
    verifyRequests += 1;
    await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ valid: false, reason: 'invalid', expires_at: null }) });
  });
  await page.goto('/');
  await page.getByLabel('License token').fill('invalid-license-token');
  await page.getByRole('button', { name: 'Verify license' }).click();
  await expect(page.getByRole('status').filter({ hasText: 'License no longer active (invalid)' })).toBeVisible();
  expect(verifyRequests).toBe(1);

  await page.reload();
  await expect(page.getByRole('status').filter({ hasText: 'License no longer active (invalid)' })).toBeVisible();
  expect(verifyRequests).toBe(1);
});

test('policy and inline legal links meet the 44px touch-target contract', async ({ page }) => {
  await page.goto('/');
  for (const link of [
    page.getByRole('link', { name: 'Read the policy reference' }),
    page.locator('.legal-line').getByRole('link', { name: 'privacy' }),
    page.locator('.legal-line').getByRole('link', { name: 'terms' }),
  ]) {
    const box = await link.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);
  }
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
