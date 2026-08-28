# Adversarial first-read review 3 — Migration Commit Witness

## Verdict: FAIL

Reviewed 2026-08-28 UTC against live
<https://migration-commit-witness.sociobot.in> and clean-clone commit
`7236cde5d5ab4f014afaa7ccd5abee5bd4504b5d`.

The first screen is clear, the sample result is immediate, all 26 declared
claim commands pass, and the build is healthy. The product still fails this
round because the mandatory demo banner stops being persistent on phones and
four claim areas that round 1 said were removed remain public without registry
entries. Four further operational promises are also absent from
`.factory/claims.json`.

## Findings, ordered by severity

### F-3-1 / BLOCKING-2 — BLOCKING — The demo banner is not persistent on phones

**Quote/location:** `/demo/` at 390×844, “Demo — sample data, nothing is
saved” with “Reset demo” and “Start for real.” In
`site/src/styles.css`, the base `.demo-banner` is `position: sticky`, but the
`max-width: 680px` rule changes it to `position: relative`.

**Observed result:** at the top, the banner occupies y=0–97.7. After scrolling
to y=900, its bounding box is y=-900–-802.3, completely outside the viewport.
Reset also focuses the result panel and can move the banner off-screen. Desktop
keeps the banner at y=0.

**Why this fails the demo:** the demo contract requires the sandbox warning,
Reset, and Start for real controls to remain available while sample data is in
use. A phone visitor inspecting the interactive evidence loses all three.
This reopens review-1 `BLOCKING-2`; the functional sandbox is present, but its
required persistent indicator is only half-fixed.

**Concrete fix:** retain `position: sticky; top: 0` at 390 px. If the two-row
banner is too tall, compact its copy and controls without removing them. Add a
390×844 test that scrolls to the stage panel, asserts the entire banner remains
inside the viewport, activates Reset there, and confirms it remains visible.

### F-3-2 / U46 — BLOCKING — A previously “removed” public claim remains unlisted

**Quote/location:** live `/privacy/`, CLI inputs: “It passes the database URL
to your migration and rollback commands.”

**Why this is unverified:** round-1 claim row U46 identified the same behavior.
`.factory/polish-1.md` says it was “Removed from public copy,” but it remains in
`site/privacy/index.html`. No `.factory/claims.json` entry claims that both
child commands receive the configured URL. Some integration fixtures happen to
use the variable, but no declared test owns this public promise.

**Concrete fix:** add an `environment-forwarding` claim and one tagged test
whose migration and rollback children record a non-secret fingerprint of the
configured value; assert both receive it and the value is absent from retained
output. Otherwise remove the sentence. Because this is an unfixed earlier
finding, it is blocking again.

### F-3-6 / U06 — BLOCKING — A removed no-telemetry claim has returned unlisted

**Quote/location:** landing install section, “The demo needs no config,
account, or database setup.” Live `/privacy/`, Browser demo: “No account or
analytics service is used.”

**Why this is unverified:** round-1 row U06 covered the public local/no-telemetry
promise. The repair replaced it with a narrower same-origin claim, but the live
copy again makes account and analytics promises. `browser-demo-privacy` asserts
only that request origins are same-origin and that storage keys are limited. A
same-origin analytics endpoint would pass, and the test does not assert the
absence of account/auth UI or storage. This is an unfixed earlier claim gap.

**Concrete fix:** list this privacy claim and make its test allow only the
known static resource paths throughout the demo flow, assert no beacon/fetch
to analytics or auth paths, and assert no login control or auth storage. Or
remove the sentence.

### F-3-8 / U52 — BLOCKING — A removed local-processing claim remains on Privacy

**Quote/location:** live `/privacy/` lede: “The CLI processes test database
details on your machine.”

**Why this is unverified:** round-1 row U52 identified the CLI's local-only
claim, and polish 1 says the absolute statement was removed. The current
Privacy sentence still promises where processing occurs. No registry entry
owns that promise, and no declared test intercepts CLI networking or subprocess
destinations for the complete workflow. This is an unfixed earlier finding.

**Concrete fix:** remove the sentence or add a narrowly worded `cli-network`
claim. Its clean-temp-dir test must intercept outbound connections for the
complete demo and configured-command flow and state any allowed database
connection explicitly.

### F-3-9 / U20 / U37 — BLOCKING — Removed tool-scope claims have returned

**Quote/location:** landing, “It does not replace your migration tool.” Live
`/terms/`, “It is not a backup, migration engine, warranty, or replacement for
review.”

