use clap::Parser;
use natlint::{
    cli::{
        cmd::{Commands, NatlintCli},
        file_finder::find_matching_files,
    },
    config::load_config,
    parser::process_file::process_file,
};

fn main() -> eyre::Result<()> {
    let cli = NatlintCli::parse();
    match cli.command {
        Commands::Run(args) => {
            println!("Running natlint with config: {}", args.config);

            // Display helpful message for using glob patterns if no include patterns are provided
            if args.include.is_empty() && args.root == "." {
                println!("Tip: Use --include/-i to specify glob patterns to search for files.");
                println!(
                    "Example: natlint run -c config.toml -i \"**/*.sol\" -e \"node_modules/**\""
                );
                println!("Searching for Solidity files in the current directory...")
            }

            // Load configuration with all rules
            let config = load_config(&args.config);

            // Find all files in the root directory that match the include globs and do not match the exclude globs
            let files = find_matching_files(&args.root, args.include, &args.exclude)?;
            println!("Found {} files to lint", files.len());

            // Process each file and collect violations
            let mut all_violations = Vec::new();
            let mut error_count = 0;
            let total_files = files.len();

            // Show progress if there are more than 5 files
            let show_progress = total_files > 5;

            for (idx, file) in files.iter().enumerate() {
                // Show progress
                if show_progress && idx % 5 == 0 {
                    print!(
                        "\rProcessing files: {}/{} ({}%)",
                        idx + 1,
                        total_files,
                        ((idx + 1) as f64 / total_files as f64 * 100.0) as u32
                    );
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                }

                match process_file(file, &config) {
                    Ok(violations) => {
                        all_violations.extend(violations);
                    }
                    Err(err) => {
                        if show_progress {
                            println!(); // New line after progress indicator
                        }
                        eprintln!("Error processing file {}: {}", file.display(), err);
                        error_count += 1;
                    }
                }
            }

            // Clear progress line when done
            if show_progress {
                println!("\rProcessed {} files                ", total_files);
            }

            // Sort violations by file, rule, and actual line number
            all_violations.sort_by(|(file_a, viol_a, line_a), (file_b, viol_b, line_b)| {
                file_a
                    .cmp(file_b)
                    .then_with(|| line_a.cmp(line_b))
                    .then_with(|| viol_a.rule_name.cmp(viol_b.rule_name))
            });

            // Report violations
            if all_violations.is_empty() {
                println!("No natspec violations found!");
            } else {
                println!("\nNatspec violations found:");

                let mut current_file = String::new();
                let mut violation_count = 0;

                for (file, violation, line_number) in all_violations {
                    // Print file name when it changes
                    if current_file != file {
                        if !current_file.is_empty() {
                            println!();
                        }
                        println!("File: {}", file);
                        current_file = file;
                    }

                    // Print violation details with converted line number
                    println!(
                        "  [{}] Line {}: {}",
                        violation.rule_name, line_number, violation.rule_description
                    );

                    violation_count += 1;
                }

                println!(
                    "\nFound {} natspec violations in {} files.",
                    violation_count,
                    files.len()
                );

                if error_count > 0 {
                    println!("Failed to process {} files due to errors.", error_count);
                }

                // Return non-zero exit code if violations were found
                std::process::exit(1);
            }

            // If there were parsing errors but no violations, still exit with error
            if error_count > 0 {
                println!("Failed to process {} files due to errors.", error_count);
                std::process::exit(1);
            }

            Ok(())
        }
    }
}

