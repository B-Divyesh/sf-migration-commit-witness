# Review 2 handoff — Migration Commit Witness

## Status

Review completed and committed without product-code changes. Verdict: **FAIL**.

## What was reviewed

- Fresh live browser contexts at 390×844 and 1440×900, including a cold first
  screen, demo entry, reset, storage namespace, offline reload, routes, links,
  metadata, focus/back navigation, and 404.
- A fresh local clone at `6d08e132fb13a6689283fe46992a6059d61fa337`.
- All 24 declared claim entries using their exact clean-clone commands.
- `npm test` / clean-clone E2E (23 passed, one expected viewport skip),
  `npm run lint`, and `npm run build`.
- Live factory URL verification and independent Playwright axe scans at desktop
  and mobile.

## Result

All claimed behavior, privacy isolation, CLI temp-directory behavior,
accessibility baseline, and site structure passed. The review records two
blocking demo presentation gaps in `.factory/review-2.md`; the first reopens
review 1's **BLOCKING-2** because its first-phone-view requirement remains
half-fixed:

1. On 390×844, `/demo/` shows only introduction copy; the first real sample
   verdict starts below the fold.
2. The CLI landing page has no self-hosted terminal recording of a real
   `mcw demo` run, as required for a CLI demo.

## How to verify

```sh
npm ci
npm test
npm run lint
npm run build
cargo test --test claims demo_runs_in_one_isolated_directory_and_matches_the_browser_record
```

For live behavior, open <https://migration-commit-witness.sociobot.in/demo/>
at 390×844 and verify the sample output's current first visible position. Run
`mcw demo` from an empty temporary directory to verify caller-directory
isolation.

## Next steps

Move an actual initial witness result into the demo's first phone viewport.
Add a self-hosted, accessible terminal recording generated from `mcw demo` to
the landing page and test its values/viewport position. Then rerun this review
from a fresh browser context and clean clone.
