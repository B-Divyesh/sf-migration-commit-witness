use hmac::{Hmac, Mac};
use rusqlite::{Connection, types::ValueRef};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type HmacSha256 = Hmac<Sha256>;

pub const EXIT_WITNESS_FAILED: i32 = 2;
pub const EXIT_CONFIG: i32 = 3;
pub const EXIT_RUNTIME: i32 = 4;

#[derive(Debug)]
pub struct AppError {
    pub code: i32,
    pub message: String,
}

impl AppError {
    fn config(message: impl Into<String>) -> Self {
        Self {
            code: EXIT_CONFIG,
            message: message.into(),
        }
    }

    fn runtime(message: impl Into<String>) -> Self {
        Self {
            code: EXIT_RUNTIME,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for AppError {}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub version: u32,
    pub database: DatabasePolicy,
    pub migration: CommandPolicy,
    pub rollback: Option<CommandPolicy>,
    #[serde(default)]
    pub invariants: Vec<InvariantPolicy>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabasePolicy {
    pub dialect: Dialect,
    pub url_env: String,
    pub environment: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    Sqlite,
    Postgres,
}

impl std::fmt::Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlite => f.write_str("sqlite"),
            Self::Postgres => f.write_str("postgres"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandPolicy {
    pub command: Vec<String>,
    #[serde(default = "default_exit_codes")]
    pub success_exit_codes: Vec<i32>,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

fn default_exit_codes() -> Vec<i32> {
    vec![0]
}
fn default_timeout() -> u64 {
    300
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvariantPolicy {
    pub name: String,
    pub query: String,
    pub expect_after: String,
    pub expect_rollback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Witness {
    pub witness_version: u32,
    pub product: String,
    pub run_id: String,
    pub created_at_unix_ms: u128,
    pub dialect: Dialect,
    pub environment: String,
    pub config_sha256: String,
    pub baseline: Snapshot,
    pub migration: CommandEvidence,
    pub after: Snapshot,
    pub after_assertions: Vec<Assertion>,
    pub rollback: RollbackEvidence,
    pub status: WitnessStatus,
    pub reasons: Vec<String>,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub captured_at_unix_ms: u128,
    pub invariants: Vec<Observation>,
    pub dialect_checks: Vec<Observation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub name: String,
    pub value: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEvidence {
    pub command: Vec<String>,
    pub exit_code: Option<i32>,
    pub duration_ms: u128,
    pub timed_out: bool,
    pub reported_success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assertion {
    pub name: String,
    pub expected: String,
    pub observed: Option<String>,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackEvidence {
    pub requested: bool,
    pub configured: bool,
    pub exercised: bool,
    pub command: Option<CommandEvidence>,
    pub snapshot: Option<Snapshot>,
    pub assertions: Vec<Assertion>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WitnessStatus {
    Passed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algorithm: String,
    pub key_source: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct WitnessOptions {
    pub config: PathBuf,
    pub output: PathBuf,
    pub confirm_test_database: bool,
    pub exercise_rollback: bool,
    pub allow_unsigned: bool,
    pub signing_key_env: String,
    pub force: bool,
    /// Internal direct value used only by the isolated bundled demo.
    pub database_url_override: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WitnessSummary {
    pub ok: bool,
    pub status: WitnessStatus,
    pub run_id: String,
    pub json: String,
    pub markdown: String,
    pub rollback_exercised: bool,
}

#[derive(Debug, Serialize)]
pub struct VerifySummary {
    pub ok: bool,
    pub run_id: String,
    pub algorithm: String,
}

#[derive(Debug, Serialize)]
pub struct DemoSummary {
    pub ok: bool,
    pub detected_status: WitnessStatus,
    pub workspace: String,
    pub json: String,
    pub markdown: String,
    pub migration_exit_code: Option<i32>,
    pub rollback_exercised: bool,
    pub rollback_restored: bool,
}

const DEMO_POLICY: &str = include_str!("../examples/demo/mcw.toml");

/// Runs the bundled fixture through the same witness path used by CI.
/// The caller's working directory and environment are not read or changed.
pub fn run_demo(requested: Option<&Path>) -> Result<DemoSummary, AppError> {
    let workspace = match requested {
        Some(path) => path.to_path_buf(),
        None => {
            let stamp = now_ms();
            env::temp_dir().join(format!("mcw-demo-{stamp}-{}", std::process::id()))
        }
    };
    fs::create_dir(&workspace).map_err(|error| {
        AppError::runtime(format!(
            "cannot create isolated demo workspace {}: {error}; choose a new path",
            workspace.display()
        ))
    })?;
    let database = workspace.join("sample.db");
    let connection = Connection::open(&database)
        .map_err(|error| AppError::runtime(format!("cannot create demo database: {error}")))?;
    connection
        .execute(
            "CREATE TABLE release_notes(id INTEGER PRIMARY KEY, note TEXT NOT NULL)",
            [],
        )
        .and_then(|_| {
            let transaction = connection.unchecked_transaction()?;
            for index in 1..=12 {
                transaction.execute(
                    "INSERT INTO release_notes(id, note) VALUES (?1, ?2)",
                    (index, format!("release note {index}")),
                )?;
            }
            transaction.commit()
        })
        .map_err(|error| AppError::runtime(format!("cannot seed demo database: {error}")))?;
    drop(connection);

    fs::write(
        workspace.join("migration.sql"),
        include_str!("../examples/demo/migration.sql"),
    )
    .map_err(|error| AppError::runtime(format!("cannot copy demo migration: {error}")))?;
    fs::write(
        workspace.join("rollback.sql"),
        include_str!("../examples/demo/rollback.sql"),
    )
    .map_err(|error| AppError::runtime(format!("cannot copy demo rollback: {error}")))?;
    let executable = env::current_exe()
        .map_err(|error| AppError::runtime(format!("cannot locate the mcw binary: {error}")))?;
    let policy = DEMO_POLICY
        .replace(
            "__MCW_BINARY__",
            &toml_string(&executable.display().to_string()),
        )
        .replace("__DEMO_DB__", &toml_string(&database.display().to_string()));
    let config = workspace.join("mcw.toml");
    fs::write(&config, policy)
        .map_err(|error| AppError::runtime(format!("cannot copy demo config: {error}")))?;
    let output = workspace.join("witness");
    let (witness, summary) = run_witness(&WitnessOptions {
        config,
        output,
        confirm_test_database: true,
        exercise_rollback: true,
        allow_unsigned: true,
        signing_key_env: "MCW_DEMO_SIGNING_KEY_NOT_USED".into(),
        force: false,
        database_url_override: Some(format!("sqlite://{}", database.display())),
    })?;
    let rollback_restored = witness
        .rollback
        .assertions
        .iter()
        .all(|assertion| assertion.passed);
    Ok(DemoSummary {
        ok: witness.status == WitnessStatus::Failed && rollback_restored,
        detected_status: witness.status,
        workspace: workspace.display().to_string(),
        json: summary.json,
        markdown: summary.markdown,
        migration_exit_code: witness.migration.exit_code,
        rollback_exercised: witness.rollback.exercised,
        rollback_restored,
    })
}

fn toml_string(value: &str) -> String {
    serde_json::to_string(value).expect("a path is JSON/TOML string compatible")
}

/// Internal fixture action used by `mcw demo`; hidden from normal help output.
pub fn demo_step(action: &str, database: &Path) -> Result<(), AppError> {
    let connection = Connection::open(database)
        .map_err(|error| AppError::runtime(format!("cannot open demo database: {error}")))?;
    let sql = match action {
        "migrate" => include_str!("../examples/demo/migration.sql"),
        "rollback" => include_str!("../examples/demo/rollback.sql"),
        _ => return Err(AppError::config("unknown demo action")),
    };
    connection
        .execute_batch(sql)
        .map_err(|error| AppError::runtime(format!("demo {action} failed: {error}")))
}

pub const STARTER_POLICY: &str = r#"version = 1

[database]
dialect = "sqlite"
url_env = "MCW_DATABASE_URL"
environment = "test"

[migration]
command = ["sh", "./migrations/up.sh"]
timeout_seconds = 300

[rollback]
command = ["sh", "./migrations/down.sh"]
timeout_seconds = 300

[[invariants]]
name = "accounts table committed"
query = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='accounts'"
expect_after = "1"
expect_rollback = "$before"
"#;

pub fn init_policy(path: &Path) -> Result<(), AppError> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| {
            AppError::runtime(format!(
                "cannot create {}: {e}; choose a new path",
                path.display()
            ))
        })?;
    file.write_all(STARTER_POLICY.as_bytes())
        .map_err(|e| AppError::runtime(format!("cannot write {}: {e}", path.display())))
}

pub fn run_witness(options: &WitnessOptions) -> Result<(Witness, WitnessSummary), AppError> {
    let config_bytes = fs::read(&options.config).map_err(|e| {
        AppError::config(format!(
            "cannot read policy {}: {e}",
            options.config.display()
        ))
    })?;
    let config_text = std::str::from_utf8(&config_bytes)
        .map_err(|_| AppError::config("policy must be UTF-8 TOML"))?;
    let policy: Policy = toml::from_str(config_text).map_err(|e| {
        AppError::config(format!("invalid policy {}: {e}", options.config.display()))
    })?;
    validate_policy(&policy, options.confirm_test_database)?;
    if options.exercise_rollback && policy.rollback.is_none() {
        return Err(AppError::config(
            "--exercise-rollback requires an explicit [rollback] command in the policy",
        ));
    }

    let database_url = match &options.database_url_override {
        Some(value) => value.clone(),
        None => env::var(&policy.database.url_env).map_err(|_| {
            AppError::config(format!(
                "database URL environment variable {} is not set",
                policy.database.url_env
            ))
        })?,
    };
    validate_database_url(&database_url, policy.database.dialect)?;

    let signing_key = match env::var(&options.signing_key_env) {
        Ok(value) if value.len() >= 32 => Some(value),
        Ok(_) => {
            return Err(AppError::config(format!(
                "{} must contain at least 32 bytes",
                options.signing_key_env
            )));
        }
        Err(_) if options.allow_unsigned => None,
        Err(_) => {
            return Err(AppError::config(format!(
                "{} is not set; provide a CI secret or pass --allow-unsigned for a local-only witness",
                options.signing_key_env
            )));
        }
    };

    let json_path = options.output.join("witness.json");
    let markdown_path = options.output.join("witness.md");
    if !options.force && (json_path.exists() || markdown_path.exists()) {
        return Err(AppError::runtime(format!(
            "{} already contains a witness; choose another directory or pass --force",
            options.output.display()
        )));
    }

    let created_at = now_ms();
    let config_sha = sha256_hex(&config_bytes);
    let run_id = sha256_hex(format!("{config_sha}:{created_at}:{}", std::process::id()).as_bytes())
        [..16]
        .to_string();
    let baseline = take_snapshot(&policy, &database_url);
    let migration = run_command(&policy.migration)?;
    let after = take_snapshot(&policy, &database_url);
    let after_assertions = compare_after(&policy.invariants, &after);

    let mut reasons = Vec::new();
    add_invariant_query_failures("baseline", &baseline, &mut reasons);
    if !migration.reported_success {
        reasons.push(if migration.timed_out {
            "migration command timed out".into()
        } else {
            format!(
                "migration command reported exit code {}",
                display_exit(migration.exit_code)
            )
        });
    }
    add_snapshot_failures("after", &after, &mut reasons);
    add_assertion_failures("after", &after_assertions, &mut reasons);

    let rollback = if options.exercise_rollback {
        // The requested rollback was preflighted before the database URL was
        // read or any snapshot/command could touch the target.
        let rollback_policy = policy
            .rollback
            .as_ref()
            .expect("rollback policy checked before database access");
        let command = run_command(rollback_policy)?;
        let snapshot = take_snapshot(&policy, &database_url);
        let assertions = compare_rollback(&policy.invariants, &baseline, &snapshot);
        if !command.reported_success {
            reasons.push(if command.timed_out {
                "rollback command timed out".into()
            } else {
                format!(
                    "rollback command reported exit code {}",
                    display_exit(command.exit_code)
                )
            });
        }
        add_snapshot_failures("rollback", &snapshot, &mut reasons);
        add_assertion_failures("rollback", &assertions, &mut reasons);
        RollbackEvidence {
            requested: true,
            configured: true,
            exercised: true,
            command: Some(command),
            snapshot: Some(snapshot),
            assertions,
        }
    } else {
        RollbackEvidence {
            requested: false,
            configured: policy.rollback.is_some(),
            exercised: false,
            command: None,
            snapshot: None,
            assertions: Vec::new(),
        }
    };

    let status = if reasons.is_empty() {
        WitnessStatus::Passed
    } else {
        WitnessStatus::Failed
    };
    let mut witness = Witness {
        witness_version: 1,
        product: "migration-commit-witness".into(),
        run_id: run_id.clone(),
        created_at_unix_ms: created_at,
        dialect: policy.database.dialect,
        environment: policy.database.environment.clone(),
        config_sha256: config_sha,
        baseline,
        migration,
        after,
        after_assertions,
        rollback,
        status,
        reasons,
        signature: None,
    };
    if let Some(key) = signing_key {
        let value = sign_witness(&witness, key.as_bytes())?;
        witness.signature = Some(Signature {
            algorithm: "HMAC-SHA256".into(),
            key_source: options.signing_key_env.clone(),
            value,
        });
    }

    fs::create_dir_all(&options.output).map_err(|e| {
        AppError::runtime(format!("cannot create {}: {e}", options.output.display()))
    })?;
    let json = serde_json::to_string_pretty(&witness)
        .map_err(|e| AppError::runtime(format!("cannot serialize witness: {e}")))?;
    atomic_write(&json_path, format!("{json}\n").as_bytes())?;
    atomic_write(&markdown_path, render_markdown(&witness).as_bytes())?;

    let summary = WitnessSummary {
        ok: status == WitnessStatus::Passed,
        status,
        run_id,
        json: json_path.display().to_string(),
        markdown: markdown_path.display().to_string(),
        rollback_exercised: witness.rollback.exercised,
    };
    Ok((witness, summary))
}

pub fn verify_witness(path: &Path, key_env: &str) -> Result<VerifySummary, AppError> {
    let bytes = fs::read(path)
        .map_err(|e| AppError::runtime(format!("cannot read witness {}: {e}", path.display())))?;
    let mut witness: Witness = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::runtime(format!("invalid witness JSON: {e}")))?;
    let signature = witness
        .signature
        .take()
        .ok_or_else(|| AppError::runtime("witness is unsigned"))?;
    if signature.algorithm != "HMAC-SHA256" {
        return Err(AppError::runtime(format!(
            "unsupported signature algorithm {}",
            signature.algorithm
        )));
    }
    let key = env::var(key_env).map_err(|_| {
        AppError::config(format!(
            "verification key environment variable {key_env} is not set"
        ))
    })?;
    let payload = serde_json::to_vec(&witness)
        .map_err(|e| AppError::runtime(format!("cannot canonicalize witness: {e}")))?;
    let decoded = decode_hex(&signature.value)
        .ok_or_else(|| AppError::runtime("signature is not valid hexadecimal"))?;
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|_| AppError::runtime("cannot initialize signature verifier"))?;
    mac.update(&payload);
    mac.verify_slice(&decoded).map_err(|_| {
        AppError::runtime("signature verification failed; artifact, key, or both do not match")
    })?;
    Ok(VerifySummary {
        ok: true,
        run_id: witness.run_id,
        algorithm: signature.algorithm,
    })
}

fn validate_policy(policy: &Policy, confirmed: bool) -> Result<(), AppError> {
    if policy.version != 1 {
        return Err(AppError::config("policy version must be 1"));
    }
    if !confirmed {
        return Err(AppError::config(
            "refusing to connect: pass --confirm-test-database after checking the target is disposable",
        ));
    }
    let allowed = ["test", "ci", "development", "ephemeral"];
    if !allowed.contains(&policy.database.environment.to_ascii_lowercase().as_str()) {
        return Err(AppError::config(format!(
            "database.environment must be one of {}; got {:?}",
            allowed.join(", "),
            policy.database.environment
        )));
    }
    if policy.database.url_env.trim().is_empty() {
        return Err(AppError::config("database.url_env cannot be empty"));
    }
    validate_command("migration", &policy.migration)?;
    if let Some(command) = &policy.rollback {
        validate_command("rollback", command)?;
    }
    if policy.invariants.is_empty() {
        return Err(AppError::config(
            "policy must define at least one [[invariants]] entry",
        ));
    }
    for (index, invariant) in policy.invariants.iter().enumerate() {
        if invariant.name.trim().is_empty() || invariant.query.trim().is_empty() {
            return Err(AppError::config(format!(
                "invariant {} needs a non-empty name and query",
                index + 1
            )));
        }
        if invariant.expect_rollback.as_deref() == Some("") {
            return Err(AppError::config(format!(
                "invariant {:?} has an empty expect_rollback",
                invariant.name
            )));
        }
    }
    Ok(())
}

fn validate_command(label: &str, command: &CommandPolicy) -> Result<(), AppError> {
    if command.command.is_empty() || command.command[0].trim().is_empty() {
        return Err(AppError::config(format!(
            "{label}.command must contain a program"
        )));
    }
    if command.success_exit_codes.is_empty() {
        return Err(AppError::config(format!(
            "{label}.success_exit_codes cannot be empty"
        )));
    }
    if command.timeout_seconds == 0 || command.timeout_seconds > 3600 {
        return Err(AppError::config(format!(
            "{label}.timeout_seconds must be between 1 and 3600"
        )));
    }
    Ok(())
}

fn validate_database_url(url: &str, dialect: Dialect) -> Result<(), AppError> {
    let lower = url.to_ascii_lowercase();
    if ["prod", "production", "primary", "live-db"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        return Err(AppError::config(
            "database URL looks production-like; use a clearly named disposable target",
        ));
    }
    match dialect {
        Dialect::Sqlite => {
            if !(url.starts_with("sqlite://") || url.starts_with("sqlite:")) {
                return Err(AppError::config(
                    "sqlite dialect requires a sqlite: or sqlite:// URL",
                ));
            }
            if url.contains(":memory:") {
                return Err(AppError::config(
                    "in-memory SQLite cannot be shared with a migration subprocess; use a temporary file",
                ));
            }
        }
        Dialect::Postgres => {
            if !(url.starts_with("postgres://") || url.starts_with("postgresql://")) {
                return Err(AppError::config(
                    "postgres dialect requires a postgres:// or postgresql:// URL",
                ));
            }
        }
    }
    Ok(())
}

fn take_snapshot(policy: &Policy, database_url: &str) -> Snapshot {
    let invariants = policy
        .invariants
        .iter()
        .map(|item| {
            observe(
                &item.name,
                query_scalar(policy.database.dialect, database_url, &item.query),
            )
        })
        .collect();
    let dialect_checks = match policy.database.dialect {
        Dialect::Sqlite => vec![
            observe(
                "SQLite quick_check",
                query_scalar(Dialect::Sqlite, database_url, "PRAGMA quick_check"),
            ),
            observe(
                "SQLite foreign key violations",
                query_scalar(
                    Dialect::Sqlite,
                    database_url,
                    "SELECT count(*) FROM pragma_foreign_key_check",
                ),
            ),
        ],
        Dialect::Postgres => vec![observe(
            "PostgreSQL connection probe",
            query_scalar(Dialect::Postgres, database_url, "SELECT 1"),
        )],
    };
    Snapshot {
        captured_at_unix_ms: now_ms(),
        invariants,
        dialect_checks,
    }
}

fn observe(name: &str, result: Result<String, String>) -> Observation {
    match result {
        Ok(value) => Observation {
            name: name.into(),
            value: Some(value),
            error: None,
        },
        Err(error) => Observation {
            name: name.into(),
            value: None,
            error: Some(redact_error(&error)),
        },
    }
}

fn query_scalar(dialect: Dialect, url: &str, query: &str) -> Result<String, String> {
    match dialect {
        Dialect::Sqlite => query_sqlite(url, query),
        Dialect::Postgres => query_postgres(url, query),
    }
}

fn sqlite_path(url: &str) -> &str {
    url.strip_prefix("sqlite://")
        .or_else(|| url.strip_prefix("sqlite:"))
        .unwrap_or(url)
}

fn query_sqlite(url: &str, query: &str) -> Result<String, String> {
    let connection =
        Connection::open(sqlite_path(url)).map_err(|e| format!("SQLite connection failed: {e}"))?;
    let mut statement = connection
        .prepare(query)
        .map_err(|e| format!("query preparation failed: {e}"))?;
    if statement.column_count() != 1 {
        return Err("query must return exactly one column".into());
    }
    let mut rows = statement
        .query([])
        .map_err(|e| format!("query failed: {e}"))?;
    let row = rows
        .next()
        .map_err(|e| format!("query failed: {e}"))?
        .ok_or("query returned no rows")?;
    let value = match row
        .get_ref(0)
        .map_err(|e| format!("cannot read query result: {e}"))?
    {
        ValueRef::Null => "NULL".into(),
        ValueRef::Integer(value) => value.to_string(),
        ValueRef::Real(value) => value.to_string(),
        ValueRef::Text(value) => String::from_utf8_lossy(value).into_owned(),
        ValueRef::Blob(value) => format!("blob:sha256:{}", sha256_hex(value)),
    };
    if rows
        .next()
        .map_err(|e| format!("query failed: {e}"))?
        .is_some()
    {
        return Err("query must return exactly one row".into());
    }
    Ok(value)
}

fn query_postgres(url: &str, query: &str) -> Result<String, String> {
    let output = Command::new("psql")
        .args([
            "--dbname",
            url,
            "-X",
            "-A",
            "-t",
            "-v",
            "ON_ERROR_STOP=1",
            "-c",
            query,
        ])
        .env_remove("PGDATABASE")
        .env("PGCONNECT_TIMEOUT", "10")
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("cannot run psql: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "psql query failed: {}",
            redact_database_url(String::from_utf8_lossy(&output.stderr).trim(), url)
        ));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let rows: Vec<_> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if rows.len() != 1 {
        return Err(format!(
            "query must return one scalar row; got {}",
            rows.len()
        ));
    }
    Ok(rows[0].to_string())
}

fn run_command(policy: &CommandPolicy) -> Result<CommandEvidence, AppError> {
    let started = Instant::now();
    let mut child = Command::new(&policy.command[0])
        .args(&policy.command[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| AppError::runtime(format!("cannot start {:?}: {e}", policy.command[0])))?;
    let deadline = started + Duration::from_secs(policy.timeout_seconds);
    let (status, timed_out) = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|e| AppError::runtime(format!("cannot wait for command: {e}")))?
        {
            break (Some(status), false);
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let status = child.wait().ok();
            break (status, true);
        }
        thread::sleep(Duration::from_millis(25));
    };
    let exit_code = status.and_then(|value| value.code());
    let reported_success =
        !timed_out && exit_code.is_some_and(|code| policy.success_exit_codes.contains(&code));
    Ok(CommandEvidence {
        command: policy.command.clone(),
        exit_code,
        duration_ms: started.elapsed().as_millis(),
        timed_out,
        reported_success,
    })
}

fn compare_after(policy: &[InvariantPolicy], snapshot: &Snapshot) -> Vec<Assertion> {
    policy
        .iter()
        .zip(&snapshot.invariants)
        .map(|(item, observed)| {
            compare(item.name.clone(), item.expect_after.clone(), observed, None)
        })
        .collect()
}

fn compare_rollback(
    policy: &[InvariantPolicy],
    baseline: &Snapshot,
    rollback: &Snapshot,
) -> Vec<Assertion> {
    policy
        .iter()
        .zip(&baseline.invariants)
        .zip(&rollback.invariants)
        .map(|((item, before), observed)| {
            let expected = item
                .expect_rollback
                .clone()
                .unwrap_or_else(|| "$before".into());
            compare(item.name.clone(), expected, observed, Some(before))
        })
        .collect()
}

fn compare(
    name: String,
    expected: String,
    observed: &Observation,
    baseline: Option<&Observation>,
) -> Assertion {
    let (passed, detail) = if expected == "$before" {
        match baseline {
            Some(before)
                if before.error.is_none()
                    && observed.error.is_none()
                    && before.value.is_some()
                    && before.value == observed.value =>
            {
                (true, "matches baseline snapshot".into())
            }
            Some(before) if before.error.is_some() => (
                false,
                format!(
                    "baseline query error: {}",
                    before.error.as_deref().unwrap_or("unknown")
                ),
            ),
            Some(_) if observed.error.is_some() => (
                false,
                format!(
                    "rollback query error: {}",
                    observed.error.as_deref().unwrap_or("unknown")
                ),
            ),
            Some(_) => (false, "does not match baseline snapshot".into()),
            None => (false, "baseline is unavailable".into()),
        }
    } else if observed.error.is_some() {
        (
            false,
            format!(
                "query error: {}",
                observed.error.as_deref().unwrap_or("unknown")
            ),
        )
    } else if observed.value.as_deref() == Some(expected.as_str()) {
        (true, "matches expected scalar".into())
    } else {
        (false, "observed scalar differs from expectation".into())
    };
    Assertion {
        name,
        expected,
        observed: observed.value.clone(),
        passed,
        detail,
    }
}

fn add_snapshot_failures(label: &str, snapshot: &Snapshot, reasons: &mut Vec<String>) {
    for check in &snapshot.dialect_checks {
        let healthy = match check.name.as_str() {
            "SQLite quick_check" => check.value.as_deref() == Some("ok"),
            "SQLite foreign key violations" => check.value.as_deref() == Some("0"),
            "PostgreSQL connection probe" => check.value.as_deref() == Some("1"),
            _ => check.error.is_none(),
        };
        if !healthy {
            reasons.push(format!("{label} dialect check failed: {}", check.name));
        }
    }
}

fn add_invariant_query_failures(label: &str, snapshot: &Snapshot, reasons: &mut Vec<String>) {
    for observation in snapshot
        .invariants
        .iter()
        .filter(|item| item.error.is_some() || item.value.is_none())
    {
        reasons.push(format!(
            "{label} invariant query failed: {}",
            observation.name
        ));
    }
}

fn add_assertion_failures(label: &str, assertions: &[Assertion], reasons: &mut Vec<String>) {
    for assertion in assertions.iter().filter(|item| !item.passed) {
        reasons.push(format!("{label} invariant failed: {}", assertion.name));
    }
}

fn sign_witness(witness: &Witness, key: &[u8]) -> Result<String, AppError> {
    let payload = serde_json::to_vec(witness)
        .map_err(|e| AppError::runtime(format!("cannot canonicalize witness: {e}")))?;
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|_| AppError::runtime("cannot initialize signature"))?;
    mac.update(&payload);
    Ok(hex(mac.finalize().into_bytes().as_slice()))
}

fn render_markdown(witness: &Witness) -> String {
    let mut text = format!(
        "# Migration commit witness\n\n**Status: {}** · Run `{}` · Dialect `{}` · Environment `{}`\n\n",
        match witness.status {
            WitnessStatus::Passed => "PASS",
            WitnessStatus::Failed => "FAIL",
        },
        witness.run_id,
        witness.dialect,
        witness.environment
    );
    text.push_str("## Commit boundary\n\n| Check | Before | After | Expected | Result |\n| --- | --- | --- | --- | --- |\n");
    for ((before, after), assertion) in witness
        .baseline
        .invariants
        .iter()
        .zip(&witness.after.invariants)
        .zip(&witness.after_assertions)
    {
        text.push_str(&format!(
            "| {} | {} | {} | `{}` | {} |\n",
            md(&assertion.name),
            md(&display_observation(before)),
            md(&display_observation(after)),
            md(&assertion.expected),
            mark(assertion.passed)
        ));
    }
    text.push_str(&format!(
        "\nMigration command: `{}` after {} ms (exit {}).\n",
        if witness.migration.reported_success {
            "reported success"
        } else {
            "reported failure"
        },
        witness.migration.duration_ms,
        display_exit(witness.migration.exit_code)
    ));
    text.push_str("\n## Rollback claim\n\n");
    if witness.rollback.exercised {
        text.push_str("The supplied rollback was **exercised**.\n\n| Check | Rolled back | Expected | Result |\n| --- | --- | --- | --- |\n");
        if let Some(snapshot) = &witness.rollback.snapshot {
            for (observed, assertion) in
                snapshot.invariants.iter().zip(&witness.rollback.assertions)
            {
                text.push_str(&format!(
                    "| {} | {} | `{}` | {} |\n",
                    md(&assertion.name),
                    md(&display_observation(observed)),
                    md(&assertion.expected),
                    mark(assertion.passed)
                ));
            }
        }
    } else if witness.rollback.configured {
        text.push_str("A rollback command was configured but **not exercised** in this run.\n");
    } else {
        text.push_str("No rollback command was supplied; no rollback claim is made.\n");
    }
    text.push_str("\n## Integrity and provenance\n\n");
    for check in &witness.after.dialect_checks {
        text.push_str(&format!(
            "- {}: `{}`\n",
            md(&check.name),
            md(&display_observation(check))
        ));
    }
    text.push_str(&format!("- Policy SHA-256: `{}`\n", witness.config_sha256));
    match &witness.signature {
        Some(signature) => text.push_str(&format!(
            "- Signature: `{}` `{}`\n",
            signature.algorithm, signature.value
        )),
        None => text.push_str("- Signature: **unsigned local witness**\n"),
    }
    if !witness.reasons.is_empty() {
        text.push_str("\n## Failure reasons\n\n");
        for reason in &witness.reasons {
            text.push_str(&format!("- {}\n", md(reason)));
        }
    }
    text.push_str("\nGenerated locally by `mcw`; database credentials and command output are intentionally omitted.\n");
    text
}

fn display_observation(observation: &Observation) -> String {
    observation.value.clone().unwrap_or_else(|| {
        format!(
            "ERROR: {}",
            observation.error.as_deref().unwrap_or("unknown")
        )
    })
}

fn display_exit(code: Option<i32>) -> String {
    code.map(|value| value.to_string())
        .unwrap_or_else(|| "signal/unknown".into())
}
fn mark(passed: bool) -> &'static str {
    if passed { "PASS" } else { "FAIL" }
}
fn md(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
fn redact_error(value: &str) -> String {
    let compact = value.lines().next().unwrap_or(value);
    compact.chars().take(240).collect()
}

fn redact_database_url(value: &str, database_url: &str) -> String {
    redact_error(&value.replace(database_url, "[REDACTED DATABASE URL]"))
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), AppError> {
    let temp = path.with_extension("tmp");
    fs::write(&temp, bytes)
        .map_err(|e| AppError::runtime(format!("cannot write {}: {e}", temp.display())))?;
    fs::rename(&temp, path)
        .map_err(|e| AppError::runtime(format!("cannot finalize {}: {e}", path.display())))
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
fn sha256_hex(bytes: &[u8]) -> String {
    hex(Sha256::digest(bytes).as_slice())
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    (0..value.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&value[index..index + 2], 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// @claim:url-name-guards
    /// @claim:environment-labels
    fn production_like_urls_are_rejected() {
        for marker in ["prod", "production", "primary", "live-db"] {
            let error =
                validate_database_url(&format!("postgres://db/test-{marker}"), Dialect::Postgres)
                    .unwrap_err();
            assert_eq!(error.code, EXIT_CONFIG, "marker {marker}");
        }
        for label in ["test", "ci", "development", "ephemeral"] {
            let policy = Policy {
                version: 1,
                database: DatabasePolicy {
                    dialect: Dialect::Sqlite,
                    url_env: "DB".into(),
                    environment: label.into(),
                },
                migration: CommandPolicy {
                    command: vec!["true".into()],
                    success_exit_codes: vec![0],
                    timeout_seconds: 1,
                },
                rollback: None,
                invariants: vec![InvariantPolicy {
                    name: "one".into(),
                    query: "SELECT 1".into(),
                    expect_after: "1".into(),
                    expect_rollback: None,
                }],
            };
            assert!(validate_policy(&policy, true).is_ok(), "label {label}");
        }
    }

    #[test]
    /// @claim:one-value-checks
    fn scalar_query_rejects_multiple_rows() {
        let directory = tempfile::tempdir().unwrap();
        let url = format!("sqlite://{}", directory.path().join("test.db").display());
        let result = query_sqlite(&url, "SELECT 1 UNION ALL SELECT 2");
        assert!(result.unwrap_err().contains("exactly one row"));
    }

    #[test]
    /// @claim:signed-witness
    fn hmac_round_trip_and_tamper_detection() {
        let key = b"12345678901234567890123456789012";
        let mut witness = sample_witness();
        let signature = sign_witness(&witness, key).unwrap();
        let payload = serde_json::to_vec(&witness).unwrap();
        let mut mac = HmacSha256::new_from_slice(key).unwrap();
        mac.update(&payload);
        assert!(mac.verify_slice(&decode_hex(&signature).unwrap()).is_ok());
        witness.environment = "changed".into();
        let changed = serde_json::to_vec(&witness).unwrap();
        let mut mac = HmacSha256::new_from_slice(key).unwrap();
        mac.update(&changed);
        assert!(mac.verify_slice(&decode_hex(&signature).unwrap()).is_err());
    }

    fn sample_witness() -> Witness {
        Witness {
            witness_version: 1,
            product: "migration-commit-witness".into(),
            run_id: "test".into(),
            created_at_unix_ms: 1,
            dialect: Dialect::Sqlite,
            environment: "test".into(),
            config_sha256: "abc".into(),
            baseline: Snapshot {
                captured_at_unix_ms: 1,
                invariants: vec![],
                dialect_checks: vec![],
            },
            migration: CommandEvidence {
                command: vec!["true".into()],
                exit_code: Some(0),
                duration_ms: 1,
                timed_out: false,
                reported_success: true,
            },
            after: Snapshot {
                captured_at_unix_ms: 1,
                invariants: vec![],
                dialect_checks: vec![],
            },
            after_assertions: vec![],
            rollback: RollbackEvidence {
                requested: false,
                configured: false,
                exercised: false,
                command: None,
                snapshot: None,
                assertions: vec![],
            },
            status: WitnessStatus::Passed,
            reasons: vec![],
            signature: None,
        }
    }
}
