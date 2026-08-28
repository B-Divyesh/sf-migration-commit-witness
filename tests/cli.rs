//! @claim:exit-code-contract

use rusqlite::Connection;
use serde_json::Value;
use std::{fs, process::Command};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_mcw")
}

#[test]
/// @claim:evidence-formats
/// @claim:rollback-exercise
/// @claim:secret-omission
/// @claim:json-stream-contract
fn documented_sqlite_migration_and_rollback_pass() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("case.db");
    Connection::open(&database).unwrap();
    let up = temp.path().join("up.py");
    let down = temp.path().join("down.py");
    fs::write(&up, "import os,sqlite3\np=os.environ['MCW_DATABASE_URL'].removeprefix('sqlite://')\nc=sqlite3.connect(p)\nc.execute('CREATE TABLE accounts(id INTEGER PRIMARY KEY)')\nc.commit()\n").unwrap();
    fs::write(&down, "import os,sqlite3\np=os.environ['MCW_DATABASE_URL'].removeprefix('sqlite://')\nc=sqlite3.connect(p)\nc.execute('DROP TABLE accounts')\nc.commit()\n").unwrap();
    let policy = policy(&up, Some(&down), "1");
    let policy_path = temp.path().join("mcw.toml");
    fs::write(&policy_path, policy).unwrap();
    let output = temp.path().join("witness");
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--confirm-test-database",
            "--exercise-rollback",
            "--json",
        ])
        .env(
            "MCW_DATABASE_URL",
            format!("sqlite://{}", database.display()),
        )
        .env("MCW_SIGNING_KEY", "12345678901234567890123456789012")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let summary: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["status"], "passed");
    assert!(String::from_utf8_lossy(&result.stderr).contains("mcw: confirming"));
    assert!(!String::from_utf8_lossy(&result.stdout).contains("confirm"));
    let witness: Value =
        serde_json::from_slice(&fs::read(output.join("witness.json")).unwrap()).unwrap();
    assert_eq!(witness["rollback"]["exercised"], true);
    assert_eq!(witness["rollback"]["assertions"][0]["passed"], true);
    assert!(witness["signature"]["value"].as_str().unwrap().len() == 64);
    let markdown = fs::read_to_string(output.join("witness.md")).unwrap();
    assert!(markdown.contains(witness["run_id"].as_str().unwrap()));
    assert!(markdown.contains("**Status: PASS**"));
    for secret in [
        database.display().to_string(),
        "12345678901234567890123456789012".into(),
    ] {
        assert!(!serde_json::to_string(&witness).unwrap().contains(&secret));
        assert!(!markdown.contains(&secret));
    }

    let verify = Command::new(binary())
        .args([
            "verify",
            output.join("witness.json").to_str().unwrap(),
            "--json",
        ])
        .env("MCW_SIGNING_KEY", "12345678901234567890123456789012")
        .output()
        .unwrap();
    assert!(
        verify.status.success(),
        "{}",
        String::from_utf8_lossy(&verify.stderr)
    );
}

#[test]
/// @claim:partial-commit-detection
fn flags_non_transactional_ddl_partial_outcome_when_command_reports_success() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("partial.db");
    Connection::open(&database).unwrap();
    let up = temp.path().join("partial.py");
    fs::write(&up, "import os,sqlite3\np=os.environ['MCW_DATABASE_URL'].removeprefix('sqlite://')\nc=sqlite3.connect(p)\nc.execute('CREATE TABLE only_half_done(id INTEGER)')\nc.commit()\n# Framework-style success despite incomplete intended state.\n").unwrap();
    let policy = policy(&up, None, "2");
    let policy_path = temp.path().join("mcw.toml");
    fs::write(&policy_path, policy).unwrap();
    let output = temp.path().join("witness");
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--confirm-test-database",
            "--allow-unsigned",
            "--json",
        ])
        .env(
            "MCW_DATABASE_URL",
            format!("sqlite://{}", database.display()),
        )
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    let witness: Value =
        serde_json::from_slice(&fs::read(output.join("witness.json")).unwrap()).unwrap();
    assert_eq!(witness["migration"]["reported_success"], true);
    assert_eq!(witness["status"], "failed");
    assert!(
        witness["reasons"][0]
            .as_str()
            .unwrap()
            .contains("invariant failed")
    );
}

