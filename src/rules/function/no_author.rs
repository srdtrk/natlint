//! This rule requires that functions do not have an author comment.

use solang_parser::pt::FunctionDefinition;

crate::no_comment_rule!(
    NoAuthor,
    FunctionDefinition,
    Author,
    "Functions must not have an author comment."
);

#[cfg(test)]
mod tests {
    use super::{FunctionDefinition, NoAuthor};
    use crate::{
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
                let func = child.as_function().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(NoAuthor::check(Some(parent), func, comments), expected);
            }
        };
    }

    test_no_author!(
        empty_no_violation,
        r"
        contract Test {
            function test() public {}
        }
        ",
        |_| None
    );

    test_no_author!(
        no_violation,
        r"
        contract Test {
            /// @custom:test Some function
            function test() public {}
        }
        ",
        |_| None
    );

    test_no_author!(
        multiline_no_violation,
        r"
        contract Test {
            /**
             * @custom:test Some function
             */
            function test() public {}
        }
        ",
        |_| None
    );

    test_no_author!(
        violation,
        r"
        contract Test {
            /// @author Some author
            function test() public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            NoAuthor::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Author),
            func.loc,
        ))
    );

    test_no_author!(
        multiline_violation,
        r"
        contract Test {
            /**
             * @author Some author
             */
            function test() public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            NoAuthor::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Author),
            func.loc,
        ))
    );
}
