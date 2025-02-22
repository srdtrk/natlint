//! This module defines the rules for the natlint linter.

pub mod contract;
pub mod r#enum;
pub mod error;
pub mod event;
pub mod function;
pub mod r#struct;
pub mod r#type;
pub mod variable;

/// A lint diagnostic.
pub struct LintDiagnostic {
    /// The rule that was violated.
    pub rule: &'static str,
    /// A message describing the violation.
    pub message: String,
    /// The line number where the violation occurred.
    pub line: usize,
}

/// Macro to generate a rule trait for different Solidity constructs.
#[macro_export]
macro_rules! define_rule {
    ($rule_name:ident, $target_type:ty) => {
        /// A trait for defining a rule that checks a specific Solidity construct.
        #[allow(clippy::module_name_repetitions)]
        pub trait $rule_name {
            /// The name of the rule.
            const NAME: &'static str;
            /// A description of the rule.
            const DESCRIPTION: &'static str;

            /// Check the construct for violations of this rule.
            /// Returns a diagnostic if the rule is violated.
            fn check(&self, item: &$target_type) -> Option<$crate::rules::LintDiagnostic>;
        }
    };
}
