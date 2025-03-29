use solang_parser::pt::VariableDefinition;

crate::too_many_comments_rule!(
    TooManyInheritdoc,
    VariableDefinition,
    Inheritdoc,
    "Variables must not have more than one inheritdoc comment."
);

#[cfg(test)]
mod tests {
    use super::{TooManyInheritdoc, VariableDefinition};
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

                assert_eq!(
                    TooManyInheritdoc::check(Some(parent), var, comments),
                    expected
                );
            }
        };
    }

    pub mod pub_const_test {
        use super::*;

        generate_too_many_comment_test_cases!(
            Inheritdoc,
            test_too_many_notice,
            TooManyInheritdoc,
            r#"
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            "#,
            "@inheritdoc",
            VariableDefinition
        );
    }

    pub mod pub_immutable_test {
        use super::*;

        generate_too_many_comment_test_cases!(
            Inheritdoc,
            test_too_many_notice,
            TooManyInheritdoc,
            r"
                bytes32 public immutable SOME_IMMUT;
            ",
            "@inheritdoc",
            VariableDefinition
        );
    }

    pub mod priv_state_test {
        use super::*;

        generate_too_many_comment_test_cases!(
            Inheritdoc,
            test_too_many_notice,
            TooManyInheritdoc,
            r"
                State private state;
            ",
            "@inheritdoc",
            VariableDefinition
        );
    }
}
