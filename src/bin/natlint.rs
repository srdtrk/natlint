use std::fs;

use clap::Parser;
use itertools::Itertools;
use natlint::{
    cli::{
        cmd::{Commands, NatlintCli},
        file_finder::find_matching_files,
    },
    config::load_default_config,
    linter::process_file,
    rules::Violation,
};

fn main() -> eyre::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            println!("Running natlint with config: {}", args.config);

            // TODO: Load specified config, default config path or default config
            let config = load_default_config();

            let file_violations: Vec<(String, Vec<(Violation, usize)>)> =
                find_matching_files(&args.root, &args.include, &args.exclude)?
                    .iter()
                    .map(|file| {
                        let content = fs::read_to_string(file).unwrap();
                        let file_path = file.to_str().unwrap().to_owned();

                        (file_path, process_file(&content, &config).unwrap())
                    })
                    .sorted_by(|(file_a, _), (file_b, _)| file_a.cmp(file_b))
                    .collect::<Vec<_>>();

            // Report violations
            let mut file_count = 0;
            let mut violation_count = 0;

            file_violations
                .iter()
                .for_each(|(file, violation_with_line)| {
                    file_count += 1;

                    println!("\nFile: {}", file);
                    for (violation, line_number) in violation_with_line {
                        // Print violation details with converted line number
                        println!(
                            "  [{}] Line {}: {}",
                            violation.rule_name, line_number, violation.rule_description
                        );
                        violation_count += 1;
                    }
                });

            if violation_count != 0 {
                println!(
                    "\nFound {} natspec violations in {} files.",
                    violation_count, file_count
                );
                // Return non-zero exit code if violations were found
                std::process::exit(1);
            }

            Ok(())
        }
    }
}

