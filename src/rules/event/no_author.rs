use solang_parser::pt::EventDefinition;

crate::no_comment_rule!(
    NoAuthor,
    EventDefinition,
    Author,
    "Events must not have an author comment."
);

#[cfg(test)]
mod tests {
    use super::{EventDefinition, NoAuthor};
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

    macro_rules! test_no_author {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let event = child.as_event().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(event);

                assert_eq!(NoAuthor::check(Some(parent), event, &comments), expected);
            }
        };
    }

    generate_no_comment_test_cases!(
        Author,
        test_no_author,
        NoAuthor,
        r"
            event Success();
        ",
        "@author",
        EventDefinition
    );
}
