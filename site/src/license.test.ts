import { describe, expect, it } from 'vitest';
import { isFreshVerdict, parseCachedVerdict, tokenFingerprint, verificationUrl } from './license';

describe('paid unlock helpers', () => {
  it('builds the product-scoped verification URL safely', () => {
    expect(verificationUrl('token + /')).toContain('/products/migration-commit-witness/verify?license=token%20%2B%20%2F');
  });

  it('accepts any fresh matching cached verdict, including an invalid result', () => {
    const token = 'license-a';
    const cached = { valid: true, reason: 'ok' as const, checkedAt: 1000, tokenFingerprint: tokenFingerprint(token) };
    const invalid = { ...cached, valid: false, reason: 'invalid' as const };
    expect(isFreshVerdict(cached, token, 2000)).toBe(true);
    expect(isFreshVerdict(invalid, token, 2000)).toBe(true);
    expect(isFreshVerdict(cached, 'license-b', 2000)).toBe(false);
    expect(isFreshVerdict(cached, token, 90_000_000)).toBe(false);
  });

  it('does not trust malformed storage', () => {
    expect(parseCachedVerdict('{oops')).toBeNull();
    expect(parseCachedVerdict('{"valid":true}')).toBeNull();
  });
});
