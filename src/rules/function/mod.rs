//! This module defines the rules for function items in the natlint linter.

mod missing_inheritdoc;
mod missing_notice;
mod missing_params;
mod missing_return;
mod no_author;
mod no_title;
mod only_inheritdoc;
mod too_many_inheritdoc;
mod too_many_notice;

// The rules are exported here:
pub use missing_inheritdoc::MissingInheritdoc;
pub use missing_notice::MissingNotice;
pub use missing_params::MissingParams;
pub use missing_return::MissingReturn;
pub use no_author::NoAuthor;
pub use no_title::NoTitle;
pub use only_inheritdoc::OnlyInheritdoc;
pub use too_many_inheritdoc::TooManyInheritdoc;
pub use too_many_notice::TooManyNotice;