**Why this is unverified:** round-1 rows U20 and U37 included the negative
migration-engine and migration-generation claims. Polish 1 says those claims
were removed, but equivalent scope promises remain live. No registry entry
asserts that the CLI creates no migration, backup, or substitute engine output.
Because the earlier claim class is only half-removed, it is blocking again.

**Concrete fix:** replace the capability claims with instructions that do not
need proof: “Keep independent backups and review each migration. The witness
reports only the checks you configure.” Otherwise add a tagged filesystem and
process-boundary test that proves the precisely retained scope statement.

### F-3-3 — MINOR — The public Git installation path is an unlisted claim

**Quote/location:** landing install section, “Build the Rust binary from
source”; README Install, “Build the Rust CLI from source,” followed by
`cargo install --git https://github.com/B-Divyesh/sf-migration-commit-witness --bin mcw`.

**Why this matters:** `build-artifacts` covers `npm run build` and local
`dist/` paths, not the command users are told to run. The command worked in
this review and installed `mcw 0.1.1`, but that result is not repeatably owned
by the claim registry.

**Concrete fix:** add a `git-install` claim and a clean temporary-root test of
the documented Git command, including `mcw --version`, or replace the public
instruction with an installation path already covered by a declared test.

### F-3-4 — MINOR — Starter-config creation is an unlisted claim

**Quote/location:** README, “Create a starter config, then update its commands
and checks,” followed by `mcw init --output mcw.toml`.

**Why this matters:** `mit-free-cli` only checks that `init` appears in help.
No claim entry asserts that the documented command creates a usable config or
refuses to overwrite an existing one.

**Concrete fix:** add an `init-config` claim with a tagged temporary-directory
test that runs the exact command, parses the generated config, uses it in a
witness run, and confirms a second invocation does not overwrite it.

### F-3-5 — MINOR — The non-interactive promise is broader than its claim

**Quote/location:** README Exit codes: “Commands never prompt for input.”

**Why this matters:** `json-stream-contract` promises and tests only the
documented `witness --json` flow with closed stdin. It does not own the broader
statement about every command.

**Concrete fix:** either rewrite this as “The documented `witness --json` flow
does not read stdin,” or add a `non-interactive-commands` claim that runs each
public subcommand with closed stdin and an execution timeout.

### F-3-7 — MINOR — The toolchain compatibility statement is unlisted

**Quote/location:** README Develop and verify: “Requirements: stable Rust,
Node 22+, and npm.”

**Why this matters:** this is a compatibility promise a contributor can rely
on, but no registry entry defines the minimum Rust version or tests the stated
Node range. `Cargo.toml` has no `rust-version` field.

**Concrete fix:** declare `rust-version`, state an exact Node support range,
and add a CI/claim matrix for the minimum versions. Otherwise replace the
sentence with the exact versions used for the verified build.

## 1. Cold first screen

Fresh Chromium contexts were opened at 390×844 and 1440×900 with no stored
data. No scrolling occurred before this reading.

| View | What it does | For whom | What to click first |
| --- | --- | --- | --- |
| 390×844 | Checks what a SQL migration committed and saves before, after, and rollback evidence. | Backend teams reviewing migrations. | “Try it with sample data.” |
| 1440×900 | The same SQL migration evidence job, with explanatory artwork alongside it. | Backend teams reviewing migrations. | “Try it with sample data.” |

The exact copy that answered all three questions was:

> “Prove what your SQL migration committed”
>
> “For backend teams reviewing migrations, run one on a test database and
> save signed before, after, and rollback evidence.”
>
> “Try it with sample data” — “See a partial commit and verified rollback;
> nothing is saved.”

This part passes. The headline is six words, the supporting sentence is 19
words, and the primary sample action is fully visible on the phone.

## 2. Copy audit

Counts use whitespace-separated words; punctuation, paths, flags, and
hyphenated terms remain attached to one token. Code blocks are commands rather
than sentences. Alternative text, collapsed transcript sentences, and runtime
status/error sentences are included because users can encounter them.

