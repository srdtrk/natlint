//! This rule requires that all contacts have an author comment.
//! This rule is off by default.

use solang_parser::pt::ContractDefinition;

crate::missing_comment_rule!(
    MissingAuthor,
    ContractDefinition,
    Author,
    "Contracts must have an author comment"
);

#[cfg(test)]
mod tests {
    use super::{ContractDefinition, MissingAuthor};
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
    macro_rules! test_missingauthor {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(MissingAuthor::check(None, contract, comments), expected);
            }
        };
    }

    test_missingauthor!(
        no_violation,
        r"
        /// @author Some author
        interface Test {
        }
        ",
        |_| None
    );

    test_missingauthor!(
        multi_no_violation,
        r"
        /// @title Some title
        /// @author Some author
        contract Test {
        }
        ",
        |_| None
    );

    test_missingauthor!(
        multi_author_no_violation,
        r"
        /// @author Some author
        /// @author Some other
        abstract contract Test {
        }
        ",
        |_| None
    );

    test_missingauthor!(
        multiline_no_violation,
        r"
        /**
         * @author Some author
         */
        interface Test {
        }
        ",
        |_| None
    );

    test_missingauthor!(
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

    test_missingauthor!(
        multiline_multi_author_no_violation,
        r"
        /**
         * @author Some author
         * @author Some other
         */
        contract Test {
        }
        ",
        |_| None
    );

    test_missingauthor!(
        empty_violation,
        r"
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingAuthor::NAME,
            MissingAuthor::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Author),
            sct.loc
        ))
    );

    test_missingauthor!(
        violation,
        r"
        /// @title Some title
        interface Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingAuthor::NAME,
            MissingAuthor::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Author),
            sct.loc
        ))
    );

    test_missingauthor!(
        multiline_violation,
        r"
        /**
         * @title Some title
         */
        library Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            MissingAuthor::NAME,
            MissingAuthor::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Author),
            sct.loc
        ))
    );
}
