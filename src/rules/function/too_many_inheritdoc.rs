use solang_parser::pt::FunctionDefinition;

crate::too_many_comments_rule!(
    TooManyInheritdoc,
    FunctionDefinition,
    Inheritdoc,
    "Functions must not have more than one inheritdoc comment."
);

#[cfg(test)]
mod tests {
    use super::{FunctionDefinition, TooManyInheritdoc};
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

    macro_rules! test_too_many_inheritdoc {
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
                    TooManyInheritdoc::check(Some(parent), func, comments),
                    expected
                );
            }
        };
    }

    generate_too_many_comment_tests!(
        Inheritdoc,
        test_too_many_inheritdoc,
        TooManyInheritdoc,
        r"
            function test() private {}
        ",
        "@inheritdoc",
        FunctionDefinition
    );
}
