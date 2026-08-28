# Polish round 1 — finding closure

Candidate repaired: `2cdb166905f7b9f64a8b6893ce2684d7b3560e41`
Review repaired: `2bc82bc4d1aeb5c88a2731bff33f5ac572031e9e`
Repair commits: `bc195d2`, `2d2a20c`
Live origin: <https://migration-commit-witness.sociobot.in>

## Review finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| BLOCKING-1 | Replaced the first screen with “Prove what your SQL migration committed,” named backend migration reviewers, and made “Try it with sample data” primary. The action and explanation fit inside 390×844. | Playwright `home first screen names the job…`; [mobile screenshot](evidence/live-home-mobile.webp); live `/`. |
| BLOCKING-2 | Added `mcw demo`, bundled SQLite config/SQL under `examples/demo/`, new-directory isolation, real before/after/rollback artifacts, `/demo/`, persistent banner, Reset demo, Start for real, and `.factory/demo.md`. | `@claim:demo-isolation`, `@claim:demo-evidence`, `@claim:demo-record-match`; [demo screenshot](evidence/live-demo-mobile.webp); live `/demo/`. |
| BLOCKING-3 | Added 24 entries to `.factory/claims.json`. Every id appears on exactly one test, and every listed command passed from the clean clone. | Tag-count audit; clean clone `/tmp/mcw-polish-final-CCB4UL/repo`; claim commands listed below. |
| BLOCKING-4 | Added real `/demo/`, static multi-page routes, 404 response override, styled 404, route titles/canonicals, focus and polite announcements, and Back/Forward coverage. | Playwright `routes have distinct metadata…` and `unknown paths return…`; live `/does-not-exist` returned 404. |
| BLOCKING-5 | Removed the unavailable $49 offer, checkout link, license storage, verification code, and paid terms. The free MIT CLI remains complete. | Link crawl in Playwright; `rg 'checkout|license-token|\$49' site` has no product UI match; live home has no paid CTA. |
| HIGH-1 | Added a 53-character plain title, per-route descriptions/canonicals, OG and Twitter metadata, `share.webp` at 1200×630, and a 180×180 Apple icon. | Playwright metadata assertions on all routes; `identify` dimensions; live page source. |
| HIGH-2 | Reused the same wordmark, Demo/Install/Privacy/GitHub header, one-line footer, legal links, and `0.1.1+polish.1` build id everywhere. | Playwright route crawl; [desktop screenshot](evidence/live-home.webp). |
| HIGH-3 | Added a three-step “How it works” section and a preceding “What it never does” section for target checks, rollback, secrets, and tool scope. | Live `/`; Playwright heading/outline checks. |
| HIGH-4 | Removed all browser-token wording with the paid flow. Replaced the absolute production claim with exact label, flag, and URL-name behavior plus its limitation. | `@claim:test-confirmation`, `@claim:url-name-guards`, `@claim:environment-labels`; README safety section. |
| MEDIUM-1 | Set every header/footer/action target to at least 44×44 CSS px, including Terms. | Playwright `skip link and every visible target…` at desktop and 390 px. |
| MEDIUM-2 | Rewrote every quoted jargon, metaphor, long sentence, and ambiguous control. Standard terms are CLI, run, witness, check, config file, and sample/demo. | `.factory/copy-audit.md`; zero banned-word matches; all landing sentences ≤22 words. |

## MEDIUM-2 copy rows

