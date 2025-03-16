//! This module defines the rules for error items in the natlint linter.

mod missing_notice;
mod no_author;
mod no_inheritdoc;
mod no_return;
mod no_title;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use no_author::NoAuthor;
pub use no_inheritdoc::NoInheritdoc;
pub use no_return::NoReturn;
pub use no_title::NoTitle;
