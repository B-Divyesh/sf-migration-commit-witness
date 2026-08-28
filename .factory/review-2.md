# Adversarial first-read review 2 — Migration Commit Witness

## Verdict: FAIL

Reviewed 2026-08-28 UTC against live
<https://migration-commit-witness.sociobot.in> and clean-clone commit
`6d08e132fb13a6689283fe46992a6059d61fa337`.

There are two blocking demo findings. All declared claims passed, and the
landing page is clear before scrolling. The sample experience nevertheless
requires a phone visitor to scroll before seeing any actual witness result, and
the CLI landing page lacks the required self-hosted terminal recording.

## Cold first screen

Fresh Chromium contexts, with no prior storage, were opened at 390×844 and
1440×900 before scrolling.

| View | What it does | For whom | First action |
| --- | --- | --- | --- |
| 390×844 | It checks what a SQL migration actually changed and saves before, after, and rollback evidence. | Backend teams reviewing migrations. | **Try it with sample data**. |
| 1440×900 | The same migration evidence tool. | Backend teams reviewing migrations. | **Try it with sample data**. |

The exact first-screen copy succeeds at the required explanation:

> “Prove what your SQL migration committed”
>
> “For backend teams reviewing migrations, run one on a test database and
> save signed before, after, and rollback evidence.”
>
> “Try it with sample data” — “See a partial commit and verified rollback;
> nothing is saved.”

## Findings

### F-2-1 — BLOCKING — The phone demo opens on explanation, not sample evidence

**Earlier finding:** this reopens **BLOCKING-2** from review 1. The functional
CLI/browser demo was added, but its required first-view demonstration is still
only half-fixed on a 390 px phone.

**Location / quote:** live `/demo/` at 390×844. The first screen shows
“Inspect a partial SQL commit,” “The real CLI returned a failed witness after
one of two tables appeared,” and “Step through the saved checks.” The actual
demo shell starts at **y=864.6**, its “Partial commit detected” verdict starts
at **y=915.4**, and the first sample output starts at **y=1377.1**. None is
visible in the initial 844 px viewport.

**Why this fails first use:** a visitor clicked the one-click sample action to
see the product working. On the phone they receive another full screen of
explanation and must discover that the result is below the fold. This fails the
required demo condition that the first screen after the click already show
realistic sample data. Desktop only exposes the verdict at the lower edge;
the actual check values are below the fold there too.

**Concrete fix:** put a compact, real initial result directly below the demo
banner and header: show “Partial commit detected,” `required tables: 1 / 2`,
the failed check text, and the rollback-restored value. Make the current large
intro a short caption after that result. Add a 390×844 Playwright regression
that enters `/?demo=1` and asserts the initial verdict plus at least one
realistic sample value are within the viewport without scrolling.

### F-2-2 — BLOCKING — The CLI landing page has no self-hosted terminal recording

**Location / quote:** the landing page’s “03 / real sample” section says
“The browser view reads a recorded artifact from `mcw demo`” and shows only:

```text
mcw demo
# Creates an isolated temporary folder
# Writes witness/witness.json and witness.md
```

The interactive `/demo/` page renders a browser tab panel from
`/demo-record.json`; it is not a terminal recording on the landing page.

**Why this fails first use:** this is a CLI product. The supplied CLI demo
contract requires a self-hosted terminal recording of the real binary doing its
main job on the bundled sample, in addition to `mcw demo`. The command itself
does work: from an empty temporary caller directory it printed a distinct
`/tmp/mcw-demo-…` workspace and left the caller directory empty. A landing
visitor, however, cannot watch that real command produce the witness or learn
what its terminal output looks like.

**Concrete fix:** add a local SVG/asciinema-style recording generated from a
fresh `mcw demo` run to the landing page’s real-sample section. It should show
the temporary workspace, partial-commit verdict, rollback restoration, and the
two witness paths. Keep it self-hosted, add an accessible text transcript, and
test its displayed values against the bundled CLI record. Preserve the existing
browser sandbox as a supplementary inspection view.

