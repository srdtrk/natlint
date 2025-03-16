use solang_parser::pt::FunctionDefinition;

crate::too_many_comments_rule!(
    TooManyNotice,
    FunctionDefinition,
    Notice,
    "Functions must not have more than one notice comment."
);

#[cfg(test)]
mod tests {
    use super::{FunctionDefinition, TooManyNotice};
    use crate::{
        generate_too_many_comment_tests,
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
                let func = child.as_function().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(TooManyNotice::check(Some(parent), func, comments), expected);
            }
        };
    }

    generate_too_many_comment_tests!(
        Notice,
        test_too_many_notice,
        TooManyNotice,
        r"
            function test() private {}
        ",
        "@notice",
        FunctionDefinition
    );
}
