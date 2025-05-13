//! Configuration for natlint rules.
//!
//! This module provides configuration for natlint rules, including loading default
//! rules and applying them to parsed Solidity items.
use std::{fs, iter, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use toml;

use crate::rules::{contract::ContractRulesConfig, DynRule}; // Add this line

/// Errors that can occur during configuration processing.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error reading the configuration file.
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    /// Error parsing the TOML configuration.
    #[error("Failed to parse TOML config: {0}")]
    TomlParse(#[from] toml::de::Error),
}

/// Configuration for natlint rules
#[derive(Serialize, Deserialize, Default, Debug)]
#[allow(missing_docs)]
pub struct Config {
    #[serde(default)]
    pub contract_rules: ContractRulesConfig,
    #[serde(default)]
    pub error_rules: ErrorRulesConfig,
    #[serde(default)]
    pub function_rules: FunctionRulesConfig,
    #[serde(default)]
    pub enum_rules: EnumRulesConfig,
    #[serde(default)]
    pub struct_rules: StructRulesConfig,
    #[serde(default)]
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

    /// Load configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if the file cannot be read or parsed.
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
