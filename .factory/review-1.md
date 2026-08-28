# Adversarial first-read review 1 — Migration Commit Witness

**Verdict: FAIL**

Reviewed 2026-08-28 UTC against live
<https://migration-commit-witness.sociobot.in> and repository commit
`2cdb166905f7b9f64a8b6893ce2684d7b3560e41`.

There are five blocking findings. The clean-clone test suite and build pass, but
the product is not tryable through the required CLI demo, has no claims
registry, serves broken routes as the home page, and links its paid action to a
404.

## Cold first screen

Fresh Chromium contexts were used with no stored data.

| View | What I understood before scrolling | For whom | What I would click first |
| --- | --- | --- | --- |
| 390×844 | A CLI runs an existing migration on a disposable database, compares database facts around commit, optionally exercises rollback, and keeps a signed record. | **Cannot answer.** No role or review situation is named. | **“Install the CLI.”** It is the only action partly visible in the initial viewport. “Replay a broken commit” begins at y=864, below the 844 px fold. |
| 1440×900 | The same migration-checking and signed-record job is understandable. | **Cannot answer.** The intended backend migration reviewer is absent. | **“Install the CLI.”** It is visually primary; “Replay a broken commit” is secondary at the bottom edge. |

The exact first-screen copy was:

> “YOUR MIGRATION SAID SUCCESS. PROVE WHAT STAYED.”
>
> “Run the migration you already have against a disposable database. Snapshot
> the facts on both sides of commit. Exercise the rollback you actually
> supplied. Keep the signed record.”
>
> “Install the CLI” / “Replay a broken commit”

## Findings, ordered by severity

### BLOCKING-1 — The first screen does not say who this is for or lead with a tryable action

**Quote:** “YOUR MIGRATION SAID SUCCESS. PROVE WHAT STAYED.” and “Install the
CLI.”

**Why this loses a first-time visitor:** the headline is seven words but does
not name SQL, a database, CI, or a reviewer. The supporting copy explains the
mechanism in four sentences but never names backend teams reviewing migrations.
On a 390 px phone, the sample action is below the fold. The visible primary
action starts installation rather than proving value in one click.

**Concrete fix:** use “Prove what your SQL migration committed” as the headline.
Follow with “For backend teams reviewing migrations, run one against a test
database and save signed before, after, and rollback evidence.” Make “Try it
with sample data” the visible primary action, with “See a partial commit and
verified rollback; nothing is saved” beside it. Keep “Install the CLI” as the
secondary action.

### BLOCKING-2 — The demo is a page animation, not the required isolated CLI demo

**Quote:** “Replay a broken commit” and “This seeded run returns exit code 0
after creating only one of two required objects.”

**Why this misleads:** one click eventually scrolls to realistic-looking sample
output, but it is hard-coded in `site/src/main.ts`; it does not run the binary.
There is no persistent “Demo — sample data, nothing is saved” banner, “Reset
demo,” or “Start for real.” `/?demo=1` opens the landing page at the top, and
`/demo` is not a demo route. There is no `.factory/demo.md`, no `examples/`
sample, and the README has no demo command. In a fresh temporary directory,
both `mcw demo` and `mcw --demo` exited 2 as unrecognized input and produced no
sample output. A visitor therefore cannot verify the CLI's real job without
creating a config, database, scripts, and secrets.

**Concrete fix:** ship an opinionated SQLite sample under `examples/` and add
`mcw demo`. It must copy the sample into a newly created temporary directory,
run the real witness path, print that directory, and produce signed-or-clearly-
demo JSON and Markdown. Add `/demo` with the required banner and controls; make
the first-screen action open it. Document the command, reset/discard behavior,
and namespace in `.factory/demo.md` and README. Add a test that runs `mcw demo`
from an empty temporary directory, checks the before/after/rollback observations
and artifacts, and confirms no files appear outside that directory.

### BLOCKING-3 — There is no claims registry, so no public claim is traceable to a claim test

**Quote:** `.factory/claims.json` is absent and `rg '@claim:'` finds no tagged
tests.

**Why this misleads:** the landing page and README make observable safety,
privacy, format, database, signing, offline, price, and storage claims. None has
the required registry entry or exactly one `@claim:<id>` test. Passing general
tests does not tell a visitor which promise was exercised from the supported
demo entry point.

