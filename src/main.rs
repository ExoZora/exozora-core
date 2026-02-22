mod core;
mod executor;
mod planner;
mod policy;
mod utils;

use std::env;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::{error, info};

// ---------------------------------------------------------------------------
// CLI definition (clap derive)
// ---------------------------------------------------------------------------

/// ExoZora — a safe, policy-gated task runner.
#[derive(Debug, Parser)]
#[command(
    name = "exozora",
    version = "0.01",
    about = "ExoZora v0.01 – Sandboxed Task Execution Runtime"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run ExoZora with a plain-text input task description.
    Run {
        /// The raw task description to plan and execute.
        #[arg(long, short)]
        input: String,

        /// Working directory for filesystem confinement (defaults to CWD).
        #[arg(long, value_name = "DIR")]
        working_dir: Option<PathBuf>,
    },
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    // Initialise structured tracing.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("ExoZora v0.01 – Booting Core Runtime");

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { input, working_dir } => {
            run(input, working_dir).await;
        }
    }
}

// ---------------------------------------------------------------------------
// Core flow: input → planner → policy → executor
// ---------------------------------------------------------------------------

async fn run(input: String, working_dir: Option<PathBuf>) {
    // Resolve and canonicalise the working directory once.
    let working_dir = match working_dir {
        Some(dir) => dir
            .canonicalize()
            .unwrap_or(dir),
        None => env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    };

    // 1️⃣ Input received
    info!(input = %input, working_dir = %working_dir.display(), "Input received");

    // 2️⃣ Planner: raw input → Plan
    let plan = planner::plan(&input);
    info!(task_count = plan.tasks().len(), "Plan generated");

    // 3️⃣ Policy: Plan → ApprovedPlan (or rejection)
    match policy::validate(plan, &working_dir) {
        Ok(approved) => {
            info!("Policy approved");

            // 4️⃣ Executor: consume the approved plan
            executor::execute(approved).await;
        }
        Err(err) => {
            error!(%err, "Policy rejected");
            std::process::exit(1);
        }
    }
}
