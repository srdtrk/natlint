use clap::Parser;
use natlint::cli::cmd::{Commands, NatlintCli};

fn main() -> eyre::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            println!("Running natlint with config: {}", args.config);
            Ok(())
        }
    }
}
