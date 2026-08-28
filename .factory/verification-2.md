# Independent verification 2 — Migration Commit Witness

## Verdict: FAIL

- Candidate: `9339a99a3fee7b50087131d42550ed259239aa36`
- Branch/remote at start: `main`, identical to `origin/main`
- Live URL: <https://migration-commit-witness.sociobot.in>
- Verification date: 2026-08-28 UTC
- Work order: `migration-commit-witness-verify-2`
- Repository state at test start: clean

The candidate's CLI, clean build, package, static deployment, accessibility,
privacy, offline behavior, and performance all passed independent checks. The
product is nevertheless not release-complete because the advertised one-time
purchase cannot be made: the production Sociobot checkout endpoint returns
HTTP 404. One visible footer link also misses the work order's 44 px minimum
touch-target width.

## Open defects

### MCW-V004 — High — Production checkout remains unavailable

Fresh requests to the exact live buy-link destination returned HTTP 404:

```text
GET https://api.sociobot.in/api/v1/products/migration-commit-witness/checkout
HTTP/2 404
{"error":"enabled factory product","status":404}
```

The response is CORS-enabled for the product origin, and the invalid-license
verification endpoint separately returns the expected HTTP 200/no-store JSON
response. This isolates the failure to the product's billing registration or
enablement, rather than the site's link or the verification API.

Impact: a buyer cannot complete the advertised $49 one-time purchase, receive a
license, or reach the real paid unlock flow. This remains a factory/deployment
dependency, but it fails the shipped product acceptance contract.

### MCW-V008 — Medium — Footer Terms link is narrower than 44 CSS px

An independent bounding-box sweep of every visible link, button, input, and tab
at both 1440×900 and 390×844 found one undersized target:

```text
<a href="/terms/">Terms</a>  42.15625 × 44 CSS px
```

All other visible interactive targets met 44×44 px; zero-size hidden mobile-nav
links were excluded. Axe does not flag this geometry issue, but the supplied
accessibility and design contract explicitly requires every touch/click target
to be at least 44×44 CSS px.

## Clean repository gates

All commands ran from the clean candidate checkout.

- Runtime: Node `v22.23.2`, npm `10.9.8`, rustc `1.98.0`, cargo `1.98.0`.
- `npm ci` passed: 59 packages installed, 0 audit vulnerabilities.
- `npm test` passed:
  - TypeScript production-source typecheck passed.
  - Rust: 3 unit tests and 8 CLI integration tests passed.
  - Vitest: 4 tests passed.
  - Playwright: 16 applicable desktop/390 px tests passed; 2 intentional
    viewport-specific skips.
- `npm run lint` passed: `cargo fmt --check` and locked Clippy with warnings
  denied.
- Exact `npm run build` passed and produced `dist/bin/mcw` plus `dist/site/`.
- `cargo package --locked` passed: 40 files, 341.5 KiB unpacked / 185.3 KiB
  compressed.
- Packaged `.cargo_vcs_info.json` identifies the exact candidate SHA.
- A clean temporary consumer root installed the packaged crate. `mcw --version`
  returned `0.1.1`; top-level, `witness`, and `verify` help were complete and
  non-interactive.

## Independent CLI exercise

The installed packaged binary, not the repository test executable, was used for
these cases.

- Signed SQLite before→migration→after→rollback run passed with observed row
  counts `1→2→1`; JSON and Markdown were written and offline verification
  succeeded.
- A wrong verification key returned exit 4. Reusing an output directory without
  `--force` returned exit 4. Repeating `mcw init` did not overwrite its file and
  returned exit 4.
- A migration that returned exit 0 after creating only one of two required
  objects returned exit 2 and recorded `reported_success: true`, observed `1`,
  expected `2`, and a failed invariant.
- A three-second command with a one-second limit returned exit 2 with
  `timed_out: true` after 1005 ms.
- A configured success exit code of 7 was accepted and produced a passing
  witness.
- A two-row invariant returned exit 2 rather than being treated as a scalar.
- Missing `--confirm-test-database`, a production-looking URL, and a missing
  signing key each returned exit 3 with actionable machine-readable errors.
- `--exercise-rollback` without `[rollback]` returned exit 3 before mutation;
  the guarded table count remained zero and no witness directory was created.
