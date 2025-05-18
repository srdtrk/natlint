//! The linter implementation

use std::any::{Any, TypeId};

use line_col::LineColLookup;

use crate::parser::ParseSource;

use crate::{
    parser::{CommentsRef, ParseItem, Parser},
    rules::{DynRule, Violation},
};
use forge_fmt::Visitable;
use solang_parser::parse;

/// Lints a string (e.g. a file) against a set of rules
/// # Errors
/// Returns an error if the content cannot be parsed or checked for whatever reason
pub fn lint(
    content: &str,
    rule_set: &Vec<Box<dyn DynRule>>,
) -> eyre::Result<Vec<(Violation, usize)>> {
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

fn process_item(
    item: &ParseItem,
    parent: Option<&ParseItem>,
    rule_set: &Vec<Box<dyn DynRule>>,
    line_lookup: &LineColLookup,
) -> Vec<(Violation, usize)> {
    let comments_ref = CommentsRef::from(&item.comments);

    // Get the inner AST node and its TypeId
    let (source_item, source_type_id): (&dyn Any, TypeId) = match &item.source {
        ParseSource::Contract(inner) => (
            inner.as_ref(),
            TypeId::of::<solang_parser::pt::ContractDefinition>(),
        ),
        ParseSource::Function(inner) => {
            (inner, TypeId::of::<solang_parser::pt::FunctionDefinition>())
        }
        ParseSource::Variable(inner) => {
            (inner, TypeId::of::<solang_parser::pt::VariableDefinition>())
        }
        ParseSource::Event(inner) => (inner, TypeId::of::<solang_parser::pt::EventDefinition>()),
        ParseSource::Error(inner) => (inner, TypeId::of::<solang_parser::pt::ErrorDefinition>()),
        ParseSource::Struct(inner) => (inner, TypeId::of::<solang_parser::pt::StructDefinition>()),
        ParseSource::Enum(inner) => (inner, TypeId::of::<solang_parser::pt::EnumDefinition>()),
        ParseSource::Type(inner) => (inner, TypeId::of::<solang_parser::pt::TypeDefinition>()),
    };

    rule_set
        .iter()
        // Filter rules based on the TypeId of the inner AST node
        .filter(|rule| rule.target_type_id() == source_type_id)
        // Pass the inner AST node (&dyn Any) to check_dyn
        .filter_map(|rule| rule.check_dyn(parent, source_item, &comments_ref))
        .map(|violation| {
            // Convert the line number to the original source line
            let (line, _) = line_lookup.get(violation.loc.start());
            (violation, line)
        })
        .chain(
            item.children
                .iter()
                .flat_map(|child| process_item(child, Some(item), rule_set, line_lookup)),
        )
        .collect::<Vec<_>>()
}
