//! Defines the client interface for natlint.

use clap::{command, Parser};

/// The command line interface for natlint.
#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct NatlintCli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Commands,
}

/// The subcommands for natlint.
#[derive(Clone, Debug, Parser)]
pub enum Commands {
    /// The subcommand to run natlint.
    Run(run::Args),
}

/// The arguments for the run subcommand.
pub mod run {
    use super::Parser;

    /// The arguments for the run subcommand.
    #[derive(Clone, Debug, Parser)]
    pub struct Args {
        /// The root directory for the linter.
        #[clap(long, default_value = ".")]
        pub root: String,

        /// The include globs for the linter (e.g., "**/*.sol" to find all Solidity files).
        /// Put multiple patterns in quotes: "**/*.sol" "**/*.sol.txt"
        #[clap(short = 'i', long)]
        pub include: Vec<String>,

        /// The exclude globs for the linter.
        /// Put multiple patterns in quotes: "`node_modules/**`" "`dist/**`"
        #[clap(short = 'e', long)]
        pub exclude: Vec<String>,

        /// The configuration file for linter.
        #[clap(short = 'c', long)]
        pub config: String,
    }
}
