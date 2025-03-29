//! This module defines the rules for the natlint linter.

use std::{any::Any, sync::Arc};

use solang_parser::pt::Loc;
use violation_error::ViolationError;

use crate::parser::{Comments, CommentsRef, ParseItem};

pub mod macros;
pub mod violation_error;

pub mod contract;
pub mod r#enum;
pub mod error;
pub mod event;
pub mod function;
pub mod r#struct;
pub mod r#type;
pub mod variable;

/// A lint diagnostic.
#[derive(Debug, PartialEq, Eq)]
pub struct Violation {
    /// The rule that was violated.
    pub rule_name: &'static str,
    /// The description of the rule violation.
    pub rule_description: &'static str,
    /// A message describing the violation.
    pub error: ViolationError,
    /// The location of the violation.
    pub loc: Loc,
}

/// A wrapper to make Rule<T> implementors work with `AnyRule`
#[derive(Default)]
pub struct RuleWrapper<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> {
    _phantom: std::marker::PhantomData<T>,
    _rule: std::marker::PhantomData<R>,
}

/// A trait for defining a rule that checks a specific Solidity construct.
/// The rule should return a diagnostic if the construct violates the rule.
pub trait Rule<T> {
    /// The name of the rule.
    const NAME: &'static str;
    /// A description of the rule.
    const DESCRIPTION: &'static str;

    /// Check the construct for violations of this rule.
    fn check(parent: Option<&ParseItem>, item: &T, comments: CommentsRef) -> Option<Violation>;
}

/// A set of rules that can be applied to a parseable item.
pub trait RuleSet {
    /// Check the item against all applicable rules.
    fn check(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: &Comments,
    ) -> Vec<Violation>;
}

/// A trait object that can check any parseable item
pub trait AnyRule: Send + Sync {
    /// Check if this rule applies to the given item
    fn applies_to(&self, item: &dyn Any) -> bool;

    /// Run the rule check on the given item if applicable
    fn check_item(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: CommentsRef,
    ) -> Option<Violation>;

    /// Get the name of the rule
    fn name(&self) -> &'static str;

    /// Get the description of the rule
    fn description(&self) -> &'static str;
}

impl Violation {
    /// Create a new violation.
    #[must_use]
    pub const fn new(
        rule_name: &'static str,
        rule_description: &'static str,
        error: ViolationError,
        loc: Loc,
    ) -> Self {
        Self {
            rule_name,
            rule_description,
            error,
            loc,
        }
    }
}

impl<T: 'static + Send + Sync, R: Rule<T> + Send + Sync> RuleWrapper<T, R> {
    /// Create new `RuleWrapper`
    #[must_use]
    pub const fn new() -> Self {
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

    fn check_item(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: CommentsRef,
    ) -> Option<Violation> {
        item.downcast_ref::<T>()
            .and_then(|concrete_item| R::check(parent, concrete_item, comments))
    }

    fn name(&self) -> &'static str {
        R::NAME
    }

    fn description(&self) -> &'static str {
        R::DESCRIPTION
    }
}

impl RuleSet for Vec<Arc<dyn AnyRule>> {
    fn check(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: &Comments,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();

        for rule in self {
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
