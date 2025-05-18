//! This module defines the rules for event items in the natlint linter.

mod missing_notice;
mod missing_param;
mod no_author;
mod no_inheritdoc;
mod no_return;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use missing_param::MissingParam;
pub use no_author::NoAuthor;
pub use no_inheritdoc::NoInheritdoc;
pub use no_return::NoReturn;