**Concrete fix:** add `.factory/claims.json`; give every item in the unlisted-
claims table below its own stable id, location, clean-sandbox command, and
observable assertion. Tag exactly one test per id. Remove any sentence that
cannot be tested through the shipped demo.

### BLOCKING-4 — Demo, unknown-route, and navigation behavior is broken

**Quote:** requesting `/demo` or `/does-not-exist` returns HTTP 200 with the home
headline and canonical URL `/`.

**Why this loses a visitor:** there is no real demo URL and no designed 404.
`/#demo` is only a hash jump. After selecting the sample, focus remains on
`BODY`; Back removed the hash but left the page near the demo at y=3,061 rather
than restoring the original top. A direct `/#demo` load aligns the section,
but focus still remains on `BODY`. Route changes are not announced and do not
move focus to the relevant heading.

**Concrete fix:** implement `/demo` as a real route titled “Demo — Migration
Commit Witness.” Add a styled 404 that returns 404 and links home. On every
client-side route change, focus the new `h1` and announce it through a polite
live region. Preserve/restore scroll on Back and Forward. Add `/demo` to the
sitemap and add Playwright coverage for direct load, reload, Back/Forward,
focus, title, canonical, and the unknown path.

### BLOCKING-5 — The paid CTA is dead

**Quote:** “Buy the rollout kit.”

**Why this misleads:** both HEAD and GET to
`https://api.sociobot.in/api/v1/products/migration-commit-witness/checkout`
returned HTTP 404. The page advertises a $49 one-time product that cannot be
purchased.

**Concrete fix:** register and enable the product in the approved Sociobot
billing API, then add a deployed smoke test that confirms the CTA reaches the
expected checkout and a non-billing test that confirms purchase return,
verification, unlock, and download. Until that works, remove the price and buy
CTA rather than advertise an unavailable sale.

### HIGH-1 — Required metadata and social assets are incomplete

**Quote:** `<title>Migration Commit Witness — evidence across the commit
boundary</title>`.

**Why this matters:** the title is 62 characters, exceeds the 60-character
contract, and “evidence across the commit boundary” does not plainly name the
result. All checked routes lack Open Graph metadata, Twitter card metadata, and
an Apple touch icon. There is no product-specific 1200×630 share image.

**Concrete fix:** use “Migration Commit Witness — prove SQL migration state”
and route-specific equivalents. Add OG title/description/image, Twitter card,
and a 180 px Apple icon to every route; derive the share image from the existing
original witness artwork.

### HIGH-2 — Header/footer structure is inconsistent and has no build identity

**Quote:** home navigation is “Method / Install / Team kit / Source”; legal-page
navigation is “Home / Privacy / Terms.” Footer one-liners also change, and no
footer contains a version or build id.

**Why this loses a visitor:** the site changes its navigation model between
pages, the home header omits Privacy, and a reviewer cannot identify the live
build.

**Concrete fix:** use one header and footer component on every route. Keep the
same wordmark and up to four destinations, include Privacy, and show the same
product one-liner plus version/build id in every footer.

### HIGH-3 — The standard landing structure is incomplete

**Quote:** “FOUR OBSERVATIONS. ONE PORTABLE RECORD.” and “HARD GUARDRAIL.”

**Why this loses a visitor:** the required “How it works” section is not a
three-step explanation, and there is no clear “What it does not do / privacy”
section before pricing. Safety and privacy facts are distributed among an
abstract four-step method, an install-side guardrail, and the footer.

**Concrete fix:** present three plain steps—configure checks, run the migration
and rollback, save the witness—then add a named “What it never does” section
covering production refusal, telemetry, network access, and credential
retention.

### HIGH-4 — Two absolute privacy and safety statements exceed the implementation

**Quote:** “Your token stays in this browser.” and “No production
connections.”

**Why this misleads:** license verification sends the full token to the
Sociobot API as a query parameter. The CLI rejects URLs only when they contain
one of four strings (`prod`, `production`, `primary`, or `live-db`); it cannot
confirm that every other URL is non-production. The Privacy page explains the
token transfer, but the landing statements are absolute.

**Concrete fix:** say “Your token is stored in this browser and sent to
Sociobot for verification.” Replace the footer claim with “Requires a test
environment label, your confirmation, and a URL without known production
names.” Add claim tests for the token destination and every documented URL
guard; do not promise that a naming heuristic identifies all production
databases.

### MEDIUM-1 — Several interactive targets are narrower than 44 px

