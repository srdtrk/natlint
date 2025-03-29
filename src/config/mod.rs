//! Configuration for natlint rules.
//!
//! This module provides configuration for natlint rules, including loading default
//! rules and applying them to parsed Solidity items.

use std::any::Any;
use std::sync::Arc;

use crate::parser::{Comments, CommentsRef, ParseItem};
use crate::rules::{
    contract::{self as contract_rules},
    error::{self as error_rules},
    function::{self as function_rules},
    r#enum::{self as enum_rules},
    r#struct::{self as struct_rules},
    variable::{self as variable_rules},
};
use crate::rules::{AnyRule, Rule, RuleWrapper, Violation};
use solang_parser::pt::{
    ContractDefinition, EnumDefinition, ErrorDefinition, FunctionDefinition, StructDefinition,
    VariableDefinition,
};

/// Configuration for natlint rules
pub struct Config {
    /// The collection of rules to apply
    rules: Vec<Arc<dyn AnyRule>>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Create a new empty configuration
    #[must_use]
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the configuration
    pub fn add_rule<T: 'static + Send + Sync, R: Rule<T> + Send + Sync + 'static>(
        &mut self,
    ) -> &mut Self {
        let rule = Arc::new(RuleWrapper::<T, R>::new());
        self.rules.push(rule);
        self
    }

    /// Check an item against all applicable rules
    pub fn check_item(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: &Comments,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();

        for rule in &self.rules {
            if rule.applies_to(item) {
                let comments_ref = CommentsRef::from(comments);
                if let Some(violation) = rule.check_item(parent, item, comments_ref) {
                    violations.push(violation);
                }
            }
        }

        violations
    }
}

/// Create a configuration with all available rules
#[must_use]
pub fn load_default_config() -> Config {
    let mut config = Config::new();

    // Contract Rules
    config.add_rule::<ContractDefinition, contract_rules::MissingAuthor>();
    config.add_rule::<ContractDefinition, contract_rules::MissingNotice>();
    config.add_rule::<ContractDefinition, contract_rules::MissingTitle>();
    config.add_rule::<ContractDefinition, contract_rules::NoInheritdoc>();
    config.add_rule::<ContractDefinition, contract_rules::NoParam>();
    config.add_rule::<ContractDefinition, contract_rules::NoReturn>();
    config.add_rule::<ContractDefinition, contract_rules::TooManyNotice>();
    config.add_rule::<ContractDefinition, contract_rules::TooManyTitle>();

    // Enum Rules
    config.add_rule::<EnumDefinition, enum_rules::MissingAuthor>();
    config.add_rule::<EnumDefinition, enum_rules::MissingNotice>();
    config.add_rule::<EnumDefinition, enum_rules::MissingTitle>();
    config.add_rule::<EnumDefinition, enum_rules::MissingVariant>();
    config.add_rule::<EnumDefinition, enum_rules::NoInheritdoc>();
    config.add_rule::<EnumDefinition, enum_rules::NoParam>();
    config.add_rule::<EnumDefinition, enum_rules::NoReturn>();
    config.add_rule::<EnumDefinition, enum_rules::TooManyNotice>();
    config.add_rule::<EnumDefinition, enum_rules::TooManyTitle>();

    // Error Rules
    config.add_rule::<ErrorDefinition, error_rules::MissingNotice>();
    config.add_rule::<ErrorDefinition, error_rules::MissingParam>();
    config.add_rule::<ErrorDefinition, error_rules::NoAuthor>();
    config.add_rule::<ErrorDefinition, error_rules::NoInheritdoc>();
    config.add_rule::<ErrorDefinition, error_rules::NoReturn>();
    config.add_rule::<ErrorDefinition, error_rules::NoTitle>();
    config.add_rule::<ErrorDefinition, error_rules::TooManyNotice>();

    // Function Rules
    config.add_rule::<FunctionDefinition, function_rules::MissingInheritdoc>();
    config.add_rule::<FunctionDefinition, function_rules::MissingNotice>();
    config.add_rule::<FunctionDefinition, function_rules::MissingParams>();
    config.add_rule::<FunctionDefinition, function_rules::MissingReturn>();
    config.add_rule::<FunctionDefinition, function_rules::NoAuthor>();
    config.add_rule::<FunctionDefinition, function_rules::NoTitle>();
    config.add_rule::<FunctionDefinition, function_rules::OnlyInheritdoc>();
    config.add_rule::<FunctionDefinition, function_rules::TooManyInheritdoc>();
    config.add_rule::<FunctionDefinition, function_rules::TooManyNotice>();

    // Struct Rules
    config.add_rule::<StructDefinition, struct_rules::MissingAuthor>();
    config.add_rule::<StructDefinition, struct_rules::MissingNotice>();
    config.add_rule::<StructDefinition, struct_rules::MissingParams>();
    config.add_rule::<StructDefinition, struct_rules::MissingTitle>();
    config.add_rule::<StructDefinition, struct_rules::NoInheritdoc>();
    config.add_rule::<StructDefinition, struct_rules::NoReturn>();
    config.add_rule::<StructDefinition, struct_rules::TooManyNotice>();
    config.add_rule::<StructDefinition, struct_rules::TooManyTitle>();

    // Variable Rules
    config.add_rule::<VariableDefinition, variable_rules::MissingInheritdoc>();
    config.add_rule::<VariableDefinition, variable_rules::MissingNotice>();
    config.add_rule::<VariableDefinition, variable_rules::NoAuthor>();
    config.add_rule::<VariableDefinition, variable_rules::NoParam>();
    config.add_rule::<VariableDefinition, variable_rules::NoReturn>();
    config.add_rule::<VariableDefinition, variable_rules::NoTitle>();
    config.add_rule::<VariableDefinition, variable_rules::TooManyInheritdoc>();
    config.add_rule::<VariableDefinition, variable_rules::TooManyNotice>();

    config
}

/// Load configuration from a file or use defaults
#[must_use]
pub fn load_config(config_path: &str) -> Config {
    // In a real implementation, this would parse TOML/YAML/JSON configuration
    // For now, we'll just use the default configuration
    if config_path.is_empty() {
        return load_default_config();
    }

    // TODO: Parse config file and selectively enable rules
    load_default_config()
}
