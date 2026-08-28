import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

const serious = async (page: Parameters<typeof AxeBuilder>[0]['page']) => {
  const results = await new AxeBuilder({ page }).analyze();
  return results.violations.filter((item) => ['serious', 'critical'].includes(item.impact ?? ''));
};

test('home first screen names the job, audience, and sample action', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('Migration Commit Witness — prove SQL migration state');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Prove what your SQL migration committed');
  await expect(page.getByText('For backend teams reviewing migrations')).toBeVisible();
  const action = page.getByRole('link', { name: 'Try it with sample data' });
  await expect(action).toBeVisible();
  const box = await action.boundingBox();
  expect(box && box.y + box.height).toBeLessThanOrEqual(page.viewportSize()!.height);
});

test('first home load stays same-origin and writes no browser data', async ({ page }) => {
  const origins = new Set<string>();
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  expect([...origins]).toEqual([new URL(page.url()).origin]);
  expect(await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length }))).toEqual({ local: 0, session: 0 });
});

test('@claim:demo-route-isolation one click enters the demo namespace and reset clears only demo state', async ({ page }) => {
  await page.goto('/?demo=1');
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await expect(page.locator('#stage-verdict')).toHaveText('Partial commit detected');
  expect(await page.evaluate(() => Object.keys(sessionStorage))).toEqual(['demo:mcw:stage']);
  await page.evaluate(() => sessionStorage.setItem('real:data', 'keep'));
  await page.getByRole('button', { name: 'Show next observation' }).click();
  await page.getByRole('button', { name: 'Reset demo' }).click();
  expect(await page.evaluate(() => ({ real: sessionStorage.getItem('real:data'), demo: sessionStorage.getItem('demo:mcw:stage') }))).toEqual({ real: 'keep', demo: '2' });
});

test('@claim:browser-demo-privacy demo uses same-origin requests and only demo session storage', async ({ page }) => {
  const origins = new Set<string>();
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  await page.goto('/demo/');
  await page.waitForLoadState('networkidle');
  expect([...origins]).toEqual([new URL(page.url()).origin]);
  expect(await page.evaluate(() => ({ local: Object.keys(localStorage), session: Object.keys(sessionStorage) }))).toEqual({ local: [], session: ['demo:mcw:stage'] });
});

test('@claim:offline-demo the demo reloads and resets offline after one online visit', async ({ page, context }) => {
  await page.goto('/demo/');
  await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();
    if (!navigator.serviceWorker.controller) await new Promise<void>((resolve) => navigator.serviceWorker.addEventListener('controllerchange', () => resolve(), { once: true }));
  });
  await page.reload();
  await expect(page.locator('#stage-verdict')).toHaveText('Partial commit detected');
  await context.setOffline(true);
  await page.reload();
  await expect(page).toHaveTitle('Demo — Migration Commit Witness');
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page.locator('#stage-verdict')).toHaveText('Partial commit detected');
  await context.setOffline(false);
});

test('demo tabs support arrow keys and visible focus', async ({ page }) => {
  await page.goto('/demo/');
  const before = page.getByRole('tab', { name: '1. Before' });
  await before.focus();
  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('tab', { name: '2. Command' })).toBeFocused();
  await expect(page.locator('#stage-verdict')).toHaveText('Command returned 0');
  await expect(page.getByRole('tab', { name: '2. Command' })).toHaveCSS('outline-style', 'solid');
});

test('routes have distinct metadata, focused headings, and working back navigation', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page).toHaveTitle('Demo — Migration Commit Witness');
  await expect(page.locator('link[rel=canonical]')).toHaveAttribute('href', /\/demo\/$/);
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
  await page.goBack();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
  await page.goForward();
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
});

test('unknown paths return the designed 404 with a way home', async ({ page }) => {
  const response = await page.goto('/does-not-exist');
  expect(response?.status()).toBe(404);
  await expect(page).toHaveTitle('Not found — Migration Commit Witness');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('This route has no witness');
  await expect(page.getByRole('link', { name: 'Return to the home page' })).toHaveAttribute('href', '/');
});

test('all routes have metadata, one h1, one main, and no serious axe findings', async ({ page }) => {
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/does-not-exist']) {
    const errors: string[] = [];
    page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
    await page.goto(path);
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', /share\.webp$/);
    await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute('content', 'summary_large_image');
    await expect(page.locator('link[rel="apple-touch-icon"]')).toHaveAttribute('href', '/apple-touch-icon.png');
    expect(await serious(page)).toEqual([]);
    if (path !== '/does-not-exist') expect(errors).toEqual([]);
  }
});

test('skip link and every visible target meet keyboard and 44px geometry rules', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to main content' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('#main')).toBeFocused();
  for (const path of ['/', '/demo/', '/privacy/', '/terms/']) {
    await page.goto(path);
    const undersized = await page.locator('a:visible, button:visible').evaluateAll((nodes) => nodes.map((node) => {
      const box = node.getBoundingClientRect();
      return { text: node.textContent?.trim(), width: box.width, height: box.height };
    }).filter((box) => box.width < 44 || box.height < 44));
    expect(undersized).toEqual([]);
  }
});

test('mobile pages have no document overflow and retain the full demo controls', async ({ page }) => {
  test.skip(page.viewportSize()?.width !== 390, 'mobile project only');
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/does-not-exist']) {
    await page.goto(path);
    const dimensions = await page.evaluate(() => ({ scroll: document.documentElement.scrollWidth, client: document.documentElement.clientWidth }));
    expect(dimensions.scroll).toBeLessThanOrEqual(dimensions.client + 1);
  }
  await page.goto('/demo/');
  await expect(page.getByRole('button', { name: 'Reset demo' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Start for real' })).toBeVisible();
});

test('reduced motion removes meaningful transitions', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  expect(parseFloat(await page.getByRole('link', { name: 'Try it with sample data' }).evaluate((node) => getComputedStyle(node).transitionDuration))).toBeLessThan(0.001);
});
