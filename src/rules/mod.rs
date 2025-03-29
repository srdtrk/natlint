//! This module defines the rules for the natlint linter.

use solang_parser::pt::Loc;
use violation_error::ViolationError;

use crate::parser::{CommentsRef, ParseItem};

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
