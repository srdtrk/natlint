use solang_parser::pt::FunctionDefinition;

use crate::parser::{CommentTag, CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that all functions have a notice or an inheritdoc comment.
pub struct MissingNotice;

impl Rule<FunctionDefinition> for MissingNotice {
    const NAME: &'static str = "Missing Notice";
    const DESCRIPTION: &'static str =
        "This rule requires that all functions have a notice or an inheritdoc comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // If the function has an inheritdoc comment, it is exempt from this rule
        if comments.find_inheritdoc_base().is_some() {
            return None;
        }

        // Function must have a notice comment
        match comments.include_tag(CommentTag::Notice).len() {
            0 => Some(Violation::new(
                Self::NAME,
                "Missing notice or inheritdoc comment".to_string(),
                func.loc,
            )),
            1 => None,
            _ => Some(Violation::new(
                Self::NAME,
                "Too many notice comments".to_string(),
                func.loc,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionDefinition, MissingNotice, Rule};
    use crate::{
        parser::{CommentsRef, Parser},
        rules::Violation,
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

                assert_eq!(MissingNotice::check(Some(parent), func, comments), expected);
            }
        };
    }

    test_missingnotice!(
        public_no_violation,
        r"
        contract Test {
            /// @notice Some function
            function test(uint256 a) public {}
        }
        ",
        |_| None
    );

    test_missingnotice!(
        private_no_violation,
        r"
        contract Test {
            /// @notice Some function
            function test(uint256 a) private {}
        }
        ",
        |_| None
    );

    test_missingnotice!(
        multiline_no_violation,
        r"
        contract Test {
            /**
             * @notice Some function
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
        public_violation,
        r"
        contract Test {
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Missing notice or inheritdoc comment".to_string(),
            func.loc
        ))
    );

    test_missingnotice!(
        too_many_comments_violation,
        r"
        contract Test {
            /// @notice Some function
            /// @notice Another function
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Too many notice comments".to_string(),
            func.loc
        ))
    );

    test_missingnotice!(
        multiline_many_comments_violation,
        r"
        contract Test {
            /**
             * @notice Some function
             * @notice Another function
             */
            function test(uint256 a) public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            MissingNotice::NAME,
            "Too many notice comments".to_string(),
            func.loc
        ))
    );
}