**Quote:** “Home” and “Terms.”

**Why this matters:** live geometry found the legal-page Home link at 33.72×44
CSS px and Terms links at 42.16×44 CSS px on desktop; Terms remains 42.16×44 on
390 px. These miss the 44×44 touch-target contract.

**Concrete fix:** give every header/footer link a minimum inline size of 44 px
without relying on adjacent whitespace, then test every interactive element at
desktop and 390 px.

### MEDIUM-2 — Copy uses unexplained terms, metaphors, and ambiguous labels

Each row is a separate copy finding.

| Quote | Flag | Concrete rewrite |
| --- | --- | --- |
| “Independent commit evidence” | Jargon; heading is vague alone | “Record what the database committed” |
| “Snapshot the facts on both sides of commit.” | “Snapshot” and “facts” hide the actual checks | “Record your checks before and after the migration commits.” |
| “A green tool status is a claim. The state across the seam is the evidence.” | Metaphor (“seam”) | “A successful migration command does not prove database state. Compare checks before and after commit.” |
| “Exit zero can still leave a partial state.” | “Exit zero” and “state” are compressed jargon | “A successful command can still leave a partly changed database.” |
| “DDL may ignore the transaction you thought contained it.” | Unexpanded initialism | “Some schema changes can outlive the transaction that ran them.” |
| “Migration status and database truth can diverge at exactly the moment a release needs certainty.” | “Database truth” is abstract | “A successful command can still leave the wrong tables or data.” |
| “Four observations. One portable record.” | Abstract marketing copy | “Record checks before migration, after commit, and after rollback.” |
| “Baseline” | Heading is unclear out of context | “Check the database before migration” |
| “Run yours” | Heading is unclear out of context | “Run your migration command” |
| “Witness commit” | Product-specific verb is unclear | “Check the database after commit” |
| “Query scalar schema and data invariants before anything moves.” | Jargon (“scalar,” “invariants”) | “Run one-value schema and data checks before the migration starts.” |
| “Only on explicit request, using only the rollback command in policy.” | “Policy” is unexplained and inconsistent with “config” | “Run only the rollback command in the config, and only when you request it.” |
| “03 / recorded fixture” | Jargon | “03 / sample migration” |
| “This seeded run returns exit code 0 after creating only one of two required objects.” | “Seeded run” and “objects” hide the concrete sample | “This sample command succeeds after creating only one of two required tables.” |
| “Move through the same observations the CLI records.” | “Observations” is abstract | “Step through the checks the CLI records.” |
| “Boring inputs. Durable evidence.” | Marketing adjective and vague heading | “Use your existing commands. Save signed evidence.” |
| “Hard guardrail” | Metaphor; unclear heading | “Production databases are blocked” |
| “Production-like URLs are rejected.” | “Production-like” does not tell the user what is checked | “The CLI rejects database URLs containing configured production names.” |
| “Specific by dialect. Never hand-waved.” | Jargon and marketing phrasing | “See the checks for SQLite and PostgreSQL.” |
| “The witness stays free. Scale the ritual once.” | “Witness” changes meaning; “ritual” is metaphor | “The CLI is free. The $49 kit adds team rollout templates.” |
| “Review-owner matrix template” | Jargon | “Template that assigns migration reviewers” |
| “Dialect policy preset pack” | Jargon | “Starter configs for SQLite and PostgreSQL” |
| “Adoption and exception checklist” | Abstract | “Checklist for rollout and approved exceptions” |
| “Secure checkout.” | Unsupported marketing adjective | “Checkout opens on the Sociobot billing site.” |
| “Sociobot/Dodo is merchant of record.” | Unexplained legal term | “Sociobot/Dodo processes the payment, receipt, and refund.” |
| “Keep the claim. Keep the proof.” | Vague out of context | “Save the witness with each release.” |
| “Evidence for the dangerous boundary.” | Metaphor | “Signed evidence of migration and rollback results.” |
| “Previous” | Button does not name its result | “Show previous observation” |
| “Run command” | Button names an action, not the result shown next | “Show command result” |
| “Copy install” | Ambiguous button | “Copy install command” |
| “Add the witness to CI” | Misstates a link that only jumps to install text | “View the CI setup command” |
| “`mcw` is a CI-focused witness for backend teams reviewing SQL migrations.” | README product category is jargon | “`mcw` is a command-line tool for backend teams reviewing SQL migrations in CI.” |
| “It runs your existing migration command … and emits signed JSON and readable Markdown.” (35 words) | **Over 22 words**; multiple ideas | “Run your existing migration against a confirmed test database and record checks before and after commit. Optionally run your rollback, then export signed JSON and readable Markdown.” |
| “`witness/witness.json` is stable machine-readable evidence; `witness/witness.md` is the PR-friendly record.” | “Stable” and “PR-friendly” are unsupported adjectives | “`witness.json` is for tools. `witness.md` is formatted for pull-request review.” |
| “Create a starter policy, edit its commands and assertions, then run it only against a disposable database.” | “Policy,” “assertions,” and “disposable” differ from the landing page's terms | “Create a starter config, update its commands and checks, then run it against a test database.” |
| “Exit codes are `0` … artifact error.” (27 words) | **Over 22 words** | Replace with a four-row exit-code table. |
| “Every invariant query must return exactly one scalar value.” | Jargon | “Each check query must return one value.” |
| “Query errors never count as observations … same error.” | Dense terminology | “A query error fails the run. Matching errors before and after do not count as matching values.” |
| “Rollback is never inferred … migration command runs.” (24 words) | **Over 22 words** | “Rollback requires both a `[rollback]` command and `--exercise-rollback`. The CLI validates both before it runs a query or command.” |
| “Dialect checks are honest and explicit … commit proof.” (26 words) | **Over 22 words**; marketing adjective “honest” | “SQLite records `quick_check` and foreign-key violations. PostgreSQL records server reachability. Your configured checks provide application-specific evidence.” |
| “To package without publishing … credentials remain with the factory.” | Internal factory language does not help a user package the CLI | “Run `cargo package --allow-dirty` to create a package without publishing it.” |
| “Source is MIT licensed … accessibility.” (23 words) | **Over 22 words** | “The source is MIT licensed. The optional Team rollout kit is a one-time purchase. Core safety, export, and accessibility remain free.” |

