# Migration Commit Witness — polish round 1 handoff

## Status

Complete. Every finding in `.factory/review-1.md` and the earlier verification reports is closed. The repaired site is live at <https://migration-commit-witness.sociobot.in>.

Implementation commits:

- `bc195d2` — real CLI/browser demo, copy, routes, metadata, responsive UI, and claim suite
- `2d2a20c` — explicit query-error claim coverage

The unavailable paid offer was removed, as required by BLOCKING-5’s fallback. No checkout or license flow is advertised.

## What changed

- Added `mcw demo` with an opinionated SQLite fixture under `examples/demo/`.
- The demo creates one new temporary workspace and runs the real witness path.
- Added `/demo/` with the required persistent banner, reset, and start-real controls.
- Added `.factory/claims.json` with 24 claims and exactly one test tag per id.
- Rewrote the first screen and README in plain words.
- Added `.factory/copy-audit.md`, `.factory/demo.md`, and the catalog description.
- Added real multi-page routes, distinct titles/canonicals, social metadata, icons, and a 404 response.
- Unified navigation/footer structure and added build identity.
- Added route focus, announcements, Back/Forward coverage, touch-target checks, and mobile overflow checks.
- Preserved the concrete-and-moss visual system. Social and touch assets derive from the original witnessed artwork.
- Removed paid UI and browser license code because the production checkout is unavailable.

The exhaustive finding map is in `.factory/polish-1.md`.

## Verification

Claim-audit clone: `/tmp/mcw-polish-final-CCB4UL/repo` at commit `e78f18b`.
Final full-suite clone: `/tmp/mcw-final-audit-1f6Yn8/repo` at commit `7cfb30c`.

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

Results:

- npm audit: 0 vulnerabilities.
- Rust: 3 unit, 2 claim-integration, and 8 CLI integration tests passed.
- Vitest: 1 deployment/config test passed.
- Playwright: 23 applicable desktop/mobile tests passed; one viewport-only case skipped.
- Clippy passed with warnings denied.
- Build produced `dist/bin/mcw` and `dist/site/`.
- Cargo packaged and verified 57 files.
- Every distinct `test` command in `.factory/claims.json` passed in the clean clone.
- Claim tag audit found exactly one `@claim:<id>` test tag for every registry id.

Live checks after deployment:

- `/`, `/demo/`, `/privacy/`, and `/terms/`: HTTP 200.
- `/does-not-exist`: HTTP 404 with the designed page.
- Factory URL verifier: home and demo passed with zero console errors.
- Full Playwright suite against the live origin: 23 passed, one viewport-only skip.
- Axe integration: zero serious or critical findings across all routes at desktop and 390 px.
- Offline demo reload/reset, same-origin request policy, demo namespace, focus, Back/Forward, reduced motion, and 44 px targets passed.
- Live Lighthouse mobile: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO.
- Lighthouse metrics: FCP 0.9 s, LCP 1.5 s, TBT 0 ms, CLS 0.
- Initial transfer: 132,746 bytes.
- JS: under 5 KB uncompressed by route. CSS: 15,020 bytes. Fonts: 0 bytes.
- Immutable asset caching and `no-cache` service-worker headers were confirmed live.

Screenshots are stored at:

- `.factory/evidence/live-home.webp`
- `.factory/evidence/live-home-mobile.webp`
- `.factory/evidence/live-demo-mobile.webp`

## Run the product

```sh
cargo run -- demo
cargo run -- witness --config mcw.toml --output witness \
  --confirm-test-database --exercise-rollback
```

The browser demo is <https://migration-commit-witness.sociobot.in/demo/>.

## Known gaps

None within the reviewed product scope. Registry publishing remains factory-owned and was not performed.
