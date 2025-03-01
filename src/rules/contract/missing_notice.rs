//! This rule requires that all contacts have a notice comment.

use solang_parser::pt::ContractDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all contracts have a notice comment.
pub struct MissingNotice;

impl Rule<ContractDefinition> for MissingNotice {
    const NAME: &'static str = "Missing Notice";
    const DESCRIPTION: &'static str =
        "This rule requires that all contracts have a notice comment.";

    fn check(
        _: Option<&ParseItem>,
        contract: &ContractDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Contract must have a title comment
        match comments.include_tag(CommentTag::Notice).len() {
            0 => Some(Violation::new(
                Self::NAME,
                ViolationError::MissingComment(CommentTag::Notice),
                contract.loc,
            )),
            1 => None,
            _ => Some(Violation::new(
                Self::NAME,
                ViolationError::TooManyComments(CommentTag::Notice),
                contract.loc,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, ContractDefinition, MissingNotice, Rule, Violation, ViolationError,
    };
    use crate::parser::Parser;
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    /// Macro to define a test case for `MissingParams` rule
    macro_rules! test_missingnotice {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(MissingNotice::check(None, contract, comments), expected);
            }
        };
    }

    test_missingnotice!(
        no_violation,
        r"
        /// @notice Some notice
        interface Test {
        }
        ",
        |_| None
    );

    test_missingnotice!(
        multi_no_violation,
        r"
        /// @title Some title
        /// @notice Some notice
        contract Test {
        }
        ",
        |_| None
    );

    test_missingnotice!(
        multiline_no_violation,
        r"
        /**
         * @notice Some notice
         */
        abstract contract Test {
        }
        ",
        |_| None
    );

    test_missingnotice!(
        multiline_multi_no_violation,
        r"
        /**
         * @title Some title
         * @notice Some notice
         */
        library Test {
        }
        ",
        |_| None
    );

    test_missingnotice!(
        empty_violation,
        r"
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingNotice::NAME,
            ViolationError::MissingComment(CommentTag::Notice),
            sct.loc
        ))
    );

    test_missingnotice!(
        violation,
        r"
        /// @author Some author
        interface Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingNotice::NAME,
            ViolationError::MissingComment(CommentTag::Notice),
            sct.loc
        ))
    );

    test_missingnotice!(
        multi_violation,
        r"
        /// @notice Some notice
        /// @notice Some notice
        abstract contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        ))
    );

    test_missingnotice!(
        multiline_violation,
        r"
        /**
         * @author Some author
         */
        library Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingNotice::NAME,
            ViolationError::MissingComment(CommentTag::Notice),
            sct.loc
        ))
    );

    test_missingnotice!(
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
            MissingNotice::NAME,
            ViolationError::TooManyComments(CommentTag::Notice),
            sct.loc
        ))
    );
}
