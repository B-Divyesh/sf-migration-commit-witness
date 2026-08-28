use serde_json::Value;
use std::{collections::BTreeSet, fs, process::Command};

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
