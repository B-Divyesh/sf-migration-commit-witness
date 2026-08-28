# Migration Commit Witness — build handoff

## What shipped

- Rust `mcw` 0.1.0 single binary with `init`, `witness`, and `verify` commands.
- TOML policy for a named SQLite or PostgreSQL test target, external migration
  command, optional explicit rollback command, timeouts, accepted exit codes,
  and scalar schema/data invariants.
- Dual production protection: an allowed non-production environment label plus
  `--confirm-test-database`; production-like URLs and shared in-memory SQLite
  are rejected. Rollback requires both policy configuration and
  `--exercise-rollback`.
- Before/after snapshots run even when a migration command fails. SQLite adds
  `quick_check` and foreign-key probes; PostgreSQL uses `psql` and labels its
  reachability check without claiming dialect-independent guarantees.
- Stable JSON and PR-readable Markdown evidence, HMAC-SHA256 signing by default,
  offline signature verification, redacted credentials/command output, atomic
  artifact writes, CI-oriented stdout JSON, and documented exit codes.
- Seeded end-to-end SQLite tests for a successful exercised rollback, a
  non-transactional-style partial DDL outcome reported as success, and a
  deferred-constraint commit failure whose surrounding engine status is still
  success. Both partial cases are caught by post-commit invariants.
- Static Vite documentation site with a keyboard-operable recorded fixture,
  responsive 390 px layout, explicit offline state/service worker, privacy and
  terms pages, and no runtime third-party scripts, fonts, analytics, or tracking.
- Optional $49 one-time Team rollout kit using the Sociobot checkout/verify
  contract. Query-string return, local token storage, cached once-daily verdict,
  optimistic cached unlock, background reconciliation, offline handling, paste
  restore, and invalid/revoked states are implemented. Core safety, signing,
  evidence export, and accessibility remain free.
- Original `witness-core.webp` hero generated with the factory image deployment
  and optimized to 122,462 bytes. Prompt and deployment provenance are recorded
  in `.factory/design.md` and `.factory/witness-core.provenance.json`.

## Run and verify

```sh
npm ci
npm test
npm run build
```

The exact build command is `npm run build`. It produces the deployable static
site at `dist/site/` (with `index.html` at that root) and the current-platform
release binary at `dist/bin/mcw`.

Additional completed checks:

- `cargo clippy --all-targets -- -D warnings` — passed.
- `npm test` — passed: 7 Rust tests, 3 site unit tests, and 7 Playwright tests
  across desktop Chromium and a 390×844 Chromium viewport; one desktop copy of
  the mobile-only overflow assertion is intentionally skipped.
- Playwright axe integration — zero serious or critical findings on home,
  privacy, and terms pages in both viewport projects.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 .factory/evidence` — HTTP
  200, one H1, `lang`, `main`, all image alt text, labeled buttons, and zero
  console errors.
- Lighthouse 13 mobile — Performance 100, Accessibility 100, Best Practices
  100, SEO 100; LCP 1.8 s, CLS 0, total blocking time 40 ms. INP is not emitted
  by a no-interaction synthetic run.
- Initial production assets — 6.30 KB primary JS (plus 0.71 KB shared helper),
  16.02 KB CSS, no font payload, 122.46 KB hero WebP.
- `cargo package --allow-dirty` — passed; package is 307.1 KiB unpacked / 175.0
  KiB compressed. Do not publish from the worker.

## Known gaps and factory next steps

- PostgreSQL support is implemented through `psql` but was not exercised
  against a live PostgreSQL server in this container. SQLite and the common
  execution/snapshot/signature paths are covered end to end.
- HMAC signatures provide integrity for teams sharing a CI secret; v1 does not
  implement public-key signatures or key rotation metadata.
- The checked-in build produces the current Linux binary. The factory should
  create platform release binaries if distributing beyond source installs.
- The factory must register the paid product/return URL and validate checkout in
  its release environment. The site intentionally uses the documented
  production Sociobot endpoint and does not hardcode an internal product ID.
- Deploy only `dist/site/`; do not deploy repository credentials or test
  databases. The CLI must continue to run exclusively on disposable targets.
