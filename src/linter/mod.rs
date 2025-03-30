//! The linter implementation

use line_col::LineColLookup;

use crate::{
    config::Config,
    parser::{ParseItem, ParseSource, Parser},
    rules::Violation,
};
use forge_fmt::Visitable;
use solang_parser::parse;

/// Lints a string (e.g. a file) against a set of rules
/// # Errors
/// Returns an error if the content cannot be parsed or checked for whatever reason
pub fn lint(content: &str, config: &Config) -> eyre::Result<Vec<(Violation, usize)>> {
    let line_lookup = LineColLookup::new(content);

    let (mut source_unit, comments) =
        parse(content, 0).map_err(|e| eyre::eyre!("Failed to parse content: {:?}", e))?;

    let mut parser = Parser::new(comments, content.to_string());
    source_unit
        .visit(&mut parser)
        .map_err(|e| eyre::eyre!("Failed to visit: {:?}", e))?;

    Ok(parser
        .items()
        .into_iter()
        .flat_map(|item| process_item(&item, None, config, &line_lookup))
        .collect::<Vec<_>>())
}

fn process_item(
    item: &ParseItem,
    parent: Option<&ParseItem>,
    config: &Config,
    line_lookup: &LineColLookup,
) -> Vec<(Violation, usize)> {
    // Check item against all applicable rules
    let item_violations = match &item.source {
        ParseSource::Contract(contract) => {
            config.check_item(parent, &**contract as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Function(function) => {
            config.check_item(parent, function as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Struct(structure) => {
            config.check_item(parent, structure as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Variable(variable) => {
            config.check_item(parent, variable as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Enum(enumeration) => {
            config.check_item(parent, enumeration as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Error(error) => {
            config.check_item(parent, error as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Event(event) => {
            config.check_item(parent, event as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Type(type_def) => {
            config.check_item(parent, type_def as &dyn std::any::Any, &item.comments)
        }
    };

    item_violations
        .into_iter()
        .map(|violation| {
            // Convert the line number to the original source line
            let (line, _) = line_lookup.get(violation.loc.start());
            (violation, line)
        })
        .chain(
            item.children
                .iter()
                .flat_map(|child| process_item(child, Some(item), config, line_lookup)),
        )
        .collect::<Vec<_>>()
}
