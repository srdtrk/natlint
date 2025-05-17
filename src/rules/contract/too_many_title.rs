use solang_parser::pt::ContractDefinition;

crate::too_many_comments_rule!(
    TooManyTitle,
    ContractDefinition,
    Title,
    "Contracts must not have more than one title comment."
);

#[cfg(test)]
mod tests {
    use super::{ContractDefinition, TooManyTitle};
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
    macro_rules! test_too_many_title {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(TooManyTitle::check(None, contract, comments), expected);
            }
        };
    }

    test_too_many_title!(
        empty_no_violation,
        r"
        contract Test {
        }
        ",
        |_| None
    );

    test_too_many_title!(
        exists_no_violation,
        r"
        /// @title Some title
        interface Test {
        }
        ",
        |_| None
    );

    test_too_many_title!(
        multi_no_violation,
        r"
        /// @custom:test Some comment
        /// @title Some title
        contract Test {
        }
        ",
        |_| None
    );

    test_too_many_title!(
        multiline_exists_no_violation,
        r"
        /**
         * @title Some title
         */
        abstract contract Test {
        }
        ",
        |_| None
    );

    test_too_many_title!(
        multiline_multi_no_violation,
        r"
        /**
         * @custom:test Some comment
         * @title Some title
         */
        library Test {
        }
        ",
        |_| None
    );

    test_too_many_title!(
        no_violation,
        r"
        /// @custom:test Some comment
        interface Test {
        }
        ",
        |_| None
    );

    test_too_many_title!(
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

    test_too_many_title!(
        multi_violation,
        r"
        /// @title Some title
        /// @title Some title
        abstract contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            TooManyTitle::NAME,
            TooManyTitle::DESCRIPTION,
            ViolationError::TooManyComments(CommentTag::Title),
            sct.loc
        ))
    );

    test_too_many_title!(
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
            TooManyTitle::NAME,
            TooManyTitle::DESCRIPTION,
            ViolationError::TooManyComments(CommentTag::Title),
            sct.loc
        ))
    );
}
