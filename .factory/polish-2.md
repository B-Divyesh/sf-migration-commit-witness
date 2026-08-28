# Polish round 2 — finding closure

Candidate repaired: `6d08e132fb13a6689283fe46992a6059d61fa337`
Review repaired: `fbcfef03e32d534e06e0548dd866be1fda2a8167`
Repair build: `0.1.1+polish.2`

## Current review findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 / BLOCKING-2 | Moved the real initial result ahead of explanation on `/demo/`: the first view now shows “Partial commit detected,” `1 / 2`, the missing `audit_log` check, and restored `0 / 2`. The existing demo banner, Reset demo, Start for real, session namespace, direct `?demo=1`, and CLI temp workspace remain intact. | `@claim:demo-first-result`; `@claim:demo-route-isolation`; [live mobile screenshot](evidence/live-polish-2-demo-phone.png); live <https://migration-commit-witness.sociobot.in/demo/>. |
| F-2-2 | Added `site/generate-demo-recording.mjs` and committed `mcw-demo-recording.svg`. The generator runs the release binary’s real `mcw demo` in a fresh temporary directory, then redacts only the random path. The landing page has the self-hosted SVG and an accessible transcript showing the partial verdict, restored rollback, workspace, JSON, and Markdown paths. | `npm run generate:demo-recording`; `@claim:terminal-recording`; [recording screenshot](evidence/polish-2-home-recording.png); live <https://migration-commit-witness.sociobot.in/>. |

## Earlier review findings rechecked

| Finding ids | Result and evidence |
| --- | --- |
| BLOCKING-1 | Still fixed: the home first screen names SQL migrations and backend reviewers and exposes the sample action. `home first screen names the job, audience, and sample action`; `.factory/evidence/polish-2-home-phone.png`. |
| BLOCKING-3 | Still fixed and expanded to 26 entries. The tag audit reports exactly one `@claim:<id>` tag for every claims entry; every declared command is rerun from the clean clone. |
| BLOCKING-4, HIGH-1, HIGH-2 | Still fixed: real `/demo/`, direct query entry, focus/title/canonical metadata, consistent navigation/footer, legal routes, and designed 404. `routes have distinct metadata, focused headings, and working back navigation`; `unknown paths return the designed 404 with a way home`; live route checks. |
| BLOCKING-5, HIGH-4, MCW-V004, MCW-V007 | Still fixed: unavailable paid flow, checkout copy, token storage, and unsupported absolute promises remain absent. Link crawl and `rg 'checkout|license-token|\$49' site` are clean. |
| HIGH-3, MEDIUM-2 | Still fixed: three-step workflow, named limits/privacy section, short plain copy, and stable terms remain. `.factory/copy-audit.md`; landing copy audit. |
| MEDIUM-1, MCW-V006, MCW-V008 | Still fixed: every visible control is at least 44 px at desktop and 390 px. `skip link and every visible target meet keyboard and 44px geometry rules`. |
| MCW-V001–V003 | Still fixed: PostgreSQL `--dbname`/redaction, query-error failure, and rollback preflight pass their declared claim tests. |
| MCW-V005 | Still fixed and extended: the new terminal SVG receives immutable caching; the service worker remains `no-cache`. `Azure Static Web Apps response policy`. |
| U01–U54 | Rechecked against the round-1 disposition: retained claims still map one-to-one to the current registry and clean-sandbox tests; removed claims remain absent from the landing page and README. The new recording and first-result claims are `terminal-recording` and `demo-first-result`. |

## Evidence commands

Run from a clean clone:

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

Then run each exact command in `.factory/claims.json`. The proof runs and live
checks are recorded in `.factory/handoff.md` after the repair is deployed.