Terminology is also inconsistent. “Witness” refers to the product, a run, a
verb, and an output artifact; “fact,” “observation,” “invariant,” “assertion,”
and “proof” refer to checks; “policy” and “config” refer to the same file;
“Team kit” and “rollout kit” refer to the same purchase. Use **CLI** for the
tool, **run** for an execution, **witness** for the saved artifact, **check** for
a query/expectation, **config file** for `mcw.toml`, and **rollout kit** for the
purchase.

## Complete sentence audit

Counts treat hyphenated terms, flags, paths, and contractions as one word.
Code blocks are commands rather than sentences and are excluded. Headings and
buttons with copy defects are included in the findings above.

### Live landing page

| Sentence | Words |
| --- | ---: |
| Offline copy — docs and demo still work; license verification will retry when connected. | 13 |
| Your migration said success. | 4 |
| Prove what stayed. | 3 |
| Run the migration you already have against a disposable database. | 10 |
| Snapshot the facts on both sides of commit. | 8 |
| Exercise the rollback you actually supplied. | 6 |
| Keep the signed record. | 4 |
| A green tool status is a claim. | 7 |
| The state across the seam is the evidence. | 8 |
| Concrete database core split by a black transaction seam, with moss on verified edges and a rust fracture stopping at commit. | 21 |
| A command can report success while an invariant fails. | 9 |
| Exit zero can still leave a partial state. | 8 |
| Deferred constraints can fail at commit. | 6 |
| DDL may ignore the transaction you thought contained it. | 9 |
| Migration status and database truth can diverge at exactly the moment a release needs certainty. | 15 |
| Four observations. | 2 |
| One portable record. | 3 |
| Query scalar schema and data invariants before anything moves. | 9 |
| Execute the existing Flyway, Liquibase, ORM, or shell command unchanged. | 10 |
| Query again even after a failed command, so partial outcomes cannot hide. | 12 |
| Only on explicit request, using only the rollback command in policy. | 11 |
| Replay a partial commit. | 4 |
| This seeded run returns exit code 0 after creating only one of two required objects. | 15 |
| Move through the same observations the CLI records. | 8 |
| JavaScript is off. | 3 |
| The CLI and docs remain usable; the recorded fixture needs JavaScript to advance. | 13 |
| Boring inputs. | 2 |
| Durable evidence. | 2 |
| One Rust binary. | 3 |
| No agent, hosted database, migration DSL, or telemetry. | 8 |
| Production is not a mode. | 5 |
| The run needs both an allowed environment label and `--confirm-test-database`. | 10 |
| Production-like URLs are rejected. | 4 |
| Rollback needs a configured command and a separate flag. | 9 |
| Specific by dialect. | 3 |
| Never hand-waved. | 2 |
| The witness stays free. | 4 |
| Scale the ritual once. | 4 |
| Safety behavior, signatures, rollback exercise, and both evidence formats are MIT licensed. | 12 |
| The optional kit packages team rollout policy. | 7 |
| Secure checkout. | 2 |
| Sociobot/Dodo is merchant of record. | 5 |
| Restore the kit on this device. | 6 |
| Your token stays in this browser. | 6 |
| Verification runs at most once per day. | 7 |
| Paste the full token from your receipt. | 7 |
| No license stored on this device. | 6 |
| One-time purchase. | 2 |
| Refunds are handled by the merchant of record and revoke the license. | 12 |
| See privacy and terms. | 4 |
| Keep the claim. | 3 |
| Keep the proof. | 3 |
| Evidence for the dangerous boundary. | 5 |
| No telemetry. | 2 |
| No production connections. | 3 |
| Built by the Param Factory. | 5 |

