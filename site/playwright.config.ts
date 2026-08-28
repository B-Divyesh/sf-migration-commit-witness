import { defineConfig, devices } from '@playwright/test';

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? 'http://127.0.0.1:4173';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  reporter: 'line',
  use: {
    baseURL,
    trace: 'retain-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'mobile', use: { ...devices['iPhone 13'], browserName: 'chromium', viewport: { width: 390, height: 844 } } },
  ],
  webServer: process.env.PLAYWRIGHT_BASE_URL ? undefined : {
    command: 'npm run preview',
    url: 'http://127.0.0.1:4173',
    reuseExistingServer: true,
    stdout: 'pipe',
  },
});
