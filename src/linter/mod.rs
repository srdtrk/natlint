//! The linter implementation

use line_col::LineColLookup;

use crate::{
    config::Config,
    parser::{ParseItem, ParseSource, Parser},
    rules::Violation,
};
use forge_fmt::Visitable;
use solang_parser::parse;

/// Lints a string (e.g. a file) return any violations with line numbers
/// # Errors
/// Returns an error if the content cannot be parsed or checked for whatever reason
pub fn process_file(content: &str, config: &Config) -> eyre::Result<Vec<(Violation, usize)>> {
    let line_lookup = LineColLookup::new(content);

    let (mut source_unit, comments) =
        parse(content, 0).map_err(|e| eyre::eyre!("Failed to parse content: {:?}", e))?;

    let mut parser = Parser::new(comments, content.to_string());
    source_unit
        .visit(&mut parser)
        .map_err(|e| eyre::eyre!("Failed to visit: {:?}", e))?;

    let items = parser.items();

    let mut all_violations = Vec::new();

    // Process all top-level items
    for item in &items {
        process_item(item, None, config, &line_lookup, &mut all_violations);
    }

    Ok(all_violations)
}

fn process_item(
    item: &ParseItem,
    parent: Option<&ParseItem>,
    config: &Config,
    line_lookup: &LineColLookup,
    all_violations: &mut Vec<(Violation, usize)>,
) {
    // Check item against all applicable rules
    match &item.source {
        ParseSource::Contract(contract) => {
            let violations =
                config.check_item(parent, &**contract as &dyn std::any::Any, &item.comments);

            for violation in violations {
                // Get line number (1-based) using efficient lookup
                let (line, _) = line_lookup.get(violation.loc.start());
                all_violations.push((violation, line));
            }
        }
        ParseSource::Function(function) => {
            let violations =
                config.check_item(parent, function as &dyn std::any::Any, &item.comments);

            for violation in violations {
                let (line, _) = line_lookup.get(violation.loc.start());
                all_violations.push((violation, line));
            }
        }
        ParseSource::Struct(structure) => {
            let violations =
                config.check_item(parent, structure as &dyn std::any::Any, &item.comments);

            for violation in violations {
                let (line, _) = line_lookup.get(violation.loc.start());
                all_violations.push((violation, line));
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
        process_item(child, Some(item), config, line_lookup, all_violations);
    }
}