Conditional runtime sentences are also landing-page copy: “Copy install
complete.” (3), “Copy command complete.” (3), “Clipboard access was blocked.”
(4), “Select the command text to copy it.” (7), “License active.” (2), “The
rollout kit is ready.” (6), “You can purchase a new license above.” (7),
“License no longer active (reason).” (5),
“Offline — using the last valid license check.” (7), “Offline — connect once to
verify this license.” (8), “Checking this license…” (3), “Verification is
unavailable — using the last valid check.” (8), “Could not reach license
verification.” (5), “Check your connection and try again.” (6), “License active
from the last verified check.” (7), “Paste the full license token, then verify
again.” (8), and “Team rollout kit downloaded.” (4). None exceeds 22 words or
uses a banned marketing word.

### README

| Sentence | Words | Length flag |
| --- | ---: | --- |
| `mcw` is a CI-focused witness for backend teams reviewing SQL migrations. | 11 | — |
| It runs your existing migration command against an explicitly confirmed disposable database, records selected invariants on both sides of the commit boundary, can exercise a rollback you supply, and emits signed JSON and readable Markdown. | 35 | **Over 22** |
| It does not generate migrations, replace your migration engine, or connect to production by design. | 15 | — |
| Live docs: https://migration-commit-witness.sociobot.in | 2 | — |
| Build the single binary with stable Rust. | 7 | — |
| The release archive produced by `npm run build` also places the local binary at `dist/bin/mcw` and the deployable documentation site at `dist/site/`. | 22 | — |
| Create a starter policy, edit its commands and assertions, then run it only against a disposable database. | 17 | — |
| `witness/witness.json` is stable machine-readable evidence; `witness/witness.md` is the PR-friendly record. | 10 | — |
| `--json` prints the final summary to stdout. | 7 | — |
| All progress and errors go to stderr. | 7 | — |
| No prompt is ever shown in CI. | 7 | — |
| Verify an artifact later with the same secret. | 8 | — |
| Local experiments may use `--allow-unsigned`, but CI witnesses should be signed. | 11 | — |
| Exit codes are `0` for a passing/verified witness, `2` for a migration or assertion failure, `3` for unsafe/invalid configuration, and `4` for a runtime or artifact error. | 27 | **Over 22** |
| Every invariant query must return exactly one scalar value. | 9 | — |
| Query errors never count as observations and fail the witness, including when both sides of a `$before` comparison return the same error. | 22 | — |
| `$before` compares two successfully observed scalars. | 6 | — |
| PostgreSQL policies require `psql` on `PATH`; the CLI passes the URL using `psql --dbname` and redacts it from retained errors. | 20 | — |
| SQLite is embedded. | 3 | — |
| Migration and rollback commands inherit the configured database URL environment variable. | 11 | — |
| `mcw witness` refuses to start unless the policy labels the environment as `test`, `ci`, `development`, or `ephemeral` and the caller passes `--confirm-test-database`. | 22 | — |
| It rejects database URLs containing common production labels. | 8 | — |
| Rollback is never inferred: both a `[rollback]` command and `--exercise-rollback` are required, and the pair is validated before any query or migration command runs. | 24 | **Over 22** |
| The database URL and signing key are never written to evidence. | 11 | — |
| Dialect checks are honest and explicit: SQLite records `quick_check` and foreign-key violations; PostgreSQL records server reachability and relies on the selected invariants for application-specific commit proof. | 26 | **Over 22** |
| Requirements: stable Rust, Node 22+, and npm. | 7 | — |
| `npm test` runs Rust unit/integration tests and site tests. | 9 | — |
| The documented CLI example is covered end to end by an isolated SQLite fixture. | 14 | — |
| To package without publishing, run `cargo package --allow-dirty`; registry credentials remain with the factory. | 14 | — |
| The CLI is local-only and has no telemetry. | 8 | — |
| The site stores a pasted Sociobot license and its last verification result in your browser; see `/privacy/` and `/terms/`. | 19 | — |
| Source is MIT licensed; the optional Team rollout kit is a one-time purchase and does not gate witness safety, evidence export, or accessibility. | 23 | **Over 22** |

