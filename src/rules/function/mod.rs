//! This module defines the rules for function items in the natlint linter.

mod missing_notice;
mod missing_params;
mod missing_return;
mod no_author;
mod no_title;
mod only_inheritdoc;
mod require_inheritdoc;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use missing_params::MissingParams;
pub use missing_return::MissingReturn;
pub use no_author::NoAuthor;
pub use no_title::NoTitle;
pub use only_inheritdoc::OnlyInheritdoc;
pub use require_inheritdoc::RequireInheritdoc;
