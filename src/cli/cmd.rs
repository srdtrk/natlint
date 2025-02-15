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
        /// The configuration file for the relayer.
        #[clap(short = 'c', long)]
        pub config: String,
    }
}
