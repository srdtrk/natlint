use solang_parser::pt::EnumDefinition;

crate::no_comment_rule!(
    NoParam,
    EnumDefinition,
    Param,
    "Enums must not have a param comment."
);

#[cfg(test)]
mod tests {
    use super::{EnumDefinition, NoParam};
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
                let item = child.as_enum().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(item);

                assert_eq!(NoParam::check(Some(parent), item, comments), expected);
            }
        };
    }

    generate_no_comment_test_cases!(
        Param,
        test_no_param,
        NoParam,
        r"
            enum Option {
                Some,
                None
            }
        ",
        "@param",
        EnumDefinition
    );
}
