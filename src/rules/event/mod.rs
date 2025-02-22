//! This module defines the rules for event items in the natlint linter.

use solang_parser::pt::EventDefinition;

crate::define_rule!(ContractRule, EventDefinition);
