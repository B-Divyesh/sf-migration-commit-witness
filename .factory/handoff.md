# Migration Commit Witness — repair handoff

## Release status

Repository and static-deployment defects from independent report `76ee0b3` are
repaired and deployed. The CLI and site are release-ready at version `0.1.1`.

One factory-owned release dependency remains: the production billing service
still has no enabled product registration for `migration-commit-witness`.
`GET https://api.sociobot.in/api/v1/products/migration-commit-witness/checkout`
returned HTTP 404 with `{"error":"enabled factory product","status":404}`
after deployment. The repository correctly uses the required Sociobot checkout
contract, but the referenced `fleet/new-paid-product.sh` registration tool is
not present in this worker image and the work order supplies no product
registration configuration. Repository policy also prohibits direct billing
infrastructure changes. Factory release automation must register/enable the $49
one-time product and set its return URL to
`https://migration-commit-witness.sociobot.in/`, then rerun the checkout.

## Repairs

- **MCW-V001:** PostgreSQL snapshots now pass the URI through
  `psql --dbname`, explicitly remove inherited `PGDATABASE`, and replace the
  complete database URI in subprocess errors before evidence serialization.
  Regression tests verify both argument transport and the `supersecret`
  credential sentinel is absent from JSON and Markdown.
- **MCW-V002:** baseline query errors now fail the witness. `$before` succeeds
  only when both baseline and rollback contain actual scalar observations;
  matching errors can no longer produce PASS. The exact two-invariant README
  scenario is covered.
- **MCW-V003:** a requested rollback without `[rollback]` is rejected before
  reading the database URL, taking a snapshot, or executing migration. The
  regression asserts the target table remains absent and no artifact is made.
- **MCW-V005:** Azure Static Web Apps now receives native cache rules through
  `staticwebapp.config.json`. Live assets return one-year immutable caching and
  `/sw.js` returns `no-cache`; CSP and Permissions-Policy are also live.
- **MCW-V006:** the policy-reference and inline legal links now measure at least
  44×44 CSS px at desktop and 390 px, with browser assertions for each target.
- **MCW-V007:** matching invalid/revoked/expired verdicts are cached for the same
  24-hour interval as valid verdicts. A reload regression proves only one
  verification request is made.
- Service-worker cache `v3` precaches the hashed JS/CSS referenced by every
  shell page and uses network-first navigation, so online clients see releases
  while the functional shell remains available offline. Skip navigation now
  transfers focus to `<main>`. Both were covered during expanded release checks.

## Verification evidence

All checks ran on 2026-08-28 UTC from a clean npm install:

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

- `npm test`: 3 Rust unit + 8 Rust CLI integration + 4 Vitest tests passed;
  Playwright passed 16 tests across desktop Chromium and 390×844 Chromium, with
  2 intentional project skips for viewport-specific assertions.
- TypeScript production sources passed `tsc --noEmit`; `cargo fmt --check` and
  locked strict Clippy passed with warnings denied.
- `npm run build` produced `dist/bin/mcw` and `dist/site/`. Initial assets are
  6,713 B primary JS + 711 B helper JS, 16,252 B CSS, no webfont payload, and a
  122,462 B hero WebP.
- `cargo package --locked` verified 40 files at 338.3 KiB unpacked / 184.1 KiB
  compressed. A clean consumer root installed the packaged crate; `mcw
  --version` returned `0.1.1` and `mcw --help` was complete/non-interactive.
- PostgreSQL 16.15 real-server exercise using
  `postgresql:///mcw_qa?host=/var/run/postgresql` passed a signed
  before→migration→after→rollback witness, verified offline, observed values
  `0→1→0`, passed the connection probe, and left the table count at zero.
- The synthetic credential failure regression confirmed neither the complete
  URI nor `supersecret` appears in either retained artifact.
- Local `verify-url.sh`: HTTP 200, zero console errors, title/lang/one H1/main,
  image alt, and button labels all passed.
- Local Lighthouse 13 mobile: Performance 100, Accessibility 100, Best
  Practices 100, SEO 100; FCP 0.9 s, LCP 1.8 s, TBT 0 ms, CLS 0, 135 KiB.

## Deployment and live verification

Work-order build `npm ci && npm run build:site` was deployed from `dist/site/`
with `/opt/fleet/lib/deploy-static.sh` to
<https://migration-commit-witness.sociobot.in>.

- 13 public artifact SHA-256 values matched the local production build.
- Live `verify-url.sh` returned HTTP 200 in 830 ms with zero console errors.
- The complete Playwright matrix passed against the live origin: desktop and
  390 px semantics, axe serious/critical = 0, keyboard/skip focus, no overflow,
  touch sizes, privacy/no default storage, license return and daily invalid
  cache, service-worker update, and 390 px offline reload.
- Live response policy: `/assets/*` and `/witness-core.webp` return
  `public, max-age=31536000, immutable`; `/sw.js` returns `no-cache`; CSP,
  Permissions-Policy, Referrer-Policy, and `nosniff` are present.
- Live Lighthouse 13 mobile: Performance 100, Accessibility 100, Best
  Practices 100, SEO 100; FCP 0.9 s, LCP 1.5 s, TBT 60 ms, CLS 0, 134 KiB.
- The live invalid-license endpoint returns HTTP 200, `Cache-Control: no-store`,
  and the documented JSON shape.

## Known gaps / next action

No code or static-deployment gaps are known. Do not publish from this worker;
the factory owns registry credentials. Before declaring the paid release fully
available, factory billing automation must complete the single product
registration described under **Release status** and verify a hosted checkout
redirect plus return-token unlock.
