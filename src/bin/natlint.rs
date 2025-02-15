use clap::Parser;
use natlint::cli::cmd::{Commands, NatlintCli};

fn main() -> anyhow::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            println!("Running natlint with config: {}", args.config);
            Ok(())
        }
    }
}
