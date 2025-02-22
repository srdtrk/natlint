//! This module defines the rules for function items in the natlint linter.

use solang_parser::pt::FunctionDefinition;

crate::define_rule!(ContractRule, FunctionDefinition);
