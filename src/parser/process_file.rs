use line_col::LineColLookup;

use crate::{
    config::Config,
    parser::{ParseItem, ParseSource, Parser},
    rules::Violation,
};

/// Process a single Solidity file and return any violations with line numbers
pub fn process_file(
    file_path: &std::path::Path,
    config: &Config,
) -> eyre::Result<Vec<(String, Violation, usize)>> {
    use forge_fmt::Visitable;
    use solang_parser::parse;
    use std::fs;

    // Read file content
    let content = fs::read_to_string(file_path)?;

    // Create line/column lookup for efficient offset-to-line conversion
    let line_lookup = LineColLookup::new(&content);

    // Parse Solidity file
    let (mut source_unit, comments) = parse(&content, 0)
        .map_err(|e| eyre::eyre!("Failed to parse {}: {:?}", file_path.display(), e))?;

    // Create parser and visit the source unit
    let mut parser = Parser::new(comments, content.clone());
    source_unit
        .visit(&mut parser)
        .map_err(|e| eyre::eyre!("Failed to visit {}: {:?}", file_path.display(), e))?;

    // Get parsed items
    let items = parser.items();

    // Collect violations
    let mut all_violations = Vec::new();

    // Function to recursively process items and their children
    fn process_item(
        item: &ParseItem,
        parent: Option<&ParseItem>,
        config: &Config,
        file_path: &std::path::Path,
        line_lookup: &LineColLookup,
        all_violations: &mut Vec<(String, Violation, usize)>,
    ) {
        // Check item against all applicable rules
        match &item.source {
            ParseSource::Contract(contract) => {
                let violations =
                    config.check_item(parent, &**contract as &dyn std::any::Any, &item.comments);

                for violation in violations {
                    // Get line number (1-based) using efficient lookup
                    let (line, _) = line_lookup.get(violation.loc.start());
                    all_violations.push((file_path.display().to_string(), violation, line));
                }
            }
            ParseSource::Function(function) => {
                let violations =
                    config.check_item(parent, function as &dyn std::any::Any, &item.comments);

                for violation in violations {
                    let (line, _) = line_lookup.get(violation.loc.start());
                    all_violations.push((file_path.display().to_string(), violation, line));
                }
            }
            ParseSource::Struct(structure) => {
                let violations =
                    config.check_item(parent, structure as &dyn std::any::Any, &item.comments);

                for violation in violations {
                    let (line, _) = line_lookup.get(violation.loc.start());
                    all_violations.push((file_path.display().to_string(), violation, line));
                }
            }
            // Add other item types as needed:
            // ParseSource::Enum(..) => { ... },
            // ParseSource::Error(..) => { ... },
            // ParseSource::Event(..) => { ... },
            // ParseSource::Variable(..) => { ... },
            // ParseSource::Type(..) => { ... },
            _ => {
                // No rules implemented for these item types yet
            }
        }

        // Process all children
        for child in &item.children {
            process_item(
                child,
                Some(item),
                config,
                file_path,
                line_lookup,
                all_violations,
            );
        }
    }

    // Process all top-level items
    for item in &items {
        process_item(
            item,
            None,
            config,
            file_path,
            &line_lookup,
            &mut all_violations,
        );
    }

    Ok(all_violations)
}
