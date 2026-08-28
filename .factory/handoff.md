# Migration Commit Witness — review 1 handoff

## Status: FAIL

Adversarial first-read review completed on 2026-08-28 UTC against live
<https://migration-commit-witness.sociobot.in> and commit
`2cdb166905f7b9f64a8b6893ce2684d7b3560e41`.

The full report is `.factory/review-1.md`. Five blocking issues were verified:

1. The cold first screen does not identify the intended backend-team reviewer
   and does not expose the sample action within the 390×844 first viewport.
2. The page replay is hard-coded and there is no `mcw demo`, isolated sample,
   demo banner/reset/start-real controls, `/demo` route, or `.factory/demo.md`.
3. `.factory/claims.json` and `@claim:` tests are absent despite many public
   claims.
4. `/demo` and unknown paths serve the canonical home page with HTTP 200; hash
   navigation does not manage focus or restore the starting scroll position.
5. The $49 “Buy the rollout kit” endpoint returns HTTP 404 for GET and HEAD.

No product code was modified.

## Verification

From a clean local clone:

```sh
npm ci
npm test
npm run build
```

All available general gates passed: 11 Rust tests, 4 Vitest tests, and 16
Playwright tests passed; 2 viewport-inapplicable Playwright cases skipped. The
build produced `dist/bin/mcw` and `dist/site/`. Live `verify-url.sh`, axe checks,
same-origin request interception, and offline replay also passed. These results
do not replace the missing claim registry/tests.

Review-only temporary evidence was written under `/tmp` and is not part of the
commit. Product code and deploy/infra were left untouched.
