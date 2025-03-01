//! This rule requires that functions do not have a title comment.

use solang_parser::pt::FunctionDefinition;

use crate::{
    parser::{CommentTag, CommentsRef, ParseItem},
    rules::violation_error::ViolationError,
};

use super::super::{Rule, Violation};

/// This rule requires that functions do not have a title comment.
pub struct NoTitle;

impl Rule<FunctionDefinition> for NoTitle {
    const NAME: &'static str = "No Title";
    const DESCRIPTION: &'static str = "Functions must not have a title comment.";

    fn check(
        _: Option<&ParseItem>,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        if !comments.include_tag(CommentTag::Title).is_empty() {
            return Some(Violation::new(
                Self::NAME,
                ViolationError::CommentNotAllowed(CommentTag::Title),
                func.loc,
            ));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommentTag, CommentsRef, FunctionDefinition, NoTitle, Rule, Violation, ViolationError,
    };
    use crate::parser::Parser;
    use forge_fmt::Visitable;
    use solang_parser::parse;

    fn parse_source(src: &str) -> Parser {
        let (mut source, comments) = parse(src, 0).expect("failed to parse source");
        let mut doc = Parser::new(comments, src.to_owned());
        source.visit(&mut doc).expect("failed to visit source");
        doc
    }

    macro_rules! test_no_title {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let src = parse_source($source);

                let parent = src.items_ref().first().unwrap();
                let child = parent.children.first().unwrap();
                let func = child.as_function().unwrap();
                let comments = CommentsRef::from(&child.comments);

                let expected = $expected(func);

                assert_eq!(NoTitle::check(Some(parent), func, comments), expected);
            }
        };
    }

    test_no_title!(
        no_violation,
        r"
        contract Test {
            /// @inheritdoc Base
            function test() public {}
        }
        ",
        |_| None
    );

    test_no_title!(
        violation,
        r"
        contract Test {
            /// @title Some function
            function test() public {}
        }
        ",
        |func: &FunctionDefinition| Some(Violation::new(
            NoTitle::NAME,
            ViolationError::CommentNotAllowed(CommentTag::Title),
            func.loc,
        ))
    );
}
