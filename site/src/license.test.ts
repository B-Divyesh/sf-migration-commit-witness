import { describe, expect, it } from 'vitest';
import { isFreshValidVerdict, parseCachedVerdict, tokenFingerprint, verificationUrl } from './license';

describe('paid unlock helpers', () => {
  it('builds the product-scoped verification URL safely', () => {
    expect(verificationUrl('token + /')).toContain('/products/migration-commit-witness/verify?license=token%20%2B%20%2F');
  });

  it('accepts only a fresh matching cached verdict', () => {
    const token = 'license-a';
    const cached = { valid: true, reason: 'ok' as const, checkedAt: 1000, tokenFingerprint: tokenFingerprint(token) };
    expect(isFreshValidVerdict(cached, token, 2000)).toBe(true);
    expect(isFreshValidVerdict(cached, 'license-b', 2000)).toBe(false);
    expect(isFreshValidVerdict(cached, token, 90_000_000)).toBe(false);
  });

  it('does not trust malformed storage', () => {
    expect(parseCachedVerdict('{oops')).toBeNull();
    expect(parseCachedVerdict('{"valid":true}')).toBeNull();
  });
});
