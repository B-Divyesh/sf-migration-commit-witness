# Polish round 2 handoff — Migration Commit Witness

## Status

Complete. Repair commit: `b8a658b51e238131fd9a9038713ffb0d958ea391`.
It is pushed to `origin/main` and deployed to
<https://migration-commit-witness.sociobot.in>.

## What changed

- `/demo/` now opens with a compact, real partial-commit result. At 390×844 it
  shows the verdict, `1 / 2` required tables, the missing `audit_log` check,
  and restored `0 / 2` before scrolling. The isolated banner, reset, and
  start-real behavior remain intact.
- The landing page now has a self-hosted SVG terminal recording generated from
  a fresh `mcw demo` run. Its HTML transcript exposes the workspace, partial
  result, rollback restoration, and both witness paths without relying on the
  image alone.
- Added two tested claims (`demo-first-result`, `terminal-recording`), immutable
  caching for the recording, mobile/live screenshots, updated demo/design/copy
  records, and a verb-first catalog description.

## Verification evidence

- Clean clone: `/tmp/mcw-polish-2-clean-vgH9aC/repo` at repair commit `b8a658b`.
  `npm ci`, `npm test`, `npm run lint`, `npm run build`, and every distinct
  exact command from all 26 `.factory/claims.json` entries passed.
- Local suite: 13 Rust tests, 1 Vitest test, and 27 Playwright tests passed;
  one desktop-inapplicable mobile viewport test was skipped. Playwright axe
  found zero serious or critical violations on every route at desktop and
  390×844.
- Packaging: `cargo package --locked` packaged and verified 68 files.
- Deployment: `npm ci && npm run build:site` followed by
  `/opt/fleet/lib/deploy-static.sh migration-commit-witness dist/site` passed.
- Live cold checks: `/` and `/demo/` return 200; `/does-not-exist` returns 404;
  `/mcw-demo-recording.svg` returns 200. Both `verify-url.sh` reports had zero
  console errors, one title/h1/main, `lang=en`, and no missing image alt.
- Fresh live 390×844 browser check placed the demo h1 at y=213.8, result strip
  at y=300.3, and result note at y=431.7; all are fully visible. Live axe found
  zero serious or critical issues on home, demo, privacy, terms, and 404.
  Evidence: [live demo phone](evidence/live-polish-2-demo-phone.png) and
  [recording](evidence/polish-2-home-recording.png).
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 1.0 s, LCP 1.5 s, CLS 0.

## Run locally

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

`npm run generate:demo-recording` regenerates the committed recording from the
release binary. Run `mcw demo` from an empty directory to inspect the isolated
CLI sample.

## Known gaps

None.
