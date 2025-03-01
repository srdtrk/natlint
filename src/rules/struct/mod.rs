//! This module defines the rules for struct items in the natlint linter.

mod missing_notice;
mod no_inheritdoc;

pub use missing_notice::MissingNotice;
pub use no_inheritdoc::NoInheritdoc;
