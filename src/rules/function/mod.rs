//! This module defines the rules for function items in the natlint linter.

mod require_inheritdoc;

// The rules are exported here:

pub use require_inheritdoc::RequireInheritdoc;
