//! This module defines the rules for struct items in the natlint linter.

mod missing_notice;
mod missing_params;
mod no_inheritdoc;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use missing_params::MissingParams;
pub use no_inheritdoc::NoInheritdoc;
