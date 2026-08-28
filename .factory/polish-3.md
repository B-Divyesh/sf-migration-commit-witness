# Polish round 3 — complete finding closure

Repair commit: `90e6f4667039ef7c1203b2fe76598f2ee15a1b18`.

The concrete-and-moss evidence-sheet identity, original artwork, static Vite
site, Rust CLI, recorded demo, and free MIT scope remain intact. This round
repairs the mobile sandbox boundary and closes every remaining or reopened
claim gap with a tagged, observable test. Local visual evidence is
`.factory/evidence/polish-3-home-desktop.png` and
`.factory/evidence/polish-3-demo-phone.png`. Live verification is recorded in
the final handoff after deployment.

## Round 3 findings

| Finding id | Change made | Evidence |
| --- | --- | --- |
| F-3-1 / BLOCKING-2 | Kept the compact mobile demo banner `position: sticky; top: 0` and retained both 44 px controls. | `@claim:demo-banner-persistence`; phone screenshot; `/demo/`. |
| F-3-2 / U46 | Kept the useful URL-forwarding explanation and registered it. The test records the configured URL in both child commands and rejects it in JSON/Markdown. | `@claim:environment-forwarding`; `/privacy/`. |
| F-3-3 | Registered the exact public `cargo install --git … --bin mcw` path. | `@claim:git-install`; landing and README install command. |
| F-3-4 | Registered `mcw init`; the test parses the result, runs a witness after supplied scripts, and checks overwrite refusal. | `@claim:init-config`; README config section. |
| F-3-5 | Registered the full public non-interactive promise; every public subcommand runs with closed stdin and a five-second bound. | `@claim:non-interactive-commands`; README exit-code section. |
| F-3-6 / U06 | Expanded browser privacy testing to permit only declared static paths, reject analytics/auth/login/account paths and controls, and inspect storage. | `@claim:browser-demo-privacy`; `/demo/`, `/privacy/`. |
| F-3-7 | Declared Rust 1.85 and Node 22 minimums, made them executable, and added a minimum-version CI matrix. | `@claim:toolchain-compatibility`; README verify section. |
| F-3-8 / U52 | Removed the unprovable local-processing sentence. Privacy now gives the tested environment-variable behavior. | `/privacy/`; `@claim:environment-forwarding`, `@claim:secret-omission`. |
| F-3-9 / U20 / U37 | Replaced capability negatives with actionable instructions to keep migration tooling, backups, and review; no backup/engine/replacement claim remains. | landing and `/terms/` copy audit; live crawl. |

## Earlier review findings

| Finding id | Change retained or rechecked | Evidence |
| --- | --- | --- |
| BLOCKING-1 | Plain job/audience/sample action stays in the first 390 px screen. | `home first screen names the job, audience, and sample action`; home screenshot; `/`. |
| BLOCKING-2 | Real isolated `mcw demo`, `/demo/`, query entry, banner, reset, start-real, immediate sample verdict, and mobile persistence all remain. | `@claim:demo-isolation`, `@claim:demo-first-result`, `@claim:demo-banner-persistence`; phone screenshot; `/demo/`. |
| BLOCKING-3 | Registry has 32 entries, with one unique tag per entry and all exact commands runnable in a clean clone. | claim-tag audit and clean-clone claim run. |
| BLOCKING-4 | Real static routes, titles, canonical URLs, h1 focus, back/forward behavior, sitemap, and designed 404 remain. | `routes have distinct metadata, focused headings, and working back navigation`; `unknown paths return the designed 404`; live routes. |
| BLOCKING-5 | The unavailable paid checkout, price, token, and license UI remain removed. | link crawl and source audit; live `/`. |
| HIGH-1 | Per-route plain metadata, OG/Twitter artwork, favicon, Apple icon, and title lengths remain. | metadata/axe route test; live routes. |
| HIGH-2 | Shared header/footer, legal links, one-line product copy, and build identifier remain. | route crawl; live routes. |
| HIGH-3 | Three-step workflow and limits/privacy section remain in the landing order. | heading/outline test; live `/`. |
| HIGH-4 | Unsupported token and absolute production statements stay removed; exact guards are tested. | `@claim:test-confirmation`, `@claim:url-name-guards`, `@claim:environment-labels`. |
| MEDIUM-1 | Every visible control remains at least 44×44 px at desktop and 390 px. | geometry/keyboard test; live `/demo/`, `/privacy/`, `/terms/`. |
| MEDIUM-2 | First-screen and supporting copy remains plain, short, and product-specific. | `.factory/copy-audit.md`; home screenshot. |
| F-2-1 | Partial verdict, `1 / 2`, missing table, and restored `0 / 2` remain in the first phone view. | `@claim:demo-first-result`; phone screenshot; `/demo/`. |
| F-2-2 | The self-hosted terminal SVG and transcript remain generated from `mcw demo`. | `@claim:terminal-recording`; landing screenshot; `/`. |
| MCW-V001 | PostgreSQL uses `psql --dbname` and does not set `PGDATABASE`. | `@claim:postgres-psql`. |
| MCW-V002 | Query errors fail rather than matching by accident. | `@claim:query-error-fails`. |
| MCW-V003 | Requested rollback without a configured rollback is rejected before mutation. | `@claim:rollback-preflight`. |
| MCW-V004 | Unavailable paid flow remains absent. | live link crawl. |
| MCW-V005 | Immutable assets and no-cache service worker policy remain. | deployment Vitest; deployed headers. |
| MCW-V006 | All desktop controls meet the 44 px target. | geometry test. |
| MCW-V007 | Removed license verification and its unsupported frequency promise remain absent. | source/link audit. |
| MCW-V008 | All 390 px controls meet the 44 px target. | geometry test. |

