//! This rule requires that if a function has an inheritdoc comment, it must be the only comment.
//! This rule will be off by default.

use solang_parser::pt::FunctionDefinition;

use crate::parser::{CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that if a function has an inheritdoc comment, then it must be the only comment.
pub struct OnlyInheritdoc;

impl Rule<FunctionDefinition> for OnlyInheritdoc {
    const NAME: &'static str = "Only Inheritdoc";
    const DESCRIPTION: &'static str =
        "If a function has an inheritdoc comment, then it must be the only comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        if comments.find_inheritdoc_base().is_some() {
            match comments.len() {
                0 => unreachable!("Inheritdoc comment should have been found"),
                1 => None,
                _ => Some(Violation::new(
                    Self::NAME,
                    Self::DESCRIPTION.to_string(),
                    func.loc,
                )),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OnlyInheritdoc, Rule};
    use crate::{
        parser::{CommentsRef, Parser},
        rules::Violation,
    };
    use forge_fmt::Visitable;
    use solang_parser::{parse, pt::FunctionDefinition};

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    /// Macro to define a test case for `RequireInheritdoc` rule
    macro_rules! test_require_inheritdoc {
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
                    OnlyInheritdoc::check(Some(parent), func, comments),
                    expected
                );
            }
        };
    }

    test_require_inheritdoc!(
        inheritdoc_no_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            function test() public {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        empty_no_violation,
        r"
        contract Test {
            function test() external {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
        no_inheritdoc_no_violation,
        r"
        contract Test {
            /// @notice A test function
            function test() override {}
        }
        ",
        |_| None
    );

    test_require_inheritdoc!(
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
            OnlyInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );

    test_require_inheritdoc!(
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
            OnlyInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );

    test_require_inheritdoc!(
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
            OnlyInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );

    test_require_inheritdoc!(
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
            OnlyInheritdoc::DESCRIPTION.to_string(),
            func.loc
        ))
    );
}
