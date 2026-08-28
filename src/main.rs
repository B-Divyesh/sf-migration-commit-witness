use clap::{Parser, Subcommand};
use migration_commit_witness::{
    AppError, EXIT_WITNESS_FAILED, WitnessOptions, WitnessStatus, demo_step, init_policy, run_demo,
    run_witness, verify_witness,
};
use serde::Serialize;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Parser)]
#[command(
    name = "mcw",
    version,
    about = "Prove what a SQL migration committed and rolled back"
)]
#[command(
    long_about = "Run an existing migration command against a confirmed disposable database, snapshot selected invariants around the commit boundary, optionally exercise an explicit rollback, and emit signed evidence. Never point mcw at production."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the bundled partial-commit sample in a new isolated directory.
    Demo {
        /// Use this new directory instead of creating one under the system temp directory.
        #[arg(long)]
        output: Option<PathBuf>,
        /// Print the demo result as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Write a documented starter policy without overwriting existing files.
    Init {
        /// Destination for the new TOML policy.
        #[arg(short, long, default_value = "mcw.toml")]
        output: PathBuf,
        /// Print a machine-readable result.
        #[arg(long)]
        json: bool,
    },
    /// Run a migration and produce the commit witness.
    Witness {
        /// TOML policy describing the test database, commands, and invariants.
        #[arg(short, long, default_value = "mcw.toml")]
        config: PathBuf,
        /// New directory for witness.json and witness.md.
        #[arg(short, long, default_value = "witness")]
        output: PathBuf,
        /// Required acknowledgement that the configured database is disposable.
        #[arg(long)]
        confirm_test_database: bool,
        /// Run only the rollback command explicitly present in the policy.
        #[arg(long)]
        exercise_rollback: bool,
        /// Permit an unsigned local artifact when no signing secret is available.
        #[arg(long)]
        allow_unsigned: bool,
        /// Environment variable containing an HMAC key of at least 32 bytes.
        #[arg(long, default_value = "MCW_SIGNING_KEY")]
        signing_key_env: String,
        /// Replace witness files in the exact output directory.
        #[arg(long)]
        force: bool,
        /// Print the final summary as JSON; progress and errors stay on stderr.
        #[arg(long)]
        json: bool,
    },
    /// Verify a signed JSON witness without connecting to a database.
    Verify {
        /// Path to witness.json.
        witness: PathBuf,
        /// Environment variable containing the original HMAC key.
        #[arg(long, default_value = "MCW_SIGNING_KEY")]
        key_env: String,
        /// Print the result as JSON.
        #[arg(long)]
        json: bool,
    },
    #[command(hide = true)]
    DemoStep {
        #[arg(value_parser = ["migrate", "rollback"])]
        action: String,
        database: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let json = match &cli.command {
        Commands::Demo { json, .. }
        | Commands::Init { json, .. }
        | Commands::Witness { json, .. }
        | Commands::Verify { json, .. } => *json,
        Commands::DemoStep { .. } => false,
    };
    match execute(cli.command, json) {
        Ok(code) => ExitCode::from(code as u8),
        Err(error) => {
            if json {
                print_json(&ErrorOutput {
                    ok: false,
                    error: &error.message,
                    code: error.code,
                });
            } else {
                eprintln!("mcw: {}", error.message);
            }
            ExitCode::from(error.code as u8)
        }
    }
}

fn execute(command: Commands, json: bool) -> Result<i32, AppError> {
    match command {
        Commands::Demo { output, .. } => {
            let summary = run_demo(output.as_deref())?;
            if json {
                print_json(&summary);
            } else {
                println!("DEMO: partial commit detected; rollback restored the starting checks.");
                println!("Workspace: {}", summary.workspace);
                println!("JSON: {}\nMarkdown: {}", summary.json, summary.markdown);
                println!(
                    "Sample data is isolated in this temporary workspace. Delete it when finished."
                );
            }
            Ok(0)
        }
        Commands::Init { output, .. } => {
            init_policy(&output)?;
            if json {
                print_json(&InitOutput {
                    ok: true,
                    policy: output.display().to_string(),
                });
            } else {
                println!(
                    "Created {}. Edit it before running a witness.",
                    output.display()
                );
            }
            Ok(0)
        }
        Commands::Witness {
            config,
            output,
            confirm_test_database,
            exercise_rollback,
            allow_unsigned,
            signing_key_env,
            force,
            ..
        } => {
            eprintln!("mcw: confirming policy and disposable target");
            let (witness, summary) = run_witness(&WitnessOptions {
                config,
                output,
                confirm_test_database,
                exercise_rollback,
                allow_unsigned,
                signing_key_env,
                force,
                database_url_override: None,
            })?;
            if json {
                print_json(&summary);
            } else {
                println!(
                    "{} witness {}",
                    if summary.ok { "PASS" } else { "FAIL" },
                    summary.run_id
                );
                println!("JSON: {}\nMarkdown: {}", summary.json, summary.markdown);
            }
            Ok(if witness.status == WitnessStatus::Passed {
                0
            } else {
                EXIT_WITNESS_FAILED
            })
        }
        Commands::Verify {
            witness, key_env, ..
        } => {
            let summary = verify_witness(&witness, &key_env)?;
            if json {
                print_json(&summary);
            } else {
                println!(
                    "VERIFIED witness {} ({})",
                    summary.run_id, summary.algorithm
                );
            }
            Ok(0)
        }
        Commands::DemoStep { action, database } => {
            demo_step(&action, &database)?;
            Ok(0)
        }
    }
}

#[derive(Serialize)]
struct ErrorOutput<'a> {
    ok: bool,
    error: &'a str,
    code: i32,
}
#[derive(Serialize)]
struct InitOutput {
    ok: bool,
    policy: String,
}

fn print_json(value: &impl Serialize) {
    println!(
        "{}",
        serde_json::to_string(value).expect("serializable CLI output")
    );
}
