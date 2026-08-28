# Adversarial first-read review 4 — Migration Commit Witness

## Verdict: PASS

Reviewed 2026-08-28 UTC against the deployed site at
<https://migration-commit-witness.sociobot.in> and a clean clone of commit
`df8ac1822599b7be2cc79bd567d8fd26aa3645b8`.

There are **zero findings**. The product is clear before scrolling, the sample
is immediately useful and isolated, every listed claim was exercised, and the
earlier finding set remains fixed in both deployed behaviour and current code.

## 1. Cold first read

Fresh Chromium browser contexts with no existing storage were used. The page
was read before scrolling at both 390×844 and 1440×900.

| View | What it does | For whom | First action |
| --- | --- | --- | --- |
| 390×844 | Checks what a SQL migration committed and saves checks before, after, and after rollback. | Backend teams reviewing migrations. | **Try it with sample data**. |
| 1440×900 | The same migration-evidence job, with a transaction-boundary illustration beside it. | Backend teams reviewing migrations. | **Try it with sample data**. |

The first screen answers all three questions with:

> “Prove what your SQL migration committed”
>
> “For backend teams reviewing migrations, run one on a test database and save
> signed before, after, and rollback evidence.”
>
> “Try it with sample data” — “See a partial commit and verified rollback;
> nothing is saved.”

The h1 is six words, the audience/result sentence is 19 words, and the primary
action plus its outcome are visible in the phone viewport.

## 2. Copy audit

Counts use whitespace-separated words. Commands are excluded. Alternative
text, transcript content, and runtime status content are included because a
visitor can encounter them. No sentence exceeds 22 words. No banned marketing
adjective, unexplained metaphor, terminology conflict, ambiguous heading, or
non-result-naming button was found.

### Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 19 | For backend teams reviewing migrations, run one on a test database and save signed before, after, and rollback evidence. |
| 10 | See a partial commit and verified rollback; nothing is saved. |
| 7 | The demo reloads after one visit. |
| 6 | Only demo session state is stored. |
| 6 | The CLI uses the MIT License. |
| 12 | A concrete database core shows checked layers around a dark commit line. |
| 9 | A successful migration command does not prove database state. |
| 6 | Compare checks before and after commit. |
| 8 | A successful command can leave the wrong tables. |
| 12 | The sample command returns 0 after creating one of two required tables. |
| 10 | The CLI records the failed check and then tests rollback. |
| 7 | Save checks before, after, and after rollback. |
| 11 | Add one-value queries for the schema and data your release requires. |
| 13 | The CLI runs your configured commands only after you confirm a test target. |
| 10 | Keep matching JSON and Markdown files with the release review. |
| 5 | Watch the bundled sample run. |
| 8 | This self-hosted terminal recording comes from `mcw demo`. |
| 8 | Run the same sample locally in one command. |
| 19 | Terminal recording of `mcw demo` showing a partial commit, restored rollback, temporary workspace, and JSON and Markdown witness paths. |
| 21 | It creates a separate temporary folder, detects one of two required tables, restores the starting checks, and writes both witness files. |
| 9 | DEMO: partial commit detected; rollback restored the starting checks. |
| 8 | Sample data is isolated in this temporary workspace. |
| 4 | Delete it when finished. |
| 7 | Choose a test target yourself. |
| 11 | A test environment label, confirmation flag, and URL-name checks are required. |
| 7 | Choose a rollback command yourself. |
| 12 | Rollback runs only when the config and command flag both request it. |
| 7 | Keep database secrets out of the witness. |
| 10 | Database URLs and signing keys are omitted from witness files. |
| 7 | Keep your migration tool and review. |
| 5 | Configure the command to run. |
| 4 | Keep independent backups. |
| 7 | Run the sample, then add your config. |
| 6 | Build the Rust binary from source. |
| 9 | The demo needs no config, account, or database setup. |
| 13 | The CLI accepts `test`, `ci`, `development`, or `ephemeral`. |
| 5 | It also requires `--confirm-test-database`. |
| 8 | Save migration and rollback results with each release. |
| 5 | Built by Param Factory. |
| 3 | Install command copied. |
| 3 | Copy was blocked. |
| 6 | Select the command and copy it. |

