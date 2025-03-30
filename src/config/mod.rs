//! Configuration for natlint rules.
//!
//! This module provides configuration for natlint rules, including loading default
//! rules and applying them to parsed Solidity items.

use crate::parser::{Comments, CommentsRef, ParseItem};
use crate::rules::{
    contract::{self as contract_rules},
    error::{self as error_rules},
    function::{self as function_rules},
    r#enum::{self as enum_rules},
    r#struct::{self as struct_rules},
    variable::{self as variable_rules},
};
use crate::rules::{DynRule, Rule, Violation};
use solang_parser::pt::{
    ContractDefinition, EnumDefinition, ErrorDefinition, FunctionDefinition, StructDefinition,
    VariableDefinition,
};
use std::any::{Any, TypeId};

/// Configuration for natlint rules
pub struct Config {
    /// A vector containing all registered rules.
    rules: Vec<Box<dyn DynRule>>,
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
    pub fn add_rule<T: Any + Send + Sync, R: Rule<T> + Send + Sync + 'static>(
        &mut self,
    ) -> &mut Self {
        self.rules.push(Box::new(R {})); // Assuming R is a ZST or has a Default impl
        self
    }

    /// Check an item against all applicable rules
    pub fn check_item(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: &Comments,
    ) -> Vec<Violation> {
        let item_type_id = item.type_id();
        let comments_ref = CommentsRef::from(comments); // Create CommentsRef once

        self.rules
            .iter()
            .filter(|rule| rule.target_type_id() == item_type_id)
            .filter_map(|rule| rule.check_dyn(parent, item, comments_ref.clone()))
            .collect()
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
