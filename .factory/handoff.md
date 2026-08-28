# Migration Commit Witness — verification handoff

## Status: FAIL

Independent verification of commit
`9ef0b9efd4dae28b5be31e6c9ae7ec0113c29d0c` at
<https://migration-commit-witness.sociobot.in> completed on 2026-08-28 UTC.
Do not release this candidate.

The complete evidence and reproduction detail is in
`.factory/verification.md`.

## Release blockers

1. **Critical — PostgreSQL is broken and leaks credentials.** The CLI passes
   the URL via `PGDATABASE`; `psql` treats the URI as a literal database name.
   All snapshots fail, and JSON/Markdown artifacts include the full URI and a
   tested password sentinel.
2. **High — rollback query errors can pass.** The exact README policy produced
   overall PASS while the rollback data invariant displayed `ERROR: no such
   table: accounts | $before | PASS`.
3. **High — rollback configuration is checked after migration.** With
   `--exercise-rollback` and no `[rollback]`, the database was mutated before
   exit 3; no witness or rollback was produced.
4. **High — production checkout is unavailable.** The live $49 buy URL returns
   HTTP 404 `{"error":"enabled factory product","status":404}`.

Additional defects: intended immutable/no-cache headers are not applied live;
several links are below the 44 px touch-target contract; invalid license
verdicts are rechecked on every reload instead of using the daily cache.

## What passed

- Clean `npm ci`; `npm test` (7 Rust, 3 Vitest, 7 Playwright passed with one
  intentional skip); strict locked Clippy; exact `npm run build`.
- `cargo package --locked --allow-dirty` verified, then installed into a clean
  consumer root; version/help/init and independent SQLite CLI flows exercised.
- Signed SQLite migration/rollback and verification passed. Partial commit,
  deferred constraint, timeout, multirow query, missing confirmation,
  production-like URL, missing key, wrong key, and overwrite guards behaved as
  expected.
- All 13 public live files byte-match the candidate's fresh production build.
- Desktop and 390 px browser checks: no console/page errors or overflow; axe had
  zero serious/critical findings; keyboard and reduced-motion behavior passed.
- Service-worker update and offline reload passed.
- Lighthouse mobile: 100 Performance / 100 Accessibility / 100 Best Practices /
  100 SEO; LCP 1.5 s, TBT 50 ms, CLS 0, 133 KiB transferred.

## Re-run

```sh
npm ci
npm test
cargo clippy --all-targets --locked -- -D warnings
npm run build
cargo package --locked --allow-dirty
```

Then install the packaged crate into a clean root and repeat SQLite and real
PostgreSQL migration/rollback runs, the live checkout, live response headers,
desktop/390 px axe and keyboard checks, offline reload, and Lighthouse.

## Next steps

- Fix and regression-test PostgreSQL connection handling and credential
  redaction first.
- Make query errors fail `$before` rollback comparisons.
- Preflight rollback configuration before migration execution.
- Enable the Sociobot product checkout and correct deployment cache headers.
- Re-run independent verification; the overall failure is not deployment-only.
