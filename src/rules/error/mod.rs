//! This module defines the rules for error items in the natlint linter.

mod missing_notice;
mod missing_param;
mod no_author;
mod no_inheritdoc;
mod no_return;
mod no_title;
mod too_many_notice;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use missing_param::MissingParam;
pub use no_author::NoAuthor;
pub use no_inheritdoc::NoInheritdoc;
pub use no_return::NoReturn;
pub use no_title::NoTitle;
pub use too_many_notice::TooManyNotice;
