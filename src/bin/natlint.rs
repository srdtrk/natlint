use std::{fs, path::Path};

use clap::Parser;
use itertools::Itertools;
use natlint::{
    cli::{
        cmd::{Commands, NatlintCli},
        file_finder::find_matching_files,
    },
    config::Config,
    linter::lint,
    rules::Violation,
};

fn main() -> eyre::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            let config = match Config::from_file(Path::new(&args.config)) {
                Ok(config) => config,
                Err(e) => {
                    println!("Error reading config file: {}.", e);
                    println!("Using default config.");
                    Config::default()
                }
            };

            let file_violations: Vec<(String, Vec<(Violation, usize)>)> =
                find_matching_files(&args.root, args.include, args.exclude)?
                    .iter()
                    .map(|file| {
                        let content = fs::read_to_string(file).unwrap();
                        let file_path = file.to_str().unwrap().to_owned();

                        (file_path, lint(&content, &config.rules()).unwrap())
                    })
                    .sorted_by(|(file_a, _), (file_b, _)| file_a.cmp(file_b))
                    .collect::<Vec<_>>();

            let violation_count = file_violations
                .iter()
                .map(|(_, violations)| violations.len())
                .sum::<usize>();

            for (file, violation_with_line) in file_violations.iter() {
                println!("\nFile: {}", file);
                if violation_with_line.is_empty() {
                    println!("  No violations found.");
                    continue;
                }
                for (violation, line_number) in violation_with_line {
                    // Print violation details with converted line number
                    println!(
                        "  [{}] Line {}: {} {}",
                        violation.rule_name,
                        line_number,
                        violation.rule_description,
                        violation.error
                    );
                }
            }

            if violation_count != 0 {
                println!(
                    "\nFound {} natspec violations in {} files.",
                    violation_count,
                    file_violations.len()
                );
                // Return non-zero exit code if violations were found
                std::process::exit(1);
            }

            Ok(())
        }
    }
}
