//! This module defines the rules for variable items in the natlint linter.

mod missing_notice;
mod no_author;
mod no_param;
mod no_title;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use no_author::NoAuthor;
pub use no_param::NoParam;
pub use no_title::NoTitle;
