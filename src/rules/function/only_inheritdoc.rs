//! This rule requires that if a function has an inheritdoc comment, it must be the only comment.
//! This rule will be off by default.

use solang_parser::pt::FunctionDefinition;

use crate::{
    parser::{CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that if a function has an inheritdoc comment, then it must be the only comment.
#[derive(Default)]
pub struct OnlyInheritdoc;

impl Rule for OnlyInheritdoc {
    type Target = FunctionDefinition;
    const NAME: &'static str = "OnlyInheritdoc";
    const DESCRIPTION: &'static str =
        "If a function has an inheritdoc comment, then it must be the only comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &Self::Target,
        comments: &CommentsRef,
    ) -> Option<Violation> {
        if comments.find_inheritdoc_base().is_some() {
            return match comments.len() {
                0 => unreachable!("Inheritdoc comment should have been found"),
                1 => None,
                _ => Some(Violation::new(
                    Self::NAME,
                    Self::DESCRIPTION,
                    ViolationError::OnlyInheritdoc,
                    func.loc,
                )),
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{CommentsRef, FunctionDefinition, OnlyInheritdoc, Rule, Violation, ViolationError};
    use crate::parser::Parser;
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    macro_rules! test_only_inheritdoc {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let func = child.as_function().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(
                    OnlyInheritdoc::check(Some(parent), func, &comments),
                    expected
                );
            }
        };
    }

    test_only_inheritdoc!(
        inheritdoc_no_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            function test() public {}
        }
        ",
        |_| None
    );

    test_only_inheritdoc!(
        empty_no_violation,
        r"
        contract Test {
            function test() external {}
        }
        ",
        |_| None
    );

    test_only_inheritdoc!(
        no_inheritdoc_no_violation,
        r"
        contract Test {
            /// @notice A test function
            function test() override {}
        }
        ",
        |_| None
    );

    test_only_inheritdoc!(
        public_violation,
        r"
        contract Test {
            /// @notice A test function
            /// @inheritdoc Base
            function test() public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            OnlyInheritdoc::NAME,
            OnlyInheritdoc::DESCRIPTION,
            ViolationError::OnlyInheritdoc,
            func.loc
        ))
    );

    test_only_inheritdoc!(
        multiline_violation,
        r"
        contract Test {
            /**
             * @notice A test function
             * @inheritdoc Base
             */
            function test() public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            OnlyInheritdoc::NAME,
            OnlyInheritdoc::DESCRIPTION,
            ViolationError::OnlyInheritdoc,
            func.loc
        ))
    );

    test_only_inheritdoc!(
        external_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            /// @notice A test function
            function test() external {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            OnlyInheritdoc::NAME,
            OnlyInheritdoc::DESCRIPTION,
            ViolationError::OnlyInheritdoc,
            func.loc
        ))
    );

    test_only_inheritdoc!(
        inheritdoc_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            /// @inheritdoc Base2
            function test() override {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            OnlyInheritdoc::NAME,
            OnlyInheritdoc::DESCRIPTION,
            ViolationError::OnlyInheritdoc,
            func.loc
        ))
    );
}
