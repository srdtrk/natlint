//! This module defines the rules for function items in the natlint linter.

mod missing_params;
mod require_inheritdoc;

// The rules are exported here:
pub use missing_params::MissingParams;
pub use require_inheritdoc::RequireInheritdoc;
