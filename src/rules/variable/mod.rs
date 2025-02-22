//! This module defines the rules for variable items in the natlint linter.

use solang_parser::pt::VariableDefinition;

crate::define_rule!(ContractRule, VariableDefinition);
