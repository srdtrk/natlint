use solang_parser::pt::VariableDefinition;

crate::no_comment_rule!(
    NoParam,
    VariableDefinition,
    Param,
    "Variables must not have a param comment."
);

#[cfg(test)]
mod tests {
    use super::{NoParam, VariableDefinition};
    use crate::{
        generate_no_comment_test_cases,
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

    macro_rules! test_no_param {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let var = child.as_variable().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(var);

                assert_eq!(NoParam::check(Some(parent), var, &comments), expected);
            }
        };
    }

    pub mod pub_const_test {
        use super::*;

        generate_no_comment_test_cases!(
            Param,
            test_no_param,
            NoParam,
            r#"
                bytes32 public constant SOME_CONST = keccak256("SOME_CONST");
            "#,
            "@param",
            VariableDefinition
        );
    }

    pub mod pub_immutable_test {
        use super::*;

        generate_no_comment_test_cases!(
            Param,
            test_no_param,
            NoParam,
            r"
                bytes32 public immutable SOME_IMMUT;
            ",
            "@param",
            VariableDefinition
        );
    }

    pub mod priv_state_test {
        use super::*;

        generate_no_comment_test_cases!(
            Param,
            test_no_param,
            NoParam,
            r"
                State private state;
            ",
            "@param",
            VariableDefinition
        );
    }
}
