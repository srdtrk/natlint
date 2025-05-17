use solang_parser::pt::VariableDefinition;

crate::too_many_comments_rule!(
    TooManyNotice,
    VariableDefinition,
    Notice,
    "Variables must not have more than one notice comment."
);

#[cfg(test)]
mod tests {
    use super::{TooManyNotice, VariableDefinition};
    use crate::{
        generate_too_many_comment_test_cases,
        parser::{CommentTag, CommentsRef, Parser},
        rules::{Rule, Violation, ViolationError},
    };
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    macro_rules! test_too_many_notice {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let var = child.as_variable().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(var);

                assert_eq!(TooManyNotice::check(Some(parent), var, comments), expected);
            }
        };
    }

    pub mod pub_const_test {
        use super::*;

        generate_too_many_comment_test_cases!(
            Notice,
            test_too_many_notice,
            TooManyNotice,
            r#"
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            "#,
            "@notice",
            VariableDefinition
        );

        test_too_many_notice!(
            no_tag_violation,
            r#"
            contract Test {
                /// Some notice
                /// @notice Some other
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            }
            "#,
            |sct: &VariableDefinition| Some(Violation::new(
                TooManyNotice::NAME,
                TooManyNotice::DESCRIPTION,
                ViolationError::TooManyComments(CommentTag::Notice),
                sct.loc
            )) // WARNING: solang parser and the natspec docs interpret no tags as a notice
        );
    }

    pub mod pub_immutable_test {
        use super::*;

        generate_too_many_comment_test_cases!(
            Notice,
            test_too_many_notice,
            TooManyNotice,
            r"
                bytes32 public immutable SOME_IMMUT;
            ",
            "@notice",
            VariableDefinition
        );

        test_too_many_notice!(
            no_tag_violation,
            r"
            contract Test {
                /// Some notice
                /// @notice Some other
                bytes32 public immutable SOME_IMMUT;
            }
            ",
            |sct: &VariableDefinition| Some(Violation::new(
                TooManyNotice::NAME,
                TooManyNotice::DESCRIPTION,
                ViolationError::TooManyComments(CommentTag::Notice),
                sct.loc
            )) // WARNING: solang parser and the natspec docs interpret no tags as a notice
        );
    }

    pub mod priv_state_test {
        use super::*;

        generate_too_many_comment_test_cases!(
            Notice,
            test_too_many_notice,
            TooManyNotice,
            r"
                State private state;
            ",
            "@notice",
            VariableDefinition
        );

        test_too_many_notice!(
            no_tag_violation,
            r"
            contract Test {
                /// Some notice
                /// @notice Some other
                State private state;
            }
            ",
            |sct: &VariableDefinition| Some(Violation::new(
                TooManyNotice::NAME,
                TooManyNotice::DESCRIPTION,
                ViolationError::TooManyComments(CommentTag::Notice),
                sct.loc
            )) // WARNING: solang parser and the natspec docs interpret no tags as a notice
        );
    }
}