### Landing-page sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 19 | For backend teams reviewing migrations, run one on a test database and save signed before, after, and rollback evidence. | — |
| 10 | See a partial commit and verified rollback; nothing is saved. | — |
| 6 | The demo reloads after one visit. | — |
| 6 | Only demo session state is stored. | — |
| 6 | The CLI uses the MIT License. | — |
| 12 | A concrete database core shows checked layers around a dark commit line. | — |
| 9 | A successful migration command does not prove database state. | — |
| 6 | Compare checks before and after commit. | — |
| 8 | A successful command can leave the wrong tables. | — |
| 12 | The sample command returns 0 after creating one of two required tables. | — |
| 10 | The CLI records the failed check and then tests rollback. | — |
| 7 | Save checks before, after, and after rollback. | — |
| 11 | Add one-value queries for the schema and data your release requires. | — |
| 13 | The CLI runs your configured commands only after you confirm a test target. | — |
| 10 | Keep matching JSON and Markdown files with the release review. | — |
| 5 | Watch the bundled sample run. | — |
| 8 | This self-hosted terminal recording comes from `mcw demo`. | — |
| 8 | Run the same sample locally in one command. | — |
| 19 | Terminal recording of `mcw demo` showing a partial commit, restored rollback, temporary workspace, and JSON and Markdown witness paths. | — |
| 21 | It creates a separate temporary folder, detects one of two required tables, restores the starting checks, and writes both witness files. | — |
| 9 | DEMO: partial commit detected; rollback restored the starting checks. | — |
| 8 | Sample data is isolated in this temporary workspace. | — |
| 4 | Delete it when finished. | — |
| 7 | It does not choose a production target. | — |
| 11 | A test environment label, confirmation flag, and URL-name checks are required. | — |
| 6 | It does not invent a rollback. | — |
| 12 | Rollback runs only when the config and command flag both request it. | — |
| 6 | It does not retain database secrets. | — |
| 10 | Database URLs and signing keys are omitted from witness files. | — |
| 7 | It does not replace your migration tool. | F-3-9, unlisted claim |
| 10 | The config names the exact command your team already reviews. | — |
| 7 | Run the sample, then add your config. | — |
| 6 | Build the Rust binary from source. | F-3-3, unlisted claim |
| 9 | The demo needs no config, account, or database setup. | F-3-6, unlisted account claim |
| 8 | The CLI accepts `test`, `ci`, `development`, or `ephemeral`. | — |
| 4 | It also requires `--confirm-test-database`. | — |
| 6 | See the checks for each database. | — |
| 8 | Save migration and rollback results with each release. | — |
| 4 | Built by Param Factory. | — |
| 3 | Install command copied. | — |
| 3 | Copy was blocked. | — |
| 6 | Select the command and copy it. | — |

### README sentences

| Words | Sentence | Flag |
| ---: | --- | --- |
| 15 | Migration Commit Witness is a command-line tool for backend teams reviewing SQL migrations in CI. | — |
| 8 | Run your migration on a confirmed test database. | — |
| 13 | The CLI records checks before commit, after commit, and after an optional rollback. | — |
| 5 | It writes JSON and Markdown. | — |
| 5 | Live site and sample: `https://migration-commit-witness.sociobot.in/demo/` | — |
| 6 | Build the Rust CLI from source. | F-3-3, unlisted claim |
| 5 | The project is MIT licensed. | — |
| 10 | Core commands do not require payment or a license token. | — |
| 7 | Run the bundled partial-commit case without setup. | — |
| 11 | The command creates a new temporary folder and prints its path. | — |
| 19 | It seeds SQLite, runs the real witness path, detects one missing table, exercises rollback, and writes both witness formats. | — |
| 10 | The sample does not read or write the current directory. | — |
| 6 | Delete the printed folder when finished. | — |
| 10 | Create a starter config, then update its commands and checks. | F-3-4, unlisted claim |
| 4 | `witness.json` is for tools. | — |
| 6 | `witness.md` is formatted for pull-request review. | — |
| 9 | Verify the signed JSON later with the same key. | — |
| 16 | The config names the database type, URL environment variable, migration command, rollback command, and one-value checks. | — |
| 7 | Each check query must return one value. | — |
| 6 | A query error fails the run. | — |
| 8 | Matching errors do not count as matching values. | — |
| 5 | SQLite uses its embedded library. | — |
| 6 | It records `quick_check` and foreign-key results. | — |
| 9 | PostgreSQL uses `psql --dbname` and records a connection probe. | — |
| 11 | The CLI requires two confirmations before it runs a configured command. | — |
| 10 | The config environment must be `test`, `ci`, `development`, or `ephemeral`. | — |
| 5 | The command must include `--confirm-test-database`. | — |
| 9 | URLs containing `prod`, `production`, `primary`, or `live-db` are rejected. | — |
| 8 | These name checks cannot identify every production database. | — |
| 4 | Review the target yourself. | — |
| 7 | Rollback requires a configured command and `--exercise-rollback`. | — |
| 10 | Both are checked before any database query or command runs. | — |
| 10 | Database URLs and signing keys are omitted from witness files. | — |
| 7 | PostgreSQL errors redact the configured database URL. | — |
| 9 | With `--json`, the final result is written to stdout. | — |
| 5 | Progress and errors use stderr. | — |
| 5 | Commands never prompt for input. | F-3-5, claim scope mismatch |
| 7 | Requirements: stable Rust, Node 22+, and npm. | F-3-7, unlisted claim |
| 7 | The build writes the binary to `dist/bin/mcw`. | — |
| 7 | It writes the static site to `dist/site/`. | — |
| 11 | The browser demo uses same-origin files and a `demo:` session key. | — |
| 8 | The CLI omits configured secrets from witness files. | — |
| 6 | See the privacy page and terms. | — |

