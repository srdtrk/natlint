//! This module defines the rules for contract items in the natlint linter.

mod missing_author;
mod missing_notice;
mod missing_title;
mod no_param;
mod no_return;

// The rules are exported here:
pub use missing_author::MissingAuthor;
pub use missing_notice::MissingNotice;
pub use missing_title::MissingTitle;
pub use no_param::NoParam;
pub use no_return::NoReturn;