Headings are concrete in isolation: “Prove what your SQL migration committed,”
“Save checks before, after, and after rollback,” “Watch the bundled sample
run,” “What to keep in your release process,” “Run the sample, then add your
config,” and “See the checks for each database.” Buttons name their result:
“Try it with sample data,” “Install the CLI,” “Open the sample result,” “Copy
install command,” and “Read the config reference.”

### README sentences

| Words | Sentence |
| ---: | --- |
| 15 | Migration Commit Witness is a command-line tool for backend teams reviewing SQL migrations in CI. |
| 8 | Run your migration on a confirmed test database. |
| 13 | The CLI records checks before commit, after commit, and after an optional rollback. |
| 5 | It writes JSON and Markdown. |
| 5 | Live site and sample: `https://migration-commit-witness.sociobot.in/demo/` |
| 6 | Build the Rust CLI from source. |
| 5 | The project is MIT licensed. |
| 10 | Core commands do not require payment or a license token. |
| 7 | Run the bundled partial-commit case without setup. |
| 11 | The command creates a new temporary folder and prints its path. |
| 19 | It seeds SQLite, runs the real witness path, detects one missing table, exercises rollback, and writes both witness formats. |
| 10 | The sample does not read or write the current directory. |
| 6 | Delete the printed folder when finished. |
| 10 | Create a starter config, then update its commands and checks. |
| 4 | `witness.json` is for tools. |
| 6 | `witness.md` is formatted for pull-request review. |
| 9 | Verify the signed JSON later with the same key. |
| 16 | The config names the database type, URL environment variable, migration command, rollback command, and one-value checks. |
| 7 | Each check query must return one value. |
| 6 | A query error fails the run. |
| 8 | Matching errors do not count as matching values. |
| 5 | SQLite uses its embedded library. |
| 6 | It records `quick_check` and foreign-key results. |
| 9 | PostgreSQL uses `psql --dbname` and records a connection probe. |
| 11 | The CLI requires two confirmations before it runs a configured command. |
| 10 | The config environment must be `test`, `ci`, `development`, or `ephemeral`. |
| 5 | The command must include `--confirm-test-database`. |
| 9 | URLs containing `prod`, `production`, `primary`, or `live-db` are rejected. |
| 8 | These name checks cannot identify every production database. |
| 4 | Review the target yourself. |
| 7 | Rollback requires a configured command and `--exercise-rollback`. |
| 10 | Both are checked before any database query or command runs. |
| 10 | Database URLs and signing keys are omitted from witness files. |
| 7 | PostgreSQL errors redact the configured database URL. |
| 9 | With `--json`, the final result is written to stdout. |
| 5 | Progress and errors use stderr. |
| 5 | Commands never prompt for input. |
| 11 | Requirements: Rust 1.85 or later, Node 22 or later, and npm. |
| 7 | The CI matrix verifies these minimum versions. |
| 7 | The build writes the binary to `dist/bin/mcw`. |
| 7 | It writes the static site to `dist/site/`. |
| 11 | The browser demo uses same-origin files and a `demo:` session key. |
| 8 | The CLI omits configured secrets from witness files. |
| 6 | See the privacy page and terms. |

## 3. Demo and sandbox

- One click from the landing action opens `/demo/`; `/?demo=1` redirects there.
- The initial 390×844 view already shows “Partial commit detected,” `1 / 2`,
  missing `audit_log`, and `0 / 2 restored`.
- The persistent banner says “Demo — sample data, nothing is saved.” Reset
  returns to the partial-commit stage and leaves an injected non-demo session
  key untouched. Start for real clears only the `demo:mcw:` namespace.
- Request interception observed only same-origin static product resources. No
  account, analytics, or login path was requested. After one online visit, the
  service-worker-backed demo reloaded and Reset worked with the browser offline.
- A direct run of `dist/bin/mcw demo` from an empty temporary caller directory
  returned 0, created its sample under a separate `/tmp/mcw-demo-*` directory,
  and left the caller directory empty.

## 4. Claims

The clean clone was `/tmp/mcw-review4-clean-bdSafX/repo`. `npm ci` completed
with 59 packages and zero audit vulnerabilities. All 32 commands declared in
`.factory/claims.json` were run exactly, including repeated commands; all
passed. The audit found 32 registry ids, 32 unique `@claim:` tags, no missing
tags, no extra tags, and no duplicates.