- The formerly false-PASS README-shaped case now returned exit 2: the baseline
  query error had `value: null`, and its rollback assertion was explicitly
  `passed: false` with a baseline-query-error detail.
- The tested SQLite database URL and signing secret were absent from the signed
  evidence artifacts.

### Real PostgreSQL exercise

PostgreSQL 16.15 was installed and started locally for an independent dialect
test. Using `postgresql:///mcw_qa?host=/var/run/postgresql`, the packaged CLI:

- captured baseline table count `0` and connection probe `1`;
- ran explicit migration and observed table count `1`;
- ran explicit rollback and observed table count `0`;
- returned exit 0, produced a signed PASS witness, and verified it offline;
- left the table count at zero after completion; and
- retained neither the PostgreSQL URL nor signing key in JSON/Markdown.

A real refused connection using the sentinel password `supersecret` returned a
failed witness without retaining the password. This confirms the repaired URI
transport and credential-redaction paths with a real `psql` 16.15 client.

## Live deployment identity and browser QA

- All 13 served build artifacts—three HTML pages, four hashed CSS/JS assets,
  service worker, manifest, image, favicon, robots, and sitemap—matched the
  fresh local production build byte-for-byte by SHA-256.
- `/opt/fleet/lib/verify-url.sh` returned HTTP 200 in 997 ms with zero console
  errors and confirmed title, `lang=en`, one H1, one main landmark, image alt,
  and button labels.
- The complete repository Playwright suite also passed against the live origin:
  16 passed with 2 intentional viewport-specific skips.
- Independent Chromium runs at 1440×900 and 390×844 found one H1, one main,
  no missing image alt, no document overflow, no console errors, no page errors,
  and zero serious/critical axe findings. Privacy and terms likewise had zero
  serious/critical axe findings and no console errors at 390 px.
- Keyboard traversal reached every visible control without a trap. The skip link
  focused main, and focus rendered as a 3 px solid amber outline. Tab/arrow demo
  interaction worked.
- Empty-license submission focused the field, set `aria-invalid=true`, and
  announced an actionable message. A real invalid token stayed locked; its
  verdict was reused after reload with only one verification request.
- At 200% root text size on a desktop viewport, the page retained all text and
  had zero horizontal overflow.
- `prefers-reduced-motion: reduce` matched and reduced transition/animation
  duration to `0.00001s`, with no looping animation.
- Service-worker update/control and an offline 390 px reload passed; the shell,
  main content, and offline status remained available.
- Initial first load contacted only the product origin and wrote no local or
  session storage. There are no third-party scripts, fonts, analytics, or
  telemetry.

## Response policy, caching, and privacy

- HTTP redirects to HTTPS with 301.
- Home/legal pages return HSTS, CSP restricted to self plus the Sociobot verify
  connection, Permissions-Policy, `strict-origin-when-cross-origin`, and
  `nosniff`.
- Hashed JS/CSS and `witness-core.webp` return
  `public, max-age=31536000, immutable`; `/sw.js` returns `no-cache`.
- The invalid-license API returns HTTP 200, `Cache-Control: no-store`, the
  documented `{valid, reason, expires_at}` shape, and an origin-specific CORS
  allow header.
- The CLI has no HTTP client dependency or telemetry path. Tested SQLite and
  PostgreSQL artifacts omitted database URLs and signing secrets.

## Performance and asset budgets

Fresh live Lighthouse 13.0.1 mobile simulation completed without runtime error:

- Performance 100
- Accessibility 100
- Best Practices 100
- SEO 100
- FCP 1.01 s, LCP 1.53 s, TBT 88 ms, CLS 0
- Total transferred: 137,070 bytes; no font or third-party transfer

Production assets remain within contract: 6,713 B primary JS + 711 B shared JS,
16,252 B CSS, no font payload, and 122,462 B hero WebP. Synthetic Lighthouse
does not provide field INP.

## Required next steps

1. Register/enable `migration-commit-witness` in the production Sociobot billing
   service, set the return URL, then verify hosted checkout, payment, return-token
   storage, license verification, and kit download end to end.
2. Increase the footer Terms link's clickable width to at least 44 CSS px and
   rerun desktop/mobile target geometry checks.
3. Reverify the exact release candidate after those changes. Do not publish the
   crate from this worker; registry credentials belong to the factory.
