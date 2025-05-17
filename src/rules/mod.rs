//! This module defines the rules for the natlint linter.

use std::any::{Any, TypeId};

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
    /// The description of the rule that was violated.
    pub rule_description: &'static str,
    /// A message describing the violation.
    pub error: ViolationError,
    /// The location of the violation.
    pub loc: Loc,
}

/// A trait for defining a rule that checks a specific Solidity construct.
/// The rule should return a diagnostic if the construct violates the rule.
pub trait Rule {
    /// The type of the construct that this rule checks.
    type Target: Any;
    /// The name of the rule.
    const NAME: &'static str;
    /// A description of the rule.
    const DESCRIPTION: &'static str;

    /// Check the construct for violations of this rule.
    fn check(
        parent: Option<&ParseItem>,
        item: &Self::Target,
        comments: &CommentsRef,
    ) -> Option<Violation>;
}

/// A dynamic version of the [`Rule`] trait.
pub trait DynRule {
    /// The name of the rule.
    fn name(&self) -> &'static str;
    /// A description of the rule.
    fn description(&self) -> &'static str;
    /// The `TypeId` of the construct this rule checks.
    fn target_type_id(&self) -> TypeId;
    /// Check the construct for violations of this rule.
    fn check_dyn(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: &CommentsRef,
    ) -> Option<Violation>;
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

impl<R: Rule> DynRule for R
where
    R::Target: Any,
{
    fn name(&self) -> &'static str {
        R::NAME
    }

    fn description(&self) -> &'static str {
        R::DESCRIPTION
    }

    fn target_type_id(&self) -> TypeId {
        TypeId::of::<R::Target>()
    }

    fn check_dyn(
        &self,
        parent: Option<&ParseItem>,
        item: &dyn Any,
        comments: &CommentsRef,
    ) -> Option<Violation> {
        let item = item
            .downcast_ref::<R::Target>()
            .expect("Item type mismatch");
        R::check(parent, item, comments)
    }
}
