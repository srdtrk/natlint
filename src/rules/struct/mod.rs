//! This module defines the rules for struct items in the natlint linter.

mod missing_notice;
mod missing_params;
mod missing_title;
mod no_inheritdoc;
mod no_return;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use missing_params::MissingParams;
pub use missing_title::MissingTitle;
pub use no_inheritdoc::NoInheritdoc;
pub use no_return::NoReturn;
