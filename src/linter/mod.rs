//! The linter implementation

use line_col::LineColLookup;

use crate::{
    parser::{ParseItem, ParseSource, Parser},
    rules::{RuleSet, Violation},
};
use forge_fmt::Visitable;
use solang_parser::parse;

/// Lints a string (e.g. a file) against a set of rules
/// # Errors
/// Returns an error if the content cannot be parsed or checked for whatever reason
pub fn lint<T>(content: &str, rule_set: &T) -> eyre::Result<Vec<(Violation, usize)>>
where
    T: RuleSet + Send + Sync + 'static,
{
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
        .flat_map(|item| process_item(&item, None, rule_set, &line_lookup))
        .collect::<Vec<_>>())
}

fn process_item<T>(
    item: &ParseItem,
    parent: Option<&ParseItem>,
    config: &T,
    line_lookup: &LineColLookup,
) -> Vec<(Violation, usize)>
where
    T: RuleSet + Send + Sync + 'static,
{
    // Check item against all applicable rules
    let item_violations = match &item.source {
        ParseSource::Contract(contract) => {
            config.check(parent, &**contract as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Function(function) => {
            config.check(parent, function as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Struct(structure) => {
            config.check(parent, structure as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Variable(variable) => {
            config.check(parent, variable as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Enum(enumeration) => {
            config.check(parent, enumeration as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Error(error) => {
            config.check(parent, error as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Event(event) => {
            config.check(parent, event as &dyn std::any::Any, &item.comments)
        }
        ParseSource::Type(type_def) => {
            config.check(parent, type_def as &dyn std::any::Any, &item.comments)
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
