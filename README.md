# Migration Commit Witness

`mcw` is a CI-focused witness for backend teams reviewing SQL migrations. It
runs your existing migration command against an explicitly confirmed disposable
database, records selected invariants on both sides of the commit boundary, can
exercise a rollback you supply, and emits signed JSON and readable Markdown.
It does not generate migrations, replace your migration engine, or connect to
production by design.

Live docs: <https://migration-commit-witness.sociobot.in>

## Install

Build the single binary with stable Rust:

```sh
cargo install --path .
mcw --help
```

The release archive produced by `npm run build` also places the local binary at
`dist/bin/mcw` and the deployable documentation site at `dist/site/`.

## Usage

Create a starter policy, edit its commands and assertions, then run it only
against a disposable database:

```sh
mcw init --output mcw.toml
export MCW_DATABASE_URL='sqlite:///tmp/mcw-ci.db'
export MCW_SIGNING_KEY='replace-with-a-CI-secret-at-least-32-bytes-long'

mcw witness \
  --config mcw.toml \
  --output witness \
  --confirm-test-database \
  --exercise-rollback \
  --json
```

`witness/witness.json` is stable machine-readable evidence;
`witness/witness.md` is the PR-friendly record. `--json` prints the final
summary to stdout. All progress and errors go to stderr. No prompt is ever
shown in CI.

Verify an artifact later with the same secret:

```sh
mcw verify witness/witness.json --key-env MCW_SIGNING_KEY --json
```

Local experiments may use `--allow-unsigned`, but CI witnesses should be
signed. Exit codes are `0` for a passing/verified witness, `2` for a migration
or assertion failure, `3` for unsafe/invalid configuration, and `4` for a
runtime or artifact error.

### Policy file

```toml
version = 1

[database]
dialect = "sqlite"             # sqlite or postgres; behavior is labeled
url_env = "MCW_DATABASE_URL"   # the URL is read from env and never recorded
environment = "test"           # test, ci, development, or ephemeral only

[migration]
command = ["sh", "./migrations/up.sh"]

[rollback]
command = ["sh", "./migrations/down.sh"]

[[invariants]]
name = "accounts table committed"
query = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='accounts'"
expect_after = "1"
expect_rollback = "$before"

[[invariants]]
name = "seed account present"
query = "SELECT count(*) FROM accounts WHERE id = 1"
expect_after = "1"
expect_rollback = "$before"
```

Every invariant query must return exactly one scalar value. `$before` compares
the rollback snapshot with the recorded baseline. PostgreSQL policies require
`psql` on `PATH`; SQLite is embedded. Migration and rollback commands inherit
the configured database URL environment variable.

## Safety model

`mcw witness` refuses to start unless the policy labels the environment as
`test`, `ci`, `development`, or `ephemeral` **and** the caller passes
`--confirm-test-database`. It rejects database URLs containing common
production labels. Rollback is never inferred: both a `[rollback]` command and
`--exercise-rollback` are required. The database URL and signing key are never
written to evidence.

Dialect checks are honest and explicit: SQLite records `quick_check` and
foreign-key violations; PostgreSQL records server reachability and relies on
the selected invariants for application-specific commit proof.

## Develop and verify

Requirements: stable Rust, Node 22+, and npm.

```sh
npm install
npm test
npm run build
npm run dev
```

`npm test` runs Rust unit/integration tests and site tests. The documented CLI
example is covered end to end by an isolated SQLite fixture. To package without
publishing, run `cargo package --allow-dirty`; registry credentials remain with
the factory.

## Privacy and license

The CLI is local-only and has no telemetry. The site stores a pasted Sociobot
license and its last verification result in your browser; see `/privacy/` and
`/terms/`. Source is MIT licensed; the optional Team rollout kit is a one-time
purchase and does not gate witness safety, evidence export, or accessibility.
