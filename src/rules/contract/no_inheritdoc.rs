//! This module defines the `NoInheritdoc` rule
//! This rule may be removed in the future: <https://github.com/ethereum/solidity/issues/14045>

use solang_parser::pt::ContractDefinition;

use crate::no_comment_rule;

no_comment_rule!(
    NoInheritdoc,
    ContractDefinition,
    Inheritdoc,
    "Contracts must not have an inheritdoc comment."
);

#[cfg(test)]
mod tests {
    use super::{ContractDefinition, NoInheritdoc};
    use crate::{
        parser::{CommentTag, CommentsRef, Parser},
        rules::{violation_error::ViolationError, Rule, Violation},
    };
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    /// Macro to define a test case for `MissingParams` rule
    macro_rules! test_no_inheritdoc {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(NoInheritdoc::check(None, contract, comments), expected);
            }
        };
    }

    test_no_inheritdoc!(
        empty_no_violation,
        r"
        interface Test {
        }
        ",
        |_| None
    );

    test_no_inheritdoc!(
        no_violation,
        r"
        /// @custom:test Some comment
        interface Test {
        }
        ",
        |_| None
    );

    test_no_inheritdoc!(
        multi_no_violation,
        r"
        /// @custom:test Some comment
        /// @custom:test Some other
        contract Test {
        }
        ",
        |_| None
    );

    test_no_inheritdoc!(
        multiline_no_violation,
        r"
        /**
         * @custom:test Some comment
         */
        abstract contract Test {
        }
        ",
        |_| None
    );

    test_no_inheritdoc!(
        multiline_multi_no_violation,
        r"
        /**
         * @custom:test Some comment
         * @custom:test Some other
         */
        library Test {
        }
        ",
        |_| None
    );

    test_no_inheritdoc!(
        violation,
        r"
        /// @inheritdoc Base
        interface Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Inheritdoc),
            sct.loc
        ))
    );

    test_no_inheritdoc!(
        multi_violation,
        r"
        /// @inheritdoc Base
        /// @inheritdoc Base
        abstract contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Inheritdoc),
            sct.loc
        ))
    );

    test_no_inheritdoc!(
        multiline_violation,
        r"
        /**
         * @inheritdoc Base
         */
        library Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Inheritdoc),
            sct.loc
        ))
    );

    test_no_inheritdoc!(
        multiline_multi_violation,
        r"
        /**
         * @inheritdoc Base
         * @inheritdoc Base
         */
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoInheritdoc::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Inheritdoc),
            sct.loc
        ))
    );
}
