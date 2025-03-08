//! This module defines the rules for enum items in the natlint linter.

mod missing_author;
mod missing_notice;
mod missing_title;

// The rules are exported here:
pub use missing_author::MissingAuthor;
pub use missing_notice::MissingNotice;
pub use missing_title::MissingTitle;
