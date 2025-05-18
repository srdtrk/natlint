//! This module defines the rules for event items in the natlint linter.

mod missing_notice;
mod missing_param;

// The rules are exported here:
pub use missing_notice::MissingNotice;
pub use missing_param::MissingParam;
