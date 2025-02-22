//! This module defines the rules for contract items in the natlint linter.

use solang_parser::pt::ContractDefinition;

crate::define_rule!(ContractRule, ContractDefinition);
