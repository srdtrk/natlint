use solang_parser::pt::ContractDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that contracts do not have a param comment.
pub struct NoReturn;

impl Rule<ContractDefinition> for NoReturn {
    const NAME: &'static str = "No Return";
    const DESCRIPTION: &'static str = "Contracts must not have a return comment.";

    fn check(
        _: Option<&ParseItem>,
        contract: &ContractDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        if !comments.include_tag(CommentTag::Return).is_empty() {
            return Some(Violation::new(
                Self::NAME,
                ViolationError::CommentNotAllowed(CommentTag::Return),
                contract.loc,
            ));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, ContractDefinition, NoReturn, Rule, Violation, ViolationError,
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
    macro_rules! test_no_return {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let contract_item = src.items_ref().first().unwrap();
                let comments = CommentsRef::from(&contract_item.comments);
                let contract = contract_item.as_contract().unwrap();

                let expected = $expected(contract);

                assert_eq!(NoReturn::check(None, contract, comments), expected);
            }
        };
    }

    test_no_return!(
        empty_no_violation,
        r"
        interface Test {
        }
        ",
        |_| None
    );

    test_no_return!(
        no_violation,
        r"
        /// @custom:test Some comment
        interface Test {
        }
        ",
        |_| None
    );

    test_no_return!(
        multi_no_violation,
        r"
        /// @custom:test Some comment
        /// @custom:test Some other
        contract Test {
        }
        ",
        |_| None
    );

    test_no_return!(
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

    test_no_return!(
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

    test_no_return!(
        violation,
        r"
        /// @return Some return
        interface Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoReturn::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Return),
            sct.loc
        ))
    );

    test_no_return!(
        multi_violation,
        r"
        /// @return Some return
        /// @return Some return
        abstract contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoReturn::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Return),
            sct.loc
        ))
    );

    test_no_return!(
        multiline_violation,
        r"
        /**
         * @return Some return
         */
        library Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoReturn::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Return),
            sct.loc
        ))
    );

    test_no_return!(
        multiline_multi_violation,
        r"
        /**
         * @return Some return
         * @return Some return
         */
        contract Test {
        }
        ",
        |sct: &ContractDefinition| Some(Violation::new(
            NoReturn::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Return),
            sct.loc
        ))
    );
}
