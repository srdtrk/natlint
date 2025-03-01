//! This rule requires that functions do not have an author comment.

use solang_parser::pt::FunctionDefinition;

use crate::parser::{CommentTag, CommentsRef, ParseItem};

use super::super::{Rule, Violation};

/// This rule requires that functions do not have an author comment.
pub struct NoAuthor;

impl Rule<FunctionDefinition> for NoAuthor {
    const NAME: &'static str = "No Author";
    const DESCRIPTION: &'static str = "Functions must not have an author comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        if !comments.include_tag(CommentTag::Author).is_empty() {
            return Some(Violation::new(
                Self::NAME,
                Self::DESCRIPTION.to_string(),
                func.loc,
            ));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{CommentsRef, FunctionDefinition, NoAuthor, Rule, Violation};
    use crate::parser::Parser;
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
        no_violation,
        r"
        contract Test {
            /// @title Some function
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
            NoAuthor::DESCRIPTION.to_string(),
            func.loc,
        ))
    );
}
