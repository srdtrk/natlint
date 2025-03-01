//! This rule requires that all contacts have a title comment.

use solang_parser::pt::ContractDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all contracts have a title comment.
pub struct MissingTitle;

impl Rule<ContractDefinition> for MissingTitle {
    const NAME: &'static str = "Missing Title";
    const DESCRIPTION: &'static str = "This rule requires that all contracts have a title comment.";

    fn check(
        _: Option<&ParseItem>,
        item: &ContractDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Contract must have a title comment
        match comments.include_tag(CommentTag::Title).len() {
            0 => Some(Violation::new(
                Self::NAME,
                ViolationError::MissingComment(CommentTag::Title),
                item.loc,
            )),
            1 => None,
            _ => Some(Violation::new(
                Self::NAME,
                ViolationError::TooManyComments(CommentTag::Title),
                item.loc,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, ContractDefinition, MissingTitle, Rule, Violation, ViolationError,
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
    macro_rules! test_missingtitle {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(MissingTitle::check(None, contract, comments), expected);
            }
        };
    }

    test_missingtitle!(
        no_violation,
        r"
        /// @title Some title
        interface Test {
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multi_no_violation,
        r"
        /// @title Some title
        /// @author Some author
        contract Test {
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multiline_no_violation,
        r"
        /**
         * @title Some title
         */
        abstract contract Test {
        }
        ",
        |_| None
    );

    test_missingtitle!(
        multiline_multi_no_violation,
        r"
        /**
         * @title Some title
         * @author Some author
         */
        library Test {
        }
        ",
        |_| None
    );

    test_missingtitle!(
        empty_violation,
        r"
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::MissingComment(CommentTag::Title),
            sct.loc
        ))
    );

    test_missingtitle!(
        violation,
        r"
        /// @author Some author
        interface Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::MissingComment(CommentTag::Title),
            sct.loc
        ))
    );

    test_missingtitle!(
        multi_violation,
        r"
        /// @title Some title
        /// @title Some title
        abstract contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::TooManyComments(CommentTag::Title),
            sct.loc
        ))
    );

    test_missingtitle!(
        multiline_violation,
        r"
        /**
         * @author Some author
         */
        library Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::MissingComment(CommentTag::Title),
            sct.loc
        ))
    );

    test_missingtitle!(
        multiline_multi_violation,
        r"
        /**
         * @title Some title
         * @title Some title
         */
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingTitle::NAME,
            ViolationError::TooManyComments(CommentTag::Title),
            sct.loc
        ))
    );
}
