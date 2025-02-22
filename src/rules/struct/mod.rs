//! This module defines the rules for struct items in the natlint linter.

use solang_parser::pt::StructDefinition;

crate::define_rule!(ContractRule, StructDefinition);