No sentence exceeds 22 words. No banned marketing adjective appears. The
headings make sense in the page outline. The actions “Try it with sample data,”
“Install the CLI,” “Open the sample result,” “Copy install command,” and “Read
the config reference” name their outcomes. Core terms—CLI, run, witness, check,
config, and sample/demo—remain consistent enough for the intended technical
reader.

## 3. Demo and sandbox

- One click from the home page opens `/demo/`.
- At 390×844, the initial screen shows “Partial commit detected,” `1 / 2`,
  missing `audit_log`, and `0 / 2 restored` without scrolling.
- The sample is realistic and matches the bundled CLI record.
- Reset returns to stage 2. An injected `real:data` session key and
  `real:local` local-storage key both remained unchanged.
- Start for real removed `demo:mcw:stage`, preserved both injected non-demo
  keys, and opened `/#install`.
- All browser-demo requests were same-origin. The offline claim passed after an
  online service-worker visit; reload and Reset worked offline.
- The banner persistence failure is F-3-1.
- Running `dist/bin/mcw demo` from the empty directory
  `/tmp/mcw-review3-caller-tVbRF4` returned 0, left the caller empty, and wrote
  the sample under `/tmp/mcw-demo-1787920932164-7476`.

## 4. Claims

The clean clone was `/tmp/mcw-review3-clean-IcwItS/repo`. Every one of the 26
registry rows was executed with its exact `test` command, including repeated
commands. All declared tests passed.

| Claim id | Result |
| --- | --- |
| `terminal-recording` | PASS |
| `demo-first-result` | PASS |
| `demo-isolation` | PASS |
| `demo-evidence` | PASS |
| `demo-record-match` | PASS |
| `sqlite-builtins` | PASS |
| `evidence-formats` | PASS |
| `rollback-exercise` | PASS |
| `secret-omission` | PASS |
| `signed-witness` | PASS |
| `partial-commit-detection` | PASS |
| `rollback-preflight` | PASS |
| `test-confirmation` | PASS |
| `url-name-guards` | PASS |
| `environment-labels` | PASS |
| `one-value-checks` | PASS |
| `postgres-psql` | PASS |
| `secret-redaction` | PASS |
| `mit-free-cli` | PASS |
| `demo-route-isolation` | PASS |
| `browser-demo-privacy` | PASS |
| `offline-demo` | PASS |
| `json-stream-contract` | PASS |
| `exit-code-contract` | PASS |
| `build-artifacts` | PASS |
| `query-error-fails` | PASS |

Each id appears exactly once as `@claim:<id>` in the repository. The unlisted
or under-scoped public claims are F-3-2 through F-3-7; therefore the claims
audit is not complete despite all declared commands passing.

## 5. Structure, routing, accessibility, and identity

- `/`, `/demo/`, `/privacy/`, and `/terms/` return 200. An unknown path returns
  the designed 404 with a home action.
- Each route has its own compliant title, one h1, one main landmark, plain meta
  description, canonical, OG/Twitter image data, SVG favicon, and Apple icon.
- Direct demo links and `?demo=1` work. The existing live Playwright suite
  confirms route focus plus Back/Forward restoration.
- The internal and GitHub link crawl returned 200 for every destination. The
  home `#install` target exists.
- The shared header/footer include Demo, Privacy, Terms, the one-line product
  description, and build `0.1.1+polish.2`.
- The live Playwright/axe suite found zero serious or critical findings at
  desktop and 390 px; 26 tests passed and two viewport-only cases skipped.
- `/opt/fleet/lib/verify-url.sh` returned 200 with zero console errors and
  confirmed title, `lang=en`, one h1, main, alt text, and button labels.
- Every visible link/button measured at least 44×44 px in the supplied suite;
  keyboard tabs, skip link, visible focus, and reduced motion passed.
