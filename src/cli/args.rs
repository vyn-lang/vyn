use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::cli::commands::VERSION;

#[derive(Parser)]
#[command(name = "vyn")]
#[command(version = VERSION)]
#[command(about = "Vyn Programming Language", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,

    /// Disable progress bar
    #[arg(long, global = true)]
    pub no_progress: bool,

    /// Show timing information for each phase
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Minimal output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Slow down compilation for debugging
    #[arg(long, global = true, hide = true)]
    pub slow_mode: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a Vyn program
    Run {
        /// Path to the .vyn file
        file: PathBuf,
    },
    /// Type check a Vyn program without running it
    Check {
        /// Path to the .vyn file
        file: PathBuf,
    },
    /// Disassemble bytecode
    Disasm {
        /// Path to the .vyn file
        file: PathBuf,
    },
    /// Show version information
    Version,
}

impl CliArgs {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
