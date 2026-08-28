# Independent verification — Migration Commit Witness

## Verdict: FAIL

- Candidate: `9ef0b9efd4dae28b5be31e6c9ae7ec0113c29d0c`
- Branch/remote at start: `main`, identical to `origin/main`
- Live URL: <https://migration-commit-witness.sociobot.in>
- Verification date: 2026-08-28 UTC
- Work order: `migration-commit-witness-verify-1`
- Repository state at test start: clean

The candidate is not releasable. The SQLite happy path and the static site are
well implemented, but PostgreSQL is unusable and can copy database credentials
into retained witness artifacts. The CLI also has two independent rollback
safety/accuracy failures, and the live paid checkout is unavailable.

## Release-blocking defects

### MCW-V001 — Critical — PostgreSQL snapshots fail and leak the full database URL

The packaged CLI was exercised against a fresh local PostgreSQL 16.15 database.
The migration and rollback scripts could connect with the configured URL, but
every built-in and policy snapshot failed. `mcw` sets the full PostgreSQL URI as
the `PGDATABASE` environment variable; `psql` interprets it as a literal database
name instead of a connection URI.

Fresh evidence:

- `psql 'postgresql:///mcw_qa?host=/var/run/postgresql' -X -Atc 'SELECT 1'`
  returned `1`.
- `PGDATABASE='postgresql:///mcw_qa?host=/var/run/postgresql' psql -X -Atc
  'SELECT 1'` returned exit 2 and attempted to open a database literally named
  `postgresql:///mcw_qa?host=/var/run/postgresql`.
- The installed `mcw` package ran both migration and rollback commands with exit
  0, but produced status `failed` because the PostgreSQL connection probe and
  invariant snapshots could not connect.
- With the synthetic URL
  `postgresql://qa_user:supersecret@127.0.0.1:65432/mcw_qa`, both
  `witness.json` and `witness.md` contained the full URL, including the
  `supersecret` sentinel. This directly violates the documented guarantee that
  database URLs and credentials are omitted from evidence.

Impact: the advertised PostgreSQL dialect cannot produce a passing witness with
the required URL format, and CI artifacts can expose live test credentials.

### MCW-V002 — High — A rollback invariant query error is reported as PASS

The exact two-invariant README policy was run against SQLite with the documented
create/drop migration and rollback. The `seed account present` query cannot run
before creation or after rollback because the table does not exist. The CLI
nevertheless exited 0 and emitted an overall `PASS` witness.

The Markdown evidence contains this contradictory row:

```text
| seed account present | ERROR: ... no such table: accounts | `$before` | PASS |
```

In JSON, both baseline and rollback observations have `value: null` plus an
error, while the rollback assertion has `passed: true`. The comparator checks
only that both sides have some error, not that a scalar was observed. This
contradicts the documented rule that every invariant query returns exactly one
scalar and can falsely certify a rollback claim.

Impact: the product can generate signed false-PASS evidence for the core job it
is intended to perform.

### MCW-V003 — High — Missing rollback configuration is validated after mutation

Running `mcw witness --exercise-rollback` with a policy that has no `[rollback]`
section should fail before executing anything. Instead, the packaged CLI:

1. ran the migration successfully;
2. created table `only_half_done` in the disposable database;
3. then returned exit 3 with `--exercise-rollback requires an explicit
   [rollback] command`; and
4. wrote no witness artifact and performed no rollback.

The post-run table count was `1`. Validation of a requested rollback must happen
before the baseline and migration command.

Impact: an invalid recovery configuration changes the target and leaves no
evidence, despite the user's explicit request to exercise rollback.

### MCW-V004 — High — Live one-time purchase checkout is unavailable

The production buy link is correctly shaped, but a fresh GET to
`https://api.sociobot.in/api/v1/products/migration-commit-witness/checkout`
returned HTTP 404 with:

```json
{"error":"enabled factory product","status":404}
```

The invalid-license verification endpoint itself returned a valid `200` JSON
verdict with `Cache-Control: no-store`, so this is specifically a product
registration/checkout failure. No real purchase could be completed.

Impact: nobody can buy the advertised $49 rollout kit on the live deployment.

## Additional defects

### MCW-V005 — Medium — Production cache policy ignores the shipped asset rules

The built `_headers` file requests one-year immutable caching for `/assets/*`
and `/witness-core.webp`, plus `no-cache` for `/sw.js`. The live server instead
returns `Cache-Control: public, must-revalidate, max-age=30` for all of those
resources. This is a deployment configuration defect and defeats the intended
hashed-asset caching policy.

### MCW-V006 — Medium — Several live touch targets are shorter than 44 CSS px

At both 1440 px and 390 px viewports, the policy-reference link measured 25 px
high and the inline privacy/terms links measured 14 px high. The supplied
accessibility contract requires touch targets of at least 44 by 44 CSS px.

### MCW-V007 — Low — Invalid license verdicts are verified again on every reload

Submitting a synthetic invalid token produced the correct locked recovery
message. Reloading immediately caused a second request to the verify endpoint.
Only fresh valid verdicts use the 24-hour cache, despite the page promising that
verification runs at most once per day.

## Clean build and repository gates

All commands below were run from the clean candidate checkout.

