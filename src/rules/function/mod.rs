//! This module defines the rules for function items in the natlint linter.

use solang_parser::pt::{
    ContractTy, FunctionAttribute, FunctionDefinition, FunctionTy, Visibility,
};

use crate::parser::{CommentsRef, ParseItem};

use super::{Rule, Violation};

/// This rule requires that all public functions have a inheritdoc comment.
pub struct RequireInheritdoc;

impl Rule<FunctionDefinition> for RequireInheritdoc {
    const NAME: &'static str = "Require Inheritdoc";
    const DESCRIPTION: &'static str =
        "All public and override functions must have an inheritdoc comment.";

    fn check(
        parent: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Parent must be a contract, not an interface or library
        match parent?.as_contract()?.ty {
            ContractTy::Interface(_) | ContractTy::Library(_) => return None,
            ContractTy::Contract(_) | ContractTy::Abstract(_) => (),
        };

        println!("debug1");
        // Function must not be a modifier or constructor
        match func.ty {
            FunctionTy::Function => (),
            FunctionTy::Receive
            | FunctionTy::Fallback
            | FunctionTy::Constructor
            | FunctionTy::Modifier => return None,
        }

        println!("debug2");
        // Function must be public, external, or an override
        func.attributes.iter().find(|attr| match attr {
            FunctionAttribute::Visibility(Visibility::Public(_) | Visibility::External(_))
            | FunctionAttribute::Override(..) => true,
            FunctionAttribute::Visibility(Visibility::Private(_) | Visibility::Internal(_))
            | FunctionAttribute::Mutability(_)
            | FunctionAttribute::Virtual(_)
            | FunctionAttribute::Immutable(_)
            | FunctionAttribute::BaseOrModifier(..)
            | FunctionAttribute::Error(_) => false,
        })?;

        println!("debug3");
        // Function must have an inheritdoc comment
        if comments.find_inheritdoc_base().is_none() {
            return Some(Violation::new(Self::NAME, Self::DESCRIPTION, func.loc));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{RequireInheritdoc, Rule};
    use crate::{
        parser::{CommentsRef, Parser},
        rules::Violation,
    };
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    #[test]
    fn test_success_public_inheritdoc() {
        let src = parse_source(
            r"
            contract Test {
                /// @inheritdoc Base
                function test() public {}
            }
        ",
        );

        let item = src.items_ref().first().unwrap();
        let func = item.children.first().unwrap().as_function().unwrap();

        assert_eq!(
            RequireInheritdoc::check(Some(item), func, CommentsRef::from(&item.comments)),
            None
        );
    }

    #[test]
    fn test_failure_public_inheritdoc() {
        let src = parse_source(
            r"
            contract Test {
                function test() public {}
            }
        ",
        );

        let item = src.items_ref().first().unwrap();
        let func = item.children.first().unwrap().as_function().unwrap();

        assert_eq!(
            RequireInheritdoc::check(Some(item), func, CommentsRef::from(&item.comments)),
            Some(Violation::new(
                RequireInheritdoc::NAME,
                RequireInheritdoc::DESCRIPTION,
                func.loc
            ))
        );
    }
}