README headings are “Migration Commit Witness,” “Install,” “Usage,” “Policy
file,” “Safety model,” “Develop and verify,” and “Privacy and license.” They are
understandable in the README outline. The jargon and terminology findings above
still apply to its body copy.

## Unlisted claims

Because the registry has zero entries, every row is an individual unlisted-
claim finding. The “test to add” is the concrete acceptance fix; each needs a
separate id and exactly one matching tag.

| Location and quote | Why a visitor could rely on it | Test to add |
| --- | --- | --- |
| Landing: “Offline copy — docs and demo still work…” | Promises offline behavior | Load `/demo`, enable offline mode, reload, run/reset sample, and assert no failed UI state. |
| Landing: “Run the migration you already have against a disposable database.” | Core CLI behavior | Run the demo binary in a temp SQLite directory and assert the configured migration ran there. |
| Landing: “Snapshot the facts on both sides of commit.” | Core evidence behavior | Assert before and after check values in JSON and Markdown. |
| Landing: “Exercise the rollback you actually supplied.” | Rollback behavior | Supply a rollback, exercise it, and assert the exact command and post-rollback values. |
| Landing: “Keep the signed record.” | Signing/output behavior | Verify the generated signature, then tamper with the file and assert verification fails. |
| Landing: “Local-only” / “No telemetry.” | Privacy promise | Block/intercept network for the complete CLI demo and browser demo; allow only same-origin static assets. |
| Landing: “2 dialects” / “Named behavior.” | SQLite/PostgreSQL support | Run equivalent disposable SQLite and PostgreSQL fixtures and assert the dialect field and checks. |
| Landing: “2 formats” / “JSON + Markdown.” | Export promise | Assert both files exist and contain the same run verdict/check counts. |
| Landing: “Exit zero can still leave a partial state.” | Failure-detection promise | Run the exit-0 partial-DDL fixture and assert a failing witness with retained after-state. |
| Landing: “Deferred constraints can fail at commit.” | Database-behavior assertion | Add a PostgreSQL deferred-constraint fixture that fails at commit and is reported correctly. |
| Landing: “DDL may ignore the transaction you thought contained it.” | Database-behavior assertion | Add a supported-dialect partial-DDL fixture and assert observed residual state. |
| Landing: “Migration status and database truth can diverge…” | Detection promise | Assert a successful command status alongside a failed configured check. |
| Landing: “Query scalar schema and data invariants before anything moves.” | Ordering/query promise | Assert baseline queries complete before the migration process starts. |
| Landing: “Execute the existing Flyway, Liquibase, ORM, or shell command unchanged.” | Tool compatibility claim | Either test each named command family or replace the list with the actually tested command contract. |
| Landing: “Query again even after a failed command…” | Failure-path behavior | Make migration exit nonzero and assert after checks still execute and are recorded. |
| Landing: “Only on explicit request, using only the rollback command in policy.” | Safety promise | Assert rollback never runs without the flag and no command other than the configured rollback executes. |
| Landing: “This seeded run returns exit code 0 after creating only one of two required objects.” | Demo realism | Run the shipped sample and assert exit 0 plus exactly one of two objects. |
| Landing: “Move through the same observations the CLI records.” | Claims UI equals CLI output | Generate demo data with the binary and compare every displayed observation with the artifact. |
| Landing: “One Rust binary.” | Packaging claim | Inspect the release archive and run the binary in a fresh container without a sidecar. |
| Landing: “No agent, hosted database, migration DSL, or telemetry.” | Architecture/privacy claim | Inspect runtime processes and requests during the demo; also assert configs use existing commands. |
| Landing: “The run needs both an allowed environment label and `--confirm-test-database`.” | Safety gate | Parameterize allowed/missing labels and flag; assert migration never starts on rejection. |
| Landing: “Production-like URLs are rejected.” | Production-safety claim | Parameterize every documented production marker and assert pre-command rejection. |
| Landing: “Rollback needs a configured command and a separate flag.” | Rollback gate | Test all missing/present config-and-flag combinations before mutation. |
| Landing table: SQLite is embedded and runs `quick_check` plus foreign keys. | Dialect implementation | Run SQLite without an external client and assert both built-in results in evidence. |
| Landing table: PostgreSQL uses `psql` on PATH and checks reachability. | Dialect implementation | Run against disposable PostgreSQL with a controlled `psql` shim and assert invocation/reachability evidence. |
| Landing: “The witness stays free.” / “$0 MIT.” | Price/license promise | Assert the repository license and that every core CLI command works without a license. |
| Landing: “Safety behavior, signatures, rollback exercise, and both evidence formats are MIT licensed.” | Feature-gating promise | Run each named feature without a license and inspect the MIT-distributed source/package. |
| Landing: “The optional kit packages team rollout policy.” | Paid-content promise | Use a test entitlement, download the kit, and assert the promised template content. |
| Landing: “$49 one time.” | Price/billing promise | Safe billing-contract test for exact amount/currency and no recurring interval. |
| Landing kit bullets, including “Future kit updates for this major version.” | Paid deliverables promise | Entitled download test must assert each listed deliverable and version entitlement. |
| Landing: “Secure checkout. Sociobot/Dodo is merchant of record.” | Checkout/security claim | Assert HTTPS checkout metadata names the merchant and uses only the approved API. |
| Landing: “Your token stays in this browser.” | Storage/privacy promise | Intercept requests and assert the token is sent only to the documented verify endpoint and stored only in the named local key. |
| Landing: “Verification runs at most once per day.” | Rate/frequency promise | Advance a fake clock across the 24-hour boundary and count verification requests. |
| Landing: “Refunds … revoke the license.” | Billing/license lifecycle | Feed a refunded verdict and assert access is revoked without exposing the token. |
| Landing footer: “No production connections.” | Safety/network promise | Attempt a production-like URL and assert no socket/process/migration action occurs. |
| README: “It runs your existing migration command … emits signed JSON and readable Markdown.” | End-to-end product promise | One demo E2E test must assert command execution, checks, optional rollback, both files, and signature. |
| README: “It does not generate migrations, replace your migration engine, or connect to production…” | Negative safety promise | Assert no migration file creation and rejection before any production-like connection or command. |
| README: “The release archive … places the local binary at `dist/bin/mcw` and … site at `dist/site/`.” | Build artifact promise | Build cleanly and assert both paths and runnable/servable outputs. |
| README: “`--json` prints the final summary to stdout. All progress and errors go to stderr. No prompt is ever shown in CI.” | Stream/noninteractive contract | Capture both streams in CI mode and assert JSON-only stdout and no stdin read. |
| README: “Verify an artifact later with the same secret.” | Verification behavior | Generate, persist, and verify an artifact in a second process with the same key. |
| README exit-code sentence | Automation contract | Parameterize success, assertion/migration failure, invalid config, and runtime error and assert 0/2/3/4. |
| README: “Every invariant query must return exactly one scalar value.” | Query contract | Test zero, one, multi-row, and multi-column results. |
| README two `$before`/query-error sentences | Error/comparison behavior | Force equal query errors and successful scalar equality; assert only the latter compares. |
| README PostgreSQL `psql`/redaction sentence | Dependency/privacy behavior | Shim `psql`, capture argv/errors, and assert `--dbname` use with complete URL redaction. |
| README: “SQLite is embedded.” | Dependency claim | Run a SQLite demo with no `sqlite3` executable on PATH. |
| README: “Migration and rollback commands inherit the configured database URL environment variable.” | Environment behavior | Child fixtures must print a fingerprint of the inherited value; assert both receive it without artifact disclosure. |
| README four safety sentences beginning “`mcw witness` refuses to start…” | Preflight and secret-retention contract | Parameterize environment/flag/rollback/URL cases and scan every artifact/stdout/stderr for URL and key. |
| README dialect-check sentence | Dialect evidence behavior | Separate SQLite and PostgreSQL tagged tests must assert every named built-in check. |
| README: “`npm test` runs Rust unit/integration tests and site tests.” | Verification-command claim | Run from a clean clone and assert all three categories execute, not merely exit 0. |
| README: “The documented CLI example is covered end to end by an isolated SQLite fixture.” | Test-coverage claim | Tag the documented-example test and assert an isolated temp database plus expected witness artifacts. |
| README packaging sentence | Packaging behavior | Run `cargo package --allow-dirty` in a disposable clone and assert no credential lookup/publish. |
| README: “The CLI is local-only and has no telemetry.” | Privacy promise | Run the complete demo with network namespace/interception and assert zero outbound connections. |
| README license-storage sentence | Browser storage promise | Fresh-context test must assert no initial storage, exact keys after restore, and documented endpoint only. |
| README final MIT/one-time/not-gated sentence | License, billing, and accessibility promise | Split into independently tagged license, price, free-feature, and accessibility tests or split/remove the sentence. |

