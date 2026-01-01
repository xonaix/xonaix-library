//! xonaix-library-tools
//!
//! Single Rust binary for all governance and manifest tooling in xonaix-library.
//!
//! Commands:
//! - generate-manifest: Generate SHA-256 manifest for governance or units
//! - enforce: Run all no-debt enforcement checks
//! - unit-validate: Validate UNIT.json files against registry
//! - graph-verify: Verify dependency graph integrity (DAG, no cycles)
//! - doctor: Verify environment and library requirements
//! - header-validate: Validate document headers against v2.1 schema
//! - governance-report: Generate governance metrics and reports

use clap::{Parser, Subcommand};
use std::process::ExitCode;

mod doctor;
mod enforce;
mod manifest;
mod header;
mod report;
mod unit;

#[derive(Parser)]
#[command(name = "xonaix-library-tools")]
#[command(about = "Governance and specification tooling for xonaix-library")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate SHA-256 manifest for governance or units
    GenerateManifest {
        /// Generate governance manifest
        #[arg(long)]
        governance: bool,

        /// Generate manifest for a specific unit (by unit_id)
        #[arg(long)]
        unit: Option<String>,

        /// Output file path (default: auto-generated)
        #[arg(long)]
        output: Option<String>,

        /// Repository root path (default: auto-detect)
        #[arg(long)]
        repo_root: Option<String>,

        /// Check mode: regenerate and compare to existing, fail if different
        #[arg(long)]
        check: bool,
    },

    /// Run all no-debt enforcement checks
    Enforce {
        /// Repository root path (default: auto-detect)
        #[arg(long)]
        repo_root: Option<String>,

        /// Enforce on current-only scope (excludes _deprecated/, _reference/)
        #[arg(long, default_value_t = true)]
        current_only: bool,
    },

    /// Validate UNIT.json files against registry
    UnitValidate {
        /// Repository root path (default: auto-detect)
        #[arg(long)]
        repo_root: Option<String>,

        /// Specific unit path to validate (default: all units)
        #[arg(long)]
        unit_path: Option<String>,
    },

    /// Verify dependency graph integrity (DAG, no cycles)
    GraphVerify {
        /// Repository root path (default: auto-detect)
        #[arg(long)]
        repo_root: Option<String>,
    },

    /// Verify environment and library requirements
    Doctor {
        /// Repository root path (default: auto-detect)
        #[arg(long)]
        repo_root: Option<String>,
    },

    /// Validate document headers against v2.1 schema
    HeaderValidate {
        /// Repository root path (default: auto-detect)
        #[arg(long)]
        repo_root: Option<String>,

        /// Specific file path to validate (default: all markdown files)
        #[arg(long)]
        file: Option<String>,
    },

    /// Generate governance report with metrics for dashboards and audits
    GovernanceReport {
        /// Repository root path (default: auto-detect)
        #[arg(long)]
        repo_root: Option<String>,

        /// Output format: json, json-pretty, table, summary
        #[arg(long, default_value = "table")]
        format: String,

        /// Output file path (prints to stdout if not specified)
        #[arg(long)]
        output: Option<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let success = match cli.command {
        Commands::GenerateManifest {
            governance,
            unit,
            output,
            repo_root,
            check,
        } => match manifest::run(governance, unit, output, repo_root, check) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("ERROR: {e}");
                false
            }
        },

        Commands::Enforce { repo_root, current_only } => match enforce::run(repo_root, current_only) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("ERROR: {e}");
                false
            }
        },

        Commands::UnitValidate { repo_root, unit_path } => match unit::validate(repo_root, unit_path) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("ERROR: {e}");
                false
            }
        },

        Commands::GraphVerify { repo_root } => match unit::graph_verify(repo_root) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("ERROR: {e}");
                false
            }
        },

        Commands::Doctor { repo_root } => match doctor::run(repo_root) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("ERROR: {e}");
                false
            }
        },

        Commands::HeaderValidate { repo_root, file } => match header::run(repo_root, file) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("ERROR: {e}");
                false
            }
        },

        Commands::GovernanceReport { repo_root, format, output } => {
            let fmt = match format.parse::<report::OutputFormat>() {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            match report::run(repo_root, fmt, output) {
                Ok(()) => true,
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    false
                }
            }
        }
    };

    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