| Review row | Result |
| --- | --- |
| “Independent commit evidence” | “Record what the database committed.” |
| “Snapshot the facts…” | “Save checks before, after, and after rollback.” |
| Green status / seam copy | “A successful migration command does not prove database state.” |
| “Exit zero…” | “A successful command can leave the wrong tables.” |
| DDL and deferred-constraint claims | Removed; the site describes only the bundled partial-table case. |
| “Database truth…” | Replaced with the exact wrong-table result. |
| “Four observations…” | Replaced by the three-step workflow. |
| “Baseline” / “Run yours” / “Witness commit” | “Check before,” “Run the migration command,” and “Check after commit.” |
| Scalar/invariant wording | “One-value queries” and “checks.” |
| Rollback policy wording | “Rollback runs only when the config and command flag both request it.” |
| “Recorded fixture” / “seeded run” | “Real sample” / “sample command.” |
| “Move through observations” | “Step through the saved checks.” |
| “Boring inputs…” | “Run the sample, then add your config.” |
| “Hard guardrail” | “Test target checks.” |
| “Production-like URLs…” | Names the four rejected URL markers and states their limit. |
| Dialect marketing | “See the checks for each database.” |
| All rollout-kit, checkout, merchant, token, and refund copy | Removed with the unavailable paid feature. |
| Closing metaphors | Replaced by “Save migration and rollback results with each release.” |
| “Previous” / “Run command” | “Show previous observation” / “Show next observation.” |
| “Copy install” | “Copy install command.” |
| Misleading CI jump | Removed. |
| README category, 35-word intro, “stable,” and “PR-friendly” | Rewritten into short command-line, JSON, and pull-request sentences. |
| README policy/assertion terminology | Uses config file and check except the literal TOML key. |
| README long exit-code sentence | Four-row table. |
| README scalar/error language | Separate plain sentences with regression coverage. |
| README rollback and dialect paragraphs | Split into sentences under 22 words. |
| Factory packaging language | Removed from user documentation. |
| README 23-word paid sentence | Removed with paid functionality. |

## Unlisted-claim rows

The IDs below follow the report’s row order.

| Rows | Resolution |
| --- | --- |
| U01 offline demo | Retained precisely; `@claim:offline-demo`. |
| U02–U05 migration, before/after, rollback, signing | Retained; `@claim:demo-evidence`, `@claim:rollback-exercise`, and `@claim:signed-witness`. |
| U06 local-only/no telemetry | Replaced by the narrower browser same-origin claim; `@claim:browser-demo-privacy`. |
| U07 dialects | Rewritten as exact SQLite and PostgreSQL behavior; `@claim:sqlite-builtins` and `@claim:postgres-psql`. |
| U08 formats | Retained; `@claim:evidence-formats`. |
| U09 partial success | Retained; `@claim:partial-commit-detection`. |
| U10–U11 deferred constraints and general DDL behavior | Removed from public copy. |
| U12 successful command with failed check | Retained as the concrete sample; `@claim:partial-commit-detection`. |
| U13 baseline ordering | Retained in the workflow; `@claim:demo-evidence` checks baseline and after values. |
| U14 named migration tool families | Removed. The copy promises only the configured command contract. |
| U15 checks after a failed command | Removed from public copy. |
| U16 explicit rollback command | Retained; `@claim:rollback-exercise` and `@claim:rollback-preflight`. |
| U17 sample realism | Retained; `@claim:demo-evidence`. |
| U18 browser values equal CLI values | Retained; `@claim:demo-record-match`. |
| U19 one-binary packaging | Removed from public copy. |
| U20 agent/hosted database/DSL/telemetry bundle | Removed rather than overclaiming. |
| U21 test label and confirmation | Retained; `@claim:environment-labels` and `@claim:test-confirmation`. |
| U22 production URL naming | Rewritten precisely; `@claim:url-name-guards`. |
| U23 rollback gate | Retained; `@claim:rollback-preflight`. |
| U24 SQLite built-ins | Retained; `@claim:sqlite-builtins`. |
| U25 PostgreSQL psql behavior | Retained; `@claim:postgres-psql`. |
| U26–U27 free/MIT/core access | Retained without paid comparison; `@claim:mit-free-cli`. |
| U28–U31 paid kit, price, deliverables, checkout | Removed because production billing returned 404. |
| U32–U34 token storage, verification frequency, refunds | Removed with the paid flow. |
| U35 “No production connections” | Removed. Exact safety checks and their limitation replace it. |
| U36 README end-to-end result | Split and retained; demo, formats, rollback, and signing claim tests cover each outcome. |
| U37 negative production/generation statement | Removed; exact safety behavior replaces it. |
| U38 build paths | Retained; `@claim:build-artifacts`. |
| U39 JSON/stdout/stderr/noninteractive | Retained; `@claim:json-stream-contract`. |
| U40 later verification | Retained; `@claim:signed-witness`. |
| U41 exit codes | Retained as a table; `@claim:exit-code-contract`. |
| U42 one-value query | Retained; `@claim:one-value-checks`. |
| U43 matching query errors | Retained; `@claim:query-error-fails`. |
| U44 PostgreSQL argv/redaction | Retained; `@claim:postgres-psql` and `@claim:secret-redaction`. |
| U45 embedded SQLite | Retained; `@claim:sqlite-builtins`. |
| U46 inherited database URL | Removed from public copy. |
| U47 safety and secret retention | Split across confirmation, environment, URL, rollback, and secret claim tests. |
| U48 database-specific checks | Retained; SQLite/PostgreSQL claim tests. |
| U49 npm test composition | Removed as marketing copy; verification commands remain. |
| U50 documented-example coverage claim | Removed. The sample itself is directly tested. |
| U51 package command | Removed from user copy; `cargo package --locked` still passed. |
| U52 CLI local-only/no telemetry | Removed as an absolute public claim. |
| U53 license storage | Removed with the paid feature. |
| U54 MIT/price/free-feature bundle | Replaced by the narrower MIT/free-core claim; `@claim:mit-free-cli`. |