## Demo, privacy, and offline evidence

- The one-click page replay does show a realistic baseline immediately after the
  hash scroll settles: schema `0 / 2`, 12 rows, `sqlite.quick_check = "ok"`.
- Advancing to rollback shows `rollback.matches_before = true` and
  `artifact.signature = "HMAC-SHA256"`; these are hard-coded display strings,
  not output produced by the CLI.
- The replay itself made only same-origin requests and wrote no local storage.
  This does not establish an isolated demo namespace because there is no demo
  mode.
- After priming the service worker, live reload and replay worked with the
  browser offline. The only storage written was session key `mcw:offline=1`.
- The fresh temp-directory CLI demo check failed because neither documented
  demo entry form exists.

## Structure and accessibility evidence

- Pass: HTTPS home, Privacy, and Terms returned 200; every internal link crawled
  returned 200; GitHub and the policy fragment returned 200.
- Pass: every checked page has `lang="en"`, one `h1`, a `main`, meta
  description, canonical, SVG favicon, alt text, and no live console error.
- Pass: live axe checks at desktop and 390 px found zero violations on home,
  Privacy, and Terms. The repository keyboard test and reduced/offline browser
  coverage passed.
- Pass: security headers include CSP, Permissions-Policy, Referrer-Policy,
  nosniff, and HSTS. The live initial JavaScript is 6,713 bytes uncompressed.
