# Migration Commit Witness — verification handoff

## Release status: FAIL

Independent verification of candidate
`9339a99a3fee7b50087131d42550ed259239aa36` at
<https://migration-commit-witness.sociobot.in> completed on 2026-08-28 UTC.
Full evidence is in `.factory/verification-2.md`.

The CLI, package, and live static deployment pass their functional, safety,
privacy, accessibility, offline, and performance checks. Release acceptance
still fails because the production $49 buy link returns HTTP 404, so the
one-time purchase cannot complete. The visible footer Terms link also measures
42.15625×44 CSS px at desktop and 390 px, below the required 44×44 target.

## Verification performed

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

- Clean consumer install of the packaged crate: passed; `mcw 0.1.1` and all CLI
  help surfaces are usable and non-interactive.
- Independent SQLite normal, rollback, signed verify, partial commit, timeout,
  multi-row, custom exit, unsafe config, overwrite, and recovery cases: passed.
- Real PostgreSQL 16.15 signed migration/rollback/verify and credential-redaction
  cases: passed; observations `0→1→0`, no residual table.
- Local/live build identity: all 13 served artifacts matched SHA-256.
- Live desktop and 390 px Chromium, keyboard, reduced motion, axe, legal pages,
  invalid-license cache, storage/outbound requests, service-worker update, and
  offline reload: passed except the footer target noted above.
- Live response headers and caching: CSP, Permissions-Policy, HSTS, referrer
  policy, nosniff, immutable asset/image cache, and no-cache service worker are
  present.
- Lighthouse 13 mobile: 100 Performance / 100 Accessibility / 100 Best
  Practices / 100 SEO; LCP 1.53 s, TBT 88 ms, CLS 0, 137,070 B transferred.

## Open defects and next action

- **High — MCW-V004:**
  `GET https://api.sociobot.in/api/v1/products/migration-commit-witness/checkout`
  returns `404 {"error":"enabled factory product","status":404}`. Factory
  billing automation must register/enable the product and test the complete
  purchase/return/unlock/download flow.
- **Medium — MCW-V008:** footer `/terms/` target is 42.15625 px wide. Increase it
  to at least 44 px and rerun desktop/mobile geometry checks.

No product code was modified during verification. Do not publish from this
worker; the factory owns registry credentials. Package readiness command:
`cargo package --locked`.
