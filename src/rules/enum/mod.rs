//! This module defines the rules for enum items in the natlint linter.

use solang_parser::pt::EnumDefinition;

crate::define_rule!(ContractRule, EnumDefinition);