## Copy audit

Counts use whitespace-separated words; code examples, navigation labels,
headings, buttons, and table cells are audited separately below. No prose
sentence exceeds 22 words. No banned marketing adjective, inconsistent core
term, or unexplained jargon needing a rewrite was found in this pass.

### Landing prose

| Words | Sentence |
| ---: | --- |
| 18 | For backend teams reviewing migrations, run one on a test database and save signed before, after, and rollback evidence. |
| 10 | See a partial commit and verified rollback; nothing is saved. |
| 7 | The demo reloads after one visit. |
| 6 | Only demo session state is stored. |
| 6 | The CLI uses the MIT License. |
| 8 | A successful migration command does not prove database state. |
| 6 | Compare checks before and after commit. |
| 12 | The sample command returns 0 after creating one of two required tables. |
| 10 | The CLI records the failed check and then tests rollback. |
| 11 | Add one-value queries for the schema and data your release requires. |
| 13 | The CLI runs your configured commands only after you confirm a test target. |
| 10 | Keep matching JSON and Markdown files with the release review. |
| 11 | The browser view reads a recorded artifact from `mcw demo`. |
| 8 | Run the same sample locally in one command. |
| 11 | A test environment label, confirmation flag, and URL-name checks are required. |
| 12 | Rollback runs only when the config and command flag both request it. |
| 10 | Database URLs and signing keys are omitted from witness files. |
| 10 | The config names the exact command your team already reviews. |
| 6 | Build the Rust binary from source. |
| 9 | The demo needs no config, account, or database setup. |
| 13 | The CLI accepts `test`, `ci`, `development`, or `ephemeral`. |
| 5 | It also requires `--confirm-test-database`. |
| 9 | Save migration and rollback results with each release. |
| 5 | Built by Param Factory. |

### README prose

| Words | Sentence |
| ---: | --- |
| 15 | Migration Commit Witness is a command-line tool for backend teams reviewing SQL migrations in CI. |
| 8 | Run your migration on a confirmed test database. |
| 13 | The CLI records checks before commit, after commit, and after an optional rollback. |
| 5 | It writes JSON and Markdown. |
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
| 7 | Requirements: stable Rust, Node 22+, and npm. |
| 7 | The build writes the binary to `dist/bin/mcw`. |
| 7 | It writes the static site to `dist/site/`. |
| 11 | The browser demo uses same-origin files and a `demo:` session key. |
| 8 | The CLI omits configured secrets from witness files. |
| 6 | See the privacy page and terms. |

### Labels and terminology

- Headings are understandable in context and retain a proper h1 → h2 → h3
  outline. Buttons name outcomes: “Try it with sample data,” “Install the
  CLI,” “Open the sample result,” “Copy install command,” and “Reset demo.”
- The terminology remains consistent: **CLI**, **run**, **witness**, **check**,
  **config**, and **sample/demo**. “One-value query” is immediately explained
  as a query that “must return one value.”
- Every claim-like landing and README statement was matched to a relevant
  `.factory/claims.json` entry and its declared observable test. No unlisted
  claim finding was raised.

## Claims and sandbox evidence

`.factory/claims.json` contains 24 entries. Every distinct declared test command
was run from a fresh clone at the reviewed SHA; all passed. Shared commands were
run once for every group of claims that names them.

| Claim area | Clean-clone evidence |
| --- | --- |
| CLI sample, SQLite checks, browser-record match | `cargo test --test claims demo_runs_in_one_isolated_directory_and_matches_the_browser_record` — pass |
| Evidence formats, rollback, secret omission, JSON stream | `cargo test --test cli documented_sqlite_migration_and_rollback_pass` — pass |
| Signing, URL labels/guards, one-value checks | declared `cargo test --lib …` commands — pass |
| Partial commit, rollback preflight, confirmation, PostgreSQL/redaction, query error | declared focused `cargo test --test cli …` commands — pass |
| MIT/no gate and all exit codes | declared claim/integration commands — pass |
| Browser demo isolation, same-origin privacy, offline reload/reset | each declared `npm run build:site && npm run test:e2e -- --grep @claim:…` command — pass on desktop and mobile |
| Build artifacts | `npm run test:claim:build-artifacts` — pass; runnable `dist/bin/mcw` and `dist/site/` present |

