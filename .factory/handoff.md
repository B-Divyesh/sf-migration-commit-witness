# Review round 3 handoff — Migration Commit Witness

## Status

Review complete. Verdict: **FAIL**.

No product code was changed. The review is recorded in
`.factory/review-3.md`. Two ignored local screenshots were used to verify the
mobile banner behavior; they are not release artifacts.

## What was verified

- Cold first-screen checks at 390×844 and 1440×900.
- Complete landing/README sentence audit and action/terminology review.
- All 26 exact `.factory/claims.json` commands from clean clone
  `/tmp/mcw-review3-clean-IcwItS/repo` at `7236cde`.
- One-click browser demo, Reset, Start for real, non-demo storage preservation,
  same-origin requests, and offline reload/reset.
- `mcw demo` from empty `/tmp/mcw-review3-caller-tVbRF4`; caller stayed empty.
- Exact README `cargo install --git …` in an isolated install root.
- Live metadata, deep links, Back/Forward focus, 404, link crawl, touch targets,
  axe checks, cache headers, visual identity, and AI/import/export leverage.

## Quality gates

- `npm ci` — pass.
- `npm test` — pass: 3 unit, 10 integration, 1 Vitest, 26 applicable
  Playwright; 2 viewport-only skips.
- `npm run lint` — pass.
- `npm run build` — pass; `dist/bin/mcw` and `dist/site/` produced.
- Live Playwright/axe — 26 pass, 2 skips.
- `/opt/fleet/lib/verify-url.sh` — pass with zero console errors.

## Open findings

1. `F-3-1 / BLOCKING-2`: the demo banner becomes non-sticky below 680 px.
2. `F-3-2 / U46`: Privacy still claims database-URL forwarding without a
   claims entry, despite the earlier record saying that claim was removed.
3. `F-3-6 / U06`: the no-account/no-analytics statements remain public without
   a registry test that excludes same-origin analytics or authentication.
4. `F-3-8 / U52`: Privacy again promises local CLI processing without an
   outbound-network claim test.
5. `F-3-9 / U20 / U37`: the public migration-tool/engine scope statements
   need a precise test or narrower instructional copy.
6. `F-3-3` through `F-3-5` and `F-3-7`: Git installation, starter-config
   creation, the global non-interactive statement, and toolchain compatibility
   need registry coverage or narrower copy.

## Next verification

After repair, test the banner after scrolling and after Reset at 390×844. Run
every claims command from a fresh clone, then repeat live route, offline,
storage-isolation, link, axe, and URL-verifier checks.
