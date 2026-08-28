export const PRODUCT_SLUG = 'migration-commit-witness';
export const LICENSE_KEY = `sb_license:${PRODUCT_SLUG}`;
export const VERDICT_KEY = `${LICENSE_KEY}:verdict`;
export const VERIFY_INTERVAL_MS = 24 * 60 * 60 * 1000;
export const API_BASE = 'https://api.sociobot.in/api/v1';

export interface LicenseVerdict {
  valid: boolean;
  reason: 'ok' | 'invalid' | 'expired' | 'revoked' | 'wrong_product';
  expires_at?: string | null;
}

export interface CachedVerdict extends LicenseVerdict {
  checkedAt: number;
  tokenFingerprint: string;
}

export function verificationUrl(token: string): string {
  return `${API_BASE}/products/${PRODUCT_SLUG}/verify?license=${encodeURIComponent(token)}`;
}

export function tokenFingerprint(token: string): string {
  let hash = 2166136261;
  for (const character of token) {
    hash ^= character.charCodeAt(0);
    hash = Math.imul(hash, 16777619);
  }
  return (hash >>> 0).toString(16).padStart(8, '0');
}

export function isFreshValidVerdict(cached: CachedVerdict | null, token: string, now = Date.now()): boolean {
  return Boolean(
    cached?.valid &&
    cached.tokenFingerprint === tokenFingerprint(token) &&
    now - cached.checkedAt < VERIFY_INTERVAL_MS,
  );
}

export function parseCachedVerdict(raw: string | null): CachedVerdict | null {
  if (!raw) return null;
  try {
    const value = JSON.parse(raw) as Partial<CachedVerdict>;
    if (typeof value.valid !== 'boolean' || typeof value.checkedAt !== 'number' || typeof value.tokenFingerprint !== 'string') return null;
    return value as CachedVerdict;
  } catch {
    return null;
  }
}
