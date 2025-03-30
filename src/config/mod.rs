//! Configuration for natlint rules.
//!
//! This module provides configuration for natlint rules, including loading default
//! rules and applying them to parsed Solidity items.

use std::iter;

use serde::{Deserialize, Serialize};

use crate::rules::{
    contract::ContractRulesConfig, error::ErrorRulesConfig, function::FunctionRulesConfig,
    r#enum::EnumRulesConfig, r#struct::StructRulesConfig, variable::VariableRulesConfig, DynRule,
};

/// Configuration for natlint rules
#[derive(Serialize, Deserialize, Default)]
#[allow(missing_docs)]
pub struct Config {
    pub contract_rules: ContractRulesConfig,
    pub error_rules: ErrorRulesConfig,
    pub function_rules: FunctionRulesConfig,
    pub enum_rules: EnumRulesConfig,
    pub struct_rules: StructRulesConfig,
    pub variable_rules: VariableRulesConfig,
}

impl Config {
    /// Returns an iterator over the rules in the configuration.
    #[must_use]
    pub fn rules(&self) -> Vec<Box<dyn DynRule>> {
        iter::empty()
            .chain(self.contract_rules.rule_set())
            .chain(self.error_rules.rule_set())
            .chain(self.function_rules.rule_set())
            .chain(self.enum_rules.rule_set())
            .chain(self.struct_rules.rule_set())
            .chain(self.variable_rules.rule_set())
            .collect()
    }
}
