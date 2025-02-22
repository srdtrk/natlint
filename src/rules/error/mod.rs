//! This module defines the rules for error items in the natlint linter.

use solang_parser::pt::ErrorDefinition;

crate::define_rule!(ContractRule, ErrorDefinition);