## Earlier verification findings

| Finding | Closure evidence |
| --- | --- |
| MCW-V001 | PostgreSQL uses `psql --dbname` and redacts the full URL; both existing integration tests pass. |
| MCW-V002 | Query errors fail rollback comparisons; `@claim:query-error-fails` passes. |
| MCW-V003 | Missing rollback config is rejected before mutation; `@claim:rollback-preflight` passes. |
| MCW-V004 | Unavailable paid offer is removed. |
| MCW-V005 | Live CSS/artwork return one-year immutable caching; `sw.js` returns `no-cache`. |
| MCW-V006 / MCW-V008 | Every visible link and button is at least 44×44 at desktop and 390 px. |
| MCW-V007 | License flow and its frequency promise were removed with the unavailable offer. |

## Verification evidence

- Final clean clone: `/tmp/mcw-polish-final-CCB4UL/repo` at commit `e78f18b`.
- `npm ci`: passed, 0 vulnerabilities.
- `npm test`: 13 Rust tests, 1 Vitest test, and 21 applicable Playwright tests passed; one viewport-inapplicable test skipped.
- `npm run lint`: rustfmt and Clippy with warnings denied passed.
- `npm run build`: produced `dist/bin/mcw` and five site HTML routes.
- `cargo package --locked`: packaged and verified 57 files.
- Every distinct command in `.factory/claims.json`: passed from the clean clone.
- Live Playwright: 21 passed, one viewport-inapplicable skip.
- Live axe integration: zero serious or critical findings on home, demo, privacy, terms, and 404 at desktop and 390 px.
- Live factory URL checks: home and demo returned 200 with zero console errors.
- Live route checks: `/`, `/demo/`, `/privacy/`, and `/terms/` returned 200; unknown route returned 404.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.5 s, TBT 0 ms, CLS 0, 132,746 bytes.
- Build sizes: route JS totals under 5 KB uncompressed; CSS 15,020 bytes; no font payload; hero WebP 122,462 bytes.
- Live screenshots: [desktop home](evidence/live-home.webp), [mobile home](evidence/live-home-mobile.webp), [mobile demo](evidence/live-demo-mobile.webp).

No review finding remains open.
