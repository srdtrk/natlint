//! This rule requires that all contacts have a title comment.

use solang_parser::pt::ContractDefinition;

crate::missing_comment_rule!(
    MissingTitle,
    ContractDefinition,
    Title,
    "Contracts must have a title comment"
);

#[cfg(test)]
mod tests {
    use super::{ContractDefinition, MissingTitle};
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
    macro_rules! test_missingtitle {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(MissingTitle::check(None, contract, &comments), expected);
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
            MissingTitle::DESCRIPTION,
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
            MissingTitle::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Title),
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
            MissingTitle::DESCRIPTION,
            ViolationError::MissingComment(CommentTag::Title),
            sct.loc
        ))
    );
}