- Runtime: Node `v22.23.2`, npm `10.9.8`, rustc `1.98.0`, cargo `1.98.0`.
- `npm ci` — passed; 59 packages installed, 0 audit vulnerabilities.
- `npm test` — passed:
  - Rust: 3 unit + 4 integration tests;
  - Vitest: 3 tests;
  - Playwright: 7 passed, 1 intentional desktop skip.
- `cargo clippy --all-targets --locked -- -D warnings` — passed.
- There is no separate JavaScript lint/typecheck script. Vite compiled the
  TypeScript sources during both test and production builds.
- Exact `npm run build` — passed and produced `dist/bin/mcw` plus `dist/site/`.
- `cargo package --locked --allow-dirty` — passed and verified 37 files; package
  size 311.4 KiB unpacked / 176.5 KiB compressed.
- Packaged `.cargo_vcs_info.json` identifies the exact candidate SHA.
- The crate was installed with `cargo install --locked --path` into a fresh
  temporary root; installed `mcw --version` returned `0.1.0` and `--help` was
  complete and non-interactive.

## Independent CLI exercise

Passing cases:

- Signed SQLite migration plus explicit rollback returned exit 0 and produced
  JSON and Markdown evidence; offline verification succeeded.
- Neither the SQLite URL nor signing secret appeared in either artifact.
- Wrong verification key returned exit 4.
- Reusing an output directory without `--force` returned exit 4.
- A configured success exit code of 7 was accepted and witnessed correctly.

Failure and boundary cases that behaved correctly:

- Successful command with partial DDL state: exit 2, `reported_success: true`,
  failed invariant recorded.
- Deferred-constraint/partial-state seeded repository test: caught.
- One-second command timeout: exit 2 and `timed_out: true` at about 1007 ms.
- Multirow scalar query: exit 2 with an exact-one-row error.
- Missing `--confirm-test-database`: exit 3 before connection.
- Production-looking SQLite URL: exit 3.
- Missing signing key without `--allow-unsigned`: exit 3.
- Duplicate `mcw init`: exit 4 without overwriting the policy.

The PostgreSQL path did not pass; see MCW-V001.

## Live deployment identity and browser QA

- All 13 public build artifacts (HTML, hashed JS/CSS, legal pages, service
  worker, manifest, image, favicon, robots, sitemap) were fetched from the live
  origin and SHA-256 matched the fresh local production build byte for byte.
- Home, privacy, and terms returned HTTP 200 over HTTP/2 with HSTS,
  `Referrer-Policy: strict-origin-when-cross-origin`, and
  `X-Content-Type-Options: nosniff`. CSP and Permissions-Policy were absent.
- Fresh home load made requests only to the product origin, had empty local
  storage, and produced no console errors, page errors, or failed requests.
- Structure passed: descriptive title, `lang=en`, one `main`, one `h1`, and no
  image missing alt text.
- Axe found 0 serious/critical issues on home at desktop and 390 px, and on both
  legal pages at 390 px.
- Desktop and 390×844 pages had no horizontal document overflow.
- Keyboard smoke passed: skip link reaches `#main`; a clean keyboard-only check
  showed a 3 px visible amber focus outline; tablist Home/End/arrow behavior and
  action buttons worked without a trap.
- `prefers-reduced-motion: reduce` reduced transition duration to 0.01 ms and
  removed the decorative transform.
- Empty license input is focused, marked `aria-invalid=true`, and receives an
  actionable live-region message. A mocked valid return token was stored under
  the documented key, stripped from the URL, URL-encoded for verification, and
  unlocked the download control. An actual invalid token remained locked.
- Service worker registration/update resolved, controlled the page, and an
  offline 390 px reload retained the title/main content and displayed the
  offline status bar.
- `/opt/fleet/lib/verify-url.sh` returned HTTP 200 in 569 ms with no errors and
  confirmed title/lang/H1/main/alt/button basics.

## Performance and budgets

Fresh live Lighthouse 13 mobile simulation:

- Performance 100
- Accessibility 100
- Best Practices 100
- SEO 100
- FCP 0.9 s, LCP 1.5 s, total blocking time 50 ms, CLS 0
- Total transferred size 133 KiB

Production build assets are within contract: primary JS 6,302 bytes plus a
711-byte shared helper, CSS 16,024 bytes, no font payload, and hero WebP 122,462
bytes. A synthetic run cannot provide field INP.

## Privacy review

- No analytics, tracking, third-party fonts, or third-party scripts were loaded.
- Default site load wrote no local storage.
- License storage and verification behavior is accurately disclosed on
  `/privacy/`; verification uses the Sociobot endpoint and returns `no-store`.
- SQLite witness artifacts omitted the tested URL and signing key.
- PostgreSQL credential privacy fails critically as described in MCW-V001.

## Required next steps

1. Pass PostgreSQL URLs to `psql` as a connection string (for example with
   `--dbname`) and redact credentials from every subprocess error before writing
   artifacts; add a real PostgreSQL integration test with a credential sentinel.
2. Treat any baseline or rollback invariant query error as a failed assertion;
   add a regression for the exact README policy.
3. Validate `--exercise-rollback` requirements before any snapshot or command.
4. Register/enable the paid product and rerun checkout through hosted test mode.
5. Apply the intended immutable/service-worker headers at the serving layer and
   enlarge undersized touch targets.

Reverify all blockers from a new clean checkout before release.
