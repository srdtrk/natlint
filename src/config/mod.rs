//! Configuration for natlint rules.
//!
//! This module provides configuration for natlint rules, including loading default
//! rules and applying them to parsed Solidity items.

use std::any::Any;
use std::sync::Arc;

use crate::parser::{Comments, CommentsRef, ParseItem};
use crate::rules::{Rule, Violation};

/// A trait object that can check any parseable item
pub trait AnyRule: Send + Sync {
    /// Check if this rule applies to the given item
    fn applies_to(&self, item: &dyn Any) -> bool;
    
    /// Run the rule check on the given item if applicable
    fn check_item(&self, parent: Option<&ParseItem>, item: &dyn Any, comments: CommentsRef) -> Option<Violation>;
    
    /// Get the name of the rule
    fn name(&self) -> &'static str;
    
    /// Get the description of the rule
    fn description(&self) -> &'static str;
}

/// A wrapper to make Rule<T> implementors work with AnyRule
struct RuleWrapper<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> {
    _phantom: std::marker::PhantomData<T>,
    _rule: std::marker::PhantomData<R>,
}

impl<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> RuleWrapper<T, R> {
    /// Create a new rule wrapper
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            _rule: std::marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> AnyRule for RuleWrapper<T, R> {
    fn applies_to(&self, item: &dyn Any) -> bool {
        item.downcast_ref::<T>().is_some()
    }
    
    fn check_item(&self, parent: Option<&ParseItem>, item: &dyn Any, comments: CommentsRef) -> Option<Violation> {
        if let Some(concrete_item) = item.downcast_ref::<T>() {
            R::check(parent, concrete_item, comments)
        } else {
            None
        }
    }
    
    fn name(&self) -> &'static str {
        R::NAME
    }
    
    fn description(&self) -> &'static str {
        R::DESCRIPTION
    }
}

/// Configuration for natlint rules
pub struct Config {
    /// The collection of rules to apply
    rules: Vec<Arc<dyn AnyRule>>,
}

impl Config {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
    
    /// Add a rule to the configuration
    pub fn add_rule<T: 'static + Send + Sync, R: Rule<T> + Send + Sync + 'static>(&mut self) -> &mut Self {
        let rule = Arc::new(RuleWrapper::<T, R>::new());
        self.rules.push(rule);
        self
    }
    
    /// Check an item against all applicable rules
    pub fn check_item(&self, parent: Option<&ParseItem>, item: &dyn Any, comments: &Comments) -> Vec<Violation> {
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
pub fn load_default_config() -> Config {
    use crate::rules::{
        contract::{
            self as contract_rules
        },
        function::{
            self as function_rules
        },
        r#struct::{
            self as struct_rules
        }
    };
    use solang_parser::pt::{ContractDefinition, FunctionDefinition, StructDefinition};

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
    
    // Function Rules
    config.add_rule::<FunctionDefinition, function_rules::MissingInheritdoc>();
    config.add_rule::<FunctionDefinition, function_rules::MissingNotice>();
    config.add_rule::<FunctionDefinition, function_rules::MissingParams>();
    config.add_rule::<FunctionDefinition, function_rules::MissingReturn>();
    config.add_rule::<FunctionDefinition, function_rules::NoAuthor>();
    config.add_rule::<FunctionDefinition, function_rules::NoTitle>();
    config.add_rule::<FunctionDefinition, function_rules::OnlyInheritdoc>();
    
    // Struct Rules
    config.add_rule::<StructDefinition, struct_rules::MissingAuthor>();
    config.add_rule::<StructDefinition, struct_rules::MissingNotice>();
    config.add_rule::<StructDefinition, struct_rules::MissingParams>();
    config.add_rule::<StructDefinition, struct_rules::MissingTitle>();
    config.add_rule::<StructDefinition, struct_rules::NoInheritdoc>();
    config.add_rule::<StructDefinition, struct_rules::NoReturn>();
    config.add_rule::<StructDefinition, struct_rules::TooManyNotice>();
    config.add_rule::<StructDefinition, struct_rules::TooManyTitle>();
    
    config
}

/// Load configuration from a file or use defaults
pub fn load_config(config_path: &str) -> Config {
    // In a real implementation, this would parse TOML/YAML/JSON configuration
    // For now, we'll just use the default configuration
    if config_path.is_empty() {
        return load_default_config();
    }
    
    // TODO: Parse config file and selectively enable rules
    load_default_config()
}