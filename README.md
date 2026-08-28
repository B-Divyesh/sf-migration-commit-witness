# Migration Commit Witness

Migration Commit Witness is a command-line tool for backend teams reviewing SQL migrations in CI.

Run your migration on a confirmed test database. The CLI records checks before commit, after commit, and after an optional rollback. It writes JSON and Markdown.

Live site and sample: <https://migration-commit-witness.sociobot.in/demo/>

## Install

Build the Rust CLI from source:

```sh
cargo install --git https://github.com/B-Divyesh/sf-migration-commit-witness --bin mcw
```

The project is MIT licensed. Core commands do not require payment or a license token.

## Try the isolated sample

Run the bundled partial-commit case without setup:

```sh
mcw demo
```

The command creates a new temporary folder and prints its path. It seeds SQLite, runs the real witness path, detects one missing table, exercises rollback, and writes both witness formats.

The sample does not read or write the current directory. Delete the printed folder when finished.

## Use your own config

Create a starter config, then update its commands and checks:

```sh
mcw init --output mcw.toml
export MCW_DATABASE_URL='sqlite:///tmp/mcw-ci.db'
export MCW_SIGNING_KEY="$CI_WITNESS_KEY"

mcw witness \
  --config mcw.toml \
  --output witness \
  --confirm-test-database \
  --exercise-rollback \
  --json
```

`witness.json` is for tools. `witness.md` is formatted for pull-request review.

Verify the signed JSON later with the same key:

```sh
mcw verify witness/witness.json --json
```

## Config file

The config names the database type, URL environment variable, migration command, rollback command, and one-value checks.

```toml
version = 1

[database]
dialect = "sqlite"
url_env = "MCW_DATABASE_URL"
environment = "test"

[migration]
command = ["sh", "./migrations/up.sh"]

[rollback]
command = ["sh", "./migrations/down.sh"]

[[invariants]]
name = "accounts table committed"
query = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='accounts'"
expect_after = "1"
expect_rollback = "$before"
```

Each check query must return one value. A query error fails the run. Matching errors do not count as matching values.

SQLite uses its embedded library. It records `quick_check` and foreign-key results. PostgreSQL uses `psql --dbname` and records a connection probe.

## Safety checks

The CLI requires two confirmations before it runs a configured command:

- The config environment must be `test`, `ci`, `development`, or `ephemeral`.
- The command must include `--confirm-test-database`.

URLs containing `prod`, `production`, `primary`, or `live-db` are rejected. These name checks cannot identify every production database. Review the target yourself.

Rollback requires a configured command and `--exercise-rollback`. Both are checked before any database query or command runs.

Database URLs and signing keys are omitted from witness files. PostgreSQL errors redact the configured database URL.

## Exit codes

| Code | Meaning |
| ---: | --- |
| 0 | The witness passed, verification passed, or the demo completed |
| 2 | A migration or check failed |
| 3 | The config or safety confirmation is invalid |
| 4 | A command, database, or artifact operation failed |

With `--json`, the final result is written to stdout. Progress and errors use stderr. Commands never prompt for input.

## Develop and verify

Requirements: Rust 1.85 or later, Node 22 or later, and npm. The CI matrix verifies these minimum versions.

```sh
npm ci
npm test
npm run lint
npm run build
```

The build writes the binary to `dist/bin/mcw`. It writes the static site to `dist/site/`.

## Privacy and terms

The browser demo uses same-origin files and a `demo:` session key. The CLI omits configured secrets from witness files.

See the [privacy page](https://migration-commit-witness.sociobot.in/privacy/) and [terms](https://migration-commit-witness.sociobot.in/terms/).
