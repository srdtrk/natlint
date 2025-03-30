use solang_parser::pt::VariableDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that all variables have a notice or an inheritdoc comment.
#[derive(Default)]
pub struct MissingNotice;

impl Rule for MissingNotice {
    type Target = VariableDefinition;
    const NAME: &'static str = "MissingNotice";
    const DESCRIPTION: &'static str = "Variables must have a notice or an inheritdoc comment.";

    fn check(
        _: Option<&ParseItem>,
        var: &Self::Target,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // If the variable has an inheritdoc comment, it is exempt from this rule
        if comments.find_inheritdoc_base().is_some() {
            return None;
        }

        // Variable must have a notice comment
        if comments.include_tag(CommentTag::Notice).is_empty() {
            return Some(Violation::new(
                Self::NAME,
                Self::DESCRIPTION,
                ViolationError::MissingComment(CommentTag::Notice),
                var.loc,
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, MissingNotice, Rule, VariableDefinition, Violation, ViolationError,
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
                let var = child.as_variable().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(var);

                assert_eq!(MissingNotice::check(Some(parent), var, comments), expected);
            }
        };
    }

    mod pub_const_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Notice,
            test_missingnotice,
            MissingNotice,
            r#"
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            "#,
            "@notice",
            VariableDefinition
        );

        test_missingnotice!(
            no_tag_no_violation,
            r#"
            contract Test {
                /// Some variable
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            }
            "#,
            |_| None
        );

        test_missingnotice!(
            multiline_no_tag_no_violation,
            r#"
            contract Test {
                /**
                 * Some function
                 */
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            }
            "#,
            |_| None
        );

        test_missingnotice!(
            inheritdoc_no_violation,
            r#"
            contract Test {
                /// @inheritdoc something
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            }
            "#,
            |_| None
        );

        test_missingnotice!(
            multiline_inheritdoc_no_violation,
            r#"
            contract Test {
                /**
                 * @inheritdoc something
                 */
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            }
            "#,
            |_| None
        );
    }

    mod pub_immutable_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Notice,
            test_missingnotice,
            MissingNotice,
            r"
                bytes32 public immutable SOME_IMMUT;
            ",
            "@notice",
            VariableDefinition
        );

        test_missingnotice!(
            no_tag_no_violation,
            r"
            contract Test {
                /// Some variable
                bytes32 public immutable SOME_IMMUT;
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
                bytes32 public immutable SOME_IMMUT;
            }
            ",
            |_| None
        );

        test_missingnotice!(
            inheritdoc_no_violation,
            r"
            contract Test {
                /// @inheritdoc something
                bytes32 public immutable SOME_IMMUT;
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
                bytes32 public immutable SOME_IMMUT;
            }
            ",
            |_| None
        );
    }

    mod priv_state_test {
        use super::*;

        generate_missing_comment_test_cases!(
            Notice,
            test_missingnotice,
            MissingNotice,
            r"
                State private state;
            ",
            "@notice",
            VariableDefinition
        );

        test_missingnotice!(
            no_tag_no_violation,
            r"
            contract Test {
                /// Some variable
                State private state;
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
                State private state;
            }
            ",
            |_| None
        );

        test_missingnotice!(
            inheritdoc_no_violation,
            r"
            contract Test {
                /// @inheritdoc something
                State private state;
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
                State private state;
            }
            ",
            |_| None
        );
    }
}