The CLI was also run directly from an empty temporary caller directory:
`mcw demo` emitted its separate `/tmp/mcw-demo-…` workspace and left the caller
directory empty. In a fresh live browser context, the demo made only
same-origin requests, stored only `sessionStorage["demo:mcw:stage"]`, and
reloaded/reset offline after one online visit. Reset preserved an injected
non-demo session key in the declared browser test.

## Structure, accessibility, and routing

- All normal routes returned 200: `/`, `/demo/`, `/privacy/`, and `/terms/`.
  An unknown route returned the designed 404 with a home link. Internal and
  GitHub links crawled successfully; the 404 page's own skip-link fragment
  naturally retains its 404 status.
- Every checked route has a route-specific title, description, canonical, OG
  image, Twitter card, favicon/apple touch icon, one h1, one main landmark, and
  no mobile horizontal overflow. Back/forward restores route focus to h1.
- `/opt/fleet/lib/verify-url.sh` passed for the live home page: 200, 586 ms,
  zero console errors, `lang=en`, one h1, main, and no missing image alt.
- Independent Playwright axe scans found zero serious/critical violations on
  home, demo, privacy, terms, and 404 at both 1440×900 and 390×844. The
  repository's clean-clone E2E suite passed 23 tests with one intentional
  viewport-only skip. `npm run lint` and `npm run build` passed.
- `npx @axe-core/cli` could not start ChromeDriver against the preinstalled
  Chromium binary. This did not leave accessibility untested: the supplied
  Playwright axe integration and the independent live Playwright axe scan both
  completed successfully.
- The concrete-and-moss system is distinct and product-specific, not a generic
  SaaS template. No missed AI, import/export, or sync feature was found: the
  brief's job is already met by configurable commands plus JSON/Markdown
  witnesses; AI would be decorative here.

## Earlier-finding verification

Every finding from `.factory/review-1.md` was confirmed fixed in both live
behavior and code. None is merely marked fixed.

| Earlier id | Verification result |
| --- | --- |
| BLOCKING-1 | Fixed: job, audience, and sample action are visible in the 390 px first screen. |
| BLOCKING-2 | **Reopened by F-2-1:** `mcw demo`, examples, a separate temp workspace, `/demo/`, banner, reset, and start-real controls work, but the required first-screen sample result is still absent on a 390 px phone. |
| BLOCKING-3 | Fixed: 24 claim entries, tags, and all declared commands passed from the clean clone. |
| BLOCKING-4 | Fixed: real `/demo/`, direct deep link, focus/back behavior, sitemap entry, and designed 404 all work. |
| BLOCKING-5 | Fixed: unavailable price/checkout/license UI is absent. |
| HIGH-1 | Fixed: concise per-route titles, descriptions, canonicals, OG/Twitter data, share art, and Apple touch icon are live. |
| HIGH-2 | Fixed: header/footer wording and build identity are consistent on all routes. |
| HIGH-3 | Fixed: three-step workflow and “What it never does” section are present in the required order. |
| HIGH-4 | Fixed: absolute token/production claims were removed; exact, tested safeguards remain. |
| MEDIUM-1 | Fixed: visible links/buttons met 44×44 px in the repository's desktop/mobile target sweep. |
| MEDIUM-2 | Fixed: current landing/README copy has no over-22-word sentence, banned marketing wording, ambiguous action, or terminology drift. |

## What would make this perfect

Show the partial-commit result immediately after the sample click on a phone,
then add the accessible self-hosted recording of the real `mcw demo` terminal
run to the landing page. Re-run the fresh-context mobile viewport test and
claim suite after those two changes. At that point, no other finding remains.
