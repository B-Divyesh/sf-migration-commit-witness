use rusqlite::Connection;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    process::{Command, Output, Stdio},
    thread,
    time::{Duration, Instant},
};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_mcw")
}

#[test]
/// @claim:demo-isolation
/// @claim:demo-evidence
/// @claim:demo-record-match
/// @claim:sqlite-builtins
fn demo_runs_in_one_isolated_directory_and_matches_the_browser_record() {
    let sandbox = tempfile::tempdir().unwrap();
    let caller = sandbox.path().join("empty-caller");
    let workspace = sandbox.path().join("isolated-demo");
    fs::create_dir(&caller).unwrap();
    let result = Command::new(binary())
        .args(["demo", "--output", workspace.to_str().unwrap(), "--json"])
        .current_dir(&caller)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        fs::read_dir(&caller).unwrap().count(),
        0,
        "demo wrote into the caller directory"
    );
    let summary: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["ok"], true);
    assert_eq!(summary["migration_exit_code"], 0);
    assert_eq!(summary["detected_status"], "failed");
    assert_eq!(summary["rollback_exercised"], true);
    assert_eq!(summary["rollback_restored"], true);

    let witness: Value =
        serde_json::from_slice(&fs::read(workspace.join("witness/witness.json")).unwrap()).unwrap();
    assert_eq!(witness["baseline"]["invariants"][0]["value"], "0");
    assert_eq!(witness["baseline"]["invariants"][1]["value"], "12");
    assert_eq!(witness["after"]["invariants"][0]["value"], "1");
    assert_eq!(witness["after_assertions"][0]["expected"], "2");
    assert_eq!(witness["baseline"]["dialect_checks"][0]["value"], "ok");
    assert_eq!(witness["baseline"]["dialect_checks"][1]["value"], "0");
    assert_eq!(
        witness["rollback"]["snapshot"]["invariants"][0]["value"],
        "0"
    );
    assert_eq!(witness["rollback"]["assertions"][0]["passed"], true);
    assert!(
        witness["signature"].is_null(),
        "the demo must be clearly unsigned"
    );
    assert!(workspace.join("witness/witness.md").is_file());

    let record: Value =
        serde_json::from_slice(&fs::read("site/public/demo-record.json").unwrap()).unwrap();
    assert_eq!(record["stages"][0]["schema"], "0 / 2");
    assert_eq!(record["stages"][0]["rows"], "12");
    assert_eq!(record["stages"][1]["verdict"], "Command returned 0");
    assert_eq!(record["stages"][2]["schema"], "1 / 2");
    assert_eq!(record["stages"][3]["schema"], "0 / 2");
    assert!(
        record["stages"][3]["output"]
            .as_str()
            .unwrap()
            .contains("unsigned demo")
    );

    let actual: BTreeSet<_> = fs::read_dir(&workspace)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    let expected: BTreeSet<_> = [
        "mcw.toml",
        "migration.sql",
        "rollback.sql",
        "sample.db",
        "witness",
    ]
    .into_iter()
    .map(Into::into)
    .collect();
    assert_eq!(actual, expected);
}

#[test]
/// @claim:mit-free-cli
fn core_cli_is_mit_licensed_and_has_no_license_gate() {
    let license = fs::read_to_string("LICENSE").unwrap();
    assert!(license.contains("Permission is hereby granted, free of charge"));
    assert!(
        fs::read_to_string("Cargo.toml")
            .unwrap()
            .contains("license = \"MIT\"")
    );
    let help = Command::new(binary()).arg("--help").output().unwrap();
    assert!(help.status.success());
    let text = String::from_utf8(help.stdout).unwrap();
    for command in ["demo", "init", "witness", "verify"] {
        assert!(text.contains(command), "missing free command {command}");
    }
    assert!(!text.to_ascii_lowercase().contains("license token"));
}

#[test]
/// @claim:environment-forwarding
fn configured_database_url_reaches_migration_and_rollback_without_reaching_witness_files() {
    let temp = tempfile::tempdir().unwrap();
    let database = temp.path().join("forwarding.db");
    Connection::open(&database).unwrap();
    let migration_receipt = temp.path().join("migration-url.txt");
    let rollback_receipt = temp.path().join("rollback-url.txt");
    let migration = temp.path().join("migration.sh");
    let rollback = temp.path().join("rollback.sh");
    fs::write(
        &migration,
        "#!/bin/sh\nprintf '%s' \"$MCW_FORWARDED_URL\" > \"$MCW_MIGRATION_RECEIPT\"\n",
    )
    .unwrap();
    fs::write(
        &rollback,
        "#!/bin/sh\nprintf '%s' \"$MCW_FORWARDED_URL\" > \"$MCW_ROLLBACK_RECEIPT\"\n",
    )
    .unwrap();
    let policy_path = temp.path().join("mcw.toml");
    fs::write(
        &policy_path,
        format!(
            r#"version = 1
[database]
dialect = "sqlite"
url_env = "MCW_FORWARDED_URL"
environment = "test"
[migration]
command = ["sh", {:?}]
[rollback]
command = ["sh", {:?}]
[[invariants]]
name = "database remains available"
query = "SELECT 0"
expect_after = "0"
expect_rollback = "$before"
"#,
            migration.display().to_string(),
            rollback.display().to_string()
        ),
    )
    .unwrap();
    let database_url = format!("sqlite://{}", database.display());
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
        .env("MCW_FORWARDED_URL", &database_url)
        .env("MCW_MIGRATION_RECEIPT", &migration_receipt)
        .env("MCW_ROLLBACK_RECEIPT", &rollback_receipt)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(fs::read_to_string(migration_receipt).unwrap(), database_url);
    assert_eq!(fs::read_to_string(rollback_receipt).unwrap(), database_url);
    for artifact in ["witness.json", "witness.md"] {
        assert!(
            !fs::read_to_string(output.join(artifact))
                .unwrap()
                .contains(&database_url),
            "{artifact} retained the configured database URL"
        );
    }
}