#[test]
fn flags_deferred_constraint_commit_failure_hidden_by_success_status() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("deferred.db");
    Connection::open(&database).unwrap();
    let up = temp.path().join("deferred.py");
    fs::write(
        &up,
        "import os,sqlite3\np=os.environ['MCW_DATABASE_URL'].removeprefix('sqlite://')\nc=sqlite3.connect(p)\nc.execute('PRAGMA foreign_keys=ON')\nc.executescript('CREATE TABLE parent(id INTEGER PRIMARY KEY); CREATE TABLE child(parent_id INTEGER, FOREIGN KEY(parent_id) REFERENCES parent(id) DEFERRABLE INITIALLY DEFERRED);')\nc.commit()\ntry:\n c.execute('BEGIN')\n c.execute('INSERT INTO child VALUES (99)')\n c.commit()\nexcept sqlite3.IntegrityError:\n c.rollback()\n# Simulate an engine that reports success after commit failed; DDL remains.\n",
    )
    .unwrap();
    let policy = format!(
        r#"version = 1
[database]
dialect = "sqlite"
url_env = "MCW_DATABASE_URL"
environment = "ci"
[migration]
command = ["python3", {:?}]
[[invariants]]
name = "all intended objects committed"
query = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name IN ('parent','child','commit_marker')"
expect_after = "3"
"#,
        up.display().to_string()
    );
    let policy_path = temp.path().join("mcw.toml");
    fs::write(&policy_path, policy).unwrap();
    let output = temp.path().join("witness");
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--confirm-test-database",
            "--allow-unsigned",
            "--json",
        ])
        .env(
            "MCW_DATABASE_URL",
            format!("sqlite://{}", database.display()),
        )
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    let witness: Value =
        serde_json::from_slice(&fs::read(output.join("witness.json")).unwrap()).unwrap();
    assert_eq!(witness["migration"]["reported_success"], true);
    assert_eq!(witness["after"]["invariants"][0]["value"], "2");
    assert_eq!(witness["after_assertions"][0]["passed"], false);
}

#[test]
/// @claim:test-confirmation
fn refuses_to_run_without_explicit_test_confirmation() {
    let temp = tempfile::tempdir().unwrap();
    let policy_path = temp.path().join("mcw.toml");
    fs::write(
        &policy_path,
        policy(temp.path().join("up.py").as_path(), None, "1"),
    )
    .unwrap();
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--allow-unsigned",
            "--json",
        ])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(3));
    let error: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert!(
        error["error"]
            .as_str()
            .unwrap()
            .contains("--confirm-test-database")
    );
}

#[test]
/// @claim:query-error-fails
fn readme_policy_fails_when_before_and_rollback_queries_both_error() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("query-error.db");
    Connection::open(&database).unwrap();
    let up = temp.path().join("up.py");
    let down = temp.path().join("down.py");
    fs::write(&up, "import os,sqlite3\np=os.environ['MCW_DATABASE_URL'].removeprefix('sqlite://')\nc=sqlite3.connect(p)\nc.executescript('CREATE TABLE accounts(id INTEGER PRIMARY KEY); INSERT INTO accounts VALUES (1);')\nc.commit()\n").unwrap();
    fs::write(&down, "import os,sqlite3\np=os.environ['MCW_DATABASE_URL'].removeprefix('sqlite://')\nc=sqlite3.connect(p)\nc.execute('DROP TABLE accounts')\nc.commit()\n").unwrap();
    let policy_path = temp.path().join("mcw.toml");
    fs::write(
        &policy_path,
        format!(
            r#"version = 1
[database]
dialect = "sqlite"
url_env = "MCW_DATABASE_URL"
environment = "test"
[migration]
command = ["python3", {:?}]
[rollback]
command = ["python3", {:?}]
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
"#,
            up.display().to_string(),
            down.display().to_string()
        ),
    )
    .unwrap();
    let output = temp.path().join("witness");
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--confirm-test-database",
            "--exercise-rollback",
            "--allow-unsigned",
            "--json",
        ])
        .env(
            "MCW_DATABASE_URL",
            format!("sqlite://{}", database.display()),
        )
        .output()
        .unwrap();

    assert_eq!(result.status.code(), Some(2));
    let witness: Value =
        serde_json::from_slice(&fs::read(output.join("witness.json")).unwrap()).unwrap();
    assert_eq!(witness["status"], "failed");
    assert_eq!(witness["baseline"]["invariants"][1]["value"], Value::Null);
    assert!(witness["baseline"]["invariants"][1]["error"].is_string());
    assert_eq!(witness["rollback"]["assertions"][1]["passed"], false);
    assert!(
        witness["rollback"]["assertions"][1]["detail"]
            .as_str()
            .unwrap()
            .contains("baseline query error")
    );
    let markdown = fs::read_to_string(output.join("witness.md")).unwrap();
    assert!(markdown.contains("| seed account present | ERROR:"));
    assert!(markdown.contains("| `$before` | FAIL |"));
}

