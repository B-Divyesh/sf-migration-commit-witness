import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

describe('Azure Static Web Apps response policy', () => {
  it('ships native cache and security headers for production', () => {
    const config = JSON.parse(readFileSync('site/public/staticwebapp.config.json', 'utf8')) as {
      globalHeaders: Record<string, string>;
      routes: Array<{ route: string; headers: Record<string, string> }>;
      responseOverrides: Record<string, { rewrite: string; statusCode: number }>;
    };
    const route = (path: string) => config.routes.find((item) => item.route === path)?.headers['Cache-Control'];

    expect(route('/assets/*')).toBe('public, max-age=31536000, immutable');
    expect(route('/witness-core.webp')).toBe('public, max-age=31536000, immutable');
    expect(route('/mcw-demo-recording.svg')).toBe('public, max-age=31536000, immutable');
    expect(route('/sw.js')).toBe('no-cache');
    expect(config.globalHeaders['Content-Security-Policy']).toContain("default-src 'self'");
    expect(config.globalHeaders['Permissions-Policy']).toContain('payment=()');
    expect(config.responseOverrides['404']).toEqual({ rewrite: '/404.html', statusCode: 404 });
    expect(readFileSync('site/public/sitemap.xml', 'utf8')).toContain('/demo/');
  });
});