## Original unlisted-claim audit

| Finding id | Final disposition and evidence |
| --- | --- |
| U01 | Retained offline demo with `@claim:offline-demo`. |
| U02 | Retained migration evidence with `@claim:demo-evidence`. |
| U03 | Retained before/after sample checks with `@claim:demo-evidence`. |
| U04 | Retained optional rollback exercise with `@claim:rollback-exercise`. |
| U05 | Retained signing behavior with `@claim:signed-witness`. |
| U06 | Retained only the bounded browser privacy promise with `@claim:browser-demo-privacy`. |
| U07 | Retained exact SQLite/PostgreSQL behavior with `@claim:sqlite-builtins` and `@claim:postgres-psql`. |
| U08 | Retained JSON/Markdown files with `@claim:evidence-formats`. |
| U09 | Retained partial-success detection with `@claim:partial-commit-detection`. |
| U10 | Removed deferred-constraint generalization from public copy. |
| U11 | Removed general DDL behavior promise from public copy. |
| U12 | Retained the concrete successful-command/failed-check sample with `@claim:partial-commit-detection`. |
| U13 | Retained ordering through `@claim:demo-evidence`. |
| U14 | Removed migration-tool-family naming. |
| U15 | Removed failed-command follow-up promise. |
| U16 | Retained explicit rollback command behavior with `@claim:rollback-exercise` and `@claim:rollback-preflight`. |
| U17 | Retained realistic bundled sample with `@claim:demo-evidence`. |
| U18 | Retained CLI/browser record equality with `@claim:demo-record-match`. |
| U19 | Removed one-binary packaging promise from public copy. |
| U20 | Removed migration-engine/agent scope promise; current page gives instructions only. |
| U21 | Retained label and confirmation behavior with `@claim:environment-labels` and `@claim:test-confirmation`. |
| U22 | Retained documented URL-name checks with `@claim:url-name-guards`. |
| U23 | Retained rollback gate with `@claim:rollback-preflight`. |
| U24 | Retained SQLite built-ins with `@claim:sqlite-builtins`. |
| U25 | Retained PostgreSQL invocation with `@claim:postgres-psql`. |
| U26 | Retained MIT license with `@claim:mit-free-cli`. |
| U27 | Retained free core commands with `@claim:mit-free-cli`. |
| U28 | Removed paid kit. |
| U29 | Removed price. |
| U30 | Removed paid deliverables. |
| U31 | Removed unavailable checkout. |
| U32 | Removed token storage. |
| U33 | Removed license verification frequency. |
| U34 | Removed refund claim. |
| U35 | Removed absolute production-connection statement; exact checks remain. |
| U36 | Split end-to-end README result into demo, format, rollback, and signing tests. |
| U37 | Removed migration-generation/replacement scope promise; current copy is instructional. |
| U38 | Retained documented build paths with `@claim:build-artifacts`. |
| U39 | Retained stdout/stderr/no-prompt behavior with `@claim:json-stream-contract` and `@claim:non-interactive-commands`. |
| U40 | Retained signature verification with `@claim:signed-witness`. |
| U41 | Retained exit-code table with `@claim:exit-code-contract`. |
| U42 | Retained one-value check requirement with `@claim:one-value-checks`. |
| U43 | Retained error handling with `@claim:query-error-fails`. |
| U44 | Retained PostgreSQL argv/redaction behavior with `@claim:postgres-psql` and `@claim:secret-redaction`. |
| U45 | Retained embedded SQLite checks with `@claim:sqlite-builtins`. |
| U46 | Restored as the precise `@claim:environment-forwarding` claim. |
| U47 | Split safety/secret behavior across confirmation, environment, URL, rollback, and secret-omission tests. |
| U48 | Retained database-specific checks with SQLite/PostgreSQL claim tests. |
| U49 | Removed test-suite marketing statement. |
| U50 | Removed documented-example coverage marketing statement. |
| U51 | Removed package-command marketing statement. |
| U52 | Removed the untestable local-processing statement. |
| U53 | Removed license storage with paid flow. |
| U54 | Retained only the exact MIT/free-core behavior with `@claim:mit-free-cli`. |

## Verification commands

The clean clone at `/tmp/mcw-polish3-clean-jbwDU0/repo` runs `npm ci`, each
exact command in `.factory/claims.json`, `npm test`, `npm run lint`, `npm run
build`, and `cargo package --locked`. Live post-deploy route, metadata,
accessibility, privacy, offline, and cold-load evidence is in
`.factory/handoff.md`.