#[test]
/// @claim:init-config
fn init_creates_a_parseable_non_overwriting_starter_config_that_runs_a_witness() {
    let temp = tempfile::tempdir().unwrap();
    let config = temp.path().join("mcw.toml");
    let created = Command::new(binary())
        .args(["init", "--output", "mcw.toml", "--json"])
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(
        created.status.success(),
        "{}",
        String::from_utf8_lossy(&created.stderr)
    );
    let generated = fs::read_to_string(&config).unwrap();
    let policy: migration_commit_witness::Policy = toml::from_str(&generated).unwrap();
    assert_eq!(policy.database.environment, "test");
    assert_eq!(policy.invariants.len(), 1);
    assert!(policy.rollback.is_some());

    let migrations = temp.path().join("migrations");
    fs::create_dir(&migrations).unwrap();
    fs::write(
        migrations.join("up.sh"),
        "#!/bin/sh\npython3 -c 'import os,sqlite3; p=os.environ[\"MCW_DATABASE_URL\"].removeprefix(\"sqlite://\"); c=sqlite3.connect(p); c.execute(\"CREATE TABLE accounts(id INTEGER PRIMARY KEY)\"); c.commit()'\n",
    )
    .unwrap();
    fs::write(
        migrations.join("down.sh"),
        "#!/bin/sh\npython3 -c 'import os,sqlite3; p=os.environ[\"MCW_DATABASE_URL\"].removeprefix(\"sqlite://\"); c=sqlite3.connect(p); c.execute(\"DROP TABLE accounts\"); c.commit()'\n",
    )
    .unwrap();
    let database = temp.path().join("starter.db");
    Connection::open(&database).unwrap();
    let witness = Command::new(binary())
        .args([
            "witness",
            "--config",
            "mcw.toml",
            "--output",
            "witness",
            "--confirm-test-database",
            "--exercise-rollback",
            "--allow-unsigned",
            "--json",
        ])
        .current_dir(temp.path())
        .env(
            "MCW_DATABASE_URL",
            format!("sqlite://{}", database.display()),
        )
        .output()
        .unwrap();
    assert!(
        witness.status.success(),
        "{}",
        String::from_utf8_lossy(&witness.stderr)
    );
    assert!(temp.path().join("witness/witness.json").is_file());

    let second = Command::new(binary())
        .args(["init", "--output", "mcw.toml", "--json"])
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert_eq!(second.status.code(), Some(4));
    assert_eq!(fs::read_to_string(config).unwrap(), generated);
}

#[test]
/// @claim:non-interactive-commands
fn every_public_subcommand_finishes_with_closed_stdin() {
    let temp = tempfile::tempdir().unwrap();
    let demo = temp.path().join("demo");
    let init = temp.path().join("starter.toml");
    let witness = temp.path().join("witness");
    let cases: Vec<Vec<String>> = vec![
        vec![
            "demo".into(),
            "--output".into(),
            demo.display().to_string(),
            "--json".into(),
        ],
        vec![
            "init".into(),
            "--output".into(),
            init.display().to_string(),
            "--json".into(),
        ],
        vec![
            "witness".into(),
            "--config".into(),
            temp.path().join("missing.toml").display().to_string(),
            "--output".into(),
            witness.display().to_string(),
            "--confirm-test-database".into(),
            "--allow-unsigned".into(),
            "--json".into(),
        ],
        vec![
            "verify".into(),
            temp.path().join("missing.json").display().to_string(),
            "--json".into(),
        ],
    ];
    for args in cases {
        let output = run_with_closed_stdin(&args, temp.path());
        assert!(
            output.status.code().is_some(),
            "{} did not return an exit status",
            args.join(" ")
        );
    }
}

fn run_with_closed_stdin(args: &[String], current_dir: &Path) -> Output {
    let started = Instant::now();
    let mut child = Command::new(binary())
        .args(args)
        .current_dir(current_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = started + Duration::from_secs(5);
    loop {
        if child.try_wait().unwrap().is_some() {
            return child.wait_with_output().unwrap();
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            panic!(
                "{} did not finish with closed stdin within five seconds",
                args.join(" ")
            );
        }
        thread::sleep(Duration::from_millis(10));
    }
}
