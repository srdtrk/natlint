//! This module defines the rules for function items in the natlint linter.

use solang_parser::pt::{
    ContractTy, FunctionAttribute, FunctionDefinition, FunctionTy, Visibility,
};

use crate::parser::{CommentsRef, ParserContext};

use super::{Rule, Violation};

/// This rule requires that all public functions have a inheritdoc comment.
pub struct RequireInheritdoc;

impl Rule<FunctionDefinition> for RequireInheritdoc {
    const NAME: &'static str = "Require Inheritdoc";
    const DESCRIPTION: &'static str =
        "All public and override functions must have an inheritdoc comment.";

    fn check(
        &self,
        ctx: &ParserContext,
        func: &FunctionDefinition,
        comments: CommentsRef,
    ) -> Option<Violation> {
        // Parent must be a contract, not an interface or library
        match ctx.parent.as_ref()?.as_contract()?.ty {
            ContractTy::Interface(_) | ContractTy::Library(_) => return None,
            ContractTy::Contract(_) | ContractTy::Abstract(_) => (),
        };

        // Function must not be a modifier or constructor
        match func.ty {
            FunctionTy::Function => (),
            FunctionTy::Receive
            | FunctionTy::Fallback
            | FunctionTy::Constructor
            | FunctionTy::Modifier => return None,
        }

        // Function must be public, external, or an override
        func.attributes.iter().find(|attr| match attr {
            FunctionAttribute::Visibility(Visibility::Public(_) | Visibility::External(_))
            | FunctionAttribute::Override(..) => true,
            FunctionAttribute::Visibility(Visibility::Private(_) | Visibility::Internal(_))
            | FunctionAttribute::Mutability(_)
            | FunctionAttribute::Virtual(_)
            | FunctionAttribute::Immutable(_)
            | FunctionAttribute::BaseOrModifier(..)
            | FunctionAttribute::Error(_) => false,
        })?;

        // Function must have an inheritdoc comment
        if comments.find_inheritdoc_base().is_none() {
            return Some(Violation::new(Self::NAME, Self::DESCRIPTION, func.loc));
        }

        None
    }
}
