use solang_parser::pt::ContractDefinition;

crate::no_comment_rule!(
    NoParam,
    ContractDefinition,
    Param,
    "Contracts must not have a param comment."
);

#[cfg(test)]
mod tests {
    use super::{ContractDefinition, NoParam};
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
    macro_rules! test_no_param {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(NoParam::check(None, contract, &comments), expected);
            }
        };
    }

    test_no_param!(
        empty_no_violation,
        r"
        interface Test {
        }
        ",
        |_| None
    );

    test_no_param!(
        no_violation,
        r"
        /// @custom:test Some comment
        interface Test {
        }
        ",
        |_| None
    );

    test_no_param!(
        multi_no_violation,
        r"
        /// @custom:test Some comment
        /// @custom:test Some other
        contract Test {
        }
        ",
        |_| None
    );

    test_no_param!(
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

    test_no_param!(
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

    test_no_param!(
        violation,
        r"
        /// @param Some param
        interface Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoParam::NAME,
            NoParam::DESCRIPTION,
            ViolationError::CommentNotAllowed(CommentTag::Param),
            sct.loc
        ))
    );

    test_no_param!(
        multi_violation,
        r"
        /// @param Some param
        /// @param Some param
        abstract contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoParam::NAME,
            NoParam::DESCRIPTION,
            ViolationError::CommentNotAllowed(CommentTag::Param),
            sct.loc
        ))
    );

    test_no_param!(
        multiline_violation,
        r"
        /**
         * @param Some param
         */
        library Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoParam::NAME,
            NoParam::DESCRIPTION,
            ViolationError::CommentNotAllowed(CommentTag::Param),
            sct.loc
        ))
    );

    test_no_param!(
        multiline_multi_violation,
        r"
        /**
         * @param Some param
         * @param Some param
         */
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoParam::NAME,
            NoParam::DESCRIPTION,
            ViolationError::CommentNotAllowed(CommentTag::Param),
            sct.loc
        ))
    );
}
