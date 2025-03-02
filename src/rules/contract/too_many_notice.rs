//! This rule requires that all contacts have a notice comment.

use solang_parser::pt::ContractDefinition;

crate::too_many_comments_rule!(
    TooManyNotice,
    ContractDefinition,
    Notice,
    "Contracts must not have more than one notice comment."
);

#[cfg(test)]
mod tests {
    use super::{ContractDefinition, TooManyNotice};
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
    macro_rules! test_too_many_notice {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(TooManyNotice::check(None, contract, comments), expected);
            }
        };
    }

    test_too_many_notice!(
        empty_no_violation,
        r"
        contract Test {
        }
        ",
        |_| None
    );

    test_too_many_notice!(
        exists_no_violation,
        r"
        /// @notice Some notice
        interface Test {
        }
        ",
        |_| None
    );

    test_too_many_notice!(
        multi_no_violation,
        r"
        /// @title Some title
        /// @notice Some notice
        contract Test {
        }
        ",
        |_| None
    );

    test_too_many_notice!(
        multiline_exists_no_violation,
        r"
        /**
         * @notice Some notice
         */
        abstract contract Test {
        }
        ",
        |_| None
    );

    test_too_many_notice!(
        multiline_multi_no_violation,
        r"
        /**
         * @custom:test Some comment
         * @notice Some notice
         */
        library Test {
        }
        ",
        |_| None
    );

    test_too_many_notice!(
        no_violation,
        r"
        /// @custom:test Some comment
        interface Test {
        }
        ",
        |_| None
    );

    test_too_many_notice!(
        multiline_no_violation,
        r"
        /**
         * @custom:test Some comment
         */
        library Test {
        }
        ",
        |_| None
    );

    test_too_many_notice!(
        multi_violation,
        r"
        /// @notice Some notice
        /// @notice Some notice
        abstract contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        ))
    );

    test_too_many_notice!(
        multiline_multi_violation,
        r"
        /**
         * @notice Some notice
         * @notice Some notice
         */
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            TooManyNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        ))
    );
}
