use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use mq_check::check_files;

/// Check syntax errors in mq files
#[derive(Parser, Debug)]
#[command(name = "mq-check")]
struct Cli {
    /// Path to the mq file to check
    files: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    check_files(&cli.files)
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|_| ExitCode::FAILURE)
}
