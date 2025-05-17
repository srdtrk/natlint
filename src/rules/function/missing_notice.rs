use solang_parser::pt::FunctionDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all functions have a notice or an inheritdoc comment.
pub struct MissingNotice;

impl Rule for MissingNotice {
    type Target = FunctionDefinition;
    const NAME: &'static str = "MissingNotice";
    const DESCRIPTION: &'static str = "Functions must have a notice or an inheritdoc comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: &CommentsRef,
    ) -> Option<Violation> {
        // If the function has an inheritdoc comment, it is exempt from this rule
        if comments.find_inheritdoc_base().is_some() {
            return None;
        }

        // Function must have a notice comment
        if comments.include_tag(CommentTag::Notice).is_empty() {
            return Some(Violation::new(
                Self::NAME,
                ViolationError::MissingComment(CommentTag::Notice),
                func.loc,
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, FunctionDefinition, MissingNotice, Rule, Violation, ViolationError,
    };
    use crate::{generate_missing_comment_test_cases, parser::Parser};
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    macro_rules! test_missingnotice {
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
                    MissingNotice::check(Some(parent), func, &comments),
                    expected
                );
            }
        };
    }

    mod public_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Notice,
            test_missingnotice,
            MissingNotice,
            r"
                function test(uint256 a) public {}
            ",
            "@notice",
            FunctionDefinition
        );

        test_missingnotice!(
            long_multiline_no_tag_no_violation,
            r"
            contract Test {
                /**
                 * Some function
                 * next line
                 */
                function test(uint256 a) public {}
            }
            ",
            |_| None
        );

        test_missingnotice!(
            inheritdoc_no_violation,
            r"
            contract Test {
                /// @inheritdoc something
                function test(uint256 a) public {}
            }
            ",
            |_| None
        );

        test_missingnotice!(
            multiline_inheritdoc_no_violation,
            r"
            contract Test {
                /**
                 * @inheritdoc something
                 */
                function test(uint256 a) public {}
            }
            ",
            |_| None
        );

        test_missingnotice!(
            no_tag_no_violation,
            r"
                contract Test {
                    /// Some function
                    function test(uint256 a) public {}
                }
            ",
            |_| None
        );

        test_missingnotice!(
            multiline_no_tag_no_violation,
            r"
            contract Test {
                /**
                 * Some function
                 */
                function test(uint256 a) public {}
            }
            ",
            |_| None
        );
    }

    mod private_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Notice,
            test_missingnotice,
            MissingNotice,
            r"
                function test(uint256 a) private {}
            ",
            "@notice",
            FunctionDefinition
        );

        test_missingnotice!(
            long_multiline_no_tag_no_violation,
            r"
            contract Test {
                /**
                 * Some function
                 * next line
                 */
                function test(uint256 a) private {}
            }
            ",
            |_| None
        );

        test_missingnotice!(
            inheritdoc_no_violation,
            r"
            contract Test {
                /// @inheritdoc something
                function test(uint256 a) private {}
            }
            ",
            |_| None
        );

        test_missingnotice!(
            multiline_inheritdoc_no_violation,
            r"
            contract Test {
                /**
                 * @inheritdoc something
                 */
                function test(uint256 a) private {}
            }
            ",
            |_| None
        );

        test_missingnotice!(
            no_tag_no_violation,
            r"
                contract Test {
                    /// Some function
                    function test(uint256 a) private {}
                }
            ",
            |_| None
        );

        test_missingnotice!(
            multiline_no_tag_no_violation,
            r"
            contract Test {
                /**
                 * Some function
                 */
                function test(uint256 a) private {}
            }
            ",
            |_| None
        );
    }
}
