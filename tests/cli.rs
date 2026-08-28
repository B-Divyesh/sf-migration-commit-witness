use rusqlite::Connection;
use serde_json::Value;
use std::{fs, process::Command};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_mcw")
}

#[test]
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
    let witness: Value =
        serde_json::from_slice(&fs::read(output.join("witness.json")).unwrap()).unwrap();
    assert_eq!(witness["rollback"]["exercised"], true);
    assert_eq!(witness["rollback"]["assertions"][0]["passed"], true);
    assert!(witness["signature"]["value"].as_str().unwrap().len() == 64);

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
fn flags_partial_outcome_when_command_reports_success() {
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
