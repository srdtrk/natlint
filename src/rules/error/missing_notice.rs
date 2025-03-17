use solang_parser::pt::ErrorDefinition;

crate::missing_comment_rule!(
    MissingNotice,
    ErrorDefinition,
    Notice,
    "Errors must have a notice comment."
);

#[cfg(test)]
mod tests {
    use super::{ErrorDefinition, MissingNotice};
    use crate::{
        generate_missing_comment_test_cases,
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

    macro_rules! test_missing_notice {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let func = child.as_error().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(MissingNotice::check(Some(parent), func, comments), expected);
            }
        };
    }

    generate_missing_comment_test_cases!(
        Notice,
        test_missing_notice,
        MissingNotice,
        r"
            error Unauthorized();
        ",
        "@notice",
        ErrorDefinition
    );
}
