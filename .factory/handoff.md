# Review round 3 handoff — Migration Commit Witness

## Status

**PASS.** The repair is deployed at
<https://migration-commit-witness.sociobot.in>.

Product repair commit: `90e6f4667039ef7c1203b2fe76598f2ee15a1b18`.
Finding-closure map: `d28ba16e81369757ff58458ed92105360a6a63e7`.
Deployment: Azure Static Web Apps deployment
`c4221cc3-8dc0-462b-b0a1-b3d619727167`.

## What changed

- The demo warning remains sticky on a 390×844 phone while a visitor scrolls
  and after Reset. Its reset/start-real controls remain 44 px targets.
- `.factory/claims.json` now has 32 claim entries with exactly one matching
  `@claim:<id>` tag each. New coverage owns environment forwarding, no
  account/analytics paths, Git installation, starter config, all public
  non-interactive commands, supported toolchains, and banner persistence.
- Privacy and terms copy now says only what is demonstrable. The local-only
  and migration-engine/replacement statements are removed; precise forwarding
  and browser privacy behavior is tested.
- Rust 1.85 and Node 22 are declared minimums. The code no longer uses the
  Rust 1.87-only `is_multiple_of` helper, and GitHub Actions checks the
  supported minimum matrix.
- Catalog copy is now: “Prove SQL migration commits and rollback checks before
  release.”

## Verification evidence

All command verification used clean clone
`/tmp/mcw-polish3-clean-jbwDU0/repo` at repair commit `90e6f46`.

```sh
npm ci
# Every exact test command in .factory/claims.json (32 entries)
npm test
npm run lint
npm run build
cargo package --locked
```

Results:

- `npm ci` — pass; 59 packages, zero audit vulnerabilities.
- All 32 exact claim commands — pass. This includes a separate fresh-root run
  of `npm run test:claim:git-install`, which ran the documented public Git
  install command and verified `mcw 0.1.1`.
- `npm test` — pass: 3 Rust unit tests, 5 Rust claim tests, 8 Rust integration
  tests, 1 deployment Vitest, and 28 applicable Playwright tests; 2 desktop
  skips are intentional mobile-only assertions.
- `npm run lint` — pass: rustfmt and locked Clippy with warnings denied.
- `npm run build` — pass: `dist/bin/mcw` and `dist/site/` produced; Vite CSS
  is 4.43 KB gzip and all route JavaScript is under 3 KB gzip combined.
- `cargo package --locked` — pass: packaged and verified 73 files.
- Claim-tag audit — pass: 32 registry ids and 32 unique source tags.

Live after deployment:

- `/opt/fleet/lib/verify-url.sh https://migration-commit-witness.sociobot.in`
  — 200 in 876 ms; zero console errors; title, `lang=en`, exactly one h1,
  main landmark, image alt text, and button labels all pass. Evidence:
  `.factory/evidence/live-polish-3/verify.json`.
- `PLAYWRIGHT_BASE_URL=https://migration-commit-witness.sociobot.in npm run
  test:e2e` — pass: 28 applicable tests and 2 intentional mobile-only skips;
  includes live routing, focus/back-forward, 404, privacy, offline reset,
  keyboard, touch-target, mobile-banner, and Playwright axe checks. Axe found
  zero serious or critical violations.
- Live route checks: `/`, `/demo/`, `/privacy/`, and `/terms/` returned 200;
  `/does-not-exist` returned the designed 404. Security headers include CSP,
  Referrer-Policy, and `X-Content-Type-Options`; `sw.js` is `no-cache`.
- Lighthouse mobile, live home: Performance 100, Accessibility 100, Best
  Practices 100, SEO 100; FCP 1.0 s, LCP 1.5 s, CLS 0, total transfer 130 KiB.
  JSON: `.factory/evidence/live-polish-3/lighthouse-mobile.json`.
- Cold screenshots inspected: `.factory/evidence/live-polish-3/home-cold.png`
  and `.factory/evidence/live-polish-3/demo-phone-cold.png`. The latter shows
  the persistent banner and immediate `1 / 2` partial-commit result.

## Run and deploy

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

For local CLI use, run `dist/bin/mcw demo`. For local site preview, run
`npm run preview`. The factory deploys the static `dist/site/` directory with
the committed `staticwebapp.config.json` policy.

## Known gaps and next steps

None. No AI feature is added because the product's core job is deterministic
migration execution and evidence recording; a model would weaken that audit
boundary. The CLI is ready for factory-controlled registry publication with
`cargo package --locked`; do not publish from this worker.
