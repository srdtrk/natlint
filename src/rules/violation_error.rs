//! This module defines all the possible violation errors that can be reported by the natlint
//! linter.

use thiserror::Error;

use crate::parser::CommentTag;

/// An error that occurs when a rule is violated.
#[derive(Debug, Error, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ViolationError {
    #[error("Missing a {0} comment")]
    MissingComment(CommentTag),
    #[error("Too many {0} comments")]
    TooManyComments(CommentTag),
    #[error("{0} comments are not allowed on this construct")]
    CommentNotAllowed(CommentTag),
    #[error("Missing a {tag} comment for `{name}`")]
    MissingCommentFor { tag: CommentTag, name: String },
    #[error("Inheritdoc comment must be the only comment")]
    OnlyInheritdoc,
    #[error("Error while parsing: {0}")]
    ParseError(String),
}

impl ViolationError {
    /// Create a new [`ViolationError::MissingComment`] error.
    #[must_use]
    pub fn missing_comment_for(tag: CommentTag, name: impl Into<String>) -> Self {
        Self::MissingCommentFor {
            tag,
            name: name.into(),
        }
    }

    /// Create a new [`ViolationError::ParseError`] error.
    #[must_use]
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::ParseError(msg.into())
    }
}
