use clap::Parser;
use natlint::{
    cli::cmd::{Commands, NatlintCli},
    cli::file_finder::find_matching_files,
};

fn main() -> eyre::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            println!("Running natlint with config: {}", args.config);

            // Find all files in the root directory that match the include globs and do not match the exclude globs
            let files = find_matching_files(&args.root, args.include, args.exclude)?;
            files.iter().for_each(|file| println!("{}", file.display()));

            Ok(())
        }
    }
}