- The production build loads about 3 KB of route JavaScript plus a 1.99 KB
  shared chunk uncompressed, well below the 150 KB limit.
- Live cache policy is correct: hashed CSS, artwork, and terminal SVG are
  immutable for one year; `sw.js` is `no-cache`.
- The concrete-and-moss evidence-sheet identity remains distinct and directly
  related to transaction boundaries. It is not a generic SaaS template.

## 6. Earlier-finding verification

Every earlier review/polish document and both handoff/verification histories
were read. Results below are based on the live site and current code, not the
closure labels in those documents.

| Earlier id | Current verification |
| --- | --- |
| `BLOCKING-1` | Fixed: job, audience, and primary sample action are visible before scrolling at both sizes. |
| `BLOCKING-2` | **Reopened by F-3-1:** sample output is now immediate and the CLI demo is isolated, but the required banner is not persistent on mobile. |
| `BLOCKING-3` | Fixed: registry exists, every id has one tag, and all 26 listed commands pass. |
| `BLOCKING-4` | Fixed: demo/deep links, route focus, Back/Forward, sitemap, and designed 404 pass. |
| `BLOCKING-5` | Fixed: unavailable paid checkout and license UI remain absent. |
| `HIGH-1` | Fixed: title, descriptions, canonical, OG/Twitter art, favicon, and Apple icon are live. |
| `HIGH-2` | Fixed: header/footer and build identity are consistent. |
| `HIGH-3` | Fixed: live preview, three-step workflow, limits/privacy, and footer follow the required order. |
| `HIGH-4` | Fixed: absolute token and production-connection statements remain absent. |
| `MEDIUM-1` | Fixed: target geometry passes at desktop and 390 px. |
| `MEDIUM-2` | Fixed: current landing/README sentences stay under 22 words, avoid banned marketing terms, and use result-naming actions. |
| `F-2-1` | Fixed: the first phone demo view now contains the real partial result and rollback value. |
| `F-2-2` | Fixed: the landing page contains a self-hosted recording generated from `mcw demo` plus an accessible transcript. |
| `MCW-V001` | Fixed: declared PostgreSQL `--dbname` and URL-redaction tests pass. |
| `MCW-V002` | Fixed: matching query errors fail the run in the declared regression. |
| `MCW-V003` | Fixed: missing rollback configuration is rejected before mutation. |
| `MCW-V004` | Fixed by scope removal: no paid offer remains. |
| `MCW-V005` | Fixed: live immutable and service-worker cache headers match policy. |
| `MCW-V006`, `MCW-V008` | Fixed: live target geometry passes. |
| `MCW-V007` | Fixed by scope removal: license verification no longer exists. |
| `U01–U45`, `U47–U54` | Retained claims map to declared tests; removed claims remain absent, except for the new gaps F-3-3 through F-3-7. |
| `U46` | **Not fixed; reopened by F-3-2:** environment forwarding remains public on `/privacy/` without a claim entry. |

## 7. Build and verification log

- Clean clone SHA: `7236cde5d5ab4f014afaa7ccd5abee5bd4504b5d`.
- `npm ci`: PASS, 59 packages, zero audit vulnerabilities.
- `npm test`: PASS — 3 Rust unit, 10 Rust integration, 1 Vitest, and 26
  applicable Playwright tests; 2 viewport-only skips.
- `npm run lint`: PASS — rustfmt and locked Clippy with warnings denied.
- `npm run build`: PASS — `dist/bin/mcw` and `dist/site/` produced.
- Live suite: PASS — 26 applicable tests, 2 viewport-only skips.
- Factory URL verifier: PASS — evidence directory
  `/tmp/mcw-review3-verify-url-rvOmuN`.
- Exact README Git install: PASS in an isolated install root; `mcw --version`
  returned `mcw 0.1.1`.
- No product code was modified during this review.

## 8. Missed leverage

No AI feature is warranted. The job is deterministic database execution,
comparison, signing, and verification; a model would weaken auditability. JSON
and Markdown export already cover the obvious portability need, and remote sync
would conflict with the local/test-database boundary. No missed-leverage
finding is raised.

## What would make this perfect

Keep the complete demo banner sticky on phones and add the scrolled-state
regression. Then resolve every public claim gap: restore U46 to the registry or
remove it, register the Git install and `init` workflows, narrow or test the
non-interactive statement, register the no-account/no-analytics promise, and
make toolchain compatibility explicit and tested. Re-run all 26 current claim
commands plus the new claim tests from a fresh clone. Nothing else remains from
this review.