- Pass: the concrete-and-moss visual system is distinct, uses original artwork
  with provenance, and does not resemble the generic centered-gradient SaaS
  template described by the review criteria.
- Fail: no OG/Twitter/Apple metadata, no `/demo`, no designed 404, inconsistent
  site chrome, no build id, and undersized links as detailed above.

## Verification log

Clean clone: `/tmp/mcw-review-clean-nwy4Mk`.

| Check | Result |
| --- | --- |
| Read `.factory/claims.json` | **BLOCKED/FAIL:** file absent |
| Run listed claim commands | **FAIL:** zero commands can be enumerated because registry is absent |
| `rg '@claim:'` | **FAIL:** no tagged claim test |
| `npm ci` | Pass |
| `npm test` | Pass: 3 Rust unit + 8 Rust integration + 4 Vitest + 16 Playwright; 2 viewport-inapplicable Playwright cases skipped |
| `npm run build` | Pass; `dist/bin/mcw` and `dist/site/` produced |
| `mcw demo` in `/tmp/mcw-demo-run-Cz2s3e` | **BLOCKING FAIL:** exit 2, unrecognized subcommand |
| `mcw --demo` in the same directory | **BLOCKING FAIL:** exit 2, unexpected argument |
| Factory `verify-url.sh` | Pass: 200, title/lang/main/alt present, no console errors; evidence `/tmp/mcw-verify-url-SnPe7D` |
| Live axe, desktop and 390 px | Pass: zero violations on `/`, `/privacy/`, `/terms/` |
| Live offline reload/replay | Pass after service-worker priming |
| Live request/storage interception during replay | Same-origin only; no local storage; session `mcw:offline` only after offline test |
| Internal/GitHub link crawl | Pass |
| Paid checkout GET/HEAD | **BLOCKING FAIL:** HTTP 404 |
| `/demo` and unknown route | **BLOCKING FAIL:** HTTP 200 home page, canonical `/` |