| Claim ids verified | Result |
| --- | --- |
| `terminal-recording`, `demo-first-result`, `demo-isolation`, `demo-evidence`, `demo-record-match`, `sqlite-builtins`, `evidence-formats`, `rollback-exercise` | PASS |
| `secret-omission`, `signed-witness`, `partial-commit-detection`, `rollback-preflight`, `test-confirmation`, `url-name-guards`, `environment-labels`, `one-value-checks` | PASS |
| `postgres-psql`, `secret-redaction`, `mit-free-cli`, `demo-route-isolation`, `browser-demo-privacy`, `offline-demo`, `json-stream-contract`, `exit-code-contract` | PASS |
| `build-artifacts`, `query-error-fails`, `environment-forwarding`, `git-install`, `init-config`, `non-interactive-commands`, `toolchain-compatibility`, `demo-banner-persistence` | PASS |

The public landing and README claims map to these declared tests. In
particular, the earlier unlisted Git install, starter-config, non-interactive,
toolchain, no-account/analytics, and database-URL-forwarding statements now
have owned claim tests. No unlisted claim finding is raised.

## 5. Structure, accessibility, and identity

- `/`, `/demo/`, `/privacy/`, and `/terms/` return 200. An unknown route
  returns the designed 404 with “Return to the home page.”
- Route titles are distinct and follow the required pattern; all routes have
  one h1, one main landmark, description, canonical, OG/Twitter metadata,
  favicon, and Apple touch icon.
- Direct links, reload, Back/Forward, focus transfer to the h1, and live route
  announcement passed. All internal/external destinations in the deployed link
  crawl responded successfully; the 404 page’s local skip link intentionally
  retains its 404 document response.
- The deployed end-to-end suite passed with 27 tests and 3 intentional
  desktop skips for phone-only assertions. Axe found no serious or critical
  issue. Keyboard, 44 px targets, mobile overflow, and reduced motion passed.
- Header/footer wording, legal links, and the runtime build identifier are
  consistent across routes.
- The concrete-and-moss evidence sheet is visually distinct and fits a release
  audit boundary. It is not a generic SaaS template.

## 6. Earlier-finding verification

Every earlier `review-*`, `polish-*`, and handoff document was read. The
following findings were confirmed fixed on the deployed product and in code:

| Earlier finding group | Current verification |
| --- | --- |
| `BLOCKING-1` through `BLOCKING-5` | Plain first screen, real isolated demo, one-to-one claims, real routes/404/focus, and removal of unavailable billing remain correct. |
| `HIGH-1` through `HIGH-4`; `MEDIUM-1` and `MEDIUM-2` | Metadata/assets, shared skeleton/build identity, information order, honest safety wording, target geometry, and plain terminology remain correct. |
| `F-2-1`, `F-2-2`, `F-3-1` through `F-3-9` | The immediate phone result, generated terminal recording, persistent banner, exact privacy/forwarding claim, and all formerly unlisted workflow/toolchain claims remain covered. |
| `MCW-V001` through `MCW-V008`; `U01` through `U54` | Retained claims pass their current tests. Removed paid, telemetry, over-broad local-processing, and migration-replacement claims remain absent. |

## 7. Verification log

```sh
npm ci
# every exact .factory/claims.json command (32 total)
npm test
PLAYWRIGHT_BASE_URL=https://migration-commit-witness.sociobot.in npm run test:e2e
```

All commands passed. `npm test` reported 3 Rust unit tests, 13 Rust
integration tests, 1 Vitest test, and 27 Playwright passes with 3 intentional
viewport-only skips. The production build emits `dist/bin/mcw` and `dist/site`.

## 8. Missed leverage

No finding. The brief calls for deterministic execution, comparison, and
signed evidence. An AI-assisted step would weaken the audit boundary rather
than improve the stated job. JSON and Markdown exports cover the obvious
portable-output need; remote sync would conflict with the test-database and
privacy boundary.

## What would make this perfect

No product change is required for this round. Preserve the current one-click
sample, narrow claim registry, and fresh-clone verification in future releases;
any new public promise should receive its own sandboxed claim test before it is
published.
