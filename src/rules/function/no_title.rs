//! This rule requires that functions do not have a title comment.

use solang_parser::pt::FunctionDefinition;

use crate::parser::{CommentTag, CommentsRef, ParseItem};

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
                Self::DESCRIPTION.to_string(),
                func.loc,
            ));
        }

        None
    }
}