#[test]
/// @claim:rollback-preflight
fn missing_requested_rollback_is_rejected_before_migration_mutates_database() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("preflight.db");
    Connection::open(&database).unwrap();
    let up = temp.path().join("up.py");
    fs::write(&up, "import os,sqlite3\np=os.environ['MCW_DATABASE_URL'].removeprefix('sqlite://')\nc=sqlite3.connect(p)\nc.execute('CREATE TABLE only_half_done(id INTEGER)')\nc.commit()\n").unwrap();
    let policy_path = temp.path().join("mcw.toml");
    fs::write(&policy_path, policy(&up, None, "1")).unwrap();
    let output = temp.path().join("witness");
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--confirm-test-database",
            "--exercise-rollback",
            "--allow-unsigned",
            "--json",
        ])
        .env(
            "MCW_DATABASE_URL",
            format!("sqlite://{}", database.display()),
        )
        .output()
        .unwrap();

    assert_eq!(result.status.code(), Some(3));
    let count: i64 = Connection::open(&database)
        .unwrap()
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='only_half_done'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 0, "migration must not run before rollback preflight");
    assert!(
        !output.exists(),
        "invalid configuration must not emit evidence"
    );
}

#[cfg(unix)]
#[test]
/// @claim:postgres-psql
fn postgres_uses_dbname_uri_without_pgdatabase() {
    let temp = tempfile::tempdir().unwrap();
    let fake_bin = temp.path().join("bin");
    fs::create_dir(&fake_bin).unwrap();
    let psql = fake_bin.join("psql");
    fs::write(
        &psql,
        "#!/bin/sh\n[ -z \"${PGDATABASE+x}\" ] || { echo PGDATABASE-was-set >&2; exit 90; }\n[ \"$1\" = --dbname ] || { echo missing-dbname >&2; exit 91; }\n[ \"$2\" = \"$MCW_DATABASE_URL\" ] || { echo wrong-dbname >&2; exit 92; }\nprintf '1\\n'\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&psql).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&psql, permissions).unwrap();
    let policy_path = temp.path().join("mcw.toml");
    fs::write(&policy_path, postgres_policy(Some("true"))).unwrap();
    let output = temp.path().join("witness");
    let path = format!(
        "{}:{}",
        fake_bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--confirm-test-database",
            "--exercise-rollback",
            "--allow-unsigned",
            "--json",
        ])
        .env(
            "MCW_DATABASE_URL",
            "postgresql:///mcw_qa?host=/var/run/postgresql",
        )
        .env("PATH", path)
        .output()
        .unwrap();

    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let witness: Value =
        serde_json::from_slice(&fs::read(output.join("witness.json")).unwrap()).unwrap();
    assert_eq!(witness["status"], "passed");
    assert_eq!(witness["after"]["dialect_checks"][0]["value"], "1");
}

#[cfg(unix)]
#[test]
/// @claim:secret-redaction
fn postgres_subprocess_errors_redact_the_complete_database_url() {
    let temp = tempfile::tempdir().unwrap();
    let fake_bin = temp.path().join("bin");
    fs::create_dir(&fake_bin).unwrap();
    let psql = fake_bin.join("psql");
    fs::write(
        &psql,
        "#!/bin/sh\necho \"connection failed for $2\" >&2\nexit 2\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&psql).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&psql, permissions).unwrap();
    let policy_path = temp.path().join("mcw.toml");
    fs::write(&policy_path, postgres_policy(None)).unwrap();
    let output = temp.path().join("witness");
    let path = format!(
        "{}:{}",
        fake_bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let database_url = "postgresql://qa_user:supersecret@127.0.0.1:65432/mcw_qa";
    let result = Command::new(binary())
        .args([
            "witness",
            "--config",
            policy_path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--confirm-test-database",
            "--allow-unsigned",
            "--json",
        ])
        .env("MCW_DATABASE_URL", database_url)
        .env("PATH", path)
        .output()
        .unwrap();

    assert_eq!(result.status.code(), Some(2));
    for artifact in ["witness.json", "witness.md"] {
        let evidence = fs::read_to_string(output.join(artifact)).unwrap();
        assert!(!evidence.contains(database_url));
        assert!(!evidence.contains("supersecret"));
        assert!(evidence.contains("REDACTED DATABASE URL"));
    }
}

fn policy(up: &std::path::Path, down: Option<&std::path::Path>, expected: &str) -> String {
    let rollback = down
        .map(|path| {
            format!(
                "\n[rollback]\ncommand = [\"python3\", {:?}]\n",
                path.display().to_string()
            )
        })
        .unwrap_or_default();
    format!(
        r#"version = 1
[database]
dialect = "sqlite"
url_env = "MCW_DATABASE_URL"
environment = "test"
[migration]
command = ["python3", {:?}]
{}
[[invariants]]
name = "accounts table committed"
query = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='accounts'"
expect_after = "{}"
expect_rollback = "$before"
"#,
        up.display().to_string(),
        rollback,
        expected
    )
}

fn postgres_policy(rollback: Option<&str>) -> String {
    let rollback = rollback
        .map(|program| format!("[rollback]\ncommand = [\"{program}\"]\n"))
        .unwrap_or_default();
    format!(
        r#"version = 1
[database]
dialect = "postgres"
url_env = "MCW_DATABASE_URL"
environment = "ci"
[migration]
command = ["true"]
{rollback}
[[invariants]]
name = "scalar query"
query = "SELECT 1"
expect_after = "1"
expect_rollback = "$before"
"#
    )
}
