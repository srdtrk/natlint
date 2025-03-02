//! This module defines the rules for struct items in the natlint linter.

mod missing_author;
mod missing_notice;
mod missing_params;
mod missing_title;
mod no_inheritdoc;
mod no_return;
mod too_many_notice;

// The rules are exported here:
pub use missing_author::MissingAuthor;
pub use missing_notice::MissingNotice;
pub use missing_params::MissingParams;
pub use missing_title::MissingTitle;
pub use no_inheritdoc::NoInheritdoc;
pub use no_return::NoReturn;
